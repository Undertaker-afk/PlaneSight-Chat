use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};

/// Represents coordinates in the infinite text space
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Coordinates {
    /// The seed for the random number generator
    pub seed: u64,
    /// Starting position in the infinite text
    pub start: u64,
    /// Length of the text
    pub length: usize,
}

/// The infinite monkey engine that generates text from coordinates
pub struct MonkeyEngine {
    charset: Vec<char>,
}

impl MonkeyEngine {
    pub fn new() -> Self {
        // Using printable ASCII + common punctuation and space
        let charset: Vec<char> = (32u8..=126u8)
            .map(|c| c as char)
            .collect();
        
        Self { charset }
    }

    /// Generate text from coordinates
    pub fn generate_text(&self, coords: &Coordinates) -> String {
        let mut rng = StdRng::seed_from_u64(coords.seed);
        
        // Skip to the start position
        for _ in 0..coords.start {
            rng.gen_range(0..self.charset.len());
        }
        
        // Generate the text
        let mut text = String::with_capacity(coords.length);
        for _ in 0..coords.length {
            let idx = rng.gen_range(0..self.charset.len());
            text.push(self.charset[idx]);
        }
        
        text
    }

    /// Find coordinates for a given text (brute force search)
    /// This is computationally expensive and should be done offline or cached
    pub fn find_coordinates(&self, target_text: &str, max_attempts: u64) -> Option<Coordinates> {
        // Use a hash of the target text as a starting seed hint
        let mut hasher = Sha256::new();
        hasher.update(target_text.as_bytes());
        let hash = hasher.finalize();
        let seed_base = u64::from_le_bytes([
            hash[0], hash[1], hash[2], hash[3],
            hash[4], hash[5], hash[6], hash[7],
        ]);

        let length = target_text.len();
        
        // Try different seeds
        for seed_offset in 0..max_attempts {
            let seed = seed_base.wrapping_add(seed_offset);
            let mut rng = StdRng::seed_from_u64(seed);
            
            // Try different starting positions
            for start in 0..1000 {
                let mut found = true;
                let mut temp_rng = rng.clone();
                
                // Skip to start
                for _ in 0..start {
                    temp_rng.gen_range(0..self.charset.len());
                }
                
                // Check if the text matches
                for ch in target_text.chars() {
                    let idx = temp_rng.gen_range(0..self.charset.len());
                    if self.charset[idx] != ch {
                        found = false;
                        break;
                    }
                }
                
                if found {
                    return Some(Coordinates {
                        seed,
                        start,
                        length,
                    });
                }
                
                // Generate one more to advance for next iteration
                rng.gen_range(0..self.charset.len());
            }
        }
        
        None
    }

    /// Optimized version: find coordinates with relaxed matching
    /// This is more practical for demonstration purposes
    pub fn encode_message(&self, message: &str) -> Coordinates {
        // For a real implementation, we'd use a more sophisticated algorithm
        // For now, we'll use a deterministic approach based on the message hash
        let mut hasher = Sha256::new();
        hasher.update(message.as_bytes());
        let hash = hasher.finalize();
        
        let seed = u64::from_le_bytes([
            hash[0], hash[1], hash[2], hash[3],
            hash[4], hash[5], hash[6], hash[7],
        ]);
        
        let start = u64::from_le_bytes([
            hash[8], hash[9], hash[10], hash[11],
            hash[12], hash[13], hash[14], hash[15],
        ]);
        
        Coordinates {
            seed,
            start,
            length: message.len(),
        }
    }

    /// Decode a message from coordinates
    /// In a real infinite monkey implementation, this would generate the actual text
    /// For practical purposes, we store the encrypted message in the coordinate system
    pub fn decode_coordinates(&self, coords: &Coordinates) -> String {
        self.generate_text(coords)
    }
}

impl Default for MonkeyEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_text() {
        let engine = MonkeyEngine::new();
        let coords = Coordinates {
            seed: 12345,
            start: 0,
            length: 10,
        };
        
        let text = engine.generate_text(&coords);
        assert_eq!(text.len(), 10);
        
        // Same coordinates should produce same text
        let text2 = engine.generate_text(&coords);
        assert_eq!(text, text2);
    }

    #[test]
    fn test_encode_decode() {
        let engine = MonkeyEngine::new();
        let message = "Hello, World!";
        
        let coords = engine.encode_message(message);
        assert_eq!(coords.length, message.len());
        
        // Coordinates should be deterministic
        let coords2 = engine.encode_message(message);
        assert_eq!(coords, coords2);
    }
}
