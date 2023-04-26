use crate::networking::networking::{get_self_ip, get_self_port};
use crate::protos::bootstrap_client::BootstrapClient;
use crate::protos::p2p_client::P2pClient;
use crate::protos::rustchain_client::RustchainClient;
use crate::protos::{GetPeersRequest, Heartbeat, Null, Peer, PeerList};
use crate::protos::{Response as ProtoResponse, Transaction};
use std::error::Error;
use tonic::transport::Channel;
use tonic::transport::Endpoint;
use tonic::Request;

pub struct PeerClient {
    bootstrap: BootstrapClient<Channel>,
    rustchain: RustchainClient<Channel>,
    p2p: P2pClient<Channel>,
}

impl PeerClient {
    pub async fn new(to_ip: &str, to_port: u16) -> Result<PeerClient, Box<dyn Error>> {
        let target = format!("https://{}:{}", to_ip, to_port);
        let endpoint = Endpoint::from_shared(target)?;
        let channel = endpoint.connect().await?;
        let rustchain = RustchainClient::new(channel.clone());
        let bootstrap = BootstrapClient::new(channel.clone());
        let p2p = P2pClient::new(channel.clone());
        Ok(PeerClient {
            bootstrap,
            rustchain,
            p2p,
        })
    }

    pub async fn register(&mut self) -> Result<PeerList, Box<dyn Error>> {
        // peer id will be assigned upon registration
        let peer = Peer {
            id: String::from(""),
            ip: get_self_ip(),
            port: get_self_port(),
        };
        let req = self.bootstrap.register(Request::new(peer)).await;
        match req {
            Ok(resp) => Ok(resp.into_inner()),
            Err(e) => Err(Box::new(e)),
        }
    }

    pub async fn send_transaction(
        mut self,
        transaction: Transaction,
    ) -> Result<ProtoResponse, Box<dyn Error>> {
        let tx = Request::new(transaction);
        let req = self.rustchain.send_transaction(tx).await;
        match req {
            Ok(resp) => Ok(resp.into_inner()),
            Err(e) => Err(Box::new(e)),
        }
    }

    pub async fn get_peers(mut self) -> Result<PeerList, Box<dyn Error>> {
        let req = self
            .p2p
            .get_peers(Request::new(GetPeersRequest::default()))
            .await;
        match req {
            Ok(resp) => Ok(resp.into_inner()),
            Err(e) => Err(Box::new(e)),
        }
    }

    pub async fn send_heartbeat(
        &mut self,
        peers: PeerList,
        peer: Peer,
    ) -> Result<Null, Box<dyn Error>> {
        let heartbeat = Heartbeat {
            peers: Some(peers),
            peer: Some(peer),
        };
        let req = self.p2p.send_heartbeat(Request::new(heartbeat)).await;
        match req {
            Ok(resp) => Ok(resp.into_inner()),
            Err(e) => Err(Box::new(e)),
        }
    }
}
