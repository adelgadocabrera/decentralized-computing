pub mod blockchain;
pub mod networking;
pub mod utils;

pub mod protos {
    use std::fmt;

    use prost::Message;
    use serde::{ser::SerializeStruct, Serialize, Serializer};

    tonic::include_proto!("protos");

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
