pub mod crypto;
pub mod monkey;
//pub mod network; // Temporarily disabled due to libp2p version compatibility
//pub mod ui; // Temporarily disabled due to Dioxus API changes

// Re-export commonly used types
pub use monkey::{Coordinates, MonkeyEngine};
// pub use network::{NetworkMode, NetworkEvent, ChatMessage};
pub use crypto::CryptoService;
