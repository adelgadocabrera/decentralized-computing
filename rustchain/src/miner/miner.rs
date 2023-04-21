use super::block::BlockHeader;
use super::wallet::Wallet;
use crate::event_bus::event_bus::EventBus;
use crate::event_bus::events::BlockchainEvent;
use crate::protos::Transaction;
use openssl::hash::MessageDigest;
use openssl::pkey::{PKey, Public};
use openssl::sign::Verifier;
use sha2::{Digest, Sha256};
use tokio::runtime::Handle;
use tokio::spawn;
use tokio::sync::mpsc::Receiver;
use std::io::ErrorKind;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;
use std::io::Error;

#[derive(Debug, Clone)]
pub struct Miner {
    wallet: Wallet,
    pending_transactions: Arc<RwLock<Vec<Transaction>>>,
    event_bus: Arc<RwLock<EventBus>>,
}
// mining_reward: f64, do I need to pass mining 
// reward or should it be notified by the system

impl Miner {
    pub fn new(event_bus: Arc<RwLock<EventBus>>) -> Arc<RwLock<Miner>> {
        let miner = Miner{
            wallet: Wallet::new(),
            pending_transactions: Arc::new(RwLock::new(vec![])),
            event_bus: event_bus.clone(),
        };
        let miner_arc = Arc::new(RwLock::new(miner));
        let handle = Handle::current();
        handle.block_on(async { 
            let event_receiver = event_bus.write().await.subscribe().await;
            let miner_clone = miner_arc.clone();
            spawn(async move {
                miner_clone
                    .write()
                    .await
                    .listen_for_events(event_receiver)
                    .await;
            });
        });
        miner_arc
    }

    async fn listen_for_events(&mut self, mut event_receiver: Receiver<BlockchainEvent>) {
        while let Some(event) = event_receiver.recv().await {
            match event {
                BlockchainEvent::NewBlock(block) => {
                    unimplemented!()
                }
                BlockchainEvent::NewTransaction(transaction) => {
                    unimplemented!()
                }
            }
        }
    }

    pub fn set_wallet(&mut self, wallet: Wallet) {
        self.wallet = wallet;
    }

    fn mine(
        self,
        prev_block_index: u64,
        prev_hash: &Vec<u8>,
        transactions: &Vec<Transaction>,
        difficulty: u64,
    ) -> Result<(Vec<u8>, u64), Box<dyn std::error::Error>> {
        for transaction in transactions {
            let public_key =
                PKey::public_key_from_pem(&transaction.from_addr.as_bytes()).unwrap();
            if !verify_signature(
                &public_key,
                &transaction.signature,
                &transaction.to_bytes().unwrap(),
            ) {
               return Err(Box::new(Error::new(ErrorKind::Other, "Error verifying tx using public key")));
            }
        }
        let mut nonce = 0;
        let mut hash: Vec<u8> = vec![];
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
            // Verify the digital signatures of the transactions

        loop {
            nonce = nonce + 1;
            let header = BlockHeader {
                timestamp,
                nonce,
                difficulty,
                previous_hash: prev_hash.to_owned(),
                block_index: prev_block_index + 1,
                merkle_root: calculate_merkle_root(&transactions.to_owned()),
            };

            // Serialize && Hash the header to a byte array
            let mut hasher = Sha256::new();
            let header_bytes = serde_json::to_string(&header).unwrap();
            hasher.update(&header_bytes);
            hash = hasher.finalize().to_vec();

            if satisfies_difficulty(&hash, header.difficulty.to_owned()) {
                return Ok((hash.to_vec(), nonce));
            }
        }
    }
}

fn verify_signature(public_key: &PKey<Public>, signature: &[u8], message: &[u8]) -> bool {
    let digest = MessageDigest::sha256();
    let mut verifier = Verifier::new(digest, public_key).unwrap();
    verifier.update(message).unwrap();
    verifier.verify(signature).unwrap()
}

// checks whether the hash has at least the difficulty number of leading zeroes
fn satisfies_difficulty(hash: &Vec<u8>, difficulty: u64) -> bool {
    let mut counter = 0;
    for &byte in hash {
        for i in (0..8).rev() {
            if byte & (1 << i) == 0 {
                counter += 1;
            } else {
                if counter >= difficulty {
                    return true;
                } else {
                    return false;
                }
            }
        }
    }
    if counter >= difficulty {
        return true;
    } else {
        return false;
    }
}

// Calculate the Merkle root of the transactions
fn calculate_merkle_root(transactions: &[Transaction]) -> Vec<u8> {
    let mut hashes: Vec<Vec<u8>> = transactions.iter().map(|tx| tx.hash()).collect();
    while hashes.len() > 1 {
        if hashes.len() % 2 != 0 {
            hashes.push(hashes.last().unwrap().clone());
        }
        let mut new_hashes: Vec<Vec<u8>> = vec![];
        for i in (0..hashes.len()).step_by(2) {
            let mut hasher = Sha256::new();
            hasher.update(&hashes[i]);
            hasher.update(&hashes[i + 1]);
            new_hashes.push(hasher.finalize().to_vec());
        }
        hashes = new_hashes;
    }
    hashes[0].clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leading_zeroes_difficulty_12() {
        let hash = vec![0x00, 0x01, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00]; // Three leading zeroes
        assert!(satisfies_difficulty(&hash, 12));
    }

    #[test]
    fn test_leading_zeroes_difficulty_48() {
        let hash = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]; // Eight leading zeroes
        assert!(satisfies_difficulty(&hash, 48));
    }

    #[test]
    fn test_leading_zeroes_difficulty_4() {
        let hash = vec![0x0a, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]; // Eight leading zeroes
        assert!(satisfies_difficulty(&hash, 4));
    }

    #[test]
    fn test_leading_zeroes_difficulty_4_fails() {
        let hash = vec![0b00010000, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]; // Eight leading zeroes
        assert!(!satisfies_difficulty(&hash, 4));
    }

    #[test]
    fn test_leading_zeroes_difficulty_0() {
        let hash = vec![0xa1, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]; // No leading zeroes
        assert!(satisfies_difficulty(&hash, 0));
    }

    // #[test]
    // fn test_proof_of_work() {
    //     let difficulty = 15;
    //     let timestamp = SystemTime::now()
    //         .duration_since(UNIX_EPOCH)
    //         .unwrap()
    //         .as_secs();
    //     let genesis = create_genesis_block();
    //     let mut header = BlockHeader {
    //         timestamp,
    //         previous_hash: genesis.block_hash,
    //         block_index: 1,
    //         merkle_root: vec![],
    //         difficulty,
    //         nonce: 0,
    //     };
    //     let (hash, nonce) = proof_of_work(&mut header);
    //     assert!(satisfies_difficulty(&hash, difficulty));
    //     assert_eq!(nonce, header.nonce);
    // }

    // #[test]
    // fn test_proof_of_work_2_blocks() {
    //     let difficulty = 15;
    //     let mut timestamp = SystemTime::now()
    //         .duration_since(UNIX_EPOCH)
    //         .unwrap()
    //         .as_secs();
    //     let genesis = create_genesis_block();
    //     let mut header_1 = BlockHeader {
    //         timestamp,
    //         previous_hash: genesis.block_hash,
    //         block_index: 1,
    //         merkle_root: vec![],
    //         difficulty,
    //         nonce: 0,
    //     };
    //     let (hash_1, nonce_1) = proof_of_work(&mut header_1);
    //     timestamp = SystemTime::now()
    //         .duration_since(UNIX_EPOCH)
    //         .unwrap()
    //         .as_secs();

    //     let mut header_2 = BlockHeader {
    //         timestamp,
    //         previous_hash: hash_1.to_owned(),
    //         block_index: 2,
    //         merkle_root: vec![],
    //         difficulty,
    //         nonce: 0,
    //     };
    //     let (hash_2, nonce_2) = proof_of_work(&mut header_2);
    //     assert!(satisfies_difficulty(&hash_1, difficulty));
    //     assert_eq!(nonce_1, header_1.nonce);
    //     assert!(satisfies_difficulty(&hash_2, difficulty));
    //     assert_eq!(nonce_2, header_2.nonce);
    // }
}
