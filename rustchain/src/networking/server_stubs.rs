use crate::{protos::rc_response::Data, blockchain::block::Block};
pub use crate::protos::rustchain_server::{Rustchain, RustchainServer};
use crate::protos::{Transaction,ValidationRequest, ValidationResponse, RcResponse};
use std::error::Error;
use std::net::SocketAddr;
use tonic::transport::server::Router;
pub use tonic::{transport::Server, Request, Response, Status};

#[derive(Debug, Default)]
struct RustchainService {}

pub struct PeerServer {
    server: Option<Server>,
    router: Option<Router>,
    addr: SocketAddr,
}

impl PeerServer {
    // let addr = "[::1]:50051".parse().unwrap();
    pub fn new(addr: SocketAddr) -> PeerServer {
        let payment_service = RustchainService::default();
        let mut server = Server::builder();
        let router = server.add_service(RustchainServer::new(payment_service));
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
    ) -> Result<Response<RcResponse>, Status> {
        unimplemented!();
    }

    async fn send_transaction(
        &self,
        request: Request<Transaction>,
    ) -> Result<Response<RcResponse>, Status> {
        println!("Got a request: {:?}", request);
        let req = request.into_inner();
        let reply = RcResponse {
            successful: true,
            message: format!(
                "Sent {}ATokens to {}.",
                req.amount,
                req.to_addr,
            ),
            data: Data::Transaction(req.to_owned()).into(),
        };
        Ok(Response::new(reply))
    }

    async fn validate(&self, _: Request<ValidationRequest>) -> Result<Response<RcResponse>, Status> {
        println!("Got a validation request");
        Ok(Response::new(RcResponse::default()))
    }
}
