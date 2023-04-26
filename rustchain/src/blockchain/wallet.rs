use crate::event_bus::event_bus::EventBus;
use crate::event_bus::events::RustchainEvent;
use crate::protos::{Block, Transaction};
use crate::protos::{UtxoInput, UtxoOutput};
use openssl::hash::hash;
use openssl::sign::Verifier;
use openssl::{
    error::ErrorStack,
    hash::MessageDigest,
    pkey::{PKey, Private, Public},
    rsa::Rsa,
    sign::Signer,
};
use ripemd::{Digest, Ripemd160};
use sha2::Sha256;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::spawn;
use tokio::sync::mpsc::Receiver;
use tokio::sync::RwLock;

#[derive(Clone, Debug)]
pub struct Wallet {
    address: String,
    private_key: PKey<Private>,
    public_key: PKey<Public>,
    utxos: HashMap<(String, u32), UtxoOutput>,
    event_bus: Arc<RwLock<EventBus>>,
}

impl Wallet {
    pub async fn new(event_bus: Arc<RwLock<EventBus>>) -> Arc<RwLock<Wallet>> {
        let rsa = Rsa::generate(2048).expect("Failed to generate RSA key pair");
        let private_key = PKey::from_rsa(rsa.clone()).expect("Failed to create private key");
        let public_key = PKey::from_rsa(
            rsa.public_key_to_pem()
                .and_then(|public_pem| Rsa::public_key_from_pem(&public_pem))
                .expect("Failed to create public RSA"),
        )
        .expect("Failed to create public key");

        let address =
            Wallet::compute_address(&public_key).expect("Failed to compute wallet address");

        let wallet = Wallet {
            address,
            private_key,
            public_key,
            utxos: HashMap::new(),
            event_bus: event_bus.clone(),
        };
        let wallet_arc = Arc::new(RwLock::new(wallet));
        let wallet_clone = wallet_arc.clone();
        let event_receiver: Receiver<RustchainEvent> = event_bus.write().await.subscribe().await;
        spawn(async move {
            Wallet::listen_for_events(wallet_clone.clone(), event_receiver).await;
        });
        wallet_arc
    }

    async fn listen_for_events(
        wallet: Arc<RwLock<Wallet>>,
        mut event_receiver: Receiver<RustchainEvent>,
    ) {
        while let Some(event) = event_receiver.recv().await {
            match event {
                RustchainEvent::NewBlock(block) => {
                    Wallet::on_block_received(wallet.clone(), block).await;
                }
                RustchainEvent::NewTransaction(tx) => {
                    Wallet::on_tx_received(wallet.clone(), tx).await;
                }
                _ => {
                    unimplemented!();
                }
            }
        }
    }

    async fn on_block_received(_: Arc<RwLock<Wallet>>, _: Block) {}

    async fn on_tx_received(wallet: Arc<RwLock<Wallet>>, tx: Transaction) {
        let mut w = wallet.write().await;
        for (index, utxo_output) in tx.clone().outputs.into_iter().enumerate() {
            if utxo_output.to_addr != w.address {
                continue;
            }
            let tx_hash = hex::encode(tx.hash());
            w.utxos.insert((tx_hash, index as u32), utxo_output.clone());
        }
    }

    fn compute_address(public_key: &PKey<Public>) -> Result<String, Box<dyn std::error::Error>> {
        let pem_data = public_key.public_key_to_pem()?;
        let sha256_hash = hash(MessageDigest::sha256(), &pem_data)?;

        let mut ripemd_hasher = Ripemd160::new();
        ripemd_hasher.update(&sha256_hash);
        let ripemd_hash = ripemd_hasher.finalize();

        // Use a network identifier byte if needed (e.g., 0x00 for Bitcoin mainnet)
        // In this example, we'll skip the network identifier byte.
        let checksum =
            Sha256::digest(&Sha256::digest(ripemd_hash.as_slice()).to_vec()).to_vec()[..4].to_vec();
        let mut extended_data = Vec::with_capacity(ripemd_hash.len() + 4);
        extended_data.extend_from_slice(ripemd_hash.as_slice());
        extended_data.extend_from_slice(&checksum[..]);
        let address = bs58::encode(extended_data).into_string();

        Ok(address)
    }

    pub async fn air_drop(&self, amount: u32) {
        let mut tx = Transaction::default();
        let utxo_output = UtxoOutput {
            to_addr: self.address.clone(),
            amount,
        };
        tx.outputs.push(utxo_output);
        self.event_bus
            .read()
            .await
            .publish(RustchainEvent::NewTransaction(tx))
            .await;
    }

    pub fn sign_transaction(
        &self,
        input: &mut UtxoInput,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut signer = Signer::new(MessageDigest::sha256(), &self.private_key)?;
        signer.update(&input.hashable_content().unwrap())?;
        let signature = signer.sign_to_vec()?;
        input.set_signature(signature);
        Ok(())
    }

    pub async fn send_transaction(
        &mut self,
        to_addr: String,
        amount: u32,
    ) -> Result<Transaction, Box<dyn std::error::Error>> {
        if self.get_balance() < amount {
            return Err("Not enough balance".into());
        }

        let mut tx = Transaction::default();
        let mut total_utxo_value: u32 = 0;
        let mut used_utxos: Vec<(String, u32)> = vec![];

        for (utxo_key, utxo) in self.utxos.iter() {
            // Create a new Utxo input using the selected UTXO
            let (prev_tx_hash, output_index) = utxo_key;
            let mut input = UtxoInput {
                from_addr: self.address.clone(),
                public_key: self.public_key.public_key_to_pem()?,
                prev_tx_hash: prev_tx_hash.clone().into_bytes(),
                output_index: *output_index,
                signature: vec![],
            };
            self.sign_transaction(&mut input)?;
            tx.inputs.push(input);
            used_utxos.push((prev_tx_hash.clone(), *output_index));
            // Added to list of utxo's that should be removed on transaction complete

            total_utxo_value += utxo.amount;
            if total_utxo_value >= amount {
                break;
            }
        }
        // Add the output for the recipient
        tx.outputs.push(UtxoOutput { to_addr, amount });

        // Add change output if necessary
        if total_utxo_value > amount {
            let change = total_utxo_value - amount;
            tx.outputs.push(UtxoOutput {
                to_addr: self.address.clone(),
                amount: change,
            });
        }

        // wait for tx to be dispatched
        self.event_bus
            .read()
            .await
            .publish(RustchainEvent::NewTransaction(tx.clone()))
            .await;

        // if tx OK then remove used utxo's
        for utxo in used_utxos {
            self.utxos.remove(&utxo);
        }
        Ok(tx)
    }

    pub fn verify_transaction_signature(
        transaction: &Transaction,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut is_verified = true;
        for input in &transaction.inputs {
            // Deserialize the public key from the input
            let public_key = PKey::public_key_from_pem(&input.public_key)?;
            // Verify the signature using the public key
            let mut verifier = Verifier::new(MessageDigest::sha256(), &public_key)?;
            verifier.update(&transaction.hashable_content().unwrap())?;
            is_verified &= verifier.verify(&input.signature)?;
        }
        Ok(is_verified)
    }

    pub fn get_balance(&self) -> u32 {
        self.utxos.values().map(|x| x.amount).sum()
    }

    pub fn get_public_key(&self) -> PKey<Public> {
        self.public_key.clone()
    }

    pub fn get_address(&self) -> String {
        return self.address.clone();
    }

    pub fn public_key_string(&self) -> Result<String, ErrorStack> {
        let pem_data = self.public_key.public_key_to_pem()?;
        let pem_string = std::str::from_utf8(&pem_data).unwrap();
        Ok(pem_string.to_owned())
    }
}

#[cfg(test)]
pub mod tests {
    use std::time::Duration;

    use tokio::time::sleep;

    use super::*;

    #[tokio::test]
    async fn not_enough_balance() {
        let event_bus = EventBus::new().await;
        let bob = Wallet::new(event_bus.clone()).await;
        let alice = Wallet::new(event_bus.clone()).await;
        let alice_key = alice.read().await.public_key_string().unwrap();
        assert!(matches!(
            bob.write().await.send_transaction(alice_key, 500).await,
            Err(_)
        ));
    }

    #[tokio::test]
    async fn test_air_drop() {
        let amount = 500;
        let event_bus = EventBus::new().await;
        let bob = Wallet::new(event_bus).await;
        bob.read().await.air_drop(amount).await;
        sleep(Duration::from_millis(100)).await;
        let balance = bob.read().await.get_balance();
        assert_eq!(amount, balance);
    }

    #[tokio::test]
    async fn test_transaction() {
        let event_bus = EventBus::new().await;
        let alice = Wallet::new(event_bus.clone()).await;
        let alice_addr = alice.read().await.address.clone();
        let bob = Wallet::new(event_bus.clone()).await;
        bob.read().await.air_drop(1000).await;
        sleep(Duration::from_millis(100)).await;
        let _ = bob.write().await.send_transaction(alice_addr, 500).await;
        sleep(Duration::from_millis(100)).await;
        assert_eq!(500, bob.read().await.get_balance());
        assert_eq!(500, alice.read().await.get_balance());
    }
}
