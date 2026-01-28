//! Dreaming mode operations for Woflang.
//!
//! Surreal debug stream with mystical glyph traces.
//! - `:dreaming` - Generate surreal dreamlog trace

use woflang_core::{InterpreterContext, WofValue};
use woflang_runtime::Interpreter;

/// Mystical glyphs for the dreamlog.
static GLYPHS: &[&str] = &[
    "⟁", "◬", "𓂀", "₪", "⚚", "⌘", "☍", "⧖", "ᚠ", "ᚨ", "ᛟ"
];

/// Surreal verbs for the dreamlog.
static VERBS: &[&str] = &[
    "rearranges", "whispers to", "devours", "mirrors", "weaves through",
    "annotates", "relabels", "erases", "reflects inside"
];

/// Get a pseudo-random index based on time.
fn random_index(max: usize, seed: usize) -> usize {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    ((nanos as usize).wrapping_add(seed * 7919)) % max
}

/// Register dreaming operations.
pub fn register(interp: &mut Interpreter) {
    // Generate surreal dreamlog trace
    // Stack: [top_value] → 0
    interp.register(":dreaming", |interp| {
        println!();
        println!("☁ Surreal Dreamlog Trace");
        println!("----------------------------------------");
        
        // Generate 4 random surreal statements
        for i in 0..4 {
            let g1 = GLYPHS[random_index(GLYPHS.len(), i)];
            let g2 = GLYPHS[random_index(GLYPHS.len(), i + 100)];
            let v = VERBS[random_index(VERBS.len(), i + 200)];
            
            println!("  {}  {}  {}", g1, v, g2);
        }
        
        // If there's something on the stack, include it in the dream
        if let Ok(top) = interp.stack().peek() {
            let approx = match top {
                WofValue::Integer(n) => *n as f64,
                WofValue::Float(f) => *f,
                _ => 0.0,
            };
            let glyph = GLYPHS[random_index(GLYPHS.len(), 999)];
            println!();
            println!("  top-of-stack drifts as {} ≈ {}", glyph, approx);
        }
        
        println!("----------------------------------------");
        println!();
        
        interp.stack_mut().push(WofValue::integer(0));
        Ok(())
    });

    // Alternative name
    interp.register(":dream", |interp| {
        // Same as :dreaming
        println!();
        println!("💤 The stack enters REM sleep...");
        println!();
        
        for i in 0..3 {
            let g = GLYPHS[random_index(GLYPHS.len(), i * 42)];
            println!("  {} floats by...", g);
        }
        
        println!();
        println!("The dream fades.");
        println!();
        
        interp.stack_mut().push(WofValue::integer(0));
        Ok(())
    });
}
