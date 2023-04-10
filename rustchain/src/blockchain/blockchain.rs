use crate::{utils::concurrent_vec::ConcurrentVec, concurrentvec};
use super::{block::{create_genesis_block, Block, Transaction, next_block}, mine::calculate_pow};

pub struct Blockchain {
    blocks: ConcurrentVec<Block>,
    transactions: ConcurrentVec<Transaction>, 
    merkle_tree: Vec<String>,
    miner_addr: String,
}

impl Blockchain {
    // TODO: miner_addr should use new cryptographic module that generates
    // public and private keys
    pub fn new() -> Self {
        let genesis_block: Block = create_genesis_block();
        let blocks = concurrentvec![genesis_block];
        let transactions = concurrentvec![];
        let miner_addr = String::from("miner address");
        let merkle_tree = vec![];
        Blockchain { blocks, transactions, miner_addr, merkle_tree }
    }

    fn get_genesis(&self) -> Option<Block> {
        return self.blocks.peek_first(); 
    }

    // TODO: how do I define 'from_addr' and 'amount'?
    fn mine(mut self, last_block: &Block) {
        let last_proof: u64 = last_block.pow;
        let proof = calculate_pow(&last_proof);
        self.transactions.push(Transaction{
            from_addr: String::from("network"),
            to_addr: self.miner_addr,
            amount: 1, 
            additional_data: String::from(""), // TODO: empty for now
        });
        let nonce = 0; // TODO: how do we know nonce?
        let difficulty = 9;
        next_block(
            last_block, 
            nonce, 
            self.transactions.flush(), 
            [0;32],  // TODO: add merkle root hash
            difficulty, 
            proof,
        );
    }
}

#[tokio::test]
async fn test_blockchain_creation() {
    let mut blockchain = Blockchain::new();
    assert_eq!(0, blockchain.get_genesis().unwrap().block_index);
}
