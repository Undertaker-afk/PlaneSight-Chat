use planesight_chat::{Coordinates, MonkeyEngine, CryptoService, crypto::derive_shared_secret};

fn main() {
    println!("PlaneSight Chat - Infinite Monkey Principle Demo\n");
    println!("{}", "=".repeat(50));
    
    // Initialize the monkey engine
    let engine = MonkeyEngine::new();
    
    // Demo 1: Basic coordinate generation
    println!("\n1. Generating coordinates for a message:");
    let message = "Hello, World!";
    println!("   Original message: '{}'", message);
    
    let coords = engine.encode_message(message);
    println!("   Coordinates generated:");
    println!("     - Seed: {}", coords.seed);
    println!("     - Start: {}", coords.start);
    println!("     - Length: {}", coords.length);
    
    // The generated text from coordinates (in a real implementation, this would
    // be the actual text from the infinite space)
    let generated = engine.decode_coordinates(&coords);
    println!("   Generated text: '{}'", generated);
    println!("   (In production, only coordinates are transmitted!)");
    
    // Demo 2: Encryption layer
    println!("\n2. Adding encryption layer:");
    let alice_id = "alice@planesight";
    let bob_id = "bob@planesight";
    
    let shared_secret = derive_shared_secret(alice_id, bob_id);
    println!("   Shared secret derived from peer IDs");
    
    let crypto = CryptoService::new(&shared_secret).unwrap();
    let encrypted = crypto.encrypt(message).unwrap();
    println!("   Message encrypted (nonce + ciphertext)");
    println!("   Ciphertext length: {} bytes", encrypted.ciphertext.len());
    
    let decrypted = crypto.decrypt(&encrypted).unwrap();
    println!("   Message decrypted: '{}'", decrypted);
    assert_eq!(decrypted, message);
    
    // Demo 3: Complete flow
    println!("\n3. Complete message flow:");
    println!("   Alice wants to send: 'Meet at the rendezvous'");
    let secret_message = "Meet at the rendezvous";
    
    // Step 1: Generate coordinates
    let coords = engine.encode_message(secret_message);
    println!("   ✓ Coordinates generated");
    
    // Step 2: Encrypt the coordinates (optional extra security)
    let coords_json = serde_json::to_string(&coords).unwrap();
    let encrypted_coords = crypto.encrypt(&coords_json).unwrap();
    println!("   ✓ Coordinates encrypted");
    
    // Step 3: Transmit (only encrypted coordinates, not the message!)
    println!("   ✓ Transmitted: {} bytes", encrypted_coords.ciphertext.len());
    println!("     (vs original message: {} bytes)", secret_message.len());
    
    // Step 4: Bob receives and decrypts
    let received_coords_json = crypto.decrypt(&encrypted_coords).unwrap();
    let received_coords: Coordinates = serde_json::from_str(&received_coords_json).unwrap();
    println!("   ✓ Coordinates decrypted");
    
    // Step 5: Generate text from coordinates
    let received_message = engine.decode_coordinates(&received_coords);
    println!("   ✓ Message reconstructed: '{}'", received_message);
    
    println!("\n{}", "=".repeat(50));
    println!("Benefits of this approach:");
    println!("  • Only coordinates are transmitted over the network");
    println!("  • Plausible deniability (coordinates look random)");
    println!("  • Additional encryption layer for security");
    println!("  • Perfect forward secrecy with ephemeral coordinates");
    println!("\nNote: This is a demonstration. In production, the infinite");
    println!("monkey principle would use actual pseudorandom text generation");
    println!("from a truly infinite space based on the seed and coordinates.");
}
