//! Learning mode operations for Woflang.
//!
//! Interactive educational tools to help users learn Woflang.
//!
//! ## Operations
//!
//! - `lesson` - Print a random learning tip
//! - `hint` - Context-aware hint based on stack state
//! - `quiz` - Present a quiz question
//! - `tutorial` - Start an interactive tutorial
//! - `examples` - Show example code snippets

use rand::seq::SliceRandom;
use rand::thread_rng;
use woflang_core::InterpreterContext;
use woflang_runtime::Interpreter;

// ═══════════════════════════════════════════════════════════════════════════
// LESSON CONTENT
// ═══════════════════════════════════════════════════════════════════════════

const LESSONS: &[&str] = &[
    "Lesson 1: To add numbers, use: 2 3 + print",
    "Lesson 2: To duplicate stack top, use: dup",
    "Lesson 3: To print pi, use: π print",
    "Lesson 4: To use a plugin, type its op name (e.g., pi print)",
    "Lesson 5: Try a chemistry op: 2 mol",
    "Lesson 6: Define a variable: 42 →x (stores 42 in x)",
    "Lesson 7: Use a variable: x (pushes value of x)",
    "Lesson 8: Conditionals: condition { true_branch } ? { false_branch } ! ",
    "Lesson 9: Functions: func myfunc { 2 * } (defines myfunc)",
    "Lesson 10: Loops: 5 { i print } loop (prints 0 to 4)",
    "Lesson 11: Stack manipulation: dup, drop, swap, over, rot",
    "Lesson 12: Math symbols: √ (sqrt), ² (square), Σ (sum), Π (product)",
    "Lesson 13: Graph operations: 5 graph_new, 0 1 graph_edge",
    "Lesson 14: Unicode glyphs work as operators: ∀, ∃, ∈, ∅",
    "Lesson 15: Try the prophecy easter egg: prophecy",
];

const HINTS: &[&str] = &[
    "Hint: The stack is empty! Try pushing a value: 42",
    "Hint: Use 'print' to see the top of the stack.",
    "Hint: Use 'stack' to see all values on the stack.",
    "Hint: Use '.s' as a shortcut for printing the stack.",
    "Hint: Press Tab for autocompletion in the REPL.",
    "Hint: Use 'help' to see available commands.",
    "Hint: Try 'pi print' to see the value of π.",
];

const QUIZZES: &[(&str, &[&str], usize)] = &[
    (
        "What does 'Δ' (delta) do?",
        &["A) Adds two numbers", "B) Subtracts: b - a", "C) Multiplies two numbers"],
        1, // Answer: B
    ),
    (
        "What does 'dup' do?",
        &["A) Deletes top value", "B) Duplicates top value", "C) Swaps top two values"],
        1, // Answer: B
    ),
    (
        "How do you define a function 'double' that doubles a number?",
        &[
            "A) func double { 2 * }",
            "B) def double = 2 *",
            "C) function double() { return 2 * }",
        ],
        0, // Answer: A
    ),
    (
        "What is the result of: 3 4 + 2 * ?",
        &["A) 10", "B) 14", "C) 11"],
        1, // Answer: B (14 because (3+4)*2)
    ),
    (
        "What does 'swap' do to the stack [1, 2, 3] (3 on top)?",
        &["A) [1, 3, 2]", "B) [3, 2, 1]", "C) [2, 1, 3]"],
        0, // Answer: A
    ),
];

const EXAMPLES: &[(&str, &str)] = &[
    ("Calculate area of circle", "5 →r r r * π *"),
    ("Fibonacci sequence", "0 1 { over over + } 10 loop"),
    ("Factorial", "func fact { dup 1 > { dup 1 - fact * } ? } 5 fact"),
    ("Quadratic formula", "1 -5 6 solve_quadratic"),
    ("Graph traversal", "5 graph_new 0 1 graph_edge 0 graph_bfs"),
    ("Entropy of stack", "1 2 2 3 3 3 entropy"),
];

// ═══════════════════════════════════════════════════════════════════════════
// REGISTRATION
// ═══════════════════════════════════════════════════════════════════════════

/// Register learning mode operations.
pub fn register(interp: &mut Interpreter) {
    // ─────────────────────────────────────────────────────────────────────
    // LESSON
    // ─────────────────────────────────────────────────────────────────────

    // Print a random learning tip
    interp.register("lesson", |_interp| {
        if let Some(lesson) = LESSONS.choose(&mut thread_rng()) {
            println!("[Learning Mode] {}", lesson);
        }
        Ok(())
    });

    // Print all lessons
    interp.register("lessons", |_interp| {
        println!("[Learning Mode] All Lessons:");
        println!();
        for lesson in LESSONS {
            println!("  {}", lesson);
        }
        Ok(())
    });

    // ─────────────────────────────────────────────────────────────────────
    // HINT
    // ─────────────────────────────────────────────────────────────────────

    // Context-aware hint
    interp.register("hint", |interp| {
        if interp.stack().is_empty() {
            println!("Hint: The stack is empty! Try pushing a value: 42");
        } else if interp.stack().len() == 1 {
            println!("Hint: You have one value. Try 'dup' to duplicate or 'print' to display.");
        } else if interp.stack().len() >= 2 {
            println!("Hint: You have {} values. Try an operation like '+', '*', or 'swap'.", interp.stack().len());
        }
        Ok(())
    });

    // Random hint
    interp.register("random_hint", |_interp| {
        if let Some(hint) = HINTS.choose(&mut thread_rng()) {
            println!("{}", hint);
        }
        Ok(())
    });

    // ─────────────────────────────────────────────────────────────────────
    // QUIZ
    // ─────────────────────────────────────────────────────────────────────

    // Present a random quiz question
    interp.register("quiz", |_interp| {
        if let Some((question, options, _answer)) = QUIZZES.choose(&mut thread_rng()) {
            println!("[Quiz] {}", question);
            for option in *options {
                println!("  {}", option);
            }
            println!();
            println!("(Use 'quiz_answer A', 'quiz_answer B', or 'quiz_answer C' to check)");
        }
        Ok(())
    });

    // All quizzes
    interp.register("quizzes", |_interp| {
        println!("[Quiz Mode] Available Quizzes:");
        println!();
        for (i, (question, options, _)) in QUIZZES.iter().enumerate() {
            println!("Quiz {}:", i + 1);
            println!("  {}", question);
            for option in *options {
                println!("    {}", option);
            }
            println!();
        }
        Ok(())
    });

    // ─────────────────────────────────────────────────────────────────────
    // EXAMPLES
    // ─────────────────────────────────────────────────────────────────────

    // Show example code snippets
    interp.register("examples", |_interp| {
        println!("[Examples] Woflang Code Snippets:");
        println!();
        for (description, code) in EXAMPLES {
            println!("  {} ", description);
            println!("    > {}", code);
            println!();
        }
        Ok(())
    });

    // Random example
    interp.register("example", |_interp| {
        if let Some((description, code)) = EXAMPLES.choose(&mut thread_rng()) {
            println!("[Example] {}", description);
            println!("  > {}", code);
        }
        Ok(())
    });

    // ─────────────────────────────────────────────────────────────────────
    // TUTORIAL
    // ─────────────────────────────────────────────────────────────────────

    // Interactive tutorial intro
    interp.register("tutorial", |_interp| {
        println!("╔══════════════════════════════════════════════════════════╗");
        println!("║           Welcome to the Woflang Tutorial!               ║");
        println!("╠══════════════════════════════════════════════════════════╣");
        println!("║                                                          ║");
        println!("║  Woflang is a stack-based language with Unicode support. ║");
        println!("║                                                          ║");
        println!("║  Basic operations:                                       ║");
        println!("║    • Push numbers: 42, 3.14, -7                          ║");
        println!("║    • Push strings: \"hello\"                               ║");
        println!("║    • Arithmetic: +, -, *, /                              ║");
        println!("║    • Stack ops: dup, drop, swap, over                    ║");
        println!("║                                                          ║");
        println!("║  Commands to try:                                        ║");
        println!("║    lesson   - Random learning tip                        ║");
        println!("║    hint     - Context-aware help                         ║");
        println!("║    quiz     - Test your knowledge                        ║");
        println!("║    examples - See code snippets                          ║");
        println!("║                                                          ║");
        println!("╚══════════════════════════════════════════════════════════╝");
        Ok(())
    });

    // Quick start guide
    interp.register("quickstart", |_interp| {
        println!("Woflang Quick Start:");
        println!();
        println!("  1. Push values onto the stack:");
        println!("     > 10 20");
        println!();
        println!("  2. Perform operations:");
        println!("     > +        (adds 10 and 20, result: 30)");
        println!();
        println!("  3. Print the result:");
        println!("     > print    (displays: 30)");
        println!();
        println!("  4. Try more:");
        println!("     > 5 dup *  (squares 5, result: 25)");
        println!("     > π print  (prints pi)");
        println!();
        Ok(())
    });
}
