pub mod p2p;
pub mod nostr_signaling;

#[cfg(feature = "tor")]
pub mod tor_transport;

use crate::monkey::Coordinates;
use serde::{Deserialize, Serialize};

/// Message exchanged between peers (only coordinates, not the actual text)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub from: String,
    pub to: String,
    pub coordinates: Coordinates,
    pub timestamp: u64,
    pub nonce: Vec<u8>, // For encryption
}

/// Network mode selection
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum NetworkMode {
    /// Standard mode using Nostr for signaling
    Standard,
    /// Hidden mode using Tor for transport and signaling
    Hidden,
}

/// Events from the network layer
#[derive(Debug, Clone)]
pub enum NetworkEvent {
    /// Connected to a peer
    PeerConnected(String),
    /// Disconnected from a peer
    PeerDisconnected(String),
    /// Received a message
    MessageReceived(ChatMessage),
    /// Error occurred
    Error(String),
}
