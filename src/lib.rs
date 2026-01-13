pub mod crypto;
pub mod monkey;
pub mod network;
pub mod ui;

// Re-export commonly used types
pub use monkey::{Coordinates, MonkeyEngine};
pub use network::{NetworkMode, NetworkEvent, ChatMessage};
pub use crypto::CryptoService;
