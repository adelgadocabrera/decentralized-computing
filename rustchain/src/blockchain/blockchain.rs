use std::fmt;
use crate::{utils::concurrent_vec::ConcurrentVec, concurrentvec};
use super::{block::{create_genesis_block, Block, Transaction, next_block}, mine::{Miner, new_miner, proof_of_work}};

#[derive(Clone)]
pub struct Blockchain {
    blocks: ConcurrentVec<Block>,
    pending_transactions: ConcurrentVec<Transaction>, 
    miner: Miner,
}

impl Blockchain {
    // TODO: miner_addr should use new cryptographic module that generates
    // public and private keys
    pub fn new() -> Self {
        let genesis_block: Block = create_genesis_block();
        let blocks = concurrentvec![genesis_block];
        let pending_transactions = concurrentvec![];
        let miner = new_miner().unwrap();
        Blockchain { blocks, pending_transactions, miner }
    }

    fn get_genesis(&self) -> Option<Block> {
        return self.blocks.peek_first(); 
    }

    fn mine(self, last_block: &Block) {
        let difficulty = 9;
        let mut block: Block = next_block(
            last_block, 
            self.pending_transactions.flush(), 
            [0;32],  // TODO: add merkle root hash
            difficulty, 
        );
        let mut header = block.header.unwrap();
        let (block_hash, _) = proof_of_work(&mut header);
        block.block_hash = block_hash;
        
        let reward: Transaction = Transaction{
            from_addr: String::from(""),
            to_addr: hex::encode(self.miner.addr),
            amount: 1, 
            additional_data: String::from(""), 
        };
        println!("{}", reward);
        //disseminate  
    }
}


impl fmt::Display for Blockchain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}\n{}\nminer: {}\nblocks: {}",
            "rustchain",
            "---------",
            hex::encode(self.miner.addr),
            self.blocks,
        )
    }
}

#[tokio::test]
async fn test_blockchain_creation() {
    let blockchain = Blockchain::new();
    let genesis = blockchain.get_genesis().unwrap();
    blockchain.mine(&genesis);
    assert_eq!(0, genesis.header.unwrap().block_index);
}
