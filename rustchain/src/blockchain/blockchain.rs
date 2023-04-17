use std::sync::Arc;
use tokio::runtime::Handle;
use tokio::spawn;
use tokio::sync::RwLock;
use tokio::sync::mpsc::Receiver;
use crate::event_bus::events::BlockchainEvent;
use crate::protos::{Block, Transaction};
use crate::event_bus::event_bus::EventBus;

#[derive(Debug)]
pub struct Blockchain {
    pending_transactions: Arc<RwLock<Vec<Transaction>>>,
    blocks: Arc<RwLock<Vec<Block>>>,

}

impl Blockchain {
    pub fn new(event_bus: Arc<RwLock<EventBus>>) -> Arc<RwLock<Self>> {
        let handle = Handle::current();
        let blockchain = Blockchain {
            blocks: Arc::new(RwLock::new(Vec::new())),
            pending_transactions: Arc::new(RwLock::new(Vec::new())),
        };
        let blockchain_arc = Arc::new(RwLock::new(blockchain));
        handle.block_on(async { 
            let event_receiver = event_bus.write().await.subscribe().await;
            let blockchain_clone = blockchain_arc.clone();
            spawn(async move {
                blockchain_clone
                    .write()
                    .await
                    .listen_for_events(event_receiver)
                    .await;
            });
            blockchain_arc
        })
    }

    async fn get_genesis(&self) -> Option<Block> {
        let blocks = self.blocks.read().await;
        return blocks.get(0).cloned();
    }

    pub fn add_block(&mut self, block: Block){
        unimplemented!()
    }

    pub fn new_tx(&mut self, transaction: Transaction){
        unimplemented!()
    }

    async fn listen_for_events(&mut self, mut event_receiver: Receiver<BlockchainEvent>) {
        while let Some(event) = event_receiver.recv().await {
            match event {
                BlockchainEvent::NewBlock(block) => {
                    self.add_block(block);
                }
                BlockchainEvent::NewTransaction(transaction) => {
                    self.new_tx(transaction);
                }
            }
        }
    }
}

// #[cfg(test)]
// pub mod tests {
//     use super::*;

//     #[tokio::test]
//     async fn test_blockchain_creation() {
//         let event_bus = EventBus::new();
//         let blockchain = Blockchain::new(event_bus);
//         let genesis = blockchain.get_genesis().unwrap();
//         assert_eq!(0, genesis.header.unwrap().block_index);
//     }
// }
