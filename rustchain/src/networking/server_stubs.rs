use crate::{protos::response::Data, blockchain::block::Block};
pub use crate::protos::rustchain_server::{Rustchain, RustchainServer};
pub use crate::protos::p2p_server::{P2p, P2pServer};
use crate::protos::{
    Transaction,
    ValidationRequest, 
    Response as RustchainResponse, 
    GetPeersRequest,PeerList, 
    Peer, 
    AddPeerResponse, 
    RemovePeerResponse, 
    Null, 
    Heartbeat, UtxoInputs, UtxoOutputs
};
use std::error::Error;
use std::net::SocketAddr;
use tonic::transport::server::Router;
pub use tonic::{transport::Server, Request, Response, Status};

#[derive(Debug, Default)]
struct RustchainService {}

#[derive(Debug, Default)]
struct P2pService {}

pub struct PeerServer {
    server: Option<Server>,
    router: Option<Router>,
    addr: SocketAddr,
}

impl PeerServer {
    // let addr = "[::1]:50051".parse().unwrap();
    pub fn new(addr: SocketAddr) -> PeerServer {
        let payment_service = RustchainService::default();
        let p2p_service = P2pService::default();
        let mut server = Server::builder();
        let router = server
            .add_service(RustchainServer::new(payment_service))
            .add_service(P2pServer::new(p2p_service));
        // add additional services to router here..
        return PeerServer {
            server: Some(server),
            router: Some(router),
            addr,
        };
    }

    pub async fn serve(mut self) -> Result<(), Box<dyn Error + Send>> {
        if let Some(router) = self.router.take() {
            router
                .serve(self.addr)
                .await
                .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;
        }
        Ok(())
    }
}

#[tonic::async_trait]
impl Rustchain for RustchainService {
    async fn send_block(
        &self,
        request: Request<Block>,
    ) -> Result<Response<RustchainResponse>, Status> {
        unimplemented!();
    }

    async fn send_transaction(
        &self,
        request: Request<Transaction>,
    ) -> Result<Response<RustchainResponse>, Status> {
        println!("Got a request: {:?}", request);
        let req = request.into_inner();
        let data = Data::Transaction(req.clone()).into();
        let inputs: UtxoInputs = req.inputs.into();
        let outputs: UtxoOutputs = req.outputs.into();
        let reply = RustchainResponse {
            successful: true,
            message: format!(
                "Sent {}ATokens to {}.",
                inputs,
                outputs,
            ),
            data,
        };
        Ok(Response::new(reply))
    }

    async fn validate(&self, _: Request<ValidationRequest>) -> Result<Response<RustchainResponse>, Status> {
        println!("Got a validation request");
        Ok(Response::new(RustchainResponse::default()))
    }
}


#[tonic::async_trait]
impl P2p for P2pService {
    async fn get_peers(&self, req: Request<GetPeersRequest>)-> Result<Response<PeerList>, Status> {
        Ok(Response::new(PeerList::default()))
    }

    async fn add_peer(&self, req: Request<Peer>)-> Result<Response<AddPeerResponse>, Status> {
        Ok(Response::new(AddPeerResponse::default()))
    }

    async fn remove_peer(&self, req: Request<Peer>)-> Result<Response<RemovePeerResponse>, Status> {
        Ok(Response::new(RemovePeerResponse::default()))
    }

    async fn send_heartbeat(&self, req: Request<Heartbeat>) -> Result<Response<Null>, Status> {
        Ok(Response::new(Null::default()))
    }
}
