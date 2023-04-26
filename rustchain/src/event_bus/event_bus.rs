use crate::event_bus::events::RustchainEvent;
use std::sync::Arc;
use tokio::{
    spawn,
    sync::{
        mpsc::{channel, Receiver, Sender},
        RwLock,
    },
};

#[derive(Debug, Clone)]
pub struct EventBus {
    sender: Sender<RustchainEvent>,
    subscribers: Vec<Sender<RustchainEvent>>,
}

impl EventBus {
    pub async fn new() -> Arc<RwLock<EventBus>> {
        let (sender, mut receiver) = channel(100);
        let event_bus = Arc::new(RwLock::new(Self {
            sender,
            subscribers: vec![],
        }));
        let event_bus_clone = Arc::clone(&event_bus);
        spawn(async move {
            while let Some(event) = receiver.recv().await {
                (event_bus_clone.write().await).dispatch(event).await;
            }
        });
        event_bus
    }

    pub async fn subscribe(&mut self) -> Receiver<RustchainEvent> {
        let (sender, receiver) = channel(100);
        self.subscribers.push(sender);
        receiver
    }

    pub async fn publish(&self, event: RustchainEvent) {
        let _ = self.sender.send(event).await;
    }

    async fn dispatch(&self, event: RustchainEvent) {
        for subscriber in &self.subscribers {
            let _ = subscriber.send(event.clone()).await;
        }
    }
}
