use crate::event_bus::events::RustchainEvent;
use crate::{
    blockchain::block::Block,
    event_bus::event_bus::EventBus,
    protos::{
        p2p_server::{P2p, P2pServer},
        response::Data,
        rustchain_server::{Rustchain, RustchainServer},
        AddPeerResponse, GetPeersRequest, Heartbeat, Null, Peer, PeerList, RemovePeerResponse,
        Response as RustchainResponse, Transaction, UtxoInputs, UtxoOutputs, ValidationRequest,
    },
};

use std::net::SocketAddr;
use std::{error::Error, sync::Arc};
use tokio::sync::RwLock;
use tonic::transport::server::Router;
pub use tonic::{transport::Server, Request, Response, Status};

use super::networking::get_self_addr;

#[derive(Debug)]
struct RustchainService {
    event_bus: Arc<RwLock<EventBus>>,
}

#[derive(Debug)]
struct P2pService {
    event_bus: Arc<RwLock<EventBus>>,
}

pub struct PeerServer {
    // server: Server, // do I really need this? simplify!
    router: Router,
    addr: SocketAddr,
}

impl PeerServer {
    pub fn new(event_bus: Arc<RwLock<EventBus>>) -> PeerServer {
        let payment_service = RustchainService {
            event_bus: event_bus.clone(),
        };
        let p2p_service = P2pService {
            event_bus: event_bus.clone(),
        };
        let mut server = Server::builder();
        let router = server
            .add_service(RustchainServer::new(payment_service))
            .add_service(P2pServer::new(p2p_service));
        // add additional services to router here..
        return PeerServer {
            // server,
            router,
            addr: get_self_addr(),
        };
    }

    pub async fn serve(self) -> Result<(), Box<dyn Error + Send>> {
        self.router
            .serve(self.addr)
            .await
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;
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
        let tx = request.into_inner();
        self.event_bus
            .write()
            .await
            .publish(RustchainEvent::NewTransaction(tx.clone()))
            .await;
        let data = Data::Transaction(tx.clone()).into();
        let inputs: UtxoInputs = tx.inputs.into();
        let outputs: UtxoOutputs = tx.outputs.into();
        let reply = RustchainResponse {
            successful: true,
            message: format!("Sent {}ATokens to {}.", inputs, outputs,),
            data,
        };
        Ok(Response::new(reply))
    }

    async fn validate(
        &self,
        _: Request<ValidationRequest>,
    ) -> Result<Response<RustchainResponse>, Status> {
        println!("Got a validation request");
        Ok(Response::new(RustchainResponse::default()))
    }
}

#[tonic::async_trait]
impl P2p for P2pService {
    async fn get_peers(&self, req: Request<GetPeersRequest>) -> Result<Response<PeerList>, Status> {
        Ok(Response::new(PeerList::default()))
    }

    async fn add_peer(&self, req: Request<Peer>) -> Result<Response<AddPeerResponse>, Status> {
        Ok(Response::new(AddPeerResponse::default()))
    }

    async fn remove_peer(
        &self,
        req: Request<Peer>,
    ) -> Result<Response<RemovePeerResponse>, Status> {
        Ok(Response::new(RemovePeerResponse::default()))
    }

    async fn send_heartbeat(&self, req: Request<Heartbeat>) -> Result<Response<Null>, Status> {
        Ok(Response::new(Null::default()))
    }
}
