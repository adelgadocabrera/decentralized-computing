tonic::include_proto!("protos");
use prost::Message;
use serde::{ser::SerializeStruct, Serialize, Serializer};
use sha2::{Digest, Sha256};
use std::fmt;
use tonic::{IntoRequest, Request};

impl From<Vec<Peer>> for PeerList {
    fn from(peers_vec: Vec<Peer>) -> Self {
        PeerList { peers: peers_vec }
    }
}

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
        for mut input in &mut self.inputs {
            input.signature = signature.clone();
        }
    }

    // hashes the whole transaction, including the signature
    pub fn hash(&self) -> Vec<u8> {
        let bytes = self.to_bytes().unwrap(); // might want to handle this error instead of using unwrap
        let mut hasher = Sha256::new();
        hasher.update(&bytes);
        hasher.finalize().to_vec()
    }

    pub fn to_bytes(&self) -> Result<Box<[u8]>, Box<dyn std::error::Error>> {
        let mut buf = Vec::new();
        self.encode(&mut buf)?;
        Ok(buf.into_boxed_slice())
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Transaction, Box<dyn std::error::Error>> {
        let decoded = Transaction::decode(bytes)?;
        Ok(decoded)
    }

    // hashes everything except for the signature. In order for the wallet
    // to get this hash and then sign it.
    pub fn hashable_content(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut input_data = Vec::new();
        for input in &self.inputs {
            let input_content = input.hashable_content()?;
            input_data.extend(input_content);
        }

        let mut output_data = Vec::new();
        for output in &self.outputs {
            let output_content = output.hashable_content()?;
            output_data.extend(output_content);
        }

        let mut buf = Vec::new();
        buf.extend(input_data);
        buf.extend(output_data);
        Ok(buf)
    }
}

impl UtxoInput {
    pub fn set_signature(&mut self, signature: Vec<u8>) {
        self.signature = signature.clone();
    }

    pub fn hashable_content(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut pb = UtxoInput {
            from_addr: self.from_addr.clone(),
            public_key: self.public_key.clone(),
            prev_tx_hash: self.prev_tx_hash.clone(),
            output_index: self.output_index,
            signature: vec![], // Leave the signature field empty
        };

        let mut buf = Vec::new();
        pb.encode(&mut buf)?;
        Ok(buf)
    }
}

impl UtxoOutput {
    pub fn hashable_content(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let pb = UtxoOutput {
            to_addr: self.to_addr.clone(),
            amount: self.amount,
        };

        let mut buf = Vec::new();
        pb.encode(&mut buf)?;
        Ok(buf)
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
            "Transaction {{ inputs: {:?}, outputs: {:?} }}",
            self.inputs, self.outputs
        )
    }
}

impl fmt::Display for UtxoInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "UTXOInput {{ prev_tx_hash: {:?}, output_index: {}, signature: {:?} }}",
            self.prev_tx_hash, self.output_index, self.signature
        )
    }
}

impl fmt::Display for UtxoOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "UTXOOutput {{ to_addr: {}, amount: {} }}",
            self.to_addr, self.amount
        )
    }
}

pub struct UtxoInputs(Vec<UtxoInput>);
pub struct UtxoOutputs(Vec<UtxoOutput>);

impl From<Vec<UtxoInput>> for UtxoInputs {
    fn from(inputs: Vec<UtxoInput>) -> Self {
        UtxoInputs(inputs)
    }
}

impl From<Vec<UtxoOutput>> for UtxoOutputs {
    fn from(outputs: Vec<UtxoOutput>) -> Self {
        UtxoOutputs(outputs)
    }
}

impl fmt::Display for UtxoInputs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "UtxoInputs [")?;
        for input in &self.0 {
            writeln!(f, "  {},", input)?;
        }
        write!(f, "]")
    }
}

impl fmt::Display for UtxoOutputs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "UtxoOutputs [")?;
        for output in &self.0 {
            writeln!(f, "  {},", output)?;
        }
        write!(f, "]")
    }
}
