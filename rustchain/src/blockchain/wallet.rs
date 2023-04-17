use super::block::Transaction;
use openssl::{
    error::ErrorStack,
    hash::MessageDigest,
    pkey::{PKey, Private, Public},
    rsa::Rsa,
    sign::Signer,
};
use sha2::{Digest, Sha256};

#[derive(Clone, Debug)]
pub struct Wallet {
    private_key: PKey<Private>,
    public_key: PKey<Public>,
}

impl Wallet {
    pub fn new() -> Wallet {
        let rsa = Rsa::generate(2048).expect("Failed to generate RSA key pair");
        let private_key = PKey::from_rsa(rsa.clone()).expect("Failed to create private key");
        let public_key = PKey::from_rsa(
            rsa.public_key_to_pem()
                .and_then(|public_pem| Rsa::public_key_from_pem(&public_pem))
                .expect("Failed to create public RSA"),
        )
        .expect("Failed to create public key");

        Wallet {
            private_key,
            public_key,
        }
    }

    pub fn sign_transaction(&self, transaction: &Transaction) -> Result<Vec<u8>, ErrorStack> {
        // hash tx
        let mut hasher = Sha256::new();
        hasher.update(&transaction.hashable_content().unwrap());
        let digest = hasher.finalize();

        // sign
        let mut signer = Signer::new(MessageDigest::sha256(), &self.private_key)?;
        signer.update(&digest)?;
        let signature = signer.sign_to_vec()?;

        Ok(signature)
    }

    pub fn public_key_string(&self) -> Result<String, ErrorStack> {
        let pem_data = self.public_key.public_key_to_pem()?;
        let pem_string = std::str::from_utf8(&pem_data).unwrap();
        Ok(pem_string.to_owned())
    }

    pub fn public_key(&self) -> PKey<Public> {
        self.public_key.clone()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_signature() {
        let wallet_1 = Wallet::new();
        let wallet_2 = Wallet::new();
        let pk_1 = wallet_1.public_key_string().unwrap();
        let pk_2 = wallet_2.public_key_string().unwrap();

        let mut transaction = Transaction {
            from_addr: pk_1.to_owned(),
            to_addr: pk_2.to_owned(),
            amount: 1,
            additional_data: String::from(""),
            signature: vec![],
        };

        let signature: Vec<u8> = wallet_1.sign_transaction(&transaction).unwrap();
        transaction.set_signature(signature);

        assert_eq!(transaction.from_addr, pk_1);
        assert_eq!(transaction.to_addr, pk_2);
    }
}
