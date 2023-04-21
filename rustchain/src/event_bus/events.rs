use crate::protos::{Block, Transaction};

#[derive(Clone)]
pub enum RustchainEvent {
    NewBlock(Block),
    NewTransaction(Transaction),
}
