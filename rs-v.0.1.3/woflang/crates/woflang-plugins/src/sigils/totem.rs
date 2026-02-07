//! Totem operations for Woflang.
//!
//! Sigil maps and ASCII art totems:
//! - `sigil_map` - List all sacred sigils
//! - `:wofsigil` - Display the Woflang glyph totem
//! - `:wolf` - Display the wolf ASCII art

use woflang_runtime::Interpreter;

/// Register totem operations.
pub fn register(interp: &mut Interpreter) {
    // Display the sigil map
    interp.register("sigil_map", |_interp| {
        println!("Sacred Sigils:");
        println!("  prophecy (🔮): Cryptic stack fate message");
        println!("  stack_slayer (☠️): Destroys the stack (forbidden)");
        println!("  :egg (🥚): Cryptic glyph haiku");
        println!("  :whitexmas (❄): Sigil snowstorm");
        println!("  :dreaming (☁): Surreal debug traces");
        println!("  :deity (👁): Divine recursion mode");
        println!("  :unlock (⚡): Unlock forbidden glyphs");
        println!("  :glitchmode (⚠): Random glyph substitutions");
        println!("  :mirror (🪞): Reverse stack mode");
        println!("  moses (🌊): Part the stack sea");
        println!("  hebrews_it (☕): The Moses tea joke");
        println!("  void_division (∅): Divide by the void");
        println!("  fortune (🥠): Glyph fortune cookie");
        println!("  :matrix (🟢): Matrix sigil rain");
        println!("  sigil_map (🗺️): This map");
        Ok(())
    });

    // Display the Woflang glyph totem
    interp.register(":wofsigil", |_interp| {
        println!(r#"

            ╭────────────────────────────╮
            │        W O F L A N G      │
            │      glyph totem v1.0     │
            ╰────────────────────────────╯
                   ⟁  ◬  𓂀  ☍  ₪
                 stack  •  sigil  •  code

        "#);
        Ok(())
    });

    // Display wolf ASCII art
    interp.register(":wolf", |_interp| {
        println!(r#"
                          __
                        .d$$b
                      .' TO$;\
                     /  : TP._;
                    / _.;  :Tb|
                   /   /   ;j$j
               _.-"       d$$$$
             .' ..       d$$$$;
            /  /P'      d$$$$P. |\
           /   "      .d$$$P' |\^"l
         .'           `T$P^"""""  :
     ._.'      _.'                ;
  `-.-".-'-' ._.       _.-"    .-"
`.-" _.-"    _.-"     .-'    .-"
 "-.-"  _.-"       _.-' W O F L A N G
                            🐺
        "#);
        Ok(())
    });

    // Display a mystical glyph circle
    interp.register(":circle", |_interp| {
        println!(r#"
               ╭─────────────╮
             ╭─┤  ⟁  ◬  𓂀  ├─╮
            ╱  ╰─────────────╯  ╲
           ╱                     ╲
          │   ₪              ⚚   │
          │                      │
          │   The Eternal Stack  │
          │                      │
          │   ☍              ⌘   │
           ╲                     ╱
            ╲  ╭─────────────╮  ╱
             ╰─┤  ᚠ  ᚨ  ᛟ  ├─╯
               ╰─────────────╯
        "#);
        Ok(())
    });

    // Display the version banner
    interp.register(":banner", |_interp| {
        println!(r#"
  ██╗    ██╗ ██████╗ ███████╗██╗      █████╗ ███╗   ██╗ ██████╗
  ██║    ██║██╔═══██╗██╔════╝██║     ██╔══██╗████╗  ██║██╔════╝
  ██║ █╗ ██║██║   ██║█████╗  ██║     ███████║██╔██╗ ██║██║  ███╗
  ██║███╗██║██║   ██║██╔══╝  ██║     ██╔══██║██║╚██╗██║██║   ██║
  ╚███╔███╔╝╚██████╔╝██║     ███████╗██║  ██║██║ ╚████║╚██████╔╝
   ╚══╝╚══╝  ╚═════╝ ╚═╝     ╚══════╝╚═╝  ╚═╝╚═╝  ╚═══╝ ╚═════╝
                                                    
              ⟁ Unicode Stack-Based Language ⟁
        "#);
        Ok(())
    });

    // Credits
    interp.register(":credits", |_interp| {
        println!();
        println!("╔═══════════════════════════════════════════════╗");
        println!("║              W O F L A N G                    ║");
        println!("║     A Unicode-native stack-based language     ║");
        println!("╠═══════════════════════════════════════════════╣");
        println!("║  Created with 🐺 and ☕                        ║");
        println!("║  Ported from C++ to Rust with love            ║");
        println!("║                                               ║");
        println!("║  Features:                                    ║");
        println!("║  • Unicode glyph operations                   ║");
        println!("║  • Stack-based computation                    ║");
        println!("║  • Variables, functions, loops                ║");
        println!("║  • Cryptic easter eggs                        ║");
        println!("║  • The Moses tea joke                         ║");
        println!("╚═══════════════════════════════════════════════╝");
        println!();
        Ok(())
    });

    // Display help for sigils
    interp.register(":sigil-help", |_interp| {
        println!("Sigil Commands:");
        println!();
        println!("  Mode Toggles:");
        println!("    :unlock      - Unlock forbidden glyphs");
        println!("    :chaos?      - Check if chaos is unlocked");
        println!("    :glitchmode  - Toggle glyph glitching");
        println!("    :deity       - Toggle divine recursion mode");
        println!("    :mirror      - Toggle and reverse stack");
        println!();
        println!("  Easter Eggs:");
        println!("    :egg         - Random glyph haiku");
        println!("    :whitexmas   - Sigil snowstorm");
        println!("    :matrix      - Matrix-style rain");
        println!("    :snow        - Gentle snowfall");
        println!("    :stars       - Starry sky");
        println!("    :dreaming    - Surreal debug trace");
        println!("    fortune      - Glyph fortune cookie");
        println!();
        println!("  Forbidden:");
        println!("    void_division - Divide by void");
        println!("    stack_slayer  - Destroy the stack");
        println!("    /0            - Quick divide by zero");
        println!();
        println!("  Moses:");
        println!("    moses         - Part the stack (view)");
        println!("    moses_split   - Part with marker");
        println!("    hebrews_it    - The tea joke");
        println!();
        println!("  Display:");
        println!("    :wofsigil     - Glyph totem");
        println!("    :wolf         - Wolf art");
        println!("    :banner       - Version banner");
        println!("    :circle       - Mystical circle");
        println!("    :credits      - Credits");
        println!("    sigil_map     - List all sigils");
        Ok(())
    });
}
