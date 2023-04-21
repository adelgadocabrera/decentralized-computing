use std::net::SocketAddr;
use std::sync::Arc;
use tokio::spawn;
use tokio::sync::RwLock;
use crate::event_bus::events::RustchainEvent;
use crate::event_bus::event_bus::EventBus;
use crate::protos::Peer;
use crate::protos::Transaction;
use crate::protos::Block;

use super::client_stubs::PeerClient;
use super::server_stubs::PeerServer;

const PORT: u16 = 5059;
const LOCALHOST: &str = "[::1]";

pub struct P2p{
    event_bus: Arc<RwLock<EventBus>>,
    peer_list: Arc<Vec<Peer>>,
}

impl P2p {
    pub fn new(
        boot_nodes: Vec<Peer>, 
        event_bus: Arc<RwLock<EventBus>>
    )-> P2p{
        // listen to other peers 
        spawn(async {
            let addr: SocketAddr = format!("{}:{}", LOCALHOST, PORT).parse().unwrap();
            let server = PeerServer::new(addr);
            server.serve().await;
        });
        for node in boot_nodes {
            let peer = PeerClient::new(node.ip.as_str(), node.port as u16);
        }
        return P2p{ event_bus, peer_list: Arc::new(vec![]) };
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
