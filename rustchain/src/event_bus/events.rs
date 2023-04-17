use crate::protos::{Block, Transaction};

#[derive(Clone)]
pub enum BlockchainEvent {
    NewBlock(Block),
    NewTransaction(Transaction),
}
