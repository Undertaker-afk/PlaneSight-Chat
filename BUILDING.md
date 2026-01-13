# Building and Compilation Notes

## Current Status

The project structure is complete with the following components:

### ✅ Working Components:
- **Infinite Monkey Principle** (`src/monkey/mod.rs`): Core coordinate-based messaging
- **Crypto Layer** (`src/crypto/mod.rs`): AES-256-GCM encryption
- **Network Types** (`src/network/mod.rs`): Network event types and modes
- **Nostr Signaling** (`src/network/nostr_signaling.rs`): Peer discovery via Nostr
- **Tor Transport** (`src/network/tor_transport.rs`): Optional Tor integration
- **UI Components** (`src/ui/`): Dioxus-based user interface

### 🔧 Known Compilation Issues:

#### libp2p NetworkBehaviour Derive
The `P2PBehaviour` struct in `src/network/p2p.rs` requires careful version alignment between libp2p crates. The `NetworkBehaviour` derive macro has API changes between versions 0.53-0.54.

**Workaround**: The p2p module can be conditionally compiled or the application can be initially built with only Nostr signaling for peer discovery.

## Building the Project

### For Development (without p2p module):
```bash
# Test core modules
cargo test --lib --no-default-features monkey crypto

# Check core modules
cargo check --lib --no-default-features
```

### For Android:
```bash
# Install Dioxus CLI
cargo install dioxus-cli

# Build for Android
dx build --platform android --release
```

## Next Steps for Full Compilation

1. **Update libp2p**: Align all libp2p crate versions to 0.54+
2. **Fix NetworkBehaviour**: Ensure the derive macro has all required dependencies
3. **Test on Desktop**: Requires GTK dependencies for dioxus-desktop
4. **Test on Android**: Use `dx serve --platform android` with a connected device

## Architecture Notes

The application follows a modular design where each layer can be independently tested:

```
├── Monkey Engine: Coordinate generation (Pure Rust, no dependencies)
├── Crypto Layer: Message encryption (Uses aes-gcm)
├── Network Layer: 
│   ├── P2P (libp2p) - Direct peer connections
│   ├── Nostr - Public relay signaling
│   └── Tor - Anonymous transport
└── UI Layer: Dioxus components for cross-platform rendering
```

This allows for progressive implementation and testing of each component.
