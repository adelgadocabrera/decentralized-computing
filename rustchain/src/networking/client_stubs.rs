use crate::protos::transaction_exchange_client::TransactionExchangeClient;
use crate::protos::{Transaction, TransactionResponse};
use std::error::Error;
use tonic::transport::Channel;
use tonic::Request;
use tonic::transport::Endpoint;

pub struct PeerClient {
    conn: TransactionExchangeClient<Channel>,
}

impl PeerClient {
    pub async fn new(to: &str, port: u16) -> Result<PeerClient, Box<dyn Error>> {
        let target = format!("https://{}:{}", to, port);
        let endpoint = Endpoint::from_shared(target)?;
        let channel = endpoint.connect().await?;
        let conn = TransactionExchangeClient::new(channel);
        Ok(PeerClient { conn })
    }

    pub async fn send_transaction(
        mut self,
        from_addr: String,
        to_addr: String,
        amount: u32,
    ) -> Result<TransactionResponse, Box<dyn Error>> {
        let tx = Request::new(Transaction {
            from_addr,
            to_addr,
            amount,
            additional_data: String::from(""), // TODO: empty for now
        });
        let req = self.conn.send_transaction(tx).await;
        match req {
            Ok(resp) => Ok(resp.into_inner()),
            Err(e) => Err(Box::new(e)),
        }
    }
}
