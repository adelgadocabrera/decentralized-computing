use crate::protos::{Block, Heartbeat, PeerList, Transaction};

#[derive(Clone)]
pub enum RustchainEvent {
    NewBlock(Block),
    NewTransaction(Transaction),
    NewPeers(PeerList),
    NewHeartbeat(Heartbeat),
}

pub enum P2pEvent {
    NewPeers(PeerList),
    NewHeartbeat(Heartbeat),
}
