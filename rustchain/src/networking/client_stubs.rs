use crate::protos::rustchain_client::RustchainClient;
use crate::protos::p2p_client::P2pClient;
use crate::protos::{Transaction, Response as ProtoResponse};
use std::error::Error;
use tonic::transport::Channel;
use tonic::Request;
use tonic::transport::Endpoint;

pub struct PeerClient {
    rustchain: RustchainClient<Channel>,
}

impl PeerClient {
    pub async fn new(to: &str, port: u16) -> Result<PeerClient, Box<dyn Error>> {
        let target = format!("https://{}:{}", to, port);
        let endpoint = Endpoint::from_shared(target)?;
        let channel = endpoint.connect().await?;
        let rustchain = RustchainClient::new(channel);
        // let p2p =P2pClient::new(channel);
        Ok(PeerClient { rustchain })
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

    pub async fn get_peers(mut self)-> Result<ProtoResponse, Box<dyn Error>>{
        let req = self.rustchain.send_transaction(Request::new(Transaction::default())).await;
        match req {
            Ok(resp) => Ok(resp.into_inner()),
            Err(e) => Err(Box::new(e)),
        }
    }
}
