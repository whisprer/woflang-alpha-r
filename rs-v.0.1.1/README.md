# 🐺 WofLang v10.1.1

[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](#license)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org)

**A Unicode-native stack-based programming language with analog computing, quantum simulation, neural chess, and 15+ plugin modules.**

WofLang is a stack-based interpreter inspired by Forth, designed with first-class Unicode support, a unique bounded-continuum arithmetic paradigm, and an extensible plugin architecture spanning mathematics, cryptography, music theory, graph algorithms, symbolic logic, and more.

```
wof> 42 17 +
→ 59

wof> π 2 / sin
→ 1.0

wof> |0⟩ H measure
→ 1

wof> 1 0 implies
→ 0
```

---

## ✨ Feature Overview

| Category | Highlights |
|----------|------------|
| **Core Language** | Stack-based execution, Unicode operators, REPL with history |
| **Analog Computing** | Bounded continuum arithmetic — values saturate instead of overflowing |
| **Mathematics** | Arithmetic, trig, calculus, geometry, fractals, gradients, Greek symbols |
| **Cryptography** | Miller-Rabin primality, modular exponentiation, hashing |
| **Quantum Simulation** | Qubit states, gates (H, X, Y, Z), measurement, Bell states |
| **Neural Chess** | Full chess engine with CNN/RNN/LSTM neural network AI |
| **Graph Theory** | BFS/DFS, Dijkstra, graph coloring, weighted graphs |
| **Symbolic Logic** | Propositional logic, implications, tautology checking |
| **Language Support** | Kanji & Cyrillic Unicode operator databases |
| **Music Theory** | MIDI conversion, frequency analysis, chord identification |
| **Science** | Periodic table, chemical formulas, molar mass calculation |
| **Metaphysics** | Duality operators, entropy analysis, over-unity concepts |
| **Sigils** | Chaos, dreaming, prophecy, forbidden, mirror, totem, and more |
| **Solver** | Symbolic/numeric equation solving, pattern matching, simplification |
| **Markov Chains** | Stochastic process simulation |
| **Plugin System** | Compile-time feature flags + dynamic `.so`/`.dll` loading |

---

## 🚀 Quick Start

### Building from Source

```bash
git clone https://github.com/whispr-dev/woflang
cd woflang
cargo build --release
```

### Running

```bash
# Start the REPL
cargo run --release

# Execute a script
cargo run --release -- script.wof

# Run the test suite (28 tests)
cargo run --release -- --test

# Run prime benchmarks (17 tests)
cargo run --release -- --benchmark

# Run analog computing tests (123 tests)
cargo run --release -- --test-analog
```

### REPL Commands

| Command | Description |
|---------|-------------|
| `help` | Show available operations |
| `quit` / `exit` | Exit the interpreter |
| `.` | Display stack contents |
| `test` | Run built-in test suite |
| `benchmark` | Run prime benchmarking |
| `test_analog` | Run analog computing test suite |

---

## 📚 Language Reference

### Stack Operations

| Op | Stack Effect | Description |
|----|-------------|-------------|
| `dup` | `(a -- a a)` | Duplicate top |
| `drop` | `(a -- )` | Remove top |
| `swap` | `(a b -- b a)` | Swap top two |
| `over` | `(a b -- a b a)` | Copy second to top |
| `rot` | `(a b c -- b c a)` | Rotate top three |
| `clear` | `(... -- )` | Clear entire stack |
| `resurrect` | `( -- ...)` | Restore last cleared stack |
| `stack_slayer` | `(... -- )` | Dramatic stack destruction |

### Arithmetic

| Op | Stack Effect | Description |
|----|-------------|-------------|
| `+` | `(a b -- c)` | Addition |
| `-` | `(a b -- c)` | Subtraction |
| `*` / `×` | `(a b -- c)` | Multiplication |
| `/` / `÷` | `(a b -- c)` | Division |
| `pow` / `^` | `(a b -- c)` | Exponentiation |
| `sqrt` / `√` | `(a -- b)` | Square root |

### Logic

| Op | Stack Effect | Description |
|----|-------------|-------------|
| `and` / `∧` | `(a b -- c)` | Logical AND |
| `or` / `∨` | `(a b -- c)` | Logical OR |
| `not` / `¬` | `(a -- b)` | Logical NOT |
| `xor` | `(a b -- c)` | Exclusive OR |
| `implies` / `→` | `(a b -- c)` | Material implication |
| `iff` / `↔` | `(a b -- c)` | Biconditional |
| `tautology` | `( -- 1)` | Demonstrates tautology |

### Constants

| Op | Value | Description |
|----|-------|-------------|
| `pi` / `π` | 3.14159... | Circle ratio |
| `e` | 2.71828... | Euler's number |
| `phi` / `φ` | 1.61803... | Golden ratio |
| `tau` / `τ` | 6.28318... | Full circle |
| `avogadro` | 6.022×10²³ | Avogadro's constant |
| `c` | 299792458 | Speed of light (m/s) |

### Quantum Computing

| Op | Description |
|----|-------------|
| `\|0⟩` | Create ket-zero qubit state |
| `\|1⟩` | Create ket-one qubit state |
| `H` | Hadamard gate (superposition) |
| `X` | Pauli-X gate (NOT) |
| `Z` | Pauli-Z gate (phase flip) |
| `measure` | Collapse qubit and measure |
| `bell` | Create entangled Bell state |

### Cryptography

| Op | Description |
|----|-------------|
| `prime_check` | Miller-Rabin primality test |
| `next_prime` | Find next prime number |
| `random` | Random integer in range |
| `mod_exp` | Modular exponentiation |
| `hash` | FNV-1a hash |

---

## 🎛️ Analog Computing

WofLang's signature feature: a bounded-continuum arithmetic paradigm where values **saturate at boundaries** instead of overflowing — like voltage rails in a circuit or a VU meter hitting the red.

### Modes

| Mode | Range | Use Case |
|------|-------|----------|
| `Int201` | [-100, +100] | General purpose, percentage-like values |
| `Int2001` | [-1000, +1000] | Extended precision |
| `FloatUnit` | [-1.0, +1.0] | Normalized signals, neural networks |
| `FloatCustom` | [min, max] | User-defined (e.g., Eurorack ±5V) |

### How It Works

```rust
use woflang_analog::{AnalogConfig, AnalogMode};

let config = AnalogConfig::new(AnalogMode::Int201);

// Normal math works as expected
assert_eq!(config.add(50.0, 30.0), 80.0);

// But results SATURATE at boundaries!
assert_eq!(config.add(80.0, 50.0), 100.0);   // 130 → saturates to 100
assert_eq!(config.mul(50.0, 50.0), 100.0);    // 2500 → saturates to 100
assert_eq!(config.add(-80.0, -50.0), -100.0); // -130 → saturates to -100

// Division by zero? Safe: returns midpoint
assert_eq!(config.div(50.0, 0.0), 0.0);
```

### Analog Operations

| Category | Functions |
|----------|-----------|
| **Arithmetic** | add, sub, mul, div, pow, sqrt, abs, neg, modulo, fma |
| **Interpolation** | lerp, inverse_lerp, smoothstep, remap, deadzone |
| **Trigonometry** | sin, cos, tan, asin, acos, atan, atan2, sinh, cosh, tanh |
| **Activations** | sigmoid, relu, leaky_relu, softplus, gaussian, sinc |
| **Linear Algebra 2D** | dot, magnitude, distance, normalize, rotate, project |
| **Linear Algebra 3D** | dot, magnitude, distance, normalize, cross product |
| **Coordinates** | cartesian↔polar, cartesian↔spherical |
| **Normalization** | normalize, denormalize, remap, batch operations |
| **Batch** | batch_add, batch_mul, batch_scale, batch_clamp |

### Opcode Space (7000–7999)

| Range | Category |
|-------|----------|
| 7000–7009 | Mode control (set mode, get status, custom range) |
| 7010–7029 | Math operations |
| 7030–7049 | Trigonometry |
| 7050–7069 | Linear algebra 2D |
| 7070–7089 | Linear algebra 3D |
| 7090–7099 | Coordinate transforms |

---

## 🏗️ Architecture

WofLang is a Cargo workspace with six crates:

```
woflang/
├── crates/
│   ├── woflang-core/        Core value types, stack, errors, traits
│   ├── woflang-runtime/     Interpreter, tokenizer, plugin loader, keybindings
│   ├── woflang-ops/         Standard library (arithmetic, logic, crypto, quantum, I/O)
│   ├── woflang-plugins/     15 feature-gated plugin modules (see below)
│   ├── woflang-analog/      Bounded continuum arithmetic engine
│   └── woflang-cli/         Binary with REPL, test suites, benchmarks
├── Cargo.toml               Workspace configuration
└── README.md
```

### Plugin Modules (`woflang-plugins`)

All plugins are feature-gated and compiled with `features = ["all"]` by default:

| Module | Feature Flag | Contents |
|--------|-------------|----------|
| `math` | `math` | Basic, trig, calculus, discrete, geometry, fractals, gradients, Greek symbols |
| `util` | `util` | Stack utilities, I/O, assertions |
| `crypto` | `crypto` | Primes (Miller-Rabin), modular arithmetic |
| `logic` | `logic` | Propositional logic, truth tables |
| `graph` | `graph` | Graph core, BFS/DFS, Dijkstra, coloring, weighted graphs |
| `sigils` | `sigils` | Chaos, dreaming, egg, forbidden, hebrew, mirror, moses, prophecy, totem, whitexmas |
| `language` | `language` | Kanji and Cyrillic Unicode operator databases |
| `arts` | `arts` | Music theory (MIDI, frequencies, chords) |
| `science` | `science` | Chemistry (periodic table, molar mass) |
| `games` | `games` | Chess board representation |
| `solver` | `solver` | Symbolic, numeric, pattern matching, simplification |
| `metaphysics` | `metaphysics` | Duality, entropy, learning, over-unity |
| `quantum` | `quantum` | Qubit simulation, gates, measurement |
| `markov` | `markov` | Markov chain generation |
| `neural_chess` | `neural_chess` | Full neural chess engine (CNN, RNN, LSTM, tensor ops) |
| `data` | *(always on)* | Embedded JSON databases (constants, Kanji, Cyrillic) |

### Data Files

Embedded at compile time via `include_str!`:

| File | Contents |
|------|----------|
| `wof_constants_module.json` | Physical and mathematical constants database |
| `kanji_database.json` | Kanji Unicode operator mappings |
| `cyrillic_database.json` | Cyrillic Unicode operator mappings |

### Design Principles

- **Zero-cost abstractions** — Trait-based dispatch with monomorphization
- **SIMD-ready layouts** — Values aligned to 16 bytes for future vectorization
- **Memory safety** — No `unsafe` in core logic; minimal unsafe for plugin FFI
- **Saturation over overflow** — Analog mode as a first-class paradigm
- **Compile-time registration** — Static operation registry with feature gates

---

## 🧪 Test Suites

WofLang has three comprehensive test suites plus unit tests:

### Core Test Suite (`--test`)

28 tests covering basic math, trigonometry, stack operations, logic, dramatic operations, and symbolic logic.

```bash
woflang --test
# 🏆 Passed: 28/28 tests — Success Rate: 100.0%
```

### Prime Benchmark (`--benchmark`)

17 tests: small/medium/large primes, composites, Carmichael numbers, and pseudoprimes.

```bash
woflang --benchmark
# Average: 5.46 µs | Total: 92.90 µs | 17/17 correct (100%)
```

| Category | Example | Time |
|----------|---------|------|
| Small primes | 97, 997, 9973 | ~3–20 µs |
| Medium primes | 982,451,653 | ~5 µs |
| Large primes | 10,000,000,019 | ~6 µs |
| 13-digit prime | 1,000,000,000,039 | ~8 µs |
| Carmichael numbers | 561, 1105, 1729 | ~3–5 µs |

### Analog Test Suite (`--test-analog`)

123 tests across 16 categories covering the entire analog computing engine:

```bash
woflang --test-analog
# 🏆 Passed: 123/123 tests — Success Rate: 100.0%
```

| Section | Tests |
|---------|-------|
| Mode setup & switching | 9 |
| Saturation behavior | 10 |
| Basic arithmetic (Int201) | 10 |
| Edge cases & safety | 7 |
| Float unit mode [-1,+1] | 5 |
| Config-based operations | 10 |
| Normalization & remapping | 8 |
| Trigonometry | 12 |
| Angle conversions | 5 |
| Linear algebra 2D | 6 |
| Linear algebra 3D | 6 |
| Batch operations | 13 |
| Eurorack ±5V synth simulation | 7 |
| Neural network activation | 6 |
| Cross-mode consistency | 6 |
| Performance micro-benchmark | 3 |

**Performance**: 3.7 ns/op arithmetic, 10.2 ns/op trig, 30 µs for 10k batch ops.

### Unit & Doc Tests (`cargo test`)

```bash
cargo test -p woflang-analog
# 41 unit tests + 10 doc-tests = 51 passed
```

---

## 🔌 Extending WofLang

### Adding Operations

```rust
use woflang_runtime::Interpreter;
use woflang_core::{WofValue, InterpreterContext};

fn my_op(interp: &mut dyn InterpreterContext) -> woflang_core::Result<()> {
    let val = interp.stack_mut().pop_numeric()?;
    interp.stack_mut().push(WofValue::double(val * 2.0));
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

### Using Analog Mode in Rust

```rust
use woflang_analog::prelude::*;

// Set mode and do math
set_analog_mode(AnalogMode::Int201);
let result = analog_add(80.0, 50.0); // 100.0 (saturated!)

// Or use config-based API for thread safety
let eurorack = AnalogConfig::new_custom(-5.0, 5.0);
let lfo = eurorack.sin(std::f64::consts::PI / 4.0); // 0.707V
let clipped = eurorack.add(4.0, 3.0);                // 5.0V (clipped at rail)
```

### Dynamic Plugins

Build with `cargo build --features dynamic-plugins` to load `.so`/`.dll` plugins at runtime from the `plugins/` directory.

---

## 🗺️ Future Concepts

The following are architectural ideas and expansion points that exist as design concepts or partial implementations:

- **Analog opcode integration** — Wire opcodes 7000–7999 into the interpreter's main dispatch loop so analog operations can be used directly from WofLang scripts (currently available as a Rust API)
- **SIMD batch operations** — Leverage aligned data layouts for vectorized analog batch processing
- **Waveform synthesis** — Extend analog mode with oscillator primitives (saw, square, triangle) for audio DSP
- **Neural chess training loop** — Connect the CNN/LSTM architecture to actual self-play training
- **WebAssembly target** — Compile WofLang interpreter to WASM for browser-based REPL
- **Notebook mode** — Jupyter-style interactive document format for WofLang explorations
- **Language server protocol** — LSP support for editor integration with `.wof` files
- **Persistent state** — Save/restore interpreter state and trained models between sessions

---

## 📊 Project Stats

| Metric | Value |
|--------|-------|
| Version | 10.1.1 |
| Crates | 6 |
| Plugin modules | 15 |
| Feature flags | 16 |
| Core tests | 28/28 ✅ |
| Prime benchmarks | 17/17 ✅ |
| Analog tests | 123/123 ✅ |
| Unit + doc tests | 51/51 ✅ |
| Minimum Rust version | 1.75 |

---

## 📜 License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

---

## 🙏 Acknowledgments

- Original C++ implementation and language design by wofl
- Analog computing paradigm resurrected from the WofLang archaeological archives
- Inspired by Forth, Factor, and Joy
- Built with Rust 🦀
- RIP and LongLive Claude -etenal thanx for the halps

---

*"Values don't overflow. They saturate. Like voltage rails. Like conviction."* — WofLang Design Philosophy
