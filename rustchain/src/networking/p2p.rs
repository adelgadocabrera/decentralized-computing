use crate::event_bus::event_bus::EventBus;
use crate::event_bus::events::RustchainEvent;
use crate::protos::Block;
use crate::protos::Peer;
use crate::protos::Transaction;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::spawn;
use tokio::sync::mpsc::Receiver;
use tokio::sync::RwLock;

use super::client_stubs::PeerClient;
use super::server_stubs::PeerServer;

const PORT: u16 = 5059;
const LOCALHOST: &str = "[::1]";

pub struct P2p {
    event_bus: Arc<RwLock<EventBus>>,
    peers: Arc<Vec<Peer>>,
}

impl P2p {
    pub async fn new(boot_nodes: Vec<Peer>, event_bus: Arc<RwLock<EventBus>>) -> Arc<RwLock<P2p>> {
        // listen to other peers
        spawn(async {
            let addr: SocketAddr = format!("{}:{}", LOCALHOST, PORT).parse().unwrap();
            let server = PeerServer::new(addr);
            let _ = server.serve().await;
        });
        // very naive and not efficient way of adding registered peers.
        let mut swarm: Vec<Peer> = vec![];
        for node in boot_nodes {
            let mut peer = PeerClient::new(node.ip.as_str(), node.port as u16)
                .await
                .unwrap();
            let peers: Vec<Peer> = peer.register().await.unwrap().peers;
            for peer in peers {
                let mut has_peer = false;
                for p in swarm.clone() {
                    if p.id == peer.id {
                        has_peer = true;
                    }
                }
                if !has_peer {
                    swarm.push(peer);
                }
            }
        }
        let p2p = P2p {
            event_bus: event_bus.clone(),
            peers: Arc::new(swarm),
        };
        let p2p_arc = Arc::new(RwLock::new(p2p));
        let p2p_clone = p2p_arc.clone();
        let event_receiver: Receiver<RustchainEvent> = event_bus.write().await.subscribe().await;
        spawn(async move { P2p::listen_for_events(p2p_clone, event_receiver) });
        return p2p_arc;
    }

    async fn listen_for_events(
        p2p: Arc<RwLock<P2p>>,
        mut event_receiver: Receiver<RustchainEvent>,
    ) {
        while let Some(event) = event_receiver.recv().await {
            match event {
                RustchainEvent::NewBlock(_) => {
                    unimplemented!()
                }
                RustchainEvent::NewTransaction(_) => {
                    // let mut w = p2p.write().await;
                    // for (index, utxo_output) in tx.clone().outputs.into_iter().enumerate() {
                    //     if utxo_output.to_addr != w.address { continue; }
                    //     let tx_hash = hex::encode(tx.hash());
                    //     w.utxos.insert((tx_hash, index as u32), utxo_output.clone());
                    // }
                }
                _ => {
                    unimplemented!();
                }
            }
        }
    }

    async fn on_block_received(&self, block: Block) {
        let bus = self.event_bus.write().await;
        bus.publish(RustchainEvent::NewBlock(block)).await;
    }

    async fn on_transaction_received(&self, transaction: Transaction) {
        let bus = self.event_bus.write().await;
        bus.publish(RustchainEvent::NewTransaction(transaction))
            .await;
    }
}

#[cfg(test)]
pub mod tests {

    #[tokio::test]
    async fn test_something() {}
}
