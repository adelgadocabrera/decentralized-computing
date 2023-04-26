use crate::event_bus::event_bus::EventBus;
use crate::event_bus::events::RustchainEvent;
use crate::protos::{Peer, RegisterResponse};
use std::collections::VecDeque;
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
    pub async fn new(boot_node: Peer, event_bus: Arc<RwLock<EventBus>>) -> Arc<RwLock<P2p>> {
        // listen to other peers
        let server = PeerServer::new(event_bus.clone());
        spawn(async { server.serve().await });
        // very naive and not efficient way of adding registered peers.
        let mut swarm: Vec<Peer> = vec![];
        let mut peer = PeerClient::new(boot_node.ip.as_str(), boot_node.port as u16)
            .await
            .unwrap();
        let resp: RegisterResponse = peer.register().await.unwrap();
        let id = resp.peer.unwrap().id;
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
        let p2p = P2p {
            id,
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
        let id = self.id.clone();

        // Sort peers by Id
        sorted_peers.sort_by(|a, b| a.id.cmp(&b.id));

        // Insert SelfId into the sorted list
        let self_index = sorted_peers
            .binary_search_by(|peer| peer.id.cmp(&id))
            .unwrap_or_else(|x| x);
        sorted_peers.insert(
            self_index,
            Peer {
                id,
                ..Default::default()
            },
        ); // Assuming you have a Default implementation for Peer

        // Create a double-ended queue and rotate it to set the SelfId as the first element
        let mut deque: VecDeque<Peer> = sorted_peers.into();
        deque.rotate_left(self_index);

        // Remove the SelfId from the deque
        deque.pop_front();

        // Select 16 peers before and 16 peers after the SelfId, considering the list as a cycle
        let closest_peers = deque
            .into_iter()
            .take(16)
            .chain(deque.into_iter().rev().take(16))
            .collect::<Vec<Peer>>();

        // Update the peers field with the new list of 32 closest peers
        *self.peers.write().await = closest_peers;
    }
}

#[cfg(test)]
pub mod tests {

    #[tokio::test]
    async fn test_something() {}
}
