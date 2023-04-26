use crate::event_bus::event_bus::EventBus;
use crate::event_bus::events::RustchainEvent;
use crate::protos::{Peer, RegisterResponse};
use std::sync::Arc;
use tokio::spawn;
use tokio::sync::mpsc::Receiver;
use tokio::sync::RwLock;

use super::client_stubs::PeerClient;
use super::server_stubs::PeerServer;

pub struct P2p {
    event_bus: Arc<RwLock<EventBus>>,
    peers: Arc<RwLock<Vec<Peer>>>,
    id: String,
}

impl P2p {
    pub async fn new(boot_nodes: Vec<Peer>, event_bus: Arc<RwLock<EventBus>>) -> Arc<RwLock<P2p>> {
        // listen to other peers
        let server = PeerServer::new(event_bus.clone());
        spawn(async { server.serve().await });
        // very naive and not efficient way of adding registered peers.
        let mut swarm: Vec<Peer> = vec![];
        for node in boot_nodes {
            let mut peer = PeerClient::new(node.ip.as_str(), node.port as u16)
                .await
                .unwrap();
            let resp: RegisterResponse = peer.register().await.unwrap();
            let peers = resp.peers.unwrap().peers;
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
            id: String::from(""), // initially empty
            event_bus: event_bus.clone(),
            peers: Arc::new(RwLock::new(swarm)),
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
                RustchainEvent::NewPeers(_) => {}
                _ => {
                    unimplemented!();
                }
            }
        }
    }

    async fn rebalance(&self) {
        let mut sorted_peers = self.peers.read().await.clone();
        // let
    }
}

#[cfg(test)]
pub mod tests {

    #[tokio::test]
    async fn test_something() {}
}
