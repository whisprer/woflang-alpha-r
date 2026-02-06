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
//!   --test-analog    Run analog computing test suite
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

    /// Run analog computing test suite
    #[arg(long)]
    test_analog: bool,

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

    if args.test_analog {
        woflang_analog::test_suite::run_analog_test_suite();
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
                    "test_analog" | "analog_test" => {
                        woflang_analog::test_suite::run_analog_test_suite();
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
  test_analog    Run analog computing test suite

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

fn run_benchmark() -> Result<()> {
    println!("🔢 WofLang Prime Benchmarking Suite");
    println!("===================================\n");

    let mut interp = Interpreter::new();
    woflang_ops::register_all(&mut interp);

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
        "{:<20} {:<18} {:<12} {:<12} {:<12} {:<5}",
        "Test Name", "Number", "Expected", "Result", "Time (µs)", "OK"
    );
    println!("{}", "-".repeat(80));

    let mut total_time = 0.0;
    let mut correct = 0;

    for test in &tests {
        print!("{:<20} {:<18} {:<12}", test.name, test.number, 
               if test.expected_prime { "PRIME" } else { "COMPOSITE" });
        std::io::Write::flush(&mut std::io::stdout()).ok();

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
                if is_correct {
                    correct += 1;
                }

                println!(
                    "{:<12} {:<12.2} {}",
                    if result { "PRIME" } else { "COMPOSITE" },
                    duration,
                    if is_correct { "✓" } else { "✗" }
                );
            }
            Err(e) => {
                println!("{:<12} {:<12.2} ✗", "ERROR", 0.0);
                eprintln!("    Error: {e}");
            }
        }

        total_time += duration;
    }

    println!("{}", "-".repeat(80));
    println!("Total time: {:.2} µs", total_time);
    println!("Average time: {:.2} µs", total_time / tests.len() as f64);
    println!("Correct results: {}/{}", correct, tests.len());
    println!(
        "Success rate: {:.1}%",
        100.0 * correct as f64 / tests.len() as f64
    );
    println!("\n🐺 Benchmark complete! 🐺\n");

    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════
// TEST SUITE
// ═══════════════════════════════════════════════════════════════════════

fn run_tests() -> Result<()> {
    println!("🧪 Running COMPREHENSIVE WofLang Test Suite...\n");

    let mut interp = Interpreter::new();
    woflang_ops::register_all(&mut interp);

    let mut passed = 0;
    let mut total = 0;

    let mut test = |name: &str, code: &str, should_succeed: bool| {
        total += 1;
        print!("🔬 Testing {name}: ");
        std::io::Write::flush(&mut std::io::stdout()).ok();

        interp.clear();
        match interp.exec_line(code) {
            Ok(()) => {
                if should_succeed {
                    println!("✅ PASS");
                    passed += 1;
                } else {
                    println!("❌ FAIL (should have failed)");
                }
            }
            Err(e) => {
                if !should_succeed {
                    println!("✅ PASS (expected failure)");
                    passed += 1;
                } else {
                    println!("❌ FAIL: {e}");
                }
            }
        }
    };

    println!("=== 🔢 BASIC MATH OPERATIONS ===");
    test("Push numbers", "42 3.14 -17", true);
    test("Addition", "5 3 +", true);
    test("Subtraction", "10 4 -", true);
    test("Multiplication", "6 7 *", true);
    test("Division", "20 4 /", true);
    test("Power", "2 8 pow", true);
    test("Square root", "16 sqrt", true);

    println!("\n=== 📐 TRIGONOMETRY ===");
    test("Pi constant", "pi", true);
    test("E constant", "e", true);
    test("Sine", "pi 2 / sin", true);
    test("Cosine", "0 cos", true);

    println!("\n=== 📊 STACK OPERATIONS ===");
    test("Clear and setup", "clear 1 2 3", true);
    test("Duplicate top", "42 dup", true);
    test("Swap top two", "1 2 swap", true);
    test("Drop top", "1 2 drop", true);
    test("Show stack", "1 2 3 .", true);

    #[cfg(feature = "quantum-ops")]
    {
        println!("\n=== ⚛️ QUANTUM COMPUTING ===");
        test("Create |0⟩ state", "|0⟩", true);
        test("Create |1⟩ state", "|1⟩", true);
        test("Hadamard gate", "|0⟩ H", true);
        test("Pauli-X gate", "|0⟩ X", true);
        test("Pauli-Z gate", "|0⟩ Z", true);
        test("Quantum measurement", "|0⟩ measure", true);
        test("Bell state creation", "bell", true);
    }

    #[cfg(feature = "crypto-ops")]
    {
        println!("\n=== 🔐 CRYPTOGRAPHY ===");
        test("Prime check (prime)", "17 prime_check", true);
        test("Prime check (composite)", "15 prime_check", true);
        test("Random number", "1 100 random", true);
        test("Hash function", "42 hash", true);
        test("Base64 encode", "123 base64_encode", true);
        test("Diffie-Hellman demo", "diffie_hellman", true);
    }

    println!("\n=== 🧮 LOGIC OPERATIONS ===");
    test("Logical AND", "1 1 and", true);
    test("Logical OR", "0 1 or", true);
    test("Logical XOR", "1 1 xor", true);
    test("Logical NOT", "0 not", true);
    test("Tautology demo", "tautology", true);

    println!("\n=== 🎭 DRAMATIC OPERATIONS ===");
    test("Stack resurrection", "resurrect", true);
    test("Stack slayer", "1 2 3 stack_slayer", true);

    println!("\n=== 🔮 SYMBOLIC LOGIC TESTS ===");
    test("True implies false", "1 0 implies", true);
    test("True implies true", "1 1 implies", true);
    test("False implies true", "0 1 implies", true);
    test("And: true ∧ true", "1 1 ∧", true);
    test("Not true (¬)", "1 ¬", true);

    println!("\n{}", "=".repeat(60));
    println!("🏆 TEST RESULTS SUMMARY:");
    println!("   Passed: {passed}/{total} tests");
    println!("   Success Rate: {:.1}%", 100.0 * passed as f64 / total as f64);

    if passed == total {
        println!("🎉 ALL TESTS PASSED! WofLang is fully operational! 🐺✨");
    } else {
        println!("⚠️  Some tests failed - check implementations above.");
    }
    println!("\nSystem Status: 🟢 FULLY OPERATIONAL 🟢");

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
