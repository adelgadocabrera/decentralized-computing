use std::{error::Error, net::SocketAddr, sync::Arc};

use crate::{
    net::p2p::print_membership_table,
    protos::{
        bootstrap_server::{Bootstrap, BootstrapServer},
        Peer, PeerList, RegisterResponse,
    },
};
use tokio::sync::RwLock;
use tonic::transport::server::Router;
pub use tonic::{transport::Server, Request, Response, Status};

#[derive(Debug)]
pub struct BootstrapService {
    peers: Arc<RwLock<Vec<Peer>>>,
    id_counter: Arc<RwLock<u64>>,
}

pub struct BootstrapNode {
    addr: SocketAddr,
    router: Router,
    peers: Arc<RwLock<Vec<Peer>>>,
    id_counter: Arc<RwLock<u64>>,
}

impl BootstrapNode {
    pub fn new(addr: SocketAddr) -> Self {
        let peers = Arc::new(RwLock::new(Vec::new()));
        let id_counter = Arc::new(RwLock::new(0));
        let bootstrap_service = BootstrapService {
            peers: Arc::clone(&peers),
            id_counter: Arc::clone(&id_counter),
        };
        let mut server = Server::builder();
        let router = server.add_service(BootstrapServer::new(bootstrap_service));
        return Self {
            addr,
            router,
            peers,
            id_counter,
        };
    }

    pub async fn serve(self) -> Result<(), Box<dyn Error + Send>> {
        self.router
            .serve(self.addr)
            .await
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;
        Ok(())
    }

    pub async fn get_peer_list(&self) -> Vec<Peer> {
        return self.peers.read().await.clone();
    }
}

impl Default for BootstrapService {
    fn default() -> Self {
        Self {
            peers: Arc::new(RwLock::new(vec![])),
            id_counter: Arc::new(RwLock::new(0)),
        }
    }
}

#[tonic::async_trait]
impl Bootstrap for BootstrapService {
    async fn register(&self, req: Request<Peer>) -> Result<Response<RegisterResponse>, Status> {
        let mut peer = req.into_inner();
        let id = {
            let mut id_counter = self.id_counter.write().await;
            *id_counter += 1;
            *id_counter
        };
        peer.id = id.to_string();
        self.peers.write().await.push(peer.clone());
        let peers = PeerList::from(self.peers.read().await.clone());
        let resp = RegisterResponse {
            peers: Some(peers),
            peer: Some(peer.clone()),
        };
        println!("New Peer registration with id: {}", id);
        // un-comment to visualize bootstrap membership table
        // print_membership_table(String::from("BOOTSTRAP"), self.peers.read().await.clone());
        Ok(Response::new(resp))
    }
}

#[cfg(test)]
pub mod tests {
    use std::time::Duration;

    use crate::net::{client_stubs::PeerClient, networking::get_addr};
    use tokio::{spawn, task::JoinHandle, time::sleep};

    use super::*;

    const SERVER_IP: &str = "[::1]";
    const SERVER_PORT: u16 = 5000;

    async fn run_mock_server() -> JoinHandle<Result<(), Box<dyn Error + Send>>> {
        let addr: SocketAddr = format!("{}:{}", SERVER_IP, SERVER_PORT).parse().unwrap();
        let bootstrap = BootstrapNode::new(addr);
        let serve_handle = spawn(async move { bootstrap.serve().await });
        sleep(Duration::from_millis(100)).await; // buffer time
        return serve_handle;
    }

    #[tokio::test]
    async fn test_registration() {
        let serve_handle = run_mock_server().await;

        // register client
        let client_ip = "127.0.0.1";
        let client_port = 7989;
        let client_addr = get_addr(client_ip, client_port);
        let mut peer_client: PeerClient = PeerClient::new(SERVER_IP, SERVER_PORT).await.unwrap();
        let resp = peer_client.register(client_addr).await.unwrap();
        sleep(Duration::from_millis(100)).await; // buffer time
        let peers: Vec<Peer> = resp.peers.unwrap().peers;
        // bootstrap node and peer should have sames nodes
        assert_eq!(peers.len(), 1);
        assert_eq!(
            Peer {
                id: 1.to_string(),
                ip: client_ip.to_string(),
                port: client_port as u32,
            },
            *peers.get(0).unwrap()
        );
        serve_handle.abort();
        ()
    }

    #[tokio::test]
    async fn test_register_multiple_clients() {
        let server_handler = run_mock_server().await;
        let mut peer_lists: Vec<Vec<Peer>> = vec![];

        // register client
        let client_ip = "127.0.0.1";
        let client_port = 7989;
        let client_addr = get_addr(client_ip, client_port);
        for _ in 0..10 {
            let mut peer_client: PeerClient =
                PeerClient::new(SERVER_IP, SERVER_PORT).await.unwrap();
            let peer_list = peer_client
                .register(client_addr)
                .await
                .unwrap()
                .peers
                .unwrap()
                .peers;
            peer_lists.push(peer_list);
        }
        // - checks that each peer is receiving i number of register clients. Meanning that the third
        // registered client should receive a list of 3 registered clients.
        // - checks that the registered clients have the expected registered ids
        for i in 0..peer_lists.len() {
            let len = peer_lists.get(i).unwrap().len();
            assert_eq!(i + 1, len);
            for j in 0..len - 1 {
                assert_eq!(
                    (j + 1).to_string(),
                    peer_lists.get(i).unwrap().get(j).unwrap().id
                );
            }
        }
        server_handler.abort();
        ()
    }
}
