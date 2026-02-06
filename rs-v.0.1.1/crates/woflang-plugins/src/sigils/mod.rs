//! Sigil easter egg operations for Woflang.
//!
//! The soul of Woflang - mystical, playful, and full of character.
//! These sigils bring the language to life with:
//!
//! - **Chaos modes**: unlock, glitch, deity
//! - **Dreaming**: surreal debug traces
//! - **Prophecy**: cryptic fate messages
//! - **Forbidden**: void division, stack slaying
//! - **Moses**: parting the stack sea
//! - **Hebrew**: RTL mode and the famous tea joke
//! - **Egg**: cryptic glyph haiku
//! - **Whitexmas**: sigil snowstorm animations
//! - **Mirror**: stack reversal
//! - **Totem**: ASCII art and sigil maps
//!
//! ## Usage
//!
//! ```text
//! :unlock           # Unlock forbidden glyphs
//! :egg              # Random glyph haiku
//! prophecy          # Cryptic stack fate message
//! moses             # Part the stack like the Red Sea
//! hebrews_it        # "How does Moses take his tea? He brews it!"
//! :whitexmas        # Sigil snowstorm
//! :dreaming         # Surreal debug traces
//! void_division     # Divide by the void (clears stack, leaves infinity)
//! sigil_map         # List all sacred sigils
//! ```

pub mod chaos;
pub mod dreaming;
pub mod prophecy;
pub mod forbidden;
pub mod moses;
pub mod hebrew;
pub mod egg;
pub mod whitexmas;
pub mod mirror;
pub mod totem;

use woflang_runtime::Interpreter;

/// Register all sigil operations.
pub fn register(interp: &mut Interpreter) {
    chaos::register(interp);
    dreaming::register(interp);
    prophecy::register(interp);
    forbidden::register(interp);
    moses::register(interp);
    hebrew::register(interp);
    egg::register(interp);
    whitexmas::register(interp);
    mirror::register(interp);
    totem::register(interp);
}

// Re-export state query functions
pub use chaos::{is_chaos_unlocked, is_deity_mode, is_glitch_mode};
pub use hebrew::is_hebrew_mode;
pub use mirror::is_mirror_mode;
