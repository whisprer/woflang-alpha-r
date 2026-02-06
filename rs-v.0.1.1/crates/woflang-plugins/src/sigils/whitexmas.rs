//! White Christmas operations for Woflang.
//!
//! Sigil snowstorm / matrix rain animation:
//! - `:whitexmas` - Display animated sigil snow
//! - `:matrix` - Matrix-style sigil rain

use woflang_runtime::Interpreter;

/// Mystical sigils for the snowstorm.
static SIGILS: &[&str] = &[
    "⟁", "◬", "𓂀", "₪", "⚚", "⌘", "☍", "✶", "✺", "✦", "ᚠ", "ᛟ",
    "❄", "❅", "❆", "✧", "✦", "⊹", "⋆", "°"
];

/// Get a random sigil.
fn random_sigil(seed: usize) -> &'static str {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let idx = ((nanos as usize).wrapping_add(seed * 7919)) % SIGILS.len();
    SIGILS[idx]
}

/// Simple delay (busy wait since we don't have std::thread::sleep).
fn small_delay() {
    // Note: In actual use, you'd use std::thread::sleep
    // For now, we just print immediately
}

/// Register whitexmas operations.
pub fn register(interp: &mut Interpreter) {
    // Sigil snowstorm
    // Stack: →
    interp.register(":whitexmas", |_interp| {
        println!();
        println!("❄ Sigil storm begins...");
        println!();
        
        let width = 40;
        let height = 16;
        
        for row in 0..height {
            let mut line = String::new();
            for col in 0..width {
                let seed = row * width + col;
                // About 1 in 8 chance of a sigil
                if (seed + row * 3) % 8 == 0 {
                    line.push_str(random_sigil(seed));
                } else {
                    line.push(' ');
                }
            }
            println!("{}", line);
            small_delay();
        }
        
        println!();
        println!("The sigils melt back into the heap.");
        println!();
        
        Ok(())
    });

    // Matrix-style sigil rain (denser)
    // Stack: →
    interp.register(":matrix", |_interp| {
        println!();
        println!("🟢 Entering the Matrix...");
        println!();
        
        // Matrix-style glyphs (more techy)
        static MATRIX_GLYPHS: &[&str] = &[
            "ｱ", "ｲ", "ｳ", "ｴ", "ｵ", "ｶ", "ｷ", "ｸ", "ｹ", "ｺ",
            "0", "1", "∅", "∞", "⊕", "⊗", "⌬", "⎔", "⟁", "◬"
        ];
        
        let width = 50;
        let height = 20;
        
        for row in 0..height {
            let mut line = String::new();
            for col in 0..width {
                let seed = row * width + col;
                // About 1 in 4 chance of a glyph
                if (seed + row * 7) % 4 == 0 {
                    let idx = (seed + row) % MATRIX_GLYPHS.len();
                    line.push_str(MATRIX_GLYPHS[idx]);
                } else {
                    line.push(' ');
                }
            }
            println!("{}", line);
        }
        
        println!();
        println!("There is no spoon. There is only the stack.");
        println!();
        
        Ok(())
    });

    // Gentle snow (sparse)
    // Stack: →
    interp.register(":snow", |_interp| {
        println!();
        static SNOWFLAKES: &[&str] = &["❄", "❅", "❆", "✦", "°", "·"];
        
        let width = 35;
        let height = 10;
        
        for row in 0..height {
            let mut line = String::new();
            for col in 0..width {
                let seed = row * width + col;
                if (seed + row * 5) % 12 == 0 {
                    let idx = (seed + row) % SNOWFLAKES.len();
                    line.push_str(SNOWFLAKES[idx]);
                } else {
                    line.push(' ');
                }
            }
            println!("{}", line);
        }
        
        println!();
        
        Ok(())
    });

    // Stars in the sky
    // Stack: →
    interp.register(":stars", |_interp| {
        println!();
        static STARS: &[&str] = &["✦", "✧", "⋆", "✶", "✷", "✸", "★", "☆", "°"];
        
        let width = 50;
        let height = 12;
        
        for row in 0..height {
            let mut line = String::new();
            for col in 0..width {
                let seed = row * width + col;
                if (seed + row * 11) % 10 == 0 {
                    let idx = (seed + row) % STARS.len();
                    line.push_str(STARS[idx]);
                } else {
                    line.push(' ');
                }
            }
            println!("{}", line);
        }
        
        println!("          🌙");
        println!();
        
        Ok(())
    });
}
