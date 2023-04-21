use std::sync::Arc;
use tokio::sync::RwLock;
use crate::event_bus::events::RustchainEvent;
use crate::event_bus::event_bus::EventBus;
use crate::protos::Transaction;
use crate::protos::Block;

struct Networking{
    event_bus: Arc<RwLock<EventBus>>,
}

impl Networking {
    pub fn new(event_bus: Arc<RwLock<EventBus>>)-> Networking{
        return Networking{ event_bus };
    }

    async fn on_block_received(&self, block: Block) {
        let bus = self.event_bus.write().await;
        bus.publish(RustchainEvent::NewBlock(block)).await;
    }

    async fn on_transaction_received(&self, transaction: Transaction) {
        let bus = self.event_bus.write().await;
        bus.publish(RustchainEvent::NewTransaction(transaction)).await;
    }
}
