//! Over Unity operations for Woflang.
//!
//! A tongue-in-cheek "free energy" easter egg module.
//!
//! ## Operations
//!
//! - `over_unity` - The mythical free energy device
//! - `perpetual_motion` - Start the perpetual motion machine
//! - `free_energy` - Generate free energy (spoiler: it doesn't work)
//! - `thermodynamics` - Print the laws of thermodynamics

use woflang_core::WofValue;
use woflang_runtime::Interpreter;
use rand::Rng;

// ═══════════════════════════════════════════════════════════════════════════
// REGISTRATION
// ═══════════════════════════════════════════════════════════════════════════

/// Register over unity (easter egg) operations.
pub fn register(interp: &mut Interpreter) {
    // The classic over-unity easter egg
    interp.register("over_unity", |_interp| {
        println!("⚡  Over Unity! Energy out exceeds energy in.");
        println!("    Next op will be disabled... (just kidding, demo only)");
        println!();
        println!("    Remember: The laws of thermodynamics are undefeated.");
        Ok(())
    });

    // Perpetual motion machine
    interp.register("perpetual_motion", |interp| {
        println!("🔄  Starting perpetual motion machine...");
        println!("    .");
        println!("    ..");
        println!("    ...");
        println!("    *click* *whirr* *slowdown*");
        println!("    ⚠️  Machine stopped. Friction wins again.");
        
        // Push 0 (entropy always wins)
        interp.stack_mut().push(WofValue::Float(0.0));
        Ok(())
    });

    // Free energy generator
    interp.register("free_energy", |interp| {
        let mut rng = rand::thread_rng();
        
        println!("🔋  Activating free energy generator...");
        println!("    Tapping into zero-point energy...");
        println!("    Accessing vacuum fluctuations...");
        
        // Generate a tiny amount of "energy" (random noise)
        let energy: f64 = rng.gen_range(-0.0001..0.0001);
        
        println!("    Generated: {} joules", energy);
        println!();
        println!("    (That's just noise. Conservation of energy is real.)");
        
        interp.stack_mut().push(WofValue::Float(energy));
        Ok(())
    });

    // Print the laws of thermodynamics
    interp.register("thermodynamics", |_interp| {
        println!("═══════════════════════════════════════════════════════");
        println!("           THE LAWS OF THERMODYNAMICS                   ");
        println!("═══════════════════════════════════════════════════════");
        println!();
        println!("  0th Law: If A = B and B = C, then A = C");
        println!("           (Thermal equilibrium is transitive)");
        println!();
        println!("  1st Law: Energy cannot be created or destroyed");
        println!("           (ΔU = Q - W)");
        println!();
        println!("  2nd Law: Entropy of an isolated system never decreases");
        println!("           (You can't break even)");
        println!();
        println!("  3rd Law: As T → 0, S → constant");
        println!("           (You can't reach absolute zero)");
        println!();
        println!("  Informal: You can't win, you can't break even,");
        println!("            and you can't quit the game.");
        println!("═══════════════════════════════════════════════════════");
        Ok(())
    });

    // Maxwell's demon
    interp.register("maxwell_demon", |interp| {
        println!("😈  Maxwell's Demon awakens...");
        println!("    Attempting to sort molecules by speed...");
        println!();
        
        if interp.stack().is_empty() {
            println!("    The demon finds nothing to sort.");
            return Ok(());
        }
        
        // Sort the stack (the demon "works")
        let stack = interp.stack_mut();
        let values: &mut [WofValue] = stack.as_mut_slice();
        
        values.sort_by(|a, b| {
            let av = match a {
                WofValue::Integer(n) => *n as f64,
                WofValue::Float(f) => *f,
                _ => 0.0,
            };
            let bv = match b {
                WofValue::Integer(n) => *n as f64,
                WofValue::Float(f) => *f,
                _ => 0.0,
            };
            av.partial_cmp(&bv).unwrap_or(std::cmp::Ordering::Equal)
        });
        
        println!("    Sorting complete! But wait...");
        println!("    The demon's information processing increased entropy.");
        println!("    The 2nd Law remains unbroken.");
        Ok(())
    });

    // Heat death of the universe
    interp.register("heat_death", |interp| {
        println!("🌌  Fast-forwarding to the heat death of the universe...");
        println!();
        println!("    10^100 years later...");
        println!();
        println!("    All stars have burned out.");
        println!("    All black holes have evaporated.");
        println!("    Maximum entropy has been reached.");
        println!("    Nothing can ever happen again.");
        println!();
        
        // Clear the stack (maximum entropy = no structure)
        interp.stack_mut().clear();
        
        println!("    Stack cleared. The universe is at peace.");
        Ok(())
    });

    // Entropy always increases
    interp.register("entropy_increases", |interp| {
        let current_entropy = interp.stack().len() as f64;
        let new_entropy = current_entropy + 1.0;
        
        println!("📈  Entropy always increases!");
        println!("    Previous entropy: {}", current_entropy);
        println!("    New entropy: {}", new_entropy);
        
        // Push a random value to increase disorder
        interp.stack_mut().push(WofValue::Float(rand::random::<f64>()));
        
        Ok(())
    });
}
