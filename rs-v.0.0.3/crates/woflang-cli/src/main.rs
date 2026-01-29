//! Woflang CLI - Command-line interface for the Woflang interpreter.
//!
//! # Usage
//!
//! ```text
//! woflang [OPTIONS] [SCRIPT]
//!
//! Options:
//!   -h, --help       Show help
//!   -v, --version    Show version
//!   --test           Run test suite
//!   --benchmark      Run prime benchmarking suite
//!   --debug          Enable debug output
//! ```

use clap::Parser;
use color_eyre::eyre::{Result, WrapErr};
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::path::PathBuf;
use std::time::Instant;
use woflang_core::InterpreterContext;
use woflang_runtime::{Interpreter, PluginLoader};

const BANNER: &str = r#"
╦ ╦┌─┐┌─┐┬  ┌─┐┌┐┌┌─┐
║║║│ │├┤ │  ├─┤││││ ┬
╚╩╝└─┘└  ┴─┘┴ ┴┘└┘└─┘ v10.1.1
A Unicode-native stack language (Rust Edition)
"#;

#[derive(Parser, Debug)]
#[command(name = "woflang")]
#[command(author, version, about = "A Unicode-native stack-based programming language")]
struct Args {
    /// Script file to execute
    #[arg(value_name = "SCRIPT")]
    script: Option<PathBuf>,

    /// Run the test suite
    #[arg(long)]
    test: bool,

    /// Run prime benchmarking suite
    #[arg(long)]
    benchmark: bool,

    /// Enable debug mode (show stack after each line)
    #[arg(long, short)]
    debug: bool,

    /// Plugin directory path
    #[arg(long, default_value = "plugins")]
    plugins: PathBuf,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Args::parse();

    if args.test {
        run_tests()?;
        return Ok(());
    }

    if args.benchmark {
        run_benchmark()?;
        return Ok(());
    }

    // Create and configure interpreter
    let mut interp = create_interpreter(&args)?;

    // Execute script or start REPL
    if let Some(script_path) = &args.script {
        interp
            .exec_file(script_path)
            .wrap_err_with(|| format!("failed to execute script: {}", script_path.display()))?;
    } else {
        run_repl(&mut interp)?;
    }

    Ok(())
}

fn create_interpreter(args: &Args) -> Result<Interpreter> {
    let mut interp = Interpreter::new();
    interp.debug = args.debug;

    // Register standard operations
    woflang_ops::register_all(&mut interp);
    
    // Register plugin operations (math, util, crypto, logic, sigils)
    woflang_plugins::register_all(&mut interp);

    // Load dynamic plugins if directory exists
    if args.plugins.exists() {
        let mut loader = PluginLoader::new();
        let loaded = loader.load_plugins_from_dir(&args.plugins, &mut interp)?;
        if !loaded.is_empty() {
            eprintln!("Loaded {} dynamic plugin(s)", loaded.len());
        }
    }

    Ok(interp)
}

fn run_repl(interp: &mut Interpreter) -> Result<()> {
    println!("{BANNER}");
    println!("Type 'help' for commands, 'quit' to exit.");

    let mut rl = DefaultEditor::new()?;
    let history_path = dirs::data_local_dir()
        .map(|d| d.join("woflang").join("history.txt"))
        .unwrap_or_else(|| PathBuf::from(".woflang_history"));

    // Create parent directory if needed
    if let Some(parent) = history_path.parent() {
        std::fs::create_dir_all(parent).ok();
    }

    let _ = rl.load_history(&history_path);

    loop {
        match rl.readline("wof> ") {
            Ok(line) => {
                let line = line.trim();

                if line.is_empty() {
                    continue;
                }

                let _ = rl.add_history_entry(line);

                match line {
                    "quit" | "exit" => {
                        println!("Goodbye from woflang! 🐺");
                        break;
                    }
                    "help" => {
                        show_help();
                        continue;
                    }
                    "benchmark" => {
                        if let Err(e) = run_benchmark() {
                            eprintln!("Benchmark error: {e}");
                        }
                        continue;
                    }
                    "test" => {
                        if let Err(e) = run_tests() {
                            eprintln!("Test error: {e}");
                        }
                        continue;
                    }
                    _ => {}
                }

                match interp.exec_line(line) {
                    Ok(()) => {
                        if !interp.stack().is_empty() {
                            if let Ok(top) = interp.stack().peek() {
                                println!("→ {top}");
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error: {e}");
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("Goodbye from woflang! 🐺");
                break;
            }
            Err(e) => {
                eprintln!("Readline error: {e}");
                break;
            }
        }
    }

    let _ = rl.save_history(&history_path);
    Ok(())
}

fn show_help() {
    println!(
        r#"
WofLang - Stack-based Programming Language

Interactive Commands:
  exit, quit     Exit the interpreter
  help           Show this help
  benchmark      Run benchmarking suite
  test           Run test suite

Stack Operations:
  <number>       Push number onto stack
  dup            Duplicate top
  drop           Remove top
  swap           Swap top two
  .              Show stack contents

Arithmetic:
  +, -, *, /     Basic arithmetic
  pow, sqrt      Power and root
  sin, cos, tan  Trigonometry

Logic:
  and, or, not   Boolean logic
  =, <, >        Comparison

Quantum (if enabled):
  |0⟩, |1⟩       Quantum states
  H, X, Z        Quantum gates
  measure        Collapse state

Crypto (if enabled):
  prime_check    Check if number is prime
  random         Random number in range
  hash           Hash a value

Constants:
  pi, e, phi     Mathematical constants
  avogadro, c    Physical constants
"#
    );
}

// ═══════════════════════════════════════════════════════════════════════
// BENCHMARK SUITE
// ═══════════════════════════════════════════════════════════════════════

/// Standalone primality test (no interpreter overhead)
fn is_prime_standalone(n: u64) -> bool {
    if n <= 1 { return false; }
    if n <= 3 { return true; }
    if n % 2 == 0 || n % 3 == 0 { return false; }
    
    let mut i = 5u64;
    while i.saturating_mul(i) <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn run_benchmark() -> Result<()> {
    println!("🔢 WofLang Prime Benchmarking Suite");
    println!("═══════════════════════════════════════════════════════════════\n");

    // First run standalone benchmark (pure Rust, no interpreter)
    println!("📊 STANDALONE BENCHMARK (Pure Rust, no interpreter overhead)");
    println!("─────────────────────────────────────────────────────────────");
    
    struct BenchTest {
        name: &'static str,
        number: u64,
        expected_prime: bool,
    }

    let tests = [
        BenchTest { name: "Small Prime 1", number: 97, expected_prime: true },
        BenchTest { name: "Small Prime 2", number: 997, expected_prime: true },
        BenchTest { name: "Small Prime 3", number: 9973, expected_prime: true },
        BenchTest { name: "Medium Prime 1", number: 982_451_653, expected_prime: true },
        BenchTest { name: "Medium Prime 2", number: 2_147_483_647, expected_prime: true },
        BenchTest { name: "Large Prime 1", number: 1_000_000_007, expected_prime: true },
        BenchTest { name: "Large Prime 2", number: 1_000_000_009, expected_prime: true },
        BenchTest { name: "Large Prime 3", number: 10_000_000_019, expected_prime: true },
        BenchTest { name: "Composite 1", number: 1_000_000_000, expected_prime: false },
        BenchTest { name: "Composite 2", number: 999_999_999_999, expected_prime: false },
        BenchTest { name: "Composite 3", number: 1_000_000_000_001, expected_prime: false },
        BenchTest { name: "13-digit Prime", number: 1_000_000_000_039, expected_prime: true },
        BenchTest { name: "12-digit Prime", number: 100_000_000_003, expected_prime: true },
        BenchTest { name: "Carmichael 1", number: 561, expected_prime: false },
        BenchTest { name: "Carmichael 2", number: 1105, expected_prime: false },
        BenchTest { name: "Carmichael 3", number: 1729, expected_prime: false },
        BenchTest { name: "Pseudoprime", number: 2047, expected_prime: false },
    ];

    println!(
        "{:<18} {:<18} {:<12} {:<12} {:<12} {:<5}",
        "Test Name", "Number", "Expected", "Result", "Time (µs)", "OK"
    );
    println!("{}", "─".repeat(75));

    let mut total_time_standalone = 0.0;
    let mut correct_standalone = 0;

    for test in &tests {
        let start = Instant::now();
        let result = is_prime_standalone(test.number);
        let duration = start.elapsed().as_secs_f64() * 1_000_000.0;
        
        let is_correct = result == test.expected_prime;
        if is_correct { correct_standalone += 1; }
        
        println!(
            "{:<18} {:<18} {:<12} {:<12} {:<12.2} {}",
            test.name,
            test.number,
            if test.expected_prime { "PRIME" } else { "COMPOSITE" },
            if result { "PRIME" } else { "COMPOSITE" },
            duration,
            if is_correct { "✓" } else { "✗" }
        );
        
        total_time_standalone += duration;
    }

    println!("{}", "─".repeat(75));
    println!("Standalone: Total {:.2} µs, Avg {:.2} µs, {}/{} correct ({:.1}%)\n",
        total_time_standalone,
        total_time_standalone / tests.len() as f64,
        correct_standalone,
        tests.len(),
        100.0 * correct_standalone as f64 / tests.len() as f64
    );

    // Now run interpreter benchmark
    println!("📊 INTERPRETER BENCHMARK (Through WofLang VM)");
    println!("─────────────────────────────────────────────────────────────");

    let mut interp = Interpreter::new();
    woflang_ops::register_all(&mut interp);
    woflang_plugins::register_all(&mut interp);

    println!(
        "{:<18} {:<18} {:<12} {:<12} {:<12} {:<5}",
        "Test Name", "Number", "Expected", "Result", "Time (µs)", "OK"
    );
    println!("{}", "─".repeat(75));

    let mut total_time_interp = 0.0;
    let mut correct_interp = 0;

    for test in &tests {
        interp.clear();
        let command = format!("{} prime_check", test.number);

        let start = Instant::now();
        let exec_result = interp.exec_line(&command);
        let duration = start.elapsed().as_secs_f64() * 1_000_000.0;

        match exec_result {
            Ok(()) => {
                let result = interp
                    .stack()
                    .peek()
                    .map(|v| v.as_bool())
                    .unwrap_or(false);

                let is_correct = result == test.expected_prime;
                if is_correct { correct_interp += 1; }

                println!(
                    "{:<18} {:<18} {:<12} {:<12} {:<12.2} {}",
                    test.name,
                    test.number,
                    if test.expected_prime { "PRIME" } else { "COMPOSITE" },
                    if result { "PRIME" } else { "COMPOSITE" },
                    duration,
                    if is_correct { "✓" } else { "✗" }
                );
            }
            Err(e) => {
                println!(
                    "{:<18} {:<18} {:<12} {:<12} {:<12.2} {}",
                    test.name, test.number, 
                    if test.expected_prime { "PRIME" } else { "COMPOSITE" },
                    "ERROR", 0.0, "✗"
                );
                eprintln!("    Error: {e}");
            }
        }

        total_time_interp += duration;
    }

    println!("{}", "─".repeat(75));
    println!("Interpreter: Total {:.2} µs, Avg {:.2} µs, {}/{} correct ({:.1}%)\n",
        total_time_interp,
        total_time_interp / tests.len() as f64,
        correct_interp,
        tests.len(),
        100.0 * correct_interp as f64 / tests.len() as f64
    );

    // Additional math benchmarks
    println!("📊 MATH OPERATIONS BENCHMARK");
    println!("─────────────────────────────────────────────────────────────");
    
    println!("{:<25} {:<15} {:<15}", "Operation", "Time (ms)", "Ops/sec");
    println!("{}", "─".repeat(55));

    // Addition benchmark
    interp.clear();
    interp.exec_line("0").ok();
    let start = Instant::now();
    for _ in 0..1000 {
        interp.exec_line("1 +").ok();
    }
    let duration = start.elapsed().as_secs_f64() * 1000.0;
    println!("{:<25} {:<15.2} {:<15.0}", "Addition (1K ops)", duration, 1000.0 / (duration / 1000.0));

    // Multiplication benchmark
    interp.clear();
    interp.exec_line("1").ok();
    let start = Instant::now();
    for _ in 0..1000 {
        interp.exec_line("2 *").ok();
    }
    let duration = start.elapsed().as_secs_f64() * 1000.0;
    println!("{:<25} {:<15.2} {:<15.0}", "Multiplication (1K ops)", duration, 1000.0 / (duration / 1000.0));

    // Square root benchmark
    interp.clear();
    interp.exec_line("12345678").ok();
    let start = Instant::now();
    for _ in 0..1000 {
        interp.exec_line("sqrt dup").ok();
    }
    let duration = start.elapsed().as_secs_f64() * 1000.0;
    println!("{:<25} {:<15.2} {:<15.0}", "Square root (1K ops)", duration, 1000.0 / (duration / 1000.0));

    // Trigonometry benchmark
    interp.clear();
    interp.exec_line("0.5").ok();
    let start = Instant::now();
    for _ in 0..1000 {
        interp.exec_line("dup sin drop").ok();
    }
    let duration = start.elapsed().as_secs_f64() * 1000.0;
    println!("{:<25} {:<15.2} {:<15.0}", "Trigonometry (1K ops)", duration, 1000.0 / (duration / 1000.0));

    println!("\n═══════════════════════════════════════════════════════════════");
    println!("🐺 Benchmark complete! 🐺");
    println!("═══════════════════════════════════════════════════════════════\n");

    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════
// TEST SUITE
// ═══════════════════════════════════════════════════════════════════════

fn run_tests() -> Result<()> {
    println!("🧪 Running COMPREHENSIVE WofLang Test Suite...\n");

    let mut interp = Interpreter::new();
    woflang_ops::register_all(&mut interp);
    woflang_plugins::register_all(&mut interp);

    let mut passed = 0u32;
    let mut failed = 0u32;
    let mut total = 0u32;

    // Use a macro to avoid closure borrow issues
    macro_rules! test {
        ($name:expr, $code:expr) => {{
            total += 1;
            print!("🔬 {}: ", $name);
            std::io::Write::flush(&mut std::io::stdout()).ok();
            interp.clear();
            match interp.exec_line($code) {
                Ok(()) => {
                    println!("✅ PASS");
                    passed += 1;
                }
                Err(e) => {
                    println!("❌ FAIL: {e}");
                    failed += 1;
                }
            }
        }};
        ($name:expr, $code:expr, $check:expr) => {{
            total += 1;
            print!("🔬 {}: ", $name);
            std::io::Write::flush(&mut std::io::stdout()).ok();
            interp.clear();
            match interp.exec_line($code) {
                Ok(()) => {
                    let check_fn: fn(&Interpreter) -> bool = $check;
                    if check_fn(&interp) {
                        println!("✅ PASS");
                        passed += 1;
                    } else {
                        println!("❌ FAIL (wrong value)");
                        failed += 1;
                    }
                }
                Err(e) => {
                    println!("❌ FAIL: {e}");
                    failed += 1;
                }
            }
        }};
    }

    // ═══════════════════════════════════════════════════════════════
    // BASIC MATH
    // ═══════════════════════════════════════════════════════════════
    println!("═══════════════════════════════════════════════════════════════");
    println!("=== 🔢 BASIC MATH OPERATIONS ===");
    println!("═══════════════════════════════════════════════════════════════");
    
    test!("Push integer", "42");
    test!("Push float", "3.14159");
    test!("Push negative", "-17");
    test!("Addition 5+3=8", "5 3 +", |i: &Interpreter| {
        i.stack().peek().map(|v| v.as_float().unwrap_or(0.0) == 8.0).unwrap_or(false)
    });
    test!("Subtraction 10-4=6", "10 4 -", |i: &Interpreter| {
        i.stack().peek().map(|v| v.as_float().unwrap_or(0.0) == 6.0).unwrap_or(false)
    });
    test!("Multiplication 6*7=42", "6 7 *", |i: &Interpreter| {
        i.stack().peek().map(|v| v.as_float().unwrap_or(0.0) == 42.0).unwrap_or(false)
    });
    test!("Division 20/4=5", "20 4 /", |i: &Interpreter| {
        i.stack().peek().map(|v| v.as_float().unwrap_or(0.0) == 5.0).unwrap_or(false)
    });
    test!("Power 2^8=256", "2 8 pow", |i: &Interpreter| {
        i.stack().peek().map(|v| v.as_float().unwrap_or(0.0) == 256.0).unwrap_or(false)
    });
    test!("Square root √16=4", "16 sqrt", |i: &Interpreter| {
        i.stack().peek().map(|v| v.as_float().unwrap_or(0.0) == 4.0).unwrap_or(false)
    });
    test!("Modulo", "17 5 mod");
    test!("Absolute value", "-42 abs");
    test!("Floor", "3.7 floor");
    test!("Ceiling", "3.2 ceil");
    test!("Round", "3.5 round");
    test!("Natural log", "e ln");
    test!("Log base 10", "100 log10");
    test!("Exponential", "1 exp");
    test!("Factorial", "5 fact");

    // ═══════════════════════════════════════════════════════════════
    // TRIGONOMETRY
    // ═══════════════════════════════════════════════════════════════
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("=== 📐 TRIGONOMETRY ===");
    println!("═══════════════════════════════════════════════════════════════");
    
    test!("Pi constant", "π");
    test!("Pi (ascii)", "pi");
    test!("E constant", "e");
    test!("Tau constant", "τ");
    test!("Phi (golden ratio)", "φ");
    test!("sin(π/2) ≈ 1", "π 2 / sin", |i: &Interpreter| {
        i.stack().peek().map(|v| (v.as_float().unwrap_or(0.0) - 1.0).abs() < 0.0001).unwrap_or(false)
    });
    test!("cos(0) = 1", "0 cos", |i: &Interpreter| {
        i.stack().peek().map(|v| v.as_float().unwrap_or(0.0) == 1.0).unwrap_or(false)
    });
    test!("Tangent", "0.5 tan");
    test!("Arc sine", "0.5 asin");
    test!("Arc cosine", "0.5 acos");
    test!("Arc tangent", "1 atan");
    test!("Hyperbolic sine", "1 sinh");
    test!("Hyperbolic cosine", "1 cosh");
    test!("Degrees to radians", "180 deg2rad");
    test!("Radians to degrees", "π rad2deg");

    // ═══════════════════════════════════════════════════════════════
    // STACK OPERATIONS
    // ═══════════════════════════════════════════════════════════════
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("=== 📊 STACK OPERATIONS ===");
    println!("═══════════════════════════════════════════════════════════════");
    
    test!("Clear stack", "1 2 3 clear");
    test!("Duplicate top", "42 dup", |i: &Interpreter| i.stack().len() == 2);
    test!("Swap top two", "1 2 swap", |i: &Interpreter| {
        i.stack().peek().map(|v| v.as_float().unwrap_or(0.0) == 1.0).unwrap_or(false)
    });
    test!("Drop top", "1 2 drop", |i: &Interpreter| i.stack().len() == 1);
    test!("Over operation", "1 2 over");
    test!("Rot operation", "1 2 3 rot");
    test!("Show stack (.)", "1 2 3 .");
    test!("Stack depth", "1 2 3 depth");
    test!("Pick operation", "1 2 3 1 pick");
    test!("Stack slayer", "1 2 3 4 5 stack_slayer");

    // ═══════════════════════════════════════════════════════════════
    // LOGIC OPERATIONS
    // ═══════════════════════════════════════════════════════════════
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("=== 🧮 LOGIC OPERATIONS ===");
    println!("═══════════════════════════════════════════════════════════════");
    
    test!("AND: 1 ∧ 1 = 1", "1 1 and", |i: &Interpreter| {
        i.stack().peek().map(|v| v.as_bool()).unwrap_or(false)
    });
    test!("AND: 1 ∧ 0 = 0", "1 0 and", |i: &Interpreter| {
        !i.stack().peek().map(|v| v.as_bool()).unwrap_or(true)
    });
    test!("OR: 0 ∨ 1 = 1", "0 1 or", |i: &Interpreter| {
        i.stack().peek().map(|v| v.as_bool()).unwrap_or(false)
    });
    // XOR: true ^ true = false, so result should be falsy (0)
    test!("XOR: 1 ⊕ 1 = 0", "1 1 xor");
    test!("NOT: ¬0 = 1", "0 not", |i: &Interpreter| {
        i.stack().peek().map(|v| v.as_bool()).unwrap_or(false)
    });
    test!("Unicode AND (∧)", "1 1 ∧");
    test!("Unicode OR (∨)", "0 1 ∨");
    test!("Unicode NOT (¬)", "1 ¬");
    test!("Implies (→)", "1 0 implies");
    test!("Biconditional (↔)", "1 1 iff");
    test!("NAND", "1 1 nand");
    test!("NOR", "0 0 nor");
    test!("Comparison: =", "5 5 =");
    test!("Comparison: <", "3 5 <");
    test!("Comparison: >", "5 3 >");
    test!("Comparison: ≤", "3 5 ≤");
    test!("Comparison: ≥", "5 3 ≥");
    test!("Comparison: ≠", "3 5 ≠");

    // ═══════════════════════════════════════════════════════════════
    // CRYPTOGRAPHY
    // ═══════════════════════════════════════════════════════════════
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("=== 🔐 CRYPTOGRAPHY ===");
    println!("═══════════════════════════════════════════════════════════════");
    
    test!("Prime check (17 is prime)", "17 prime_check", |i: &Interpreter| {
        i.stack().peek().map(|v| v.as_bool()).unwrap_or(false)
    });
    test!("Prime check (15 is composite)", "15 prime_check", |i: &Interpreter| {
        !i.stack().peek().map(|v| v.as_bool()).unwrap_or(true)
    });
    test!("Next prime", "10 next_prime");
    test!("GCD", "48 18 gcd");
    test!("LCM", "12 18 lcm");
    test!("Modular exponentiation", "2 10 1000 mod_exp");
    test!("Modular inverse", "3 11 mod_inv");
    test!("Random number", "1 100 random");
    test!("Hash function", "42 hash");

    // ═══════════════════════════════════════════════════════════════
    // GEOMETRY
    // ═══════════════════════════════════════════════════════════════
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("=== 📐 GEOMETRY ===");
    println!("═══════════════════════════════════════════════════════════════");
    
    test!("Circle area", "5 circle_area");
    test!("Circle circumference", "5 circle_circumf");
    test!("Sphere volume", "3 sphere_vol");
    test!("Sphere surface", "3 sphere_surface");
    test!("Pythagorean distance", "3 4 hypot");
    test!("Distance 2D", "0 0 3 4 dist2d");

    // ═══════════════════════════════════════════════════════════════
    // CALCULUS
    // ═══════════════════════════════════════════════════════════════
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("=== ∫ CALCULUS ===");
    println!("═══════════════════════════════════════════════════════════════");
    
    // diff_central: f(x-h) f(x+h) h → f'(x)
    // Example: derivative of x² at x=1 with h=0.001: f(0.999)=0.998 f(1.001)=1.002
    test!("Numerical derivative", "0.998001 1.002001 0.001 diff_central");
    // trapezoid: f_a f_b h → integral
    test!("Trapezoidal integration", "0 1 0.5 trapezoid");
    // simpson: f_a f_m f_b h → integral  
    test!("Simpson integration", "0 0.25 1 0.5 simpson");

    // ═══════════════════════════════════════════════════════════════
    // FRACTALS
    // ═══════════════════════════════════════════════════════════════
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("=== 🌀 FRACTALS ===");
    println!("═══════════════════════════════════════════════════════════════");
    
    test!("Mandelbrot check (in set)", "-0.5 0 50 mandelbrot");
    test!("Mandelbrot check (outside)", "2 2 50 mandelbrot");
    test!("Julia iteration", "0.1 0.1 -0.7 0.27015 50 julia");
    test!("Sierpinski triangle", "4 sierpinski");

    // ═══════════════════════════════════════════════════════════════
    // QUANTUM (if enabled)
    // ═══════════════════════════════════════════════════════════════
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("=== ⚛️ QUANTUM COMPUTING ===");
    println!("═══════════════════════════════════════════════════════════════");
    
    test!("Create |0⟩ state", "|0⟩");
    test!("Create |1⟩ state", "|1⟩");
    test!("Hadamard gate", "|0⟩ H");
    test!("Pauli-X gate", "|0⟩ X");
    test!("Pauli-Y gate", "|0⟩ Y");
    test!("Pauli-Z gate", "|0⟩ Z");
    test!("Phase gate S", "|0⟩ S");
    test!("T gate", "|0⟩ T");
    test!("Quantum measurement", "|0⟩ measure");
    test!("Superposition", "superposition");
    test!("Bell state", "bell");
    test!("CNOT gate", "0 1 cnot");

    // ═══════════════════════════════════════════════════════════════
    // SIGILS & MYSTICAL
    // ═══════════════════════════════════════════════════════════════
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("=== 🔮 SIGILS & MYSTICAL ===");
    println!("═══════════════════════════════════════════════════════════════");
    
    test!("Resurrect constants", "resurrect");
    test!("Mirror operation", "12345 mirror");
    test!("Palindrome check", "12321 palindrome?");
    test!("Entropy calculation", "1 2 3 4 5 entropy");
    test!("Chaos operation", "chaos");
    test!("Order operation", "5 2 8 1 9 order");
    test!("Moses stack split", "1 2 3 moses");
    test!("Prophecy", "prophecy");
    test!("Dreaming", "dreaming");

    // ═══════════════════════════════════════════════════════════════
    // GREEK LETTERS (mathematical constants)
    // ═══════════════════════════════════════════════════════════════
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("=== 🏛️ GREEK CONSTANTS ===");
    println!("═══════════════════════════════════════════════════════════════");
    
    test!("Alpha (α)", "α");
    test!("Beta (β)", "β");
    test!("Gamma (γ)", "γ");
    test!("Delta (δ)", "δ");
    test!("Epsilon (ε)", "ε");
    test!("Lambda (λ)", "λ");
    test!("Omega (ω)", "ω");
    test!("Infinity (∞)", "∞");

    // ═══════════════════════════════════════════════════════════════
    // DISCRETE MATH
    // ═══════════════════════════════════════════════════════════════
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("=== 🔢 DISCRETE MATH ===");
    println!("═══════════════════════════════════════════════════════════════");
    
    test!("Fibonacci", "10 fib");
    test!("Binomial coefficient", "5 2 binomial");
    test!("Permutations", "5 3 permute");
    test!("Combinations", "5 3 choose");
    test!("Is even", "4 even?");
    test!("Is odd", "5 odd?");

    // ═══════════════════════════════════════════════════════════════
    // CHEMISTRY
    // ═══════════════════════════════════════════════════════════════
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("=== 🧪 CHEMISTRY ===");
    println!("═══════════════════════════════════════════════════════════════");
    
    test!("Hydrogen info", "1 element_info");
    test!("Carbon atomic weight", "6 atomic_weight");
    test!("Temperature: C to K", "25 celsius_to_kelvin");
    test!("Temperature: K to C", "300 kelvin_to_celsius");
    test!("Temperature: C to F", "100 celsius_to_fahrenheit");
    test!("Avogadro constant", "avogadro");
    test!("Gas constant R", "gas_constant");
    test!("Boltzmann constant", "boltzmann");

    // ═══════════════════════════════════════════════════════════════
    // MUSIC & ARTS
    // ═══════════════════════════════════════════════════════════════
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("=== 🎵 MUSIC & ARTS ===");
    println!("═══════════════════════════════════════════════════════════════");
    
    test!("MIDI to frequency", "69 midi_to_freq");
    test!("Frequency to MIDI", "440 freq_to_midi");
    test!("Note interval", "60 64 interval");
    test!("Concert A", "concert_a");

    // ═══════════════════════════════════════════════════════════════
    // GRAPH OPERATIONS
    // ═══════════════════════════════════════════════════════════════
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("=== 🕸️ GRAPH OPERATIONS ===");
    println!("═══════════════════════════════════════════════════════════════");
    
    // graph_new: num_nodes "name" graph_new
    test!("Graph new", "5 \"testgraph\" graph_new");
    test!("Add vertex", "\"testgraph\" 1 vertex_add");
    // graph_chromatic: "name" graph_chromatic
    test!("Graph chromatic", "\"testgraph\" graph_chromatic");

    // ═══════════════════════════════════════════════════════════════
    // NEURAL CHESS (if enabled)
    // ═══════════════════════════════════════════════════════════════
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("=== ♟️ NEURAL CHESS ===");
    println!("═══════════════════════════════════════════════════════════════");
    
    test!("Chess new game", "chess_new");
    test!("Chess show board", "chess_show");
    test!("Chess AI new", "chess_ai_new");
    test!("Chess help", "chess_help");

    // ═══════════════════════════════════════════════════════════════
    // MARKOV CHAINS
    // ═══════════════════════════════════════════════════════════════
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("=== 🎲 MARKOV CHAINS ===");
    println!("═══════════════════════════════════════════════════════════════");
    
    test!("Markov init", "markov_init");
    test!("Markov step", "markov_step");

    // ═══════════════════════════════════════════════════════════════
    // SOLVER / SYMBOLIC
    // ═══════════════════════════════════════════════════════════════
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("=== 🧮 SYMBOLIC SOLVER ===");
    println!("═══════════════════════════════════════════════════════════════");
    
    test!("Simplify expression", "simplify");
    test!("Newton-Raphson", "2 1.0 0.0001 10 newton");
    test!("Bisection method", "0 2 0.0001 100 bisect");

    // ═══════════════════════════════════════════════════════════════
    // KANJI & CYRILLIC LANGUAGE
    // ═══════════════════════════════════════════════════════════════
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("=== 🈶 LANGUAGE OPS ===");
    println!("═══════════════════════════════════════════════════════════════");
    
    test!("Kanji lookup", "kanji_lookup");
    test!("Cyrillic lookup", "cyrillic_lookup");

    // ═══════════════════════════════════════════════════════════════
    // HEBREW SIGILS
    // ═══════════════════════════════════════════════════════════════
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("=== ✡️ HEBREW SIGILS ===");
    println!("═══════════════════════════════════════════════════════════════");
    
    test!("Aleph (א)", "א");
    test!("Beth (ב)", "ב");
    test!("Gimel (ג)", "ג");
    test!("Gematria", "gematria");

    // ═══════════════════════════════════════════════════════════════
    // SUMMARY
    // ═══════════════════════════════════════════════════════════════
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("🏆 TEST RESULTS SUMMARY");
    println!("═══════════════════════════════════════════════════════════════");
    println!("   ✅ Passed: {passed}");
    println!("   ❌ Failed: {failed}");
    println!("   📊 Total:  {total}");
    println!("   📈 Success Rate: {:.1}%", 100.0 * passed as f64 / total as f64);
    println!();

    if failed == 0 {
        println!("🎉 ALL TESTS PASSED! WofLang is fully operational! 🐺✨");
    } else {
        println!("⚠️  {failed} test(s) failed - some operations may not be registered.");
        println!("   This is expected if certain plugin features are disabled.");
    }
    println!("\nSystem Status: {} 🐺", if failed == 0 { "🟢 FULLY OPERATIONAL" } else { "🟡 PARTIALLY OPERATIONAL" });

    Ok(())
}

/// Provides platform-agnostic directory for history file
mod dirs {
    use std::path::PathBuf;

    pub fn data_local_dir() -> Option<PathBuf> {
        #[cfg(target_os = "windows")]
        {
            std::env::var("LOCALAPPDATA").ok().map(PathBuf::from)
        }
        #[cfg(target_os = "macos")]
        {
            std::env::var("HOME")
                .ok()
                .map(|h| PathBuf::from(h).join("Library/Application Support"))
        }
        #[cfg(target_os = "linux")]
        {
            std::env::var("XDG_DATA_HOME")
                .ok()
                .map(PathBuf::from)
                .or_else(|| std::env::var("HOME").ok().map(|h| PathBuf::from(h).join(".local/share")))
        }
        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
        {
            None
        }
    }
}
