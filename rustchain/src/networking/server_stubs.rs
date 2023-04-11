use crate::protos::transaction_response::Data;
pub use crate::protos::transaction_exchange_server::{TransactionExchange, TransactionExchangeServer};
use crate::protos::{Transaction, TransactionResponse};
use crate::protos::{ValidationRequest, ValidationResponse};
use std::error::Error;
use std::net::SocketAddr;
use tonic::transport::server::Router;
pub use tonic::{transport::Server, Request, Response, Status};

#[derive(Debug, Default)]
struct TransactionService {}

pub struct PeerServer {
    server: Option<Server>,
    router: Option<Router>,
    addr: SocketAddr,
}

impl PeerServer {
    // let addr = "[::1]:50051".parse().unwrap();
    pub fn new(addr: SocketAddr) -> PeerServer {
        let payment_service = TransactionService::default();
        let mut server = Server::builder();
        let router = server.add_service(TransactionExchangeServer::new(payment_service));
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
impl TransactionExchange for TransactionService {
    async fn send_transaction(
        &self,
        request: Request<Transaction>,
    ) -> Result<Response<TransactionResponse>, Status> {
        println!("Got a request: {:?}", request);
        let req = request.into_inner();
        let reply = TransactionResponse {
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

    async fn validate(
        &self,
        _: Request<ValidationRequest>,
    ) -> Result<Response<ValidationResponse>, Status> {
        println!("Got a validation request");
        let reply = ValidationResponse {};
        Ok(Response::new(reply))
    }
}
