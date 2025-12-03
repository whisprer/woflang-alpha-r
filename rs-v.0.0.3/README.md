# 🐺 Woflang

[![CI](https://github.com/wofl/woflang-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/wofl/woflang-rs/actions)
[![Crates.io](https://img.shields.io/crates/v/woflang-core.svg)](https://crates.io/crates/woflang-core)
[![Documentation](https://docs.rs/woflang-core/badge.svg)](https://docs.rs/woflang-core)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](#license)

**A Unicode-native stack-based programming language.**

Woflang is a stack-based interpreter inspired by Forth, designed with first-class Unicode support for mathematical notation, quantum computing symbols, and expressive operators.

```
wof> 42 17 +
→ 59

wof> π 2 / sin
→ 1.0

wof> |0⟩ H measure
→ 1

wof> 1 0 → 
→ 0
```

## ✨ Features

- **Unicode-native**: Use `π`, `∧`, `→`, `|0⟩` directly as operators
- **Stack-based paradigm**: Simple, powerful, Forth-inspired execution model
- **Quantum simulation**: Qubit states, gates (H, X, Y, Z), measurement
- **Cryptography**: Miller-Rabin primality testing, modular arithmetic, hashing
- **Plugin system**: Extend with compile-time or dynamic plugins
- **SIMD-ready**: Data layouts optimized for future vectorization
- **Zero-cost abstractions**: Trait-based design with minimal runtime overhead

## 🚀 Quick Start

### Installation

```bash
# From crates.io (when published)
cargo install woflang-cli

# From source
git clone https://github.com/wofl/woflang-rs
cd woflang-rs
cargo install --path crates/woflang-cli
```

### Usage

```bash
# Start REPL
woflang

# Execute a script
woflang script.wof

# Run benchmarks
woflang --benchmark

# Run test suite
woflang --test
```

### REPL Commands

| Command | Description |
|---------|-------------|
| `help` | Show available operations |
| `quit` / `exit` | Exit the interpreter |
| `.` / `.s` | Display stack contents |
| `test` | Run built-in test suite |
| `benchmark` | Run prime benchmarking |

## 📚 Language Reference

### Stack Operations

| Op | Effect | Description |
|----|--------|-------------|
| `dup` | `(a -- a a)` | Duplicate top |
| `drop` | `(a -- )` | Remove top |
| `swap` | `(a b -- b a)` | Swap top two |
| `over` | `(a b -- a b a)` | Copy second to top |
| `rot` | `(a b c -- b c a)` | Rotate top three |

### Arithmetic

| Op | Effect | Description |
|----|--------|-------------|
| `+` | `(a b -- c)` | Addition |
| `-` | `(a b -- c)` | Subtraction |
| `*` / `×` | `(a b -- c)` | Multiplication |
| `/` / `÷` | `(a b -- c)` | Division |
| `pow` / `^` | `(a b -- c)` | Exponentiation |
| `sqrt` / `√` | `(a -- b)` | Square root |

### Logic

| Op | Effect | Description |
|----|--------|-------------|
| `and` / `∧` | `(a b -- c)` | Logical AND |
| `or` / `∨` | `(a b -- c)` | Logical OR |
| `not` / `¬` | `(a -- b)` | Logical NOT |
| `implies` / `→` | `(a b -- c)` | Material implication |
| `iff` / `↔` | `(a b -- c)` | Biconditional |

### Quantum Computing

| Op | Effect | Description |
|----|--------|-------------|
| `\|0⟩` | `( -- q)` | Create ket-zero state |
| `\|1⟩` | `( -- q)` | Create ket-one state |
| `H` | `(q -- q')` | Hadamard gate |
| `X` | `(q -- q')` | Pauli-X (NOT) gate |
| `Z` | `(q -- q')` | Pauli-Z gate |
| `measure` | `(q -- n)` | Collapse and measure |
| `bell` | `( -- state)` | Create Bell state |

### Cryptography

| Op | Effect | Description |
|----|--------|-------------|
| `prime_check` | `(n -- b)` | Miller-Rabin primality test |
| `next_prime` | `(n -- p)` | Find next prime |
| `random` | `(lo hi -- n)` | Random integer in range |
| `mod_exp` | `(b e m -- r)` | Modular exponentiation |
| `hash` | `(n -- h)` | FNV-1a hash |

### Constants

| Op | Value | Description |
|----|-------|-------------|
| `pi` / `π` | 3.14159... | Circle ratio |
| `e` | 2.71828... | Euler's number |
| `phi` / `φ` | 1.61803... | Golden ratio |
| `tau` / `τ` | 6.28318... | Full circle |
| `avogadro` | 6.022×10²³ | Avogadro's constant |
| `c` | 299792458 | Speed of light (m/s) |

## 🏗️ Architecture

The project is organized as a Cargo workspace with four crates:

```
woflang/
├── crates/
│   ├── woflang-core/      # Value types, stack, errors
│   ├── woflang-runtime/   # Interpreter, tokenizer, plugins
│   ├── woflang-ops/       # Standard library operations
│   └── woflang-cli/       # Binary with REPL
├── Cargo.toml             # Workspace configuration
└── .github/workflows/     # CI configuration
```

### Design Principles

1. **Zero-cost abstractions**: Trait-based dispatch with monomorphization where possible
2. **SIMD-ready layouts**: Values aligned to 16 bytes for future vectorization
3. **Memory safety**: No `unsafe` in core logic; minimal unsafe for plugin FFI
4. **Compile-time registration**: Prefer static operation registry over dynamic loading

## 🔌 Extending Woflang

### Adding Operations

```rust
use woflang_runtime::Interpreter;
use woflang_core::{WofValue, Result};

fn my_op(interp: &mut Interpreter) -> Result<()> {
    let val = interp.stack_mut().pop_numeric()?;
    interp.push(WofValue::double(val * 2.0));
    Ok(())
}

fn main() {
    let mut interp = Interpreter::new();
    woflang_ops::register_all(&mut interp);
    interp.register("double", my_op);
    
    interp.exec_line("21 double").unwrap();
    // Stack: [42.0]
}
```

### Dynamic Plugins

Enable with `cargo build --features dynamic-plugins`:

```rust
// plugin.rs - compile as shared library
use woflang_runtime::Interpreter;

#[no_mangle]
pub extern "C" fn register_plugin(interp: &mut Interpreter) {
    interp.register("my_op", |i| {
        // ...
        Ok(())
    });
}
```

## 🧪 Testing

```bash
# Run all tests
cargo test --workspace --all-features

# Run with Miri (undefined behavior check)
cargo +nightly miri test -p woflang-core

# Run benchmarks
cargo bench
```

## 📊 Benchmarks

Prime checking performance (Miller-Rabin):

| Number | Time |
|--------|------|
| 97 | ~0.5 µs |
| 2,147,483,647 | ~2 µs |
| 10,000,000,019 | ~3 µs |

Run benchmarks with: `cargo bench` or `woflang --benchmark`

## 📜 License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## 🙏 Acknowledgments

- Original C++ implementation by wofl
- Inspired by Forth, Factor, and Joy
- Built with Rust 🦀

---

*"The type system is not your enemy. It is your shield." — ℝUST∅PHAGE*
