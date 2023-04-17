use rustchain::networking::client_stubs::PeerClient;
use rustchain::networking::server_stubs::PeerServer;
use rustchain::protos;
use std::error::Error;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::time::sleep;
use protos::response::Data::{Transaction, ValidationRequest, Block};

#[tokio::test]
async fn test_payment() -> Result<(), Box<dyn Error>> {
    // setup server
    let server_ip  = "[::1]";
    let server_port: u16 = 50051;
    let addr: SocketAddr = format!("{}:{}", "[::1]", server_port)
        .parse()
        .unwrap();
    let peer_server = PeerServer::new(addr);
    let server_handle = tokio::spawn(async { peer_server.serve().await });

    // buffer time
    sleep(Duration::from_millis(100)).await; // buffer time

    // setup client && send transaction
    let peer_client = PeerClient::new(server_ip, server_port).await?;
    let from_addr = "123456";
    let to_addr = "654321";
    let amount = 22;
    let tx = peer_client.send_transaction(
        String::from(from_addr), 
        String::from(to_addr), 
        amount
    ).await?;

    // buffer time
    sleep(Duration::from_millis(100)).await; 
    server_handle.abort(); // close server
    
    if let Some(data) = tx.data {
        match data {
            Transaction(payment_request) => {
                assert_eq!(amount, payment_request.amount);
                assert_eq!(from_addr.to_owned(), payment_request.from_addr);
                assert_eq!(to_addr.to_owned(), payment_request.to_addr);
            }
            (_) => todo!(),
        }
    } 
    Ok(())
}
