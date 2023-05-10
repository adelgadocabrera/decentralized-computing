use crate::event_bus::event_bus::EventBus;
use crate::event_bus::events::RustchainEvent;
use crate::protos::{Peer, PeerList};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::spawn;
use tokio::sync::mpsc::Receiver;
use tokio::sync::RwLock;

use super::client_stubs::PeerClient;
use super::server_stubs::PeerServer;

const MAX_PEERS_LEN: usize = 32;

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
                    // add membership table and peer who sent heartbeat
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
        let lock = p2p.write().await;
        let self_id = lock.id.clone();
        let curr_peer_list = lock.peers.clone();
        let mut peer_lock = curr_peer_list.write().await;
        if self_id == peer.id || peer_lock.iter().any(|x| x.id == peer.id) {
            return;
        }
        peer_lock.push(peer);
    }

    async fn add_peers(p2p: Arc<RwLock<P2p>>, new_peer_list: PeerList) {
        let curr_peer_list = p2p.write().await.peers.clone();
        let mut peer_lock = curr_peer_list.write().await;
        let new_peers = new_peer_list.peers;
        for peer in new_peers {
            let self_id = p2p.read().await.id.clone();
            if peer.id == self_id || peer_lock.iter().any(|x| x.id == peer.id) {
                continue;
            }
            peer_lock.push(peer.clone());
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
        let self_id = peer.id.clone();
        // Sort peers by Id
        sorted_peers.sort_by(sort_peers_by_id);
        if sorted_peers.len() < MAX_PEERS_LEN {
            *peers.write().await = sorted_peers;
            return;
        }

        let mut self_index = 0;
        for i in 0..sorted_peers.len() {
            if sorted_peers.get(i).unwrap().id.parse::<u32>().unwrap()
                > self_id.parse::<u32>().unwrap()
            {
                self_index = i;
                break;
            }
        }
        let mut closest_peers = vec![];
        let mut i: usize; 
        let mut j: usize;
        if self_index == 0 {
            i = self_index;
            j = self_index + 1;
        } else {
            i = self_index - 1;
            j = self_index;
        }
        let mut counter: usize = 0;
        while counter < MAX_PEERS_LEN {
            counter += 2;
            closest_peers.push(sorted_peers.get(i).unwrap().clone());
            closest_peers.push(sorted_peers.get(j).unwrap().clone());
            if i == 0 {
                i = sorted_peers.len() - 1;
            } else {
                i -= 1;
            };
            if j == sorted_peers.len() - 1 {
                j = 0;
            } else {
                j += 1;
            }
        }
        closest_peers.sort_by(sort_peers_by_id);

        // Update the peers field with the new list of closest peers
        *peers.write().await = closest_peers.clone();
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

fn sort_peers_by_id(a: &Peer, b: &Peer) -> std::cmp::Ordering {
    a.id.parse::<u32>()
        .unwrap_or(0)
        .cmp(&b.id.parse::<u32>().unwrap_or(0))
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

        print_membership_table(1.to_string(), peer_1.read().await.peers.read().await.clone());
        for i in 1..5 {
            if i > 1 {
                assert!(peer_1.read().await.peers.read().await.iter().any(|p| (*p.id).parse::<u32>().unwrap() == i));
            }
            if i == 1 || i > 3 {
                assert!(peer_2.read().await.peers.read().await.iter().any(|p| (*p.id).parse::<u32>().unwrap() == i));
            }
            if (i >= 1 && i <= 2) || (i == 4) {
                assert!(peer_3.read().await.peers.read().await.iter().any(|p| (*p.id).parse::<u32>().unwrap() == i));
            }
            if i < 4 {
                assert!(peer_4.read().await.peers.read().await.iter().any(|p| (*p.id).parse::<u32>().unwrap() == i));
            }
        }
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
    async fn test_rebalance_below_max_size() {
        let event_bus = EventBus::new().await;
        let peer_1 = P2p::new(event_bus, Peer::default(), addr_1(), heartbeat_interval()).await;
        let mut expected = vec![];
        for i in (0..5).rev() {
            let peer = Peer {
                id: (i * 2).to_string(),
                ..Peer::default()
            };
            // inserting nodes in different order
            let node_lock = peer_1.write().await;
            let mut peers_lock = node_lock.peers.write().await;
            if i % 2 == 0 {
                peers_lock.insert(0, peer.clone());
            } else {
                peers_lock.push(peer.clone());
            }
            expected.insert(0, peer.clone());
        }
        let peer = Peer {
            id: 5.to_string(),
            ..Peer::default()
        };
        P2p::rebalance(peer_1.read().await.peers.clone(), peer.clone()).await;
        assert_eq!(expected, peer_1.read().await.peers.read().await.clone());
    }

    #[tokio::test]
    async fn test_rebalance_above_max_size() {
        let event_bus = EventBus::new().await;
        let peer_1 = P2p::new(event_bus, Peer::default(), addr_1(), heartbeat_interval()).await;
        let target_peer_id = 13;
        let num_peers = 32;
        let target_peer = Peer {
            id: target_peer_id.to_string(),
            ..Peer::default()
        };

        let mut all_peers = vec![];
        for i in (0..num_peers).rev() {
            let peer = Peer {
                id: (i * 2).to_string(),
                ..Peer::default()
            };
            all_peers.push(peer.clone());
        }

        // The expected list should contain the closest 32 peers.
        let mut expected = all_peers.into_iter().take(32).collect::<Vec<Peer>>();
        expected.sort_by(sort_peers_by_id);

        // Insert peers into the peers list in a different order.
        for i in (0..num_peers).rev() {
            let peer = Peer {
                id: (i * 2).to_string(),
                ..Peer::default()
            };
            let node_lock = peer_1.write().await;
            let mut peers_lock = node_lock.peers.write().await;
            if i % 2 == 0 {
                peers_lock.insert(0, peer.clone());
            } else {
                peers_lock.push(peer.clone());
            }
        }

        // Run the rebalance function.
        P2p::rebalance(peer_1.read().await.peers.clone(), target_peer.clone()).await;

        // Check the results.
        let rebalanced = peer_1.read().await.peers.read().await.clone();
        assert_eq!(expected, rebalanced);
    }

    #[tokio::test]
    async fn test_rebalance_above_max_size_cyclical() {
        let event_bus = EventBus::new().await;
        let peer_1 = P2p::new(event_bus, Peer::default(), addr_1(), heartbeat_interval()).await;
        let target_peer_id = 53;
        let num_peers = 33;
        let target_peer = Peer {
            id: target_peer_id.to_string(),
            ..Peer::default()
        };

        let mut all_peers = vec![];
        for i in (0..10).rev() {
            let peer = Peer {
                id: (i * 2).to_string(),
                ..Peer::default()
            };
            all_peers.push(peer.clone());
        }
        for i in (11..num_peers).rev() {
            let peer = Peer {
                id: (i * 2).to_string(),
                ..Peer::default()
            };
            all_peers.push(peer.clone());
        }

        // The expected list should contain the closest 32 peers.
        let mut expected = all_peers.into_iter().take(32).collect::<Vec<Peer>>();
        expected.sort_by(sort_peers_by_id);

        // Insert peers into the peers list in a different order.
        for i in (0..num_peers).rev() {
            let peer = Peer {
                id: (i * 2).to_string(),
                ..Peer::default()
            };
            let node_lock = peer_1.write().await;
            let mut peers_lock = node_lock.peers.write().await;
            if i % 2 == 0 {
                peers_lock.insert(0, peer.clone());
            } else {
                peers_lock.push(peer.clone());
            }
        }

        // Run the rebalance function.
        P2p::rebalance(peer_1.read().await.peers.clone(), target_peer.clone()).await;

        // Check the results.
        let rebalanced = peer_1.read().await.peers.read().await.clone();
        assert_eq!(expected, rebalanced);
    }
}
