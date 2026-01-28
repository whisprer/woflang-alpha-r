//! Data loading and constants for Woflang plugins.
//!
//! Provides embedded data and runtime loading utilities.
//! Uses the COMPLETE embedded constants database from wof_constants_module.json.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::OnceLock;

/// Embed the FULL constants database at compile time.
const CONSTANTS_JSON: &str = include_str!("../../data/wof_constants_module.json");

/// A physical or mathematical constant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constant {
    pub name: String,
    pub symbol: String,
    #[serde(default)]
    pub opcode: i32,
    pub value: f64,
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub unit: String,
}

/// An operation definition from the JSON.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
    pub name: String,
    pub symbol: String,
    pub opcode: i32,
    pub arity: i32,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub example: String,
}

/// A unit definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Unit {
    pub name: String,
    pub symbol: String,
    pub multiplier: f64,
    #[serde(default)]
    pub dimension: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub base_units: String,
    #[serde(default)]
    pub exponent: i32,
}

/// Database of constants, operations, and units.
#[derive(Debug, Default)]
pub struct ConstantsDb {
    // Constants
    pub math_constants: Vec<Constant>,
    pub physics_constants: Vec<Constant>,
    pub modelica_constants: Vec<Constant>,
    
    // Operations
    pub exponential_ops: Vec<Operation>,
    pub calculus_ops: Vec<Operation>,
    
    // Units
    pub base_si_units: Vec<Unit>,
    pub derived_si_units: Vec<Unit>,
    pub si_prefixes: Vec<Unit>,
    
    // Lookups
    by_name: HashMap<String, Constant>,
    by_symbol: HashMap<String, Constant>,
    by_opcode: HashMap<i32, Constant>,
    
    // Metadata
    pub name: String,
    pub version: String,
    pub description: String,
    pub categories: Vec<String>,
}

impl ConstantsDb {
    /// Load from the embedded JSON.
    fn load() -> Result<Self, String> {
        #[derive(Deserialize)]
        struct Root {
            metadata: Metadata,
            constants: ConstantsSection,
            operations: OperationsSection,
            units: UnitsSection,
        }

        #[derive(Deserialize)]
        struct Metadata {
            name: String,
            version: String,
            description: String,
            categories: Vec<String>,
        }

        #[derive(Deserialize)]
        struct ConstantsSection {
            mathematics: Vec<Constant>,
            physics: Vec<Constant>,
            modelica: Vec<Constant>,
        }

        #[derive(Deserialize)]
        struct OperationsSection {
            exponentials: Vec<Operation>,
            calculus: Vec<Operation>,
        }

        #[derive(Deserialize)]
        struct UnitsSection {
            base_si: Vec<Unit>,
            derived_si: Vec<Unit>,
            si_prefixes: Vec<Unit>,
        }

        let root: Root = serde_json::from_str(CONSTANTS_JSON)
            .map_err(|e| format!("Failed to parse constants JSON: {}", e))?;
        
        let mut db = Self {
            math_constants: root.constants.mathematics,
            physics_constants: root.constants.physics,
            modelica_constants: root.constants.modelica,
            exponential_ops: root.operations.exponentials,
            calculus_ops: root.operations.calculus,
            base_si_units: root.units.base_si,
            derived_si_units: root.units.derived_si,
            si_prefixes: root.units.si_prefixes,
            by_name: HashMap::new(),
            by_symbol: HashMap::new(),
            by_opcode: HashMap::new(),
            name: root.metadata.name,
            version: root.metadata.version,
            description: root.metadata.description,
            categories: root.metadata.categories,
        };
        
        // Build lookup tables
        for c in db.math_constants.iter()
            .chain(db.physics_constants.iter())
            .chain(db.modelica_constants.iter())
        {
            db.by_name.insert(c.name.clone(), c.clone());
            db.by_symbol.insert(c.symbol.clone(), c.clone());
            if c.opcode > 0 {
                db.by_opcode.insert(c.opcode, c.clone());
            }
        }
        
        Ok(db)
    }

    /// Get a constant by name.
    pub fn get_by_name(&self, name: &str) -> Option<&Constant> {
        self.by_name.get(name)
    }

    /// Get a constant by symbol.
    pub fn get_by_symbol(&self, symbol: &str) -> Option<&Constant> {
        self.by_symbol.get(symbol)
    }

    /// Get a constant by opcode.
    pub fn get_by_opcode(&self, opcode: i32) -> Option<&Constant> {
        self.by_opcode.get(&opcode)
    }

    /// Get all category names.
    pub fn categories(&self) -> &[String] {
        &self.categories
    }

    /// Get all constant names.
    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.by_name.keys().map(|s| s.as_str())
    }

    /// Get all symbols.
    pub fn symbols(&self) -> impl Iterator<Item = &str> {
        self.by_symbol.keys().map(|s| s.as_str())
    }

    /// Get a unit by symbol.
    pub fn get_unit(&self, symbol: &str) -> Option<&Unit> {
        self.base_si_units.iter()
            .chain(self.derived_si_units.iter())
            .find(|u| u.symbol == symbol)
    }

    /// Get a prefix by symbol.
    pub fn get_prefix(&self, symbol: &str) -> Option<&Unit> {
        self.si_prefixes.iter().find(|p| p.symbol == symbol)
    }
}

/// Get the global constants database (lazy initialization).
pub fn get_constants_db() -> &'static ConstantsDb {
    static DB: OnceLock<ConstantsDb> = OnceLock::new();
    DB.get_or_init(|| {
        ConstantsDb::load().expect("Failed to load embedded constants database")
    })
}

/// Embedded mathematical constants (compile-time, for fast access).
pub mod embedded {
    /// Mathematical constants.
    pub const PI: f64 = std::f64::consts::PI;
    pub const TAU: f64 = std::f64::consts::TAU;
    pub const E: f64 = std::f64::consts::E;
    pub const PHI: f64 = 1.618033988749895; // Golden ratio
    pub const SQRT2: f64 = std::f64::consts::SQRT_2;
    pub const SQRT3: f64 = 1.7320508075688772;
    pub const LN2: f64 = std::f64::consts::LN_2;
    pub const LN10: f64 = std::f64::consts::LN_10;

    /// Physical constants (SI units).
    pub const SPEED_OF_LIGHT: f64 = 299_792_458.0; // m/s
    pub const PLANCK: f64 = 6.62607015e-34; // J⋅s
    pub const PLANCK_REDUCED: f64 = 1.054571817e-34; // J⋅s
    pub const BOLTZMANN: f64 = 1.380649e-23; // J/K
    pub const AVOGADRO: f64 = 6.02214076e23; // mol⁻¹
    pub const ELECTRON_MASS: f64 = 9.1093837015e-31; // kg
    pub const PROTON_MASS: f64 = 1.67262192369e-27; // kg
    pub const ELEMENTARY_CHARGE: f64 = 1.602176634e-19; // C
    pub const GRAVITATIONAL: f64 = 6.67430e-11; // m³/(kg⋅s²)
    pub const FINE_STRUCTURE: f64 = 7.2973525693e-3; // dimensionless
    pub const RYDBERG: f64 = 10973731.56850865; // m⁻¹
    pub const GRAVITATIONAL_ACCEL: f64 = 9.807; // m/s²
}
