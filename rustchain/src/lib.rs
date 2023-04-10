pub mod blockchain;
pub mod networking;
pub mod utils;

pub mod protos {
    use prost::Message;
    use serde::{Serialize, Serializer};

    tonic::include_proto!("protos");

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
}
