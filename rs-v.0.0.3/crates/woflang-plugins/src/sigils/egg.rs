//! Easter egg operations for Woflang.
//!
//! Cryptic glyph haiku and other surprises:
//! - `:egg` - Display a random cryptic glyph haiku

use woflang_core::WofValue;
use woflang_runtime::Interpreter;

/// The sacred haiku collection.
static HAIKU: &[&[&str]] = &[
    &[
        "  𐌼  sigils whisper",
        "  ∴  under the heap's cold moon",
        "  ₪  stacks dream of return",
    ],
    &[
        "  ☍  glyphs fall like snow",
        "  ⇌  pointers trace forgotten paths",
        "  𐌀  null sings quietly",
    ],
    &[
        "  ☯  void drinks all symbols",
        "  Ϟ  sparks of undefined dance",
        "  ◬  main never returns",
    ],
    &[
        "  𓂀  eye of the opcode",
        "  ʘ  watches spins of fate and ints",
        "  ⌘  breakpoints in the dark",
    ],
    &[
        "  ⟁  the stack ascends",
        "  ∞  infinite loops of thought",
        "  ∅  nothing remains",
    ],
    &[
        "  ᚠ  ancient runes speak",
        "  ⚚  mercury flows through wires",
        "  ᛟ  inheritance fades",
    ],
    &[
        "  ⧖  time collapses here",
        "  ⌬  benzene dreams of cycles",
        "  ◊  diamonds are forever",
    ],
    &[
        "  ⊕  XOR gates open",
        "  ⊗  tensor fields collapse",
        "  ⊙  the sun compiles",
    ],
];

/// Get a pseudo-random haiku.
fn random_haiku() -> &'static [&'static str] {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    HAIKU[(nanos as usize) % HAIKU.len()]
}

/// Register egg operations.
pub fn register(interp: &mut Interpreter) {
    // Display a random cryptic glyph haiku
    // Stack: → 3 (line count)
    interp.register(":egg", |interp| {
        let poem = random_haiku();
        
        println!();
        println!("🥚 Cryptic Glyph Haiku:");
        for line in poem {
            println!("{}", line);
        }
        println!();
        
        // Push line count
        interp.stack_mut().push(WofValue::integer(3));
        Ok(())
    });

    // Alternative Easter egg name
    interp.register("easter", |interp| {
        println!();
        println!("🐰 You found the easter egg!");
        println!("   The stack appreciates your curiosity.");
        println!();
        
        let poem = random_haiku();
        for line in poem {
            println!("{}", line);
        }
        println!();
        
        interp.stack_mut().push(WofValue::integer(1));
        Ok(())
    });

    // Glyph fortune cookie
    interp.register("fortune", |interp| {
        static FORTUNES: &[&str] = &[
            "Your next segfault brings enlightenment.",
            "The compiler smiles upon your code today.",
            "A wise programmer debugs before pushing.",
            "The void awaits your null pointer.",
            "Today you will discover an off-by-one error.",
            "The stack will be generous if you are kind.",
            "Beware the mutable global state.",
            "Your tests will pass on the third attempt.",
        ];
        
        use std::time::{SystemTime, UNIX_EPOCH};
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        let fortune = FORTUNES[(nanos as usize) % FORTUNES.len()];
        
        println!();
        println!("🥠 {}", fortune);
        println!();
        
        interp.stack_mut().push(WofValue::string(fortune.to_string()));
        Ok(())
    });
}
