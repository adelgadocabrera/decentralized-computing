use tonic::{transport::Server, Request, Response, Status};

use blockchain::bitcoin_server::{Bitcoin, BitcoinServer};
use blockchain::{BtcPaymentResponse, BtcPaymentRequest};
use blockchain::{ValidationRequest, ValidationResponse};

pub mod blockchain {
    mod merkle;

    tonic::include_proto!("blockchain");
}

#[derive(Debug, Default)]
pub struct BitcoinService {}

#[tonic::async_trait]
impl Bitcoin for BitcoinService {
    async fn send_payment(
        &self,
        request: Request<BtcPaymentRequest>,
    ) -> Result<Response<BtcPaymentResponse>, Status> {
        println!("Got a request: {:?}", request);
        let req = request.into_inner();
        let reply = BtcPaymentResponse {
            successful: true,
            message: format!("Sent {}BTC to {}.", req.amount, req.to_addr).into(),
        };
        Ok(Response::new(reply))
    }

    async fn validate(
        &self,
        _: Request<ValidationRequest>,
    ) -> Result<Response<ValidationResponse>, Status> {
        println!("Got a validation request");
        let reply = ValidationResponse{};
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let btc_service = BitcoinService::default();
    Server::builder()
        .add_service(BitcoinServer::new(btc_service))
        .serve(addr)
        .await?;
    Ok(())
}
