use crate::event_bus::event_bus::EventBus;
use crate::event_bus::events::RustchainEvent;
use crate::protos::Block;
use std::sync::Arc;
use tokio::runtime::Handle;
use tokio::spawn;
use tokio::sync::mpsc::Receiver;
use tokio::sync::RwLock;

#[derive(Debug)]
pub struct Blockchain {
    blocks: Arc<RwLock<Vec<Block>>>,
    block_hashes: Arc<RwLock<Vec<String>>>,
}

impl Blockchain {
    pub fn new(event_bus: Arc<RwLock<EventBus>>) -> Arc<RwLock<Self>> {
        let handle = Handle::current();
        let blockchain = Blockchain {
            blocks: Arc::new(RwLock::new(Vec::new())),
            block_hashes: Arc::new(RwLock::new(vec![])),
        };
        let blockchain_arc = Arc::new(RwLock::new(blockchain));
        handle.block_on(async {
            let event_receiver = event_bus.write().await.subscribe().await;
            let blockchain_clone = blockchain_arc.clone();
            spawn(async move { Blockchain::listen_for_events(blockchain_clone, event_receiver) });
            blockchain_arc
        })
    }

    pub async fn add_block(&mut self, block: Block) {
        self.blocks.write().await.push(block);
    }

    async fn listen_for_events(
        b: Arc<RwLock<Blockchain>>,
        mut event_receiver: Receiver<RustchainEvent>,
    ) {
        while let Some(event) = event_receiver.recv().await {
            match event {
                RustchainEvent::NewHeartbeat(_heartbeat) => {
                    // TO DO: check whether blockchain is up to date with all the
                    // blocks or else request peers for all previous blocks
                    // (maybe up to a certain block?)
                    // let block_hashes = heartbeat.block_hashes;
                    unimplemented!()
                }
                RustchainEvent::NewBlock(block) => {
                    let block_header = block.clone().header.unwrap();
                    let block_hash = hex::encode(block_header.hash());
                    let mut b_lock = b.write().await;
                    b_lock.block_hashes.write().await.push(block_hash);
                    b_lock.add_block(block.clone()).await;
                }
                _ => {
                    unimplemented!();
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
