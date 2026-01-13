use anyhow::{Context, Result};
use futures::StreamExt;
use libp2p::{
    gossipsub, identify, mdns, noise, ping,
    swarm::{NetworkBehaviour, SwarmEvent},
    tcp, yamux, PeerId, Swarm,
};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::Duration;
use tokio::sync::mpsc;
use tracing::{debug, error, info};

use super::{ChatMessage, NetworkEvent};

/// P2P Network behaviour combining multiple protocols
#[derive(NetworkBehaviour)]
pub struct P2PBehaviour {
    pub gossipsub: gossipsub::Behaviour,
    pub mdns: mdns::Behaviour,
    pub identify: identify::Behaviour,
    pub ping: ping::Behaviour,
}

/// P2P Network manager
pub struct P2PNetwork {
    swarm: Swarm<P2PBehaviour>,
    event_tx: mpsc::UnboundedSender<NetworkEvent>,
    chat_topic: gossipsub::IdentTopic,
}

impl P2PNetwork {
    /// Create a new P2P network
    pub async fn new(event_tx: mpsc::UnboundedSender<NetworkEvent>) -> Result<Self> {
        // Create a Gossipsub topic
        let chat_topic = gossipsub::IdentTopic::new("planesight-chat");

        // Create the Gossipsub behaviour config
        let message_id_fn = |message: &gossipsub::Message| {
            let mut s = DefaultHasher::new();
            message.data.hash(&mut s);
            gossipsub::MessageId::from(s.finish().to_string())
        };

        let gossipsub_config = gossipsub::ConfigBuilder::default()
            .heartbeat_interval(Duration::from_secs(10))
            .validation_mode(gossipsub::ValidationMode::Strict)
            .message_id_fn(message_id_fn)
            .build()
            .context("Invalid gossipsub config")?;

        let chat_topic_clone = chat_topic.clone();

        // Create the swarm
        let swarm = libp2p::SwarmBuilder::with_new_identity()
            .with_tokio()
            .with_tcp(
                tcp::Config::default(),
                noise::Config::new,
                yamux::Config::default,
            )?
            .with_behaviour(move |key| -> Result<P2PBehaviour, std::io::Error> {
                // Create behaviours with the generated identity
                let local_peer_id = PeerId::from(key.public());
                
                let mut gossipsub_new = gossipsub::Behaviour::new(
                    gossipsub::MessageAuthenticity::Signed(key.clone()),
                    gossipsub_config.clone(),
                ).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
                gossipsub_new.subscribe(&chat_topic_clone)
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
                
                let mdns_new = mdns::Behaviour::new(mdns::Config::default(), local_peer_id)?;
                let identify_new = identify::Behaviour::new(identify::Config::new(
                    "/planesight/1.0.0".to_string(),
                    key.public(),
                ));
                let ping_new = ping::Behaviour::new(ping::Config::new());
                
                Ok(P2PBehaviour {
                    gossipsub: gossipsub_new,
                    mdns: mdns_new,
                    identify: identify_new,
                    ping: ping_new,
                })
            })?
            .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(60)))
            .build();
            
        info!("P2P network created");

        Ok(Self {
            swarm,
            event_tx,
            chat_topic,
        })
    }

    /// Start listening on a specific address
    pub fn listen(&mut self, addr: &str) -> Result<()> {
        let listen_addr: libp2p::Multiaddr = addr.parse()?;
        self.swarm.listen_on(listen_addr)?;
        Ok(())
    }

    /// Send a message to all peers
    pub fn send_message(&mut self, message: &ChatMessage) -> Result<()> {
        let data = serde_json::to_vec(message)?;
        self.swarm
            .behaviour_mut()
            .gossipsub
            .publish(self.chat_topic.clone(), data)?;
        Ok(())
    }

    /// Run the network event loop
    pub async fn run(&mut self) {
        loop {
            match self.swarm.next().await {
                Some(SwarmEvent::Behaviour(event)) => {
                    // Handle gossipsub events
                    if let P2PBehaviourEvent::Gossipsub(gossipsub::Event::Message {
                        propagation_source: peer_id,
                        message,
                        ..
                    }) = event
                    {
                        debug!("Received message from {:?}", peer_id);
                        
                        match serde_json::from_slice::<ChatMessage>(&message.data) {
                            Ok(chat_msg) => {
                                let _ = self.event_tx.send(NetworkEvent::MessageReceived(chat_msg));
                            }
                            Err(e) => {
                                error!("Failed to deserialize message: {}", e);
                            }
                        }
                    }
                    // Handle mDNS events
                    else if let P2PBehaviourEvent::Mdns(mdns_event) = event {
                        match mdns_event {
                            mdns::Event::Discovered(peers) => {
                                for (peer_id, addr) in peers {
                                    debug!("Discovered peer: {:?} at {:?}", peer_id, addr);
                                    self.swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                                    let _ = self.event_tx.send(NetworkEvent::PeerConnected(peer_id.to_string()));
                                }
                            }
                            mdns::Event::Expired(peers) => {
                                for (peer_id, addr) in peers {
                                    debug!("Peer expired: {:?} at {:?}", peer_id, addr);
                                    self.swarm.behaviour_mut().gossipsub.remove_explicit_peer(&peer_id);
                                    let _ = self.event_tx.send(NetworkEvent::PeerDisconnected(peer_id.to_string()));
                                }
                            }
                            _ => {}
                        }
                    }
                }
                Some(SwarmEvent::NewListenAddr { address, .. }) => {
                    info!("Listening on {:?}", address);
                }
                Some(SwarmEvent::ConnectionEstablished { peer_id, .. }) => {
                    info!("Connected to {:?}", peer_id);
                }
                Some(SwarmEvent::ConnectionClosed { peer_id, cause, .. }) => {
                    info!("Disconnected from {:?}: {:?}", peer_id, cause);
                }
                Some(event) => {
                    debug!("Unhandled swarm event: {:?}", event);
                }
                None => break,
            }
        }
    }
}
