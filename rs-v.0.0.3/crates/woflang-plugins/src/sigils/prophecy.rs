//! Prophecy operations for Woflang.
//!
//! Cryptic stack fate messages and prophecy chains.
//! - `prophecy` - Generate a random prophecy
//! - `prophecy_chain` - View all prophecies from this session

use std::sync::{Mutex, OnceLock};
use woflang_core::WofValue;
use woflang_runtime::Interpreter;

/// The sacred prophecies.
static PROPHECIES: &[&str] = &[
    "In the glyph's shadow, your stack's fate is sealed.",
    "Beware: the next push may tip the void.",
    "The stack will echo your intent, not your command.",
    "A silent glyph is the most powerful of all.",
    "When the top is light, the bottom bears the weight.",
    "Three swaps from now, a revelation will surface.",
    "Between ∅ and ∞, your next op chooses the path.",
    "The slayer sleeps… for now.",
    "A stack unbalanced is a prophecy unfulfilled.",
    "Beware the glyph echoing twice.",
    "The void grows with each lost symbol.",
];

/// Chain of prophecies revealed this session.
fn prophecy_chain() -> &'static Mutex<Vec<String>> {
    static CHAIN: OnceLock<Mutex<Vec<String>>> = OnceLock::new();
    CHAIN.get_or_init(|| Mutex::new(Vec::new()))
}

/// Get a pseudo-random prophecy.
fn random_prophecy() -> &'static str {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    PROPHECIES[(nanos as usize) % PROPHECIES.len()]
}

/// Register prophecy operations.
pub fn register(interp: &mut Interpreter) {
    // Generate a random prophecy
    // Stack: → prophecy_string
    interp.register("prophecy", |interp| {
        let chosen = random_prophecy().to_string();
        
        // Add to chain
        if let Ok(mut chain) = prophecy_chain().lock() {
            chain.push(chosen.clone());
        }
        
        println!("[Prophecy] {}", chosen);
        
        interp.stack_mut().push(WofValue::string(chosen));
        Ok(())
    });

    // View the prophecy chain
    // Stack: →
    interp.register("prophecy_chain", |_interp| {
        println!("🔗  Prophecy Chain:");
        
        if let Ok(chain) = prophecy_chain().lock() {
            if chain.is_empty() {
                println!("  (no prophecies yet revealed)");
            } else {
                for p in chain.iter() {
                    println!("  {}", p);
                }
            }
        }
        
        Ok(())
    });

    // Clear the prophecy chain
    // Stack: →
    interp.register("prophecy_clear", |_interp| {
        if let Ok(mut chain) = prophecy_chain().lock() {
            chain.clear();
        }
        println!("[Prophecy] The chain has been broken. All is forgotten.");
        Ok(())
    });

    // Count prophecies in chain
    // Stack: → count
    interp.register("prophecy_count", |interp| {
        let count = prophecy_chain()
            .lock()
            .map(|c| c.len())
            .unwrap_or(0);
        interp.stack_mut().push(WofValue::integer(count as i64));
        Ok(())
    });
}
