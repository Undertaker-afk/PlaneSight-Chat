# PlaneSight-Chat

**Plain Sight is often the most hidden**

A secure peer-to-peer chat application built with Rust and Dioxus, using the infinite monkey principle for message transmission. Instead of sending actual message content, PlaneSight exchanges only mathematical coordinates (seed + position) that deterministically generate the message text.

## 🌟 Features

- **Infinite Monkey Principle**: Messages are transmitted as coordinates in infinite text space
- **P2P Architecture**: Direct peer-to-peer communication using libp2p
- **Dual Mode Operation**:
  - **Standard Mode**: Fast peer discovery via Nostr relays
  - **Hidden Mode**: Maximum privacy using Tor for transport and signaling
- **End-to-End Encryption**: Additional AES-256-GCM encryption layer
- **Cross-Platform**: Built with Dioxus for desktop and Android
- **No Central Server**: Fully decentralized architecture

## 🔐 How It Works

### The Infinite Monkey Principle

Traditional chat apps send the actual message content over the network. PlaneSight takes a different approach:

1. **Message Encoding**: Your message is represented as coordinates (seed, start position, length) in an infinite pseudorandom text space
2. **Transmission**: Only these small coordinates are sent to peers, not the actual message
3. **Message Decoding**: The recipient uses the coordinates to deterministically generate the same text

This provides an additional layer of security and plausible deniability - the network only sees mathematical coordinates, not message content.

### Network Modes

#### Standard Mode (Nostr Signaling)
- Uses public Nostr relays for peer discovery
- Fast connection establishment
- Suitable for most users

#### Hidden Mode (Tor)
- Routes all traffic through Tor network
- Anonymous peer discovery
- Maximum privacy at the cost of speed
- Requires `tor` feature compilation

## 🚀 Getting Started

### Prerequisites

- Rust 1.70 or later
- For Android builds: Android SDK and NDK

### Building for Desktop

```bash
# Clone the repository
git clone https://github.com/Undertaker-afk/PlaneSight-Chat.git
cd PlaneSight-Chat

# Build and run (default: desktop with standard mode)
cargo run --features desktop

# Build with Tor support
cargo run --features "desktop,tor"
```

### Building for Android

```bash
# Install Dioxus CLI
cargo install dioxus-cli

# Add Android targets
rustup target add aarch64-linux-android armv7-linux-androideabi

# Build APK
dx build --platform android --release

# Or build and deploy to connected device
dx serve --platform android
```

### First Run

On first launch, you'll be prompted to select a network mode:

1. **Standard Mode**: Recommended for most users
   - Fast peer discovery
   - Uses public Nostr relays
   - End-to-end encrypted

2. **Hidden Mode**: For maximum privacy (requires `tor` feature)
   - All traffic routed through Tor
   - Anonymous connections
   - Slower but more secure

## 🏗️ Project Structure

```
planesight-chat/
├── src/
│   ├── main.rs              # Application entry point
│   ├── monkey/              # Infinite monkey principle implementation
│   │   └── mod.rs           # Coordinate generation and text derivation
│   ├── crypto/              # Encryption layer
│   │   └── mod.rs           # AES-GCM encryption/decryption
│   ├── network/             # Networking layer
│   │   ├── mod.rs           # Network types and events
│   │   ├── p2p.rs           # libp2p implementation
│   │   ├── nostr_signaling.rs  # Nostr-based peer discovery
│   │   └── tor_transport.rs    # Tor integration (optional)
│   └── ui/                  # User interface
│       ├── mod.rs
│       ├── app.rs           # Main UI components
│       ├── components.rs    # Reusable UI elements
│       └── styles.css       # Styling
├── Cargo.toml               # Dependencies and configuration
├── Dioxus.toml             # Dioxus/Android configuration
└── README.md               # This file
```

## 🔧 Configuration

### Environment Variables

- `RUST_LOG`: Control logging level (default: `planesight_chat=debug,libp2p=info`)

### Features

- `desktop`: Enable desktop platform support (default)
- `mobile`: Enable Android/iOS platform support
- `tor`: Enable Tor transport and hidden mode

## 🛠️ Development

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test module
cargo test monkey::tests
```

### Code Structure

The application is organized into several key modules:

- **monkey**: Implements the infinite monkey principle for coordinate-based messaging
- **crypto**: Provides additional encryption layer using AES-256-GCM
- **network**: Handles P2P networking with libp2p, Nostr signaling, and optional Tor
- **ui**: Dioxus-based user interface for setup and chat

## 🔒 Security Considerations

1. **Coordinate-Based Messaging**: Messages are represented as coordinates, providing plausible deniability
2. **End-to-End Encryption**: Additional AES-256-GCM encryption layer
3. **P2P Architecture**: No central server to compromise
4. **Tor Support**: Optional routing through Tor network for anonymity
5. **No Message Storage**: Messages exist only in memory during the session

## 📱 Android Permissions

The Android app requires the following permissions:
- `INTERNET`: For P2P networking
- `ACCESS_NETWORK_STATE`: For connection monitoring

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

This project is open source. See LICENSE file for details.

## ⚠️ Disclaimer

This is experimental software. While it implements multiple layers of security, it should not be relied upon for critical communications without thorough security auditing. The "infinite monkey principle" implementation is a novel approach that prioritizes privacy and plausible deniability over traditional encryption-only approaches.

## 🙏 Acknowledgments

- Built with [Dioxus](https://dioxuslabs.com/) for cross-platform UI
- P2P networking powered by [libp2p](https://libp2p.io/)
- Nostr integration via [nostr-sdk](https://github.com/rust-nostr/nostr)
- Tor support through [Arti](https://gitlab.torproject.org/tpo/core/arti)
