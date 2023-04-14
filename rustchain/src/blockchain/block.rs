pub use crate::protos::{Block, BlockHeader, Transaction};
use hex;
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn create_genesis_block() -> Block {
    let previous_hash = vec![];
    let transactions: Vec<Transaction> = vec![];
    let block_index = 0;
    let merkle_root: [u8; 32] = [0; 32];
    let difficulty: u64 = 9;
    return new_block(
        previous_hash,
        transactions,
        block_index,
        merkle_root,
        difficulty,
    );
}

pub fn next_block(
    last_block: &Block,
    transactions: Vec<Transaction>,
    merkle_root: [u8; 32],
    difficulty: u64,
) -> Block {
    let block_index = last_block.header.as_ref().unwrap().block_index + 1;
    let previous_hash = last_block.to_owned().block_hash;
    return new_block(
        previous_hash,
        transactions,
        block_index,
        merkle_root,
        difficulty,
    );
}

fn new_block(
    previous_hash: Vec<u8>,
    transactions: Vec<Transaction>,
    block_index: u64,
    merkle_root: [u8; 32],
    difficulty: u64,
) -> Block {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let block_header = BlockHeader {
        timestamp,
        previous_hash,
        block_index,
        merkle_root: merkle_root.try_into().unwrap(),
        difficulty,
        nonce: 0,
    };

    let mut block = Block::default();
    block.header = Some(block_header);
    block.transactions = transactions;
    block.block_hash = block.hash_block();
    return block;
}

impl Block {
    fn hash_block(&self) -> Vec<u8> {
        let header = self.header.to_owned().unwrap();
        let mut hasher = Sha256::new();
        hasher.update(header.timestamp.to_be_bytes());
        hasher.update(header.difficulty.to_be_bytes());
        hasher.update(header.nonce.to_be_bytes());
        hasher.update(header.previous_hash);
        hasher.update(header.block_index.to_be_bytes());
        hasher.update(header.merkle_root);
        hasher.finalize().to_vec()
    }
}

impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let header = self.header.to_owned().unwrap();
        let shrunk_previous_hash = self
            .header
            .to_owned()
            .unwrap()
            .previous_hash
            .iter()
            .cloned()
            .take(10)
            .collect::<Vec<u8>>();

        let shrunk_hash = self
            .block_hash
            .iter()
            .cloned()
            .take(10)
            .collect::<Vec<u8>>();

        let shrunk_merkle_root = header
            .merkle_root
            .iter()
            .cloned()
            .take(10)
            .collect::<Vec<u8>>();

        let title: String;
        if header.block_index == 0 {
            title = String::from(
                "
Genesis Block
-------------",
            );
        } else {
            title = String::from(format!(
                "
Block {}
--------",
                header.block_index
            ));
        }
        let block = format!(
            "
{}
Header:
    timestamp:          {},
    nonce:              {}, 
    previous_hash:      {}...,
Transactions({})
block_index:            {},
merkle_root:            {}...,
difficulty:             {},
block_hash:             {}...,
",
            title,
            self.header.to_owned().unwrap().timestamp,
            self.header.to_owned().unwrap().nonce,
            hex::encode(shrunk_previous_hash),
            self.transactions.len(),
            header.block_index,
            hex::encode(shrunk_merkle_root),
            self.header.to_owned().unwrap().difficulty,
            hex::encode(shrunk_hash),
        );
        return write!(f, "{}", block);
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_genesis_block() {
        let genesis: Block = create_genesis_block();
        assert_eq!(0, genesis.header.unwrap().block_index);
    }

    #[test]
    fn test_first_block() {
        let genesis: Block = create_genesis_block();
        let transactions: Vec<Transaction> = vec![];
        let merkle_root: [u8; 32] = [0; 32];
        let difficulty: u64 = 0;
        let new_block = next_block(&genesis, transactions, merkle_root, difficulty);
        let header = new_block.header.unwrap();
        assert_eq!(genesis.block_hash, header.previous_hash);
        assert_eq!(1, header.block_index);
    }
}
