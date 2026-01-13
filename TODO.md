# TODO - Next Steps for PlaneSight Chat

## High Priority - Core Functionality

### [ ] Fix LibP2P Networking Module
**File**: `src/network/p2p.rs`

**Issue**: NetworkBehaviour derive macro not working with current libp2p version

**Steps**:
1. Update Cargo.toml libp2p dependencies to 0.56+:
   ```toml
   libp2p = { version = "0.56", features = ["tcp", "noise", "yamux", "gossipsub", "mdns", "kad", "identify", "ping", "tokio"] }
   ```
2. Fix mdns::Behaviour to include generic parameter:
   ```rust
   pub mdns: mdns::Behaviour<mdns::tokio::Tokio>,
   ```
3. Or manually implement NetworkBehaviour instead of using derive
4. Test compilation: `cargo check --lib`
5. Verify P2P message exchange works

### [ ] Fix Dioxus UI Components  
**Files**: `src/ui/app.rs`, `src/ui/components.rs`

**Issue**: Component API changed between Dioxus 0.4 and 0.5+

**Options**:
A. **Update to Dioxus 0.5+** (preferred for new features)
   ```toml
   dioxus = "0.5"
   dioxus-mobile = "0.5"
   ```
   - Update component signatures
   - Fix event handlers
   - Update rsx! macro usage

B. **Fix for Dioxus 0.4** (if mobile 0.5 not stable)
   - Add lifetime annotations: `Element<'a>`
   - Use #[component] macro correctly
   - Fix EventHandler usage

**Steps**:
1. Choose approach A or B above
2. Update src/ui/app.rs component signatures
3. Fix event handler definitions
4. Test: `cargo check --features mobile`

### [ ] Fix Nostr Signaling
**File**: `src/network/nostr_signaling.rs`

**Issue**: subscribe() method API changed

**Fix**:
```rust
let subscription_id = self.client.subscribe(vec![filter], None).await;
// Handle SubscriptionId instead of using ?
```

**Steps**:
1. Update subscribe call to handle SubscriptionId
2. Remove ? operator or wrap properly
3. Test compilation

### [ ] Re-enable Network and UI Modules
**File**: `src/lib.rs`

**Current State**: Modules commented out due to compilation errors

**Steps**:
1. Once p2p.rs and ui/ modules compile
2. Uncomment in lib.rs:
   ```rust
   pub mod network;
   pub mod ui;
   ```
3. Re-export types
4. Update main.rs to use modules

## Medium Priority - Integration

### [ ] Connect UI to Network Layer
**Create**: `src/app_state.rs` or similar

**Requirements**:
- Shared state between UI and network
- Message queue for sending/receiving
- Peer list management
- Network mode selection storage

**Suggested Approach**:
```rust
struct AppState {
    network_tx: mpsc::Sender<NetworkCommand>,
    message_rx: mpsc::Receiver<ChatMessage>,
    peers: Arc<RwLock<Vec<String>>>,
    mode: NetworkMode,
}
```

### [ ] Implement Message Sending
**Files**: UI components + network layer

**Flow**:
1. User types message in UI
2. UI → monkey engine: encode_message()
3. UI → crypto: encrypt(coordinates)
4. UI → network: send_message()
5. Network → P2P: broadcast encrypted coords

### [ ] Implement Message Receiving
**Flow**:
1. Network receives encrypted coords
2. Crypto → decrypt(coords)
3. Monkey engine → decode_coordinates()
4. Update UI with message

### [ ] Add Peer Management UI
- Display connected peers
- Show connection status
- Peer discovery indicators

## Low Priority - Polish

### [ ] Add Settings/Configuration
- Save network mode preference
- Custom peer bootstrap nodes
- Message history (optional)

### [ ] Improve Monkey Engine
- Optimize text generation performance
- Add compression for coordinates
- Consider alternative coordinate schemes

### [ ] Add Tests
- Integration tests for full message flow
- UI tests (if framework supports)
- Network layer tests (mock P2P)

### [ ] Error Handling
- Better error messages
- Recovery from network failures
- Handle malformed messages

## Android Specific

### [ ] Test Android Build
**Prerequisites**: UI and network layers working

**Steps**:
1. Install Android SDK and NDK
2. Configure `ANDROID_SDK_ROOT`and `ANDROID_NDK_ROOT`
3. Run: `dx build --platform android --release`
4. Fix any Android-specific compilation issues
5. Test on emulator or device

### [ ] Android Permissions
- Ensure `AndroidManifest.xml` has required permissions
- Handle runtime permissions for network access
- Test on various Android versions

### [ ] Optimize for Mobile
- Reduce binary size (already has strip = true)
- Test battery usage
- Optimize network for mobile connections

## Documentation

### [ ] API Documentation
- Add rustdoc comments to public APIs
- Generate docs: `cargo doc --open`

### [ ] User Guide
- How to use the app
- Network mode explanation
- Security considerations
- Troubleshooting

### [ ] Developer Guide
- Architecture overview (expand SUMMARY.md)
- How to contribute
- Testing procedures

## Future Enhancements

### [ ] Group Chat Support
- Multi-peer message distribution
- Group key management

### [ ] File Transfer
- Apply infinite monkey principle to file coordinates
- Chunk large files

### [ ] Contact Management
- Peer IDs/addresses storage
- Nicknames
- Trust/verification system

### [ ] Advanced Security
- Signal protocol integration
- Post-quantum cryptography option
- Steganography for coordinate transmission

## Quick Wins (Can Do Now)

### [ ] Add More Examples
- Example for crypto module usage
- Example for coordinate generation
- Network module example (once fixed)

### [ ] Improve Error Messages
- Better error types
- More context in errors
- User-friendly error display

### [ ] Add Logging
- Structured logging with tracing
- Log levels for debugging
- Network activity logging

---

## Getting Started with Contributions

1. **Pick a task** from "High Priority" section
2. **Create a branch**: `git checkout -b fix-libp2p-networking`
3. **Make changes** and test thoroughly
4. **Run tests**: `cargo test`
5. **Submit PR** with clear description

## Questions or Stuck?

- Check SUMMARY.md for architecture details
- Check BUILDING.md for compilation notes
- Run the working demo to understand the core concept
- Open an issue for help
