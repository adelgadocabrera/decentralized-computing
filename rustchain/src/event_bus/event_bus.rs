use std::sync::Arc;
use tokio::sync::{mpsc::{channel, Receiver, Sender}, RwLock};
use crate::event_bus::events::BlockchainEvent;

#[derive(Debug, Clone)]
pub struct EventBus {
    sender: Sender<BlockchainEvent>,
    subscribers: Vec<Sender<BlockchainEvent>>,
}

impl EventBus {
    pub fn new() -> Arc<RwLock<EventBus>> {
        let (sender, mut receiver) = channel(100);
        let event_bus = Arc::new(RwLock::new(
            Self {
            sender,
            subscribers: vec![],
        }));
        let event_bus_clone = Arc::clone(&event_bus);
        tokio::spawn(async move {
            while let Some(event) = receiver.recv().await {
                (event_bus_clone.write().await).dispatch(event).await;
            }
        });
        event_bus
    }

    pub async fn subscribe(&mut self) -> Receiver<BlockchainEvent> {
        let (sender, receiver) = channel(100);
        self.subscribers.push(sender);
        receiver
    }

    pub async fn publish(&self, event: BlockchainEvent) {
        let _ = self.sender.send(event).await;
    }

    async fn dispatch(&self, event: BlockchainEvent) {
        for subscriber in &self.subscribers {
            let _ = subscriber.send(event.clone()).await;
        }
    }
}

