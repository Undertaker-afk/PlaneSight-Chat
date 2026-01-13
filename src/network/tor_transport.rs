#[cfg(feature = "tor")]
use arti_client::{TorClient, TorClientConfig};
#[cfg(feature = "tor")]
use tor_rtcompat::PreferredRuntime;
#[cfg(feature = "tor")]
use anyhow::Result;
#[cfg(feature = "tor")]
use tracing::info;

#[cfg(feature = "tor")]
pub struct TorTransport {
    client: TorClient<PreferredRuntime>,
}

#[cfg(feature = "tor")]
impl TorTransport {
    /// Create a new Tor transport
    pub async fn new() -> Result<Self> {
        info!("Initializing Tor client...");
        
        let config = TorClientConfig::default();
        let client = TorClient::create_bootstrapped(config).await?;
        
        info!("Tor client bootstrapped successfully");

        Ok(Self { client })
    }

    /// Get the Tor client
    pub fn client(&self) -> &TorClient<PreferredRuntime> {
        &self.client
    }
}

#[cfg(not(feature = "tor"))]
pub struct TorTransport;

#[cfg(not(feature = "tor"))]
impl TorTransport {
    pub async fn new() -> anyhow::Result<Self> {
        anyhow::bail!("Tor support not compiled in. Enable the 'tor' feature.")
    }
}
