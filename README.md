[README.md]

<p align="center">
  <a href="https://github.com/whisprer/woflang-alpha-r">
    <img src="https://img.shields.io/github/stars/whisprer/woflang-alpha-r?style=for-the-badge" alt="GitHub stars" />
  </a>
  <a href="https://github.com/whisprer/woflang-alpha-r/issues">
    <img src="https://img.shields.io/github/issues/whisprer/woflang-alpha-r?style=for-the-badge" alt="GitHub issues" />
  </a>
  <a href="https://github.com/whisprer/woflang-alpha-r/fork">
    <img src="https://img.shields.io/github/forks/whisprer/woflang-alpha-r?style=for-the-badge" alt="GitHub forks" />
  </a>
</p>

# WofLang Alpha-R
## A Unicode-Native Stack Language for Creative & Symbolic Computation

<p align="center">
  <a href="https://github.com/whisprer/woflang-alpha-r/releases"> 
    <img src="https://img.shields.io/github/v/release/whisprer/woflang-alpha-r?color=4CAF50&label=release" alt="Release"> 
  </a>
  <a href="https://github.com/whisprer/woflang-alpha-r/blob/main/LICENSE"> 
    <img src="https://img.shields.io/badge/license-MIT-green.svg" alt="License"> 
    <img src="https://img.shields.io/badge/License-CC0_1.0-lightgrey.svg" alt="License">
  </a>
  <img src="https://img.shields.io/badge/rust-1.70%2B-orange.svg" alt="Rust">
  <img src="https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS-lightgrey.svg" alt="Platform">
</p>

[![GitHub](https://img.shields.io/badge/GitHub-whisprer%2Fwoflang--alpha--r-blue?logo=github&style=flat-square)](https://github.com/whisprer/woflang-alpha-r)
[![C++ Version](https://img.shields.io/badge/C%2B%2B-v10.1.1-00599C?logo=cplusplus&style=flat-square)](https://github.com/whisprer/woflang-alpha-r/tree/main/cpp-v10.1.1)
[![Rust Version](https://img.shields.io/badge/Rust-v0.0.3-CE422B?logo=rust&style=flat-square)](https://github.com/whisprer/woflang-alpha-r/tree/main/rs-v0.0.1)
[![License](https://img.shields.io/badge/License-MIT%20%2F%20Apache--2.0-green?style=flat-square)](LICENSE.md)
[![Status](https://img.shields.io/badge/Status-Alpha%20Release-orange?style=flat-square)](CHANGELOG.md)

---


----------------------------------------------------------------

╦ ╦┌─┐┌─┐┬  ┌─┐┌┐┌┌─┐  
║║║│ │├┤ │  ├─┤││││ ┬  
╚╩╝└─┘└  ┴─┘┴ ┴┘└┘└─┘ v10.1.1/v.0.3

# WofLang - A Unicode-native Stack Language  
**Creative, Symbolic, Secure Computation for the Modern Era**

----------------------------------------------------------------

---


## What is WofLang?

**WofLang isn't just another programming language—it's a symbolic operating system for thought.**

Inspired by Forth, J, and symbolic mathematics, WofLang lets you express computation as visual glyphs, manipulate meaning through stack operations, and explore code as art, logic, and ritual. It combines:

- **Stack-Based Elegance**: Clean, Forth-like execution model
- **Unicode Native**: Every symbol, rune, and glyph is valid code
- **Modular & Extensible**: Dynamic plugin architecture for endless expansion
- **Symbolic Power**: Pattern matching, simplification, and equation solving
- **Philosophical Edge**: Support for sacred geometry, consciousness functions, and creative exploration
- **Dual Implementation**: Production C++23 (v10.1.1) + emerging Rust port (v0.0.1)

---

## Core Philosophy

> Code must be **visual**. Function structure must **feel like a block**. Every instruction is a **glyph**. Every glyph, a **memory**. Syntax should be interpretable by **human intuition and machine precision** alike.

WofLang is **expression-first**, **block-structured**, **symbol-powered**—where clarity and compression coexist.

---

## Quick Start

### Prerequisites

**C++ v10.1.1:**
- C++23 Compiler (MSVC, GCC, or Clang)
- CMake 3.16+
- Windows-first, Linux port-friendly

**Rust v0.0.1:**
- Rust 1.70+ (stable channel)
- Cargo

### Building C++ v10.1.1

```bash
git clone https://github.com/whisprer/woflang-alpha-r.git
cd woflang-alpha-r/cpp-v10.1.1

# Build
./clean-n-build.sh  # Unix/macOS
# OR manually:
cmake -B build
cmake --build build --config Release

# Run REPL
./build/woflang  # Unix/macOS
# OR
build\woflang.exe  # Windows
```

### Building Rust v0.0.1

```bash
cd woflang-alpha-r/rs-v0.0.1
cargo build --release
cargo run --bin woflang-cli
```

### Your First Program

```wof
# Simple stack arithmetic (C++ or Rust)
2 3 +    # Push 2, push 3, add → 5
5 *      # Multiply by 5 → 25
print    # Output: 25

# Symbolic math (C++ v10.1.1)
x x +    # Add symbol x to itself
simplify # Simplify: 2x
```

---

## Features at a Glance

### ✅ Implemented

| Feature                   | Status | Details                                      |
|---------------------------|--------|----------------------------------------------|
| Stack Frame Isolation     |   ✅   | Local scope per function call                |
| Return Stack              |   ✅   | Proper function call/return semantics        |
| Scope Stack               |   ✅   | Variable isolation across blocks             |
| Function Definition       |   ✅   | `⊕name ⺆...⺘` syntax with recursion       |
| Jump-to-Labels            |   ✅   | `:label` and `@label` directives             |
| Conditional Execution     |   ✅   | `若` (if), `則` (then), `或` (else)          |
| Block Preprocessing       |   ✅   | Fast nested block parsing                    |
| Plugin Loading            |   ✅   | Dynamic `.so`/`.dll` plugin system           |
| REPL Commands             |   ✅   | `:stack`, `:scope`, `:labels`, `:help`, etc. |
| Error Diagnostics         |   ✅   | Detailed glyph + IP reporting                |
| History + Multiline REPL  |   ✅   | Full line editing with persistence           |
| Trigonometric Ops         |   ✅   | `sin`, `cos`, `tan`, `asin`, `acos`, etc.    |
| Symbolic Math             |   ✅   | Simplification rules, pattern matching       |
| Logical Operations        |   ✅   | `and`, `or`, `not`, `implies`, `iff`         |
| Exponential & Logarithmic |   ✅   | `exp`, `ln`, `log2`, `log10`                 |

### ⏳ Planned

| Feature              | Timeline      | Notes                           |
|----------------------|---------------|---------------------------------|
| Glyph Autocompletion | Next Release  | TAB-based symbol completion     |
| Debug Trace Mode     | Stabilization | Step-by-step execution tracing  |
| Quantum Operations   | Experimental  | Quantum state primitives        |
| Fractal Engine       | Experimental  | Mandelbrot/Julia set generation |
| Neural Chess         | Experimental  | AI-powered chess evaluator      |

---

## Architecture

### C++ v10.1.1 Structure

```
cpp-v10.1.1/
├── src/
│   ├── core/
│   │   ├── woflang.hpp          # Main interpreter API
│   │   ├── woflang.cpp          # Core implementation
│   │   ├── simd.hpp             # SIMD token processing
│   │   └── woflang_compat.hpp   # Compatibility layer
│   ├── io/
│   │   └── tokenizer.{hpp,cpp}  # Unicode token parsing
│   └── repl/
│       └── repl_main.cpp        # Interactive environment
├── plugins/                      # 40+ plugin modules
│   ├── mathlib_exponentials.cpp
│   ├── symbolic_logic_ops.cpp
│   ├── trig_ops.cpp
│   ├── prophecy_ops.cpp         # Easter eggs!
│   └── ... (40+ more)
├── CMakeLists.txt               # Build configuration
└── clean-n-build.sh            # One-command build

```

### Rust v0.0.1 Structure

```
rs-v0.0.1/
├── crates/
│   ├── woflang-core/            # Core types & stack machine
│   │   └── src/
│   │       ├── lib.rs           # Public API
│   │       ├── stack.rs         # Stack implementation
│   │       ├── value.rs         # WofValue types
│   │       ├── instruction.rs   # Instruction set
│   │       ├── opcode.rs        # Opcode table
│   │       ├── block.rs         # Block structures
│   │       ├── scope.rs         # Scope management
│   │       ├── unit.rs          # Unit handling
│   │       ├── error.rs         # Error types
│   │       └── span.rs          # Source spans
│   ├── woflang-ops/             # Standard operations
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── arithmetic.rs
│   │       ├── logic.rs
│   │       ├── math.rs
│   │       ├── crypto.rs
│   │       ├── quantum.rs
│   │       ├── io.rs
│   │       ├── stack.rs
│   │       └── constants.rs
│   ├── woflang-runtime/         # Interpreter engine
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── interpreter.rs
│   │       ├── tokenizer.rs
│   │       ├── registry.rs
│   │       └── plugin.rs
│   └── woflang-cli/             # Command-line interface
│       └── src/
│           └── main.rs
├── Cargo.toml                   # Workspace manifest
└── Cargo.lock                   # Lock file
```

---

## Plugin System

WofLang ships with **40+ plugins** covering:

### Mathematics
- **Constants**: π, e, φ, physical constants, Modelica units
- **Trigonometry**: sin, cos, tan, asin, acos, atan, sinh, cosh, tanh
- **Exponentials**: exp, ln, log2, log10
- **Calculus**: derivative (symbolic), integral (stub)
- **Symbolic**: simplification rules, linear equation solving

### Logic & Control
- **Logical Ops**: and, or, not, implies (→), iff (↔)
- **Quantifiers**: forall (∀), exists (∃) – partial
- **Control Flow**: if/then/else with pattern matching

### Creative & Experimental
- **Prophecy Ops**: Mystical stack divination
- **Void Division**: Division by the abyss
- **Stack Slayer**: Dramatic stack demolition ⚔️
- **Learning Mode**: Educational hints and lessons
- **Markov Suggestions**: AI-powered math suggestions

### Cryptography & Advanced
- **Crypto Ops**: Modular exponentiation, prime checking
- **Graph Theory**: Coloring, shortest path, DFS/BFS
- **Discrete Math**: Category theory, discrete transforms

### Easter Eggs
- `:egg` – Cryptic glyph haiku
- `:unlock chaos` – Forbidden glyphs (one session)
- `:dreamlog` – Surreal debug output
- `:mirror` – Reverse-stack mode
- `:deity` – Unlimited recursion (no guards)

---

## REPL Commands

| Command                 | Purpose                   |
|-------------------------|---------------------------|
| `:stack`                | Display current stack     |
| `:scope`                | Show variable scope       |
| `:labels`               | List defined labels       |
| `:frames`               | Show call stack           |
| `:dump`                 | Full interpreter state    |
| `:help [glyph]`         | Show help for glyph       |
| `:trace`                | Enable execution tracing  |
| `:untrace`              | Disable tracing           |
| `:bind <alias> <glyph>` | Create keybinding         |
| `:bindings`             | List all keybindings      |
| `:blocks`               | Show defined blocks       |
| `:tree`                 | Display block tree        |
| `:egg`                  | Random Easter egg         |
| `:exit` or `Ctrl+D`     | Exit REPL                 |

---

## Unicode Glyph Cheatsheet

### Arithmetic
| Symbol | Operation | Example      |
|--------|-----------|--------------|
| `+`    | Add       | `2 3 +` → 5  |
| `-`    | Subtract  | `5 3 -` → 2  |
| `*`    | Multiply  | `3 4 *` → 12 |
| `/`    | Divide    | `12 3 /` → 4 |
| `%`    | Modulo    | `7 3 %` → 1  |

### Math Functions
| Symbol | Function    | Example          |
|--------|-------------|------------------|
| `√`    | Square root | `16 √` → 4      |
| `∛`    | Cube root   | `8 ∛` → 2        |
| `^`    | Power       | `2 10 ^` → 1024  |
| `sin`  | Sine        | `π sin` → 0      |
| `cos`  | Cosine      | `0 cos` → 1      |
| `∑`    | Sum (block) | `[1 2 3] ∑` → 6 |

### Logic & Control
| Symbol | Meaning | Example                 |
|--------|---------|-------------------------|
| `若`   | If      | `若 condition ⺆ ... ⺘` |
| `則`   | Then    | Part of if-then-else    |
| `或`   | Else    | Part of if-then-else    |
| `∧`    | And     | `true false ∧` → false |
| `∨`    | Or      | `true false ∨` → true  |
| `¬`    | Not     | `true ¬` → false       |

### Function Definition
| Symbol   | Meaning         |
|----------|-----------------|
| `⊕name` | Define function |
| `⺆`     | Block start     |
| `⺘`     | Block end       |
| `至`     | Return          |

### Stack Operations
| Symbol | Operation          |
|--------|--------------------|
| `dup`  | Duplicate top      |
| `drop` | Remove top         |
| `swap` | Swap top two       |
| `over` | Copy second to top |
| `rot`  | Rotate top 3       |

---

## API Compatibility

### C++ v10.1.1 Plugin API

All plugins use the modern v10.1.1 API:

```cpp
#include "../../src/core/woflang.hpp"

class MyPlugin : public woflang::WoflangPlugin {
public:
    void register_ops(woflang::WoflangInterpreter& interp) override {
        interp.register_op("my_op", [](woflang::WoflangInterpreter& ip) {
            if (ip.stack.empty()) return;
            auto val = ip.stack.back();
            ip.stack.pop_back();
            // Process val
            ip.stack.push_back(woflang::WofValue::make_double(result));
        });
    }
};

extern "C" WOFLANG_PLUGIN_EXPORT void
register_plugin(woflang::WoflangInterpreter& interp) {
    static MyPlugin plugin;
    plugin.register_ops(interp);
}
```

### Rust v0.0.1 API

```rust
use woflang_runtime::{Interpreter, Operation};
use woflang_ops::math;

fn main() {
    let mut interpreter = Interpreter::new();
    
    // Load operations
    interpreter.register_op("sin", |stack| {
        // Implementation
    });
    
    // Execute
    interpreter.execute("2 3 +")?;
}
```

---

## Building & Deployment

### C++ (Linux/macOS)

```bash
cd cpp-v10.1.1
./clean-n-build.sh
# Output: ./build/woflang (REPL executable)
```

### C++ (Windows MSVC)

```bash
cd cpp-v10.1.1
cmake -B build -G "Visual Studio 16"
cmake --build build --config Release
# Output: build\Release\woflang.exe
```

### Rust

```bash
cd rs-v0.0.1
cargo build --release
cargo run --release --bin woflang-cli
```

### Docker (Optional)

```dockerfile
FROM rust:latest
WORKDIR /app
COPY rs-v0.0.1 .
RUN cargo build --release
CMD ["./target/release/woflang-cli"]
```

---

## Documentation

- **[Language Philosophy](useful-docs/language_philosophy.md)** – Design principles
- **[Unicode Glyph Spec](useful-docs/unicode_glyphmap_spec.md)** – Complete symbol reference
- **[Setup Guide](useful-docs/woflang_setup_guide.md)** – Integration instructions
- **[Easter Eggs & Secrets](useful-docs/woflang_easter_eggs_and_secrets.md)** – Hidden gems
- **[Features Registry](useful-docs/features.md)** – Implemented & planned features
- **[File Structure](useful-docs/file-structure.md)** – Repository layout
- **[API Reference](useful-docs/old-v-new-api.md)** – Plugin development guide

---

## Community & Contributing

- **Bugs & Features**: [GitHub Issues](https://github.com/whisprer/woflang-alpha-r/issues)
- **Pull Requests**: Welcome! See [CONTRIBUTING.md](CONTRIBUTING.md)
- **Discussions**: [GitHub Discussions](https://github.com/whisprer/woflang-alpha-r/discussions)

### Plugin Development

Interested in extending WofLang? Check out the plugin template in `useful-docs/how_to_module.md` and existing plugins in `cpp-v10.1.1/plugins/`.

---

## Roadmap

### v10.2 (C++)
- [ ] Glyph autocompletion in REPL
- [ ] Comprehensive trace mode
- [ ] Performance optimizations (SIMD improvements)
- [ ] Extended unit system

### v0.1 (Rust)
- [ ] Feature parity with C++ v10.1.1
- [ ] WebAssembly (WASM) backend
- [ ] Distributed plugin loading
- [ ] Improved error messages

### Experimental
- [ ] Quantum operation primitives
- [ ] Fractal generation engine
- [ ] Neural network integration
- [ ] Multi-dimensional arrays

---

## License

Dual licensed under:
- **Hybrid MIT/CC0 License** – See [LICENSE.md](LICENSE.md)
- **Apache License 2.0** – See [LICENSE-APACHE](LICENSE-APACHE)

Choose whichever license works best for your project.

---

## Gallery

### C++ v10.1.1 Example

```wof
# Fibonacci sequence
⊕fib ⺆
  dup 2 < 若 ⺆ 至 ⺘ 則
  dup 1 - fib
  swap 2 - fib +
⺘

10 fib print  # Output: 55
```

### Rust v0.0.1 Example

```rust
interpreter.execute(r#"
  2 3 +
  print
"#)?;
// Output: 5
```

---

## Credits & Acknowledgments

**WofLang** is inspired by:
- **Forth** – Stack-based execution model
- **J** – Symbolic mathematics
- **Lisp** – Functional paradigms
- **Unicode** – Global symbol support

Built with <3 by [whisprer](https://github.com/whisprer)

specila thnx to Claude Opus4.5/4.1, Claude Sonnet4.5/4.1/4/3.5,
ChatGPT5.1/5/4o/o1/3.5, Grok4.1/4/3, GeminiProo3/2.5/2.5flash,
Perplexity, Kimi K2 Thinking, and Sonar. RIP/Long Live!

---

## Star History

If you find WofLang useful, please consider starring the repo!

---

## Support

Need help? 

- Check the [documentation](useful-docs/)
- Open a [discussion](https://github.com/whisprer/woflang-alpha-r/discussions)
- Report a [bug](https://github.com/whisprer/woflang-alpha-r/issues)
- Request a [feature](https://github.com/whisprer/woflang-alpha-r/issues)

---

**WofLang v10.1.1 (C++) + v0.0.1 (Rust) – The language of glyphs and infinite possibility.**

*"Every symbol is a memory. Every glyph, a thought. Code as art."*
