//! Logic operations for Woflang.
//!
//! Includes boolean logic, comparison operators, propositional calculus,
//! and category theory concepts.
//!
//! ## Operations
//!
//! ### Basic Logic
//! - `and`, `or`, `xor`, `not` - Boolean operations
//! - `nand`, `nor`, `xnor` - Derived gates
//! - `implies` - Logical implication (a ⇒ b)
//!
//! ### Comparison
//! - `eq`, `neq` - Equality/inequality
//! - `gt`, `lt`, `gte`, `lte` - Numeric comparisons
//!
//! ### Quantifiers
//! - `∀` - For all (universal)
//! - `∃` - Exists (existential)
//!
//! ### Category Theory
//! - `cat_obj`, `cat_mor`, `cat_comp` - Define categories
//! - `cat_hom`, `cat_show`, `cat_clear` - Query and manage

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use woflang_core::WofValue;
use woflang_runtime::Interpreter;

// ═══════════════════════════════════════════════════════════════════════════
// CATEGORY THEORY STATE
// ═══════════════════════════════════════════════════════════════════════════

/// A morphism in the category.
#[derive(Clone)]
struct Morphism {
    name: String,
    from: String,
    to: String,
}

/// Category state (objects and morphisms).
struct CategoryState {
    objects: Vec<String>,
    morphisms: Vec<Morphism>,
}

impl CategoryState {
    fn new() -> Self {
        CategoryState {
            objects: Vec::new(),
            morphisms: Vec::new(),
        }
    }

    fn has_object(&self, obj: &str) -> bool {
        self.objects.iter().any(|o| o == obj)
    }

    fn add_object(&mut self, obj: String) {
        if !self.has_object(&obj) {
            self.objects.push(obj);
        }
    }

    fn find_morphism(&self, name: &str) -> Option<&Morphism> {
        self.morphisms.iter().find(|m| m.name == name)
    }

    fn add_morphism(&mut self, name: String, from: String, to: String) {
        // Ensure objects exist
        self.add_object(from.clone());
        self.add_object(to.clone());

        // Overwrite if same name exists
        if let Some(m) = self.morphisms.iter_mut().find(|m| m.name == name) {
            m.from = from;
            m.to = to;
        } else {
            self.morphisms.push(Morphism { name, from, to });
        }
    }

    fn hom(&self, from: &str, to: &str) -> Vec<&str> {
        self.morphisms
            .iter()
            .filter(|m| m.from == from && m.to == to)
            .map(|m| m.name.as_str())
            .collect()
    }

    fn clear(&mut self) {
        self.objects.clear();
        self.morphisms.clear();
    }

    fn summary(&self) -> String {
        let mut s = String::from("Category summary:\n");

        s.push_str("Objects: ");
        if self.objects.is_empty() {
            s.push_str("{}\n");
        } else {
            s.push_str("{ ");
            s.push_str(&self.objects.join(", "));
            s.push_str(" }\n");
        }

        s.push_str("Morphisms:\n");
        if self.morphisms.is_empty() {
            s.push_str("  (none)\n");
        } else {
            for m in &self.morphisms {
                s.push_str(&format!("  {} : {} -> {}\n", m.name, m.from, m.to));
            }
        }

        s
    }
}

fn category_state() -> &'static Mutex<CategoryState> {
    static STATE: OnceLock<Mutex<CategoryState>> = OnceLock::new();
    STATE.get_or_init(|| Mutex::new(CategoryState::new()))
}

// ═══════════════════════════════════════════════════════════════════════════
// HELPER FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════

/// Convert value to boolean (truthiness).
fn to_bool(v: &WofValue) -> bool {
    match v {
        WofValue::Integer(n) => *n != 0,
        WofValue::Float(f) => *f != 0.0,
        WofValue::Bool(b) => *b,
        WofValue::String(s) => !s.is_empty(),
        WofValue::Nil => false,
        _ => true,
    }
}

/// Make a boolean result value (as float 1.0 or 0.0 for compatibility).
fn make_bool(b: bool) -> WofValue {
    WofValue::Float(if b { 1.0 } else { 0.0 })
}

/// Check if two values are equal.
fn values_equal(a: &WofValue, b: &WofValue) -> bool {
    match (a, b) {
        (WofValue::String(s1), WofValue::String(s2)) => s1 == s2,
        (WofValue::Integer(n1), WofValue::Integer(n2)) => n1 == n2,
        (WofValue::Float(f1), WofValue::Float(f2)) => (f1 - f2).abs() < f64::EPSILON,
        (WofValue::Integer(n), WofValue::Float(f)) |
        (WofValue::Float(f), WofValue::Integer(n)) => (*n as f64 - f).abs() < f64::EPSILON,
        (WofValue::Bool(b1), WofValue::Bool(b2)) => b1 == b2,
        (WofValue::Nil, WofValue::Nil) => true,
        _ => false,
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// REGISTRATION
// ═══════════════════════════════════════════════════════════════════════════

/// Register all logic operations with the interpreter.
pub fn register(interp: &mut Interpreter) {
    // ─────────────────────────────────────────────────────────────────────
    // BASIC BOOLEAN OPERATIONS
    // ─────────────────────────────────────────────────────────────────────

    // Logical AND
    interp.register("and", |interp| {
        let b = interp.stack_mut().pop()?;
        let a = interp.stack_mut().pop()?;
        let result = to_bool(&a) && to_bool(&b);
        interp.stack_mut().push(make_bool(result));
        Ok(())
    });

    // Logical OR
    interp.register("or", |interp| {
        let b = interp.stack_mut().pop()?;
        let a = interp.stack_mut().pop()?;
        let result = to_bool(&a) || to_bool(&b);
        interp.stack_mut().push(make_bool(result));
        Ok(())
    });

    // Logical XOR
    interp.register("xor", |interp| {
        let b = interp.stack_mut().pop()?;
        let a = interp.stack_mut().pop()?;
        let result = to_bool(&a) ^ to_bool(&b);
        interp.stack_mut().push(make_bool(result));
        Ok(())
    });

    // Logical NOT
    interp.register("not", |interp| {
        let a = interp.stack_mut().pop()?;
        let result = !to_bool(&a);
        interp.stack_mut().push(make_bool(result));
        Ok(())
    });

    // Logical NAND
    interp.register("nand", |interp| {
        let b = interp.stack_mut().pop()?;
        let a = interp.stack_mut().pop()?;
        let result = !(to_bool(&a) && to_bool(&b));
        interp.stack_mut().push(make_bool(result));
        Ok(())
    });

    interp.register("⊼", |interp| {
        let b = interp.stack_mut().pop()?;
        let a = interp.stack_mut().pop()?;
        let result = !(to_bool(&a) && to_bool(&b));
        interp.stack_mut().push(make_bool(result));
        Ok(())
    });

    // Logical NOR
    interp.register("nor", |interp| {
        let b = interp.stack_mut().pop()?;
        let a = interp.stack_mut().pop()?;
        let result = !(to_bool(&a) || to_bool(&b));
        interp.stack_mut().push(make_bool(result));
        Ok(())
    });

    interp.register("⊽", |interp| {
        let b = interp.stack_mut().pop()?;
        let a = interp.stack_mut().pop()?;
        let result = !(to_bool(&a) || to_bool(&b));
        interp.stack_mut().push(make_bool(result));
        Ok(())
    });

    // Logical XNOR (equivalence)
    interp.register("xnor", |interp| {
        let b = interp.stack_mut().pop()?;
        let a = interp.stack_mut().pop()?;
        let result = to_bool(&a) == to_bool(&b);
        interp.stack_mut().push(make_bool(result));
        Ok(())
    });

    // Logical implication: a ⇒ b is (!a) || b
    interp.register("implies", |interp| {
        let b = interp.stack_mut().pop()?;
        let a = interp.stack_mut().pop()?;
        let result = !to_bool(&a) || to_bool(&b);
        interp.stack_mut().push(make_bool(result));
        Ok(())
    });

    interp.register("⇒", |interp| {
        let b = interp.stack_mut().pop()?;
        let a = interp.stack_mut().pop()?;
        let result = !to_bool(&a) || to_bool(&b);
        interp.stack_mut().push(make_bool(result));
        Ok(())
    });

    // ─────────────────────────────────────────────────────────────────────
    // COMPARISON OPERATIONS
    // ─────────────────────────────────────────────────────────────────────

    // Equality
    interp.register("eq", |interp| {
        let b = interp.stack_mut().pop()?;
        let a = interp.stack_mut().pop()?;
        let result = values_equal(&a, &b);
        interp.stack_mut().push(make_bool(result));
        Ok(())
    });

    // Inequality
    interp.register("neq", |interp| {
        let b = interp.stack_mut().pop()?;
        let a = interp.stack_mut().pop()?;
        let result = !values_equal(&a, &b);
        interp.stack_mut().push(make_bool(result));
        Ok(())
    });

    // Greater than
    interp.register("gt", |interp| {
        let b = interp.stack_mut().pop()?.as_float()?;
        let a = interp.stack_mut().pop()?.as_float()?;
        interp.stack_mut().push(make_bool(a > b));
        Ok(())
    });

    // Less than
    interp.register("lt", |interp| {
        let b = interp.stack_mut().pop()?.as_float()?;
        let a = interp.stack_mut().pop()?.as_float()?;
        interp.stack_mut().push(make_bool(a < b));
        Ok(())
    });

    // Greater than or equal
    interp.register("gte", |interp| {
        let b = interp.stack_mut().pop()?.as_float()?;
        let a = interp.stack_mut().pop()?.as_float()?;
        interp.stack_mut().push(make_bool(a >= b));
        Ok(())
    });

    // Less than or equal
    interp.register("lte", |interp| {
        let b = interp.stack_mut().pop()?.as_float()?;
        let a = interp.stack_mut().pop()?.as_float()?;
        interp.stack_mut().push(make_bool(a <= b));
        Ok(())
    });

    // ─────────────────────────────────────────────────────────────────────
    // QUANTIFIERS
    // ─────────────────────────────────────────────────────────────────────

    // For all: check if top n values are all truthy
    // Stack: v1 v2 ... vn n → 1|0
    interp.register("∀", |interp| {
        let n = interp.stack_mut().pop()?.as_int()? as usize;
        let mut all_true = true;
        for _ in 0..n {
            let v = interp.stack_mut().pop()?;
            if !to_bool(&v) {
                all_true = false;
            }
        }
        interp.stack_mut().push(make_bool(all_true));
        Ok(())
    });

    // Exists: check if any of top n values is truthy
    // Stack: v1 v2 ... vn n → 1|0
    interp.register("∃", |interp| {
        let n = interp.stack_mut().pop()?.as_int()? as usize;
        let mut any_true = false;
        for _ in 0..n {
            let v = interp.stack_mut().pop()?;
            if to_bool(&v) {
                any_true = true;
            }
        }
        interp.stack_mut().push(make_bool(any_true));
        Ok(())
    });

    // ─────────────────────────────────────────────────────────────────────
    // BOOLEAN CONVERSIONS
    // ─────────────────────────────────────────────────────────────────────

    // Convert to boolean (0 or 1)
    interp.register("bool", |interp| {
        let v = interp.stack_mut().pop()?;
        interp.stack_mut().push(make_bool(to_bool(&v)));
        Ok(())
    });

    // Logical negation (returns 0 or 1)
    interp.register("lnot", |interp| {
        let v = interp.stack_mut().pop()?;
        interp.stack_mut().push(make_bool(!to_bool(&v)));
        Ok(())
    });

    // ─────────────────────────────────────────────────────────────────────
    // CATEGORY THEORY
    // ─────────────────────────────────────────────────────────────────────

    // Add an object to the category
    // Stack: "A" → ()
    interp.register("cat_obj", |interp| {
        let obj = interp.stack_mut().pop()?.as_string()?;
        if let Ok(mut cat) = category_state().lock() {
            cat.add_object(obj.clone());
            println!("[category_theory] added object: {}", obj);
        }
        Ok(())
    });

    // Add a morphism to the category
    // Stack: "A" "B" "f" → () (defines f : A -> B)
    interp.register("cat_mor", |interp| {
        let name = interp.stack_mut().pop()?.as_string()?;
        let to = interp.stack_mut().pop()?.as_string()?;
        let from = interp.stack_mut().pop()?.as_string()?;

        if let Ok(mut cat) = category_state().lock() {
            cat.add_morphism(name.clone(), from.clone(), to.clone());
            println!("[category_theory] added morphism: {} : {} -> {}", name, from, to);
        }
        Ok(())
    });

    // Compose two morphisms
    // Stack: "f" "g" → "g ∘ f" (if composable)
    interp.register("cat_comp", |interp| {
        let g_name = interp.stack_mut().pop()?.as_string()?;
        let f_name = interp.stack_mut().pop()?.as_string()?;

        if let Ok(cat) = category_state().lock() {
            let f = cat.find_morphism(&f_name);
            let g = cat.find_morphism(&g_name);

            match (f, g) {
                (Some(f), Some(g)) => {
                    if f.to == g.from {
                        let comp_name = format!("{} ∘ {}", g_name, f_name);
                        interp.stack_mut().push(WofValue::string(comp_name));
                    } else {
                        println!(
                            "cat_comp: cannot compose {} ∘ {} (cod(f) = {} ≠ dom(g) = {})",
                            g_name, f_name, f.to, g.from
                        );
                    }
                }
                _ => {
                    println!("cat_comp: unknown morphism(s): {}, {}", f_name, g_name);
                }
            }
        }
        Ok(())
    });

    // Get hom-set
    // Stack: "A" "B" → "Hom(A,B) = {...}"
    interp.register("cat_hom", |interp| {
        let to = interp.stack_mut().pop()?.as_string()?;
        let from = interp.stack_mut().pop()?.as_string()?;

        if let Ok(cat) = category_state().lock() {
            let homset = cat.hom(&from, &to);
            let result = format!("Hom({},{}) = {{{}}}", from, to, homset.join(", "));
            interp.stack_mut().push(WofValue::string(result));
        }
        Ok(())
    });

    // Show category summary
    // Stack: () → summary-string
    interp.register("cat_show", |interp| {
        if let Ok(cat) = category_state().lock() {
            let summary = cat.summary();
            interp.stack_mut().push(WofValue::string(summary));
        }
        Ok(())
    });

    // Clear the category
    interp.register("cat_clear", |_interp| {
        if let Ok(mut cat) = category_state().lock() {
            cat.clear();
            println!("[category_theory] category cleared");
        }
        Ok(())
    });

    // ─────────────────────────────────────────────────────────────────────
    // HELP
    // ─────────────────────────────────────────────────────────────────────

    interp.register("logic_help", |_interp| {
        println!("Logic Operations:");
        println!();
        println!("  Boolean Gates:");
        println!("    a b and       → a ∧ b");
        println!("    a b or        → a ∨ b");
        println!("    a b xor       → a ⊕ b");
        println!("    a not         → ¬a");
        println!("    a b nand/⊼    → ¬(a ∧ b)");
        println!("    a b nor/⊽     → ¬(a ∨ b)");
        println!("    a b xnor      → a ↔ b");
        println!("    a b implies/⇒ → a → b");
        println!();
        println!("  Comparisons:");
        println!("    a b eq        → a = b");
        println!("    a b neq       → a ≠ b");
        println!("    a b gt/lt     → a > b / a < b");
        println!("    a b gte/lte   → a ≥ b / a ≤ b");
        println!();
        println!("  Quantifiers:");
        println!("    v1..vn n ∀    → all true?");
        println!("    v1..vn n ∃    → any true?");
        println!();
        println!("  Category Theory:");
        println!("    \"A\" cat_obj               → add object");
        println!("    \"A\" \"B\" \"f\" cat_mor       → add f : A → B");
        println!("    \"f\" \"g\" cat_comp          → g ∘ f");
        println!("    \"A\" \"B\" cat_hom           → Hom(A,B)");
        println!("    cat_show                  → summary");
        println!("    cat_clear                 → reset");
        Ok(())
    });
}
