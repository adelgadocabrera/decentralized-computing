use std::sync::Arc;
use tokio::sync::RwLock;
use crate::event_bus::events::BlockchainEvent;
use crate::event_bus::event_bus::EventBus;
use crate::protos::Peer;
use crate::protos::Transaction;
use crate::protos::Block;

pub struct P2p{
    event_bus: Arc<RwLock<EventBus>>,
}

impl P2p {
    pub fn new(boot_nodes: Vec<Peer>, event_bus: Arc<RwLock<EventBus>>)-> P2p{
        return P2p{ event_bus };
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
