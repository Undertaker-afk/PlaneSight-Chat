mod crypto;
mod monkey;
mod network;
mod ui;

use anyhow::Result;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "planesight_chat=debug,libp2p=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Launch the Dioxus app based on the platform
    #[cfg(feature = "desktop")]
    {
        dioxus_desktop::launch(ui::App);
    }

    #[cfg(all(feature = "mobile", not(feature = "desktop")))]
    {
        dioxus_mobile::launch(ui::App);
    }

    #[cfg(all(feature = "web", not(any(feature = "desktop", feature = "mobile"))))]
    {
        dioxus_web::launch(ui::App);
    }

    #[cfg(not(any(feature = "desktop", feature = "mobile", feature = "web")))]
    {
        eprintln!("No platform feature enabled. Please enable 'desktop', 'mobile', or 'web'.");
        eprintln!("Example: cargo run --features desktop");
        std::process::exit(1);
    }

    Ok(())
}
