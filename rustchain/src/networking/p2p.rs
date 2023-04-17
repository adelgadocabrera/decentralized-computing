use std::sync::Arc;
use tokio::sync::RwLock;
use crate::event_bus::events::BlockchainEvent;
use crate::event_bus::event_bus::EventBus;
use crate::protos::Transaction;
use crate::protos::Block;

trait IPeer {
    fn dial(&mut self, addr: String);
}

pub struct Peer{
    event_bus: Arc<RwLock<EventBus>>,
}

impl Peer {
    pub fn new(event_bus: Arc<RwLock<EventBus>>)-> Peer{
        return Peer{ event_bus };
    }

    async fn on_block_received(&self, block: Block) {
        let bus = self.event_bus.write().await;
        bus.publish(BlockchainEvent::NewBlock(block)).await;
    }

    async fn on_transaction_received(&self, transaction: Transaction) {
        let bus = self.event_bus.write().await;
        bus.publish(BlockchainEvent::NewTransaction(transaction)).await;
    }
}
