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
}

impl Blockchain {
    pub fn new(event_bus: Arc<RwLock<EventBus>>) -> Arc<RwLock<Self>> {
        let handle = Handle::current();
        let blockchain = Blockchain {
            blocks: Arc::new(RwLock::new(Vec::new())),
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

    pub async fn add_block(&mut self, block: Block) {
        self.blocks.write().await.push(block);
    }

    async fn listen_for_events(&mut self, mut event_receiver: Receiver<RustchainEvent>) {
        while let Some(event) = event_receiver.recv().await {
            match event {
                RustchainEvent::NewBlock(block) => {
                    self.add_block(block).await;
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
