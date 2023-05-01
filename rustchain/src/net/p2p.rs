use crate::event_bus::event_bus::EventBus;
use crate::event_bus::events::RustchainEvent;
use crate::protos::{Peer, PeerList, RegisterResponse};
use std::collections::VecDeque;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::spawn;
use tokio::sync::mpsc::Receiver;
use tokio::sync::RwLock;

use super::client_stubs::PeerClient;
use super::server_stubs::PeerServer;

pub struct P2p {
    event_bus: Arc<RwLock<EventBus>>,
    id: String,
    addr: SocketAddr,
    peers: Arc<RwLock<Vec<Peer>>>,
}

impl P2p {
    pub async fn new(
        event_bus: Arc<RwLock<EventBus>>,
        boot_node: Peer,
        addr: SocketAddr,
        heartbeat_interval: Duration,
    ) -> Arc<RwLock<P2p>> {
        let fallback_p2p = Arc::new(RwLock::new(P2p {
            event_bus: event_bus.clone(),
            id: String::from(""),
            addr,
            peers: Arc::new(RwLock::new(vec![])),
        }));
        // listen to other peers
        let server = PeerServer::new(event_bus.clone(), addr);
        spawn(async { server.serve().await });
        let peer_list: Vec<Peer> = vec![];
        let bootstrap_conn = PeerClient::new(boot_node.ip.as_str(), boot_node.port as u16).await;
        // gotta figure out a cleaner way of avoiding this ugly match nesting
        match bootstrap_conn {
            Ok(mut peer) => {
                let resp = peer.register(addr).await;
                match resp {
                    Ok(register_response) => {
                        let id = register_response.peer.unwrap().id;
                        let registered_peers = register_response.peers.unwrap();
                        event_bus
                            .write()
                            .await
                            .publish(RustchainEvent::NewPeers(registered_peers))
                            .await;
                        let peers = Arc::new(RwLock::new(peer_list));
                        let p2p = P2p {
                            event_bus: event_bus.clone(),
                            id: id.clone(),
                            addr,
                            peers: peers.clone(),
                        };
                        let p2p_arc = Arc::new(RwLock::new(p2p));
                        let p2p_clone = p2p_arc.clone();
                        let event_receiver: Receiver<RustchainEvent> =
                            event_bus.write().await.subscribe().await;
                        spawn(
                            async move { P2p::listen_for_events(p2p_clone, event_receiver).await },
                        );
                        let self_peer = Arc::new(Peer {
                            id: id.clone(),
                            ip: addr.ip().to_string(),
                            port: addr.port() as u32,
                        });
                        spawn(async move {
                            loop {
                                P2p::send_heartbeats(peers.clone(), self_peer.clone()).await;
                                tokio::time::sleep(heartbeat_interval).await;
                            }
                        });
                        return p2p_arc;
                    }
                    Err(_) => {
                        println!(
                            "Peer running at port {} could not register to bootstrap node",
                            addr.port()
                        );
                        return fallback_p2p.clone();
                    }
                }
            }
            Err(_) => return fallback_p2p.clone(),
        }
    }

    async fn listen_for_events(
        p2p: Arc<RwLock<P2p>>,
        mut event_receiver: Receiver<RustchainEvent>,
    ) {
        while let Some(event) = event_receiver.recv().await {
            match event {
                RustchainEvent::NewHeartbeat(heartbeat) => {
                    P2p::add_peers(p2p.clone(), heartbeat.peers.unwrap()).await;
                    P2p::add_peer(p2p.clone(), heartbeat.peer.unwrap()).await;
                }
                RustchainEvent::NewPeers(peer_list) => {
                    P2p::add_peers(p2p.clone(), peer_list).await;
                }
                _ => {
                    unimplemented!();
                }
            }
        }
    }

    async fn add_peer(p2p: Arc<RwLock<P2p>>, peer: Peer) {
        let curr_peer_list = p2p.write().await.peers.clone();
        let mut peer_lock = curr_peer_list.write().await;
        let mut has_peer = false;
        for p in peer_lock.iter() {
            if p.id == peer.id {
                has_peer = true;
            }
        }
        if !has_peer {
            peer_lock.push(peer);
        }
    }

    async fn add_peers(p2p: Arc<RwLock<P2p>>, new_peer_list: PeerList) {
        let curr_peer_list = p2p.write().await.peers.clone();
        let mut peer_lock = curr_peer_list.write().await;
        let new_peers = new_peer_list.peers;
        for peer in new_peers {
            let mut has_peer = false;
            for p in peer_lock.iter() {
                if p.id == peer.id {
                    has_peer = true;
                }
            }
            if !has_peer {
                peer_lock.push(peer);
            }
        }
    }

    async fn send_heartbeats(peers: Arc<RwLock<Vec<Peer>>>, self_peer: Arc<Peer>) {
        let peers_copy = {
            let guard = peers.read().await;
            (*guard).clone()
        };
        P2p::rebalance(peers.clone(), (*self_peer).clone()).await;
        for remote_peer in peers.clone().read().await.iter() {
            let conn = PeerClient::new(&remote_peer.ip, remote_peer.port as u16).await;
            match conn {
                Ok(mut client) => {
                    let block_hashes = vec![];
                    let peer_list = PeerList {
                        peers: peers_copy.clone(),
                    };
                    let _ = client.send_heartbeat(peer_list, (*self_peer).clone(), block_hashes);
                }
                Err(_) => {
                    unimplemented!()
                }
            }
        }
    }

    async fn rebalance(peers: Arc<RwLock<Vec<Peer>>>, peer: Peer) {
        let mut sorted_peers = peers.read().await.clone();
        let id = peer.id.clone();

        // Sort peers by Id
        sorted_peers.sort_by(|a, b| a.id.cmp(&b.id));

        let self_index: usize;
        if sorted_peers.contains(&peer) {
            self_index = sorted_peers
                .iter()
                .position(|p| p.to_owned() == peer)
                .unwrap();
        } else {
            // Insert SelfId into the sorted list
            self_index = sorted_peers
                .binary_search_by(|peer| peer.id.clone().cmp(&id))
                .unwrap_or_else(|x| x);
            sorted_peers.insert(self_index, peer);
        }

        // Create a double-ended queue and rotate it to set the SelfId as the first element
        let mut deque: VecDeque<Peer> = sorted_peers.into();
        deque.rotate_left(self_index);

        // Remove the SelfId from the deque
        deque.pop_front();

        // Calculate the number of nodes to select on each side
        let total_peers = deque.len();
        let num_peers_to_select = usize::min(16, total_peers / 2);

        // Select num_peers_to_select before and after the SelfId, considering the list as a cycle
        let closest_peers = deque
            .clone()
            .into_iter()
            .take(num_peers_to_select)
            .chain(deque.clone().into_iter().rev().take(num_peers_to_select))
            .collect::<Vec<Peer>>();

        // Update the peers field with the new list of closest peers
        *peers.write().await = closest_peers;
    }

    pub fn id(&self) -> String {
        return self.id.clone();
    }

    pub fn get_addr(&self) -> SocketAddr {
        return self.addr.clone();
    }

    pub async fn get_peers(&self) -> Vec<Peer> {
        let peers_guard = self.peers.read().await;
        peers_guard.clone()
    }
}

pub fn print_membership_table(id: String, peers: Vec<Peer>) {
    println!("Membership Table for {}", id);
    for peer in peers.iter() {
        println!(
            "Peer {}:\n  ip: {},\n  port: {}",
            peer.id, peer.ip, peer.port
        );
        println!();
    }
}

#[cfg(test)]
pub mod tests {
    use tokio::time::sleep;

    use super::*;
    use crate::{net::bootstrap_node::BootstrapNode, protos::Peer};
    use std::{net::SocketAddr, time::Duration};

    use crate::{event_bus::event_bus::EventBus, net::networking::get_addr};

    fn addr_1() -> SocketAddr {
        get_addr("127.0.0.1", 5001)
    }

    fn addr_2() -> SocketAddr {
        get_addr("127.0.0.1", 5002)
    }

    fn addr_3() -> SocketAddr {
        get_addr("127.0.0.1", 5003)
    }

    fn addr_4() -> SocketAddr {
        get_addr("127.0.0.1", 5004)
    }

    fn heartbeat_interval() -> Duration {
        Duration::from_millis(200)
    }

    fn get_bootstrap_node() -> (Peer, BootstrapNode) {
        let boot_node_addr: SocketAddr = get_addr("127.0.0.1", 5000);
        let boot_node = Peer {
            id: String::from(""),
            ip: boot_node_addr.ip().to_string(),
            port: boot_node_addr.port() as u32,
        };
        (boot_node, BootstrapNode::new(boot_node_addr))
    }

    #[tokio::test]
    async fn test_heartbeat() {
        println!("testing registration");
        let (boot_peer, bootstrap) = get_bootstrap_node();
        spawn(async { bootstrap.serve().await });
        sleep(Duration::from_millis(100)).await;
        let event_bus = EventBus::new().await;
        let peer_1 = P2p::new(
            event_bus.clone(),
            boot_peer.clone(),
            addr_1(),
            heartbeat_interval(),
        )
        .await;
        let peer_2 = P2p::new(
            event_bus.clone(),
            boot_peer.clone(),
            addr_2(),
            heartbeat_interval(),
        )
        .await;
        let peer_3 = P2p::new(
            event_bus.clone(),
            boot_peer.clone(),
            addr_3(),
            heartbeat_interval(),
        )
        .await;
        let peer_4 = P2p::new(
            event_bus.clone(),
            boot_peer.clone(),
            addr_4(),
            heartbeat_interval(),
        )
        .await;
        sleep(Duration::from_secs(2)).await;

        // print_membership_table(peer_1.read().await.id(), peer_1.read().await.peers.read().await.clone());
        // print_membership_table(peer_2.read().await.id(), peer_2.read().await.peers.read().await.clone());
        // print_membership_table(peer_3.read().await.id(), peer_3.read().await.peers.read().await.clone());
        // print_membership_table(peer_4.read().await.id(), peer_4.read().await.peers.read().await.clone());
        // assert_eq!(
        //     *peer_3.read().await.peers.read().await,
        //     *peer_4.read().await.peers.read().await
        // );
        // assert_eq!(
        //     *peer_1.read().await.peers.read().await,
        //     *peer_3.read().await.peers.read().await,
        // );
    }

    #[tokio::test]
    async fn test_registration() {
        // create event_bus, bootstrap node
        let event_bus = EventBus::new().await;
        let (boot_peer, bootstrap) = get_bootstrap_node();
        spawn(async { bootstrap.serve().await });
        // wait for bootstrap_node to start
        // create 2 P2P peers and register
        sleep(Duration::from_millis(100)).await;
        let peer_1 = P2p::new(
            event_bus.clone(),
            boot_peer.clone(),
            addr_1(),
            heartbeat_interval(),
        )
        .await;
        let peer_2 = P2p::new(
            event_bus.clone(),
            boot_peer.clone(),
            addr_2(),
            heartbeat_interval(),
        )
        .await;
        sleep(Duration::from_millis(100)).await;
        let id_1 = 1.to_string();
        let id_2 = 2.to_string();
        assert_eq!(id_1, peer_1.read().await.id()); // first assigned id should be 1
        assert_eq!(id_2, peer_2.read().await.id()); // first assigned id should be 1
    }

    #[tokio::test]
    async fn test_rebalance() {
        let event_bus = EventBus::new().await;
        let peer_1 = P2p::new(event_bus, Peer::default(), addr_1(), heartbeat_interval()).await;
        let mut expected = vec![];
        for i in (0..5).rev() {
            let peer = Peer {
                id: (i * 2).to_string(),
                ip: String::from("[::1]"),
                port: 5000,
            };
            peer_1.write().await.peers.write().await.push(peer.clone());
            expected.insert(0, peer.clone());
        }
        let peer = Peer {
            id: 5.to_string(),
            ip: String::from("[::1]"),
            port: 5001,
        };
        P2p::rebalance(peer_1.read().await.peers.clone(), peer.clone()).await;
        expected.insert(3, peer.clone());
        assert_eq!(expected, peer_1.read().await.peers.read().await.clone());
    }
}
