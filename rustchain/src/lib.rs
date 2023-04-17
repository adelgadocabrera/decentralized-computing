pub mod blockchain;
pub mod event_bus;
pub mod networking;
pub mod utils;

pub mod protos {
    tonic::include_proto!("protos");
    use prost::Message;
    use serde::{ser::SerializeStruct, Serialize, Serializer};
    use sha2::{Digest, Sha256};
    use std::fmt;
    use tonic::{IntoRequest, Request};

    impl Serialize for BlockHeader {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            let mut state = serializer.serialize_struct("BlockHeader", 6)?;
            state.serialize_field("timestamp", &self.timestamp)?;
            state.serialize_field("nonce", &self.nonce)?;
            state.serialize_field("difficulty", &self.difficulty)?;
            state.serialize_field("previous_hash", &self.previous_hash)?;
            state.serialize_field("block_index", &self.block_index)?;
            state.serialize_field("merkle_root", &self.merkle_root)?;
            state.end()
        }
    }

    impl Transaction {
        pub fn set_signature(&mut self, signature: Vec<u8>) {
            self.signature = signature;
        }

        pub fn hash(&self) -> Vec<u8> {
            let bytes = self.to_bytes().unwrap(); // You might want to handle this error instead of using unwrap
            let mut hasher = Sha256::new();
            hasher.update(&bytes);
            hasher.finalize().to_vec()
        }

        pub fn hashable_content(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
            let mut pb = Transaction::default();
            pb.from_addr = self.from_addr.clone();
            pb.to_addr = self.to_addr.clone();
            pb.amount = self.amount;
            pb.additional_data = self.additional_data.clone();
            // Leave the signature field empty

            let mut buf = Vec::new();
            pb.encode(&mut buf)?;
            Ok(buf)
        }

        pub fn to_bytes(&self) -> Result<Box<[u8]>, Box<dyn std::error::Error>> {
            let mut pb = Transaction::default();
            pb.from_addr = self.from_addr.clone();
            pb.to_addr = self.to_addr.clone();
            pb.amount = self.amount;
            pb.additional_data = self.additional_data.clone();
            pb.signature = self.signature.clone();

            let mut buf = Vec::new();
            pb.encode(&mut buf)?;
            Ok(buf.into_boxed_slice())
        }

        pub fn from_bytes(bytes: &[u8]) -> Result<Transaction, Box<dyn std::error::Error>> {
            let pb = Transaction::decode(bytes)?;
            Ok(Transaction {
                from_addr: pb.from_addr,
                to_addr: pb.to_addr,
                amount: pb.amount,
                additional_data: pb.additional_data,
                signature: pb.signature,
            })
        }
    }

    impl IntoRequest<Result<Box<[u8]>, Box<dyn std::error::Error>>> for Transaction {
        fn into_request(self) -> Request<Result<Box<[u8]>, Box<dyn std::error::Error>>> {
            let bytes = self.to_bytes();
            let request = Request::new(bytes);
            request
        }
    }

    impl Serialize for Transaction {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut buf = Vec::new();
            self.encode(&mut buf).unwrap();
            serde_json::Value::from(buf).serialize(serializer)
        }
    }

    impl fmt::Display for Transaction {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "Transaction {{ from_addr: {}, to_addr: {}, amount: {}, additional_data: {} }}",
                self.from_addr, self.to_addr, self.amount, self.additional_data
            )
        }
    }
}
