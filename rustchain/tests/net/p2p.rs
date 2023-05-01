#[cfg(test)]
pub mod test {
    use protos::Transaction;
    use rustchain::blockchain::wallet::Wallet;
    use rustchain::event_bus::event_bus::EventBus;
    use rustchain::net::client_stubs::PeerClient;
    use rustchain::net::networking::get_addr;
    use rustchain::net::server_stubs::PeerServer;
    use rustchain::protos::response::Data;
    use rustchain::protos::{self, UtxoInput, UtxoOutput};
    use std::error::Error;
    use std::time::Duration;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_payment() -> Result<(), Box<dyn Error>> {
        // setup server
        let server_ip = "[::1]";
        let server_port = 5000;
        let server_addr = get_addr(server_ip, server_port);
        let peer_server = PeerServer::new(EventBus::new().await, server_addr);
        let server_handle = tokio::spawn(async { peer_server.serve().await });

        // buffer time
        sleep(Duration::from_millis(100)).await;

        // setup client && send transaction
        let event_bus = EventBus::new().await;
        let bob = Wallet::new(event_bus.clone()).await;
        let (from_addr, public_key): (String, Vec<u8>) = {
            let bob_read = bob.read().await;
            (
                bob_read.get_address(),
                bob_read.public_key_string().unwrap().into(),
            )
        };
        let amount: u32 = 22;
        let prev_tx_hash: Vec<u8> = "previous_tx".into();
        let mut tx = Transaction::default();
        let utxo_input = UtxoInput {
            from_addr: from_addr.clone(),
            public_key,
            prev_tx_hash,
            output_index: 0,
            signature: vec![],
        };
        let _ = bob.read().await.sign_transaction(&mut utxo_input.clone());
        let alice = Wallet::new(event_bus.clone()).await;
        let to_addr = alice.read().await.get_address();
        let utxo_output = UtxoOutput {
            to_addr: to_addr.clone(),
            amount,
        };
        tx.inputs.push(utxo_input);
        tx.outputs.push(utxo_output);

        // send tx
        let peer_client: PeerClient = PeerClient::new(server_ip, server_port).await?;
        let resp = peer_client.send_transaction(tx).await?;

        // buffer time
        sleep(Duration::from_millis(100)).await;
        server_handle.abort(); // close server

        if let Some(data) = resp.data {
            match data {
                Data::Transaction(payment_request) => {
                    let output = payment_request.outputs.get(0).unwrap();
                    let input = payment_request.inputs.get(0).unwrap();
                    assert_eq!(amount, output.amount);
                    assert_eq!(from_addr.clone(), input.from_addr.clone());
                    assert_eq!(to_addr.clone(), output.to_addr.clone());
                }
                _ => {
                    // Handle other cases here
                }
            }
        }
        Ok(())
    }
}
