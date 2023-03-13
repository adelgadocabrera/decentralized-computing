use blockchain::BtcPaymentResponse;
use blockchain::bitcoin_client::BitcoinClient;
use blockchain::BtcPaymentRequest;
use tonic::transport::Channel;
use std::error::Error;

pub mod blockchain {
    tonic::include_proto!("blockchain");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = BitcoinClient::connect("http://[::1]:50051").await?;
    let request = BtcPaymentRequest {
        from_addr: "123456".to_owned(),
        to_addr: "654321".to_owned(),
        amount: 22,
    };
    let payment: BtcPaymentResponse = send_btc_payment(client, request).await?;
    print!("{:?}", payment);
    Ok(())
}

async fn send_btc_payment(
    mut client: BitcoinClient<Channel>,
    request: BtcPaymentRequest,
) -> Result<BtcPaymentResponse, Box<dyn std::error::Error>> {
    let request = tonic::Request::new(request);
    let response = client.send_payment(request).await?.into_inner();
    Ok(response)
}

