//! Chemistry operations for Woflang.
//!
//! Provides basic element data, atomic/molecular weights,
//! temperature conversions, and fundamental constants.
//!
//! ## Operations
//!
//! - `element_info` - Get element info (symbol/name/Z → description)
//! - `atomic_weight` - Get atomic weight (symbol → g/mol)
//! - `molecular_weight` - Calculate molecular weight (formula → g/mol)
//! - `temp_convert` - Temperature conversion (value mode → converted)
//! - `avogadro` - Push Avogadro's number

use woflang_core::WofValue;
use woflang_runtime::Interpreter;

// ═══════════════════════════════════════════════════════════════════════════
// PERIODIC TABLE DATA
// ═══════════════════════════════════════════════════════════════════════════

/// Element data entry.
struct Element {
    z: i32,
    symbol: &'static str,
    name: &'static str,
    atomic_weight: f64,
}

/// Periodic table subset (common elements).
static ELEMENTS: &[Element] = &[
    Element { z: 1,  symbol: "H",  name: "Hydrogen",    atomic_weight: 1.008 },
    Element { z: 2,  symbol: "He", name: "Helium",      atomic_weight: 4.002602 },
    Element { z: 3,  symbol: "Li", name: "Lithium",     atomic_weight: 6.94 },
    Element { z: 4,  symbol: "Be", name: "Beryllium",   atomic_weight: 9.0121831 },
    Element { z: 5,  symbol: "B",  name: "Boron",       atomic_weight: 10.81 },
    Element { z: 6,  symbol: "C",  name: "Carbon",      atomic_weight: 12.011 },
    Element { z: 7,  symbol: "N",  name: "Nitrogen",    atomic_weight: 14.007 },
    Element { z: 8,  symbol: "O",  name: "Oxygen",      atomic_weight: 15.999 },
    Element { z: 9,  symbol: "F",  name: "Fluorine",    atomic_weight: 18.998403163 },
    Element { z: 10, symbol: "Ne", name: "Neon",        atomic_weight: 20.1797 },
    Element { z: 11, symbol: "Na", name: "Sodium",      atomic_weight: 22.98976928 },
    Element { z: 12, symbol: "Mg", name: "Magnesium",   atomic_weight: 24.305 },
    Element { z: 13, symbol: "Al", name: "Aluminium",   atomic_weight: 26.9815385 },
    Element { z: 14, symbol: "Si", name: "Silicon",     atomic_weight: 28.085 },
    Element { z: 15, symbol: "P",  name: "Phosphorus",  atomic_weight: 30.973761998 },
    Element { z: 16, symbol: "S",  name: "Sulfur",      atomic_weight: 32.06 },
    Element { z: 17, symbol: "Cl", name: "Chlorine",    atomic_weight: 35.45 },
    Element { z: 18, symbol: "Ar", name: "Argon",       atomic_weight: 39.948 },
    Element { z: 19, symbol: "K",  name: "Potassium",   atomic_weight: 39.0983 },
    Element { z: 20, symbol: "Ca", name: "Calcium",     atomic_weight: 40.078 },
    Element { z: 21, symbol: "Sc", name: "Scandium",    atomic_weight: 44.955908 },
    Element { z: 22, symbol: "Ti", name: "Titanium",    atomic_weight: 47.867 },
    Element { z: 23, symbol: "V",  name: "Vanadium",    atomic_weight: 50.9415 },
    Element { z: 24, symbol: "Cr", name: "Chromium",    atomic_weight: 51.9961 },
    Element { z: 25, symbol: "Mn", name: "Manganese",   atomic_weight: 54.938044 },
    Element { z: 26, symbol: "Fe", name: "Iron",        atomic_weight: 55.845 },
    Element { z: 27, symbol: "Co", name: "Cobalt",      atomic_weight: 58.933194 },
    Element { z: 28, symbol: "Ni", name: "Nickel",      atomic_weight: 58.6934 },
    Element { z: 29, symbol: "Cu", name: "Copper",      atomic_weight: 63.546 },
    Element { z: 30, symbol: "Zn", name: "Zinc",        atomic_weight: 65.38 },
    Element { z: 35, symbol: "Br", name: "Bromine",     atomic_weight: 79.904 },
    Element { z: 47, symbol: "Ag", name: "Silver",      atomic_weight: 107.8682 },
    Element { z: 50, symbol: "Sn", name: "Tin",         atomic_weight: 118.710 },
    Element { z: 53, symbol: "I",  name: "Iodine",      atomic_weight: 126.90447 },
    Element { z: 74, symbol: "W",  name: "Tungsten",    atomic_weight: 183.84 },
    Element { z: 78, symbol: "Pt", name: "Platinum",    atomic_weight: 195.084 },
    Element { z: 79, symbol: "Au", name: "Gold",        atomic_weight: 196.966569 },
    Element { z: 80, symbol: "Hg", name: "Mercury",     atomic_weight: 200.592 },
    Element { z: 82, symbol: "Pb", name: "Lead",        atomic_weight: 207.2 },
    Element { z: 92, symbol: "U",  name: "Uranium",     atomic_weight: 238.02891 },
];

/// Find element by symbol (case-sensitive).
fn find_by_symbol(symbol: &str) -> Option<&'static Element> {
    ELEMENTS.iter().find(|e| e.symbol == symbol)
}

/// Find element by name (case-insensitive).
fn find_by_name(name: &str) -> Option<&'static Element> {
    let needle = name.to_lowercase();
    ELEMENTS.iter().find(|e| e.name.to_lowercase() == needle)
}

/// Find element by atomic number.
fn find_by_z(z: i32) -> Option<&'static Element> {
    ELEMENTS.iter().find(|e| e.z == z)
}

/// Find element by any identifier (symbol, name, or atomic number).
fn find_element(token: &str) -> Option<&'static Element> {
    // Try symbol first (case-sensitive)
    if let Some(e) = find_by_symbol(token) {
        return Some(e);
    }
    
    // Try name (case-insensitive)
    if let Some(e) = find_by_name(token) {
        return Some(e);
    }
    
    // Try as atomic number
    if let Ok(z) = token.parse::<i32>() {
        if let Some(e) = find_by_z(z) {
            return Some(e);
        }
    }
    
    // Try normalized symbol (e.g., "h" -> "H")
    if !token.is_empty() {
        let mut sym = String::new();
        let mut chars = token.chars();
        if let Some(c) = chars.next() {
            sym.push(c.to_ascii_uppercase());
        }
        if let Some(c) = chars.next() {
            sym.push(c.to_ascii_lowercase());
        }
        if let Some(e) = find_by_symbol(&sym) {
            return Some(e);
        }
    }
    
    None
}

// ═══════════════════════════════════════════════════════════════════════════
// MOLECULAR FORMULA PARSER
// ═══════════════════════════════════════════════════════════════════════════

/// Parse a molecular formula and calculate molecular weight.
/// Supports simple formulas like "H2O", "CO2", "C6H12O6".
fn molecular_weight(formula: &str) -> Result<f64, String> {
    if formula.is_empty() {
        return Ok(0.0);
    }
    
    let mut total = 0.0;
    let chars: Vec<char> = formula.chars().collect();
    let n = chars.len();
    let mut i = 0;
    
    while i < n {
        // Expect uppercase letter
        if !chars[i].is_ascii_uppercase() {
            return Err(format!("Invalid character '{}' in formula", chars[i]));
        }
        
        // Element symbol: uppercase + optional lowercase
        let mut sym = String::new();
        sym.push(chars[i]);
        i += 1;
        
        if i < n && chars[i].is_ascii_lowercase() {
            sym.push(chars[i]);
            i += 1;
        }
        
        // Find element
        let elem = find_by_symbol(&sym)
            .ok_or_else(|| format!("Unknown element symbol '{}' in formula", sym))?;
        
        // Optional numeric count
        let mut count = 0i32;
        while i < n && chars[i].is_ascii_digit() {
            count = count * 10 + (chars[i] as i32 - '0' as i32);
            i += 1;
        }
        if count == 0 {
            count = 1;
        }
        
        total += elem.atomic_weight * count as f64;
    }
    
    Ok(total)
}

// ═══════════════════════════════════════════════════════════════════════════
// TEMPERATURE CONVERSION
// ═══════════════════════════════════════════════════════════════════════════

/// Convert temperature between scales.
fn temp_convert(value: f64, mode: &str) -> Result<f64, String> {
    let m = mode.to_lowercase()
        .replace(' ', ">")
        .replace('_', ">")
        .replace("2", ">")
        .replace("to", ">");
    
    if m.starts_with("c>k") {
        Ok(value + 273.15)
    } else if m.starts_with("k>c") {
        Ok(value - 273.15)
    } else if m.starts_with("c>f") {
        Ok(value * 9.0 / 5.0 + 32.0)
    } else if m.starts_with("f>c") {
        Ok((value - 32.0) * 5.0 / 9.0)
    } else if m.starts_with("k>f") {
        Ok((value - 273.15) * 9.0 / 5.0 + 32.0)
    } else if m.starts_with("f>k") {
        Ok((value - 32.0) * 5.0 / 9.0 + 273.15)
    } else {
        Err(format!("Unknown conversion mode: {}", mode))
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// CONSTANTS
// ═══════════════════════════════════════════════════════════════════════════

/// Avogadro's number (exact definition since 2019).
const AVOGADRO: f64 = 6.02214076e23;

/// Gas constant R (J/(mol·K)).
const GAS_CONSTANT: f64 = 8.314462618;

/// Faraday constant (C/mol).
const FARADAY: f64 = 96485.33212;

// ═══════════════════════════════════════════════════════════════════════════
// REGISTRATION
// ═══════════════════════════════════════════════════════════════════════════

/// Register all chemistry operations.
pub fn register(interp: &mut Interpreter) {
    // ─────────────────────────────────────────────────────────────────────
    // ELEMENT INFO
    // ─────────────────────────────────────────────────────────────────────
    
    // Get element info by symbol, name, or atomic number
    // Stack: "Fe" → "Iron (Fe), Z = 26, atomic weight ≈ 55.84500 g/mol"
    interp.register("element_info", |interp| {
        let key = interp.stack_mut().pop()?;
        
        let elem = match &key {
            WofValue::String(s) => find_element(s),
            WofValue::Integer(z) => find_by_z(*z as i32),
            WofValue::Float(f) => find_by_z(*f as i32),
            _ => None,
        };
        
        let result = match elem {
            Some(e) => format!("{} ({}), Z = {}, atomic weight ≈ {:.5} g/mol",
                e.name, e.symbol, e.z, e.atomic_weight),
            None => format!("Unknown element: {:?}", key),
        };
        
        interp.stack_mut().push(WofValue::string(result));
        Ok(())
    });

    // Get just the atomic weight
    // Stack: "O" → 15.999
    interp.register("atomic_weight", |interp| {
        let key = interp.stack_mut().pop()?;
        
        let elem = match &key {
            WofValue::String(s) => find_element(s),
            WofValue::Integer(z) => find_by_z(*z as i32),
            WofValue::Float(f) => find_by_z(*f as i32),
            _ => None,
        };
        
        let weight = elem.map(|e| e.atomic_weight).unwrap_or(0.0);
        interp.stack_mut().push(WofValue::Float(weight));
        Ok(())
    });

    // ─────────────────────────────────────────────────────────────────────
    // MOLECULAR WEIGHT
    // ─────────────────────────────────────────────────────────────────────
    
    // Calculate molecular weight from formula
    // Stack: "H2O" → 18.015
    interp.register("molecular_weight", |interp| {
        let formula = interp.stack_mut().pop()?.as_string()?;
        
        let weight = molecular_weight(&formula).unwrap_or(0.0);
        interp.stack_mut().push(WofValue::Float(weight));
        Ok(())
    });

    // Alias
    interp.register("molar_mass", |interp| {
        let formula = interp.stack_mut().pop()?.as_string()?;
        let weight = molecular_weight(&formula).unwrap_or(0.0);
        interp.stack_mut().push(WofValue::Float(weight));
        Ok(())
    });

    // ─────────────────────────────────────────────────────────────────────
    // TEMPERATURE CONVERSION
    // ─────────────────────────────────────────────────────────────────────
    
    // Convert temperature between scales
    // Stack: 100 "C->F" → 212.0
    // Stack: "C->F" 100 → 212.0 (flexible arg order)
    interp.register("temp_convert", |interp| {
        let top = interp.stack_mut().pop()?;
        let next = interp.stack_mut().pop()?;
        
        // Figure out which is mode and which is value
        let (value, mode) = if let Ok(s) = top.as_string() {
            (next.as_float().unwrap_or(0.0), s)
        } else if let Ok(s) = next.as_string() {
            (top.as_float().unwrap_or(0.0), s)
        } else {
            interp.stack_mut().push(top);
            return Ok(());
        };
        
        let result = temp_convert(value, &mode).unwrap_or(value);
        interp.stack_mut().push(WofValue::Float(result));
        Ok(())
    });

    // Convenience functions for specific conversions
    interp.register("c_to_f", |interp| {
        let c = interp.stack_mut().pop()?.as_float()?;
        interp.stack_mut().push(WofValue::Float(c * 9.0 / 5.0 + 32.0));
        Ok(())
    });

    interp.register("f_to_c", |interp| {
        let f = interp.stack_mut().pop()?.as_float()?;
        interp.stack_mut().push(WofValue::Float((f - 32.0) * 5.0 / 9.0));
        Ok(())
    });

    interp.register("c_to_k", |interp| {
        let c = interp.stack_mut().pop()?.as_float()?;
        interp.stack_mut().push(WofValue::Float(c + 273.15));
        Ok(())
    });

    interp.register("k_to_c", |interp| {
        let k = interp.stack_mut().pop()?.as_float()?;
        interp.stack_mut().push(WofValue::Float(k - 273.15));
        Ok(())
    });

    // ─────────────────────────────────────────────────────────────────────
    // CONSTANTS
    // ─────────────────────────────────────────────────────────────────────
    
    // Avogadro's number
    interp.register("avogadro", |interp| {
        interp.stack_mut().push(WofValue::Float(AVOGADRO));
        Ok(())
    });

    // Gas constant
    interp.register("gas_constant", |interp| {
        interp.stack_mut().push(WofValue::Float(GAS_CONSTANT));
        Ok(())
    });

    // Faraday constant
    interp.register("faraday", |interp| {
        interp.stack_mut().push(WofValue::Float(FARADAY));
        Ok(())
    });

    // ─────────────────────────────────────────────────────────────────────
    // HELP
    // ─────────────────────────────────────────────────────────────────────
    
    interp.register("chem_help", |_interp| {
        println!("Chemistry Operations:");
        println!();
        println!("  Element Data:");
        println!("    \"Fe\" element_info       → full element description");
        println!("    \"O\" atomic_weight       → 15.999");
        println!("    26 element_info          → lookup by atomic number");
        println!();
        println!("  Molecular Weight:");
        println!("    \"H2O\" molecular_weight  → 18.015");
        println!("    \"C6H12O6\" molar_mass    → 180.156 (glucose)");
        println!();
        println!("  Temperature:");
        println!("    100 \"C->F\" temp_convert → 212.0");
        println!("    32 f_to_c               → 0.0");
        println!("    0 c_to_k                → 273.15");
        println!();
        println!("  Conversion modes: C->K, K->C, C->F, F->C, K->F, F->K");
        println!();
        println!("  Constants:");
        println!("    avogadro                → 6.02214076e23");
        println!("    gas_constant            → 8.314 J/(mol·K)");
        println!("    faraday                 → 96485 C/mol");
        Ok(())
    });
}
