use anyhow::Result;
use nostr_sdk::prelude::*;
use tokio::sync::mpsc;
use tracing::{debug, error, info};

use super::{ChatMessage, NetworkEvent};

/// Nostr-based signaling service for peer discovery
pub struct NostrSignaling {
    client: Client,
    event_tx: mpsc::UnboundedSender<NetworkEvent>,
    peer_id: String,
}

impl NostrSignaling {
    /// Create a new Nostr signaling service
    pub async fn new(
        peer_id: String,
        event_tx: mpsc::UnboundedSender<NetworkEvent>,
    ) -> Result<Self> {
        let keys = Keys::generate();
        let client = Client::new(&keys);

        // Connect to public Nostr relays
        client.add_relay("wss://relay.damus.io").await?;
        client.add_relay("wss://relay.nostr.band").await?;
        client.add_relay("wss://nos.lol").await?;
        
        client.connect().await;

        Ok(Self {
            client,
            event_tx,
            peer_id,
        })
    }

    /// Publish peer information for signaling
    pub async fn announce_presence(&self) -> Result<()> {
        let content = serde_json::json!({
            "type": "presence",
            "peer_id": self.peer_id,
            "timestamp": std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs(),
        });

        let builder = EventBuilder::new(
            Kind::Custom(30078), // Custom application-specific kind
            content.to_string(),
            [],
        );

        self.client.send_event_builder(builder).await?;
        info!("Announced presence on Nostr");

        Ok(())
    }

    /// Subscribe to signaling events
    pub async fn subscribe(&self) -> Result<()> {
        let filter = Filter::new()
            .kind(Kind::Custom(30078))
            .since(Timestamp::now());

        self.client.subscribe(vec![filter], None).await?;
        info!("Subscribed to Nostr signaling events");

        Ok(())
    }

    /// Run the signaling event loop
    pub async fn run(&mut self) -> Result<()> {
        info!("Starting Nostr signaling loop");
        
        self.client
            .handle_notifications(|notification| async {
                if let RelayPoolNotification::Event { event, .. } = notification {
                    debug!("Received Nostr event: {:?}", event);
                    
                    // Parse the event content
                    if let Ok(content) = serde_json::from_str::<serde_json::Value>(&event.content) {
                        if content.get("type").and_then(|t| t.as_str()) == Some("presence") {
                            if let Some(peer_id) = content.get("peer_id").and_then(|p| p.as_str()) {
                                if peer_id != self.peer_id {
                                    let _ = self.event_tx.send(NetworkEvent::PeerConnected(peer_id.to_string()));
                                }
                            }
                        }
                    }
                }
                Ok(false)
            })
            .await?;

        Ok(())
    }
}
