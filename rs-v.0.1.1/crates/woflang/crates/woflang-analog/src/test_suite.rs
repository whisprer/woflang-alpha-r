//! Comprehensive Analog Computing Test Suite
//!
//! Run with: cargo test -p woflang-analog --test analog_test_suite
//! Or integrate into the CLI with --test-analog

use std::time::Instant;

/// Test result tracking
struct TestRunner {
    passed: usize,
    failed: usize,
    section: String,
}

impl TestRunner {
    fn new() -> Self {
        Self {
            passed: 0,
            failed: 0,
            section: String::new(),
        }
    }

    fn section(&mut self, name: &str) {
        self.section = name.to_string();
        println!("=== {} ===", name);
    }

    fn test(&mut self, name: &str, condition: bool) {
        if condition {
            println!("🔬 Testing {}: ✅ PASS", name);
            self.passed += 1;
        } else {
            println!("🔬 Testing {}: ❌ FAIL", name);
            self.failed += 1;
        }
    }

    fn test_eq_f64(&mut self, name: &str, actual: f64, expected: f64) {
        let ok = (actual - expected).abs() < 1e-10;
        if ok {
            println!("🔬 Testing {}: ✅ PASS", name);
            self.passed += 1;
        } else {
            println!(
                "🔬 Testing {}: ❌ FAIL (expected {}, got {})",
                name, expected, actual
            );
            self.failed += 1;
        }
    }

    fn test_approx(&mut self, name: &str, actual: f64, expected: f64, tolerance: f64) {
        let ok = (actual - expected).abs() < tolerance;
        if ok {
            println!("🔬 Testing {}: ✅ PASS", name);
            self.passed += 1;
        } else {
            println!(
                "🔬 Testing {}: ❌ FAIL (expected ~{}, got {}, tol={})",
                name, expected, actual, tolerance
            );
            self.failed += 1;
        }
    }

    fn summary(&self) {
        let total = self.passed + self.failed;
        println!("============================================================");
        println!("🏆 ANALOG TEST RESULTS:");
        println!(
            "   Passed: {}/{} tests",
            self.passed, total
        );
        let rate = if total > 0 {
            (self.passed as f64 / total as f64) * 100.0
        } else {
            0.0
        };
        println!("   Success Rate: {:.1}%", rate);
        if self.failed == 0 {
            println!("🎉 ALL ANALOG TESTS PASSED! Bounded continuum computing is GO! 🐺⚡");
            println!("System Status: 🟢 ANALOG MODE FULLY OPERATIONAL 🟢");
        } else {
            println!("⚠️  {} test(s) failed", self.failed);
            println!("System Status: 🔴 ISSUES DETECTED 🔴");
        }
    }
}

/// Run the full analog test suite. Call from CLI with --test-analog.
pub fn run_analog_test_suite() {
    use crate::prelude::*;

    println!("🧪 Running COMPREHENSIVE WofLang Analog Computing Test Suite...");
    println!();

    let start = Instant::now();
    let mut t = TestRunner::new();

    // ═══════════════════════════════════════════════════════════════════
    // MODE SETUP & SWITCHING
    // ═══════════════════════════════════════════════════════════════════
    t.section("⚙️  MODE SETUP & SWITCHING");

    reset_analog_mode();
    t.test_eq_f64("Default mode is Int201 (min=-100)", analog_min(), -100.0);
    t.test_eq_f64("Default mode is Int201 (max=+100)", analog_max(), 100.0);

    set_analog_mode(AnalogMode::Int2001);
    t.test_eq_f64("Int2001 mode (min=-1000)", analog_min(), -1000.0);
    t.test_eq_f64("Int2001 mode (max=+1000)", analog_max(), 1000.0);

    set_analog_mode(AnalogMode::FloatUnit);
    t.test_eq_f64("FloatUnit mode (min=-1.0)", analog_min(), -1.0);
    t.test_eq_f64("FloatUnit mode (max=+1.0)", analog_max(), 1.0);

    set_analog_custom(-5.0, 5.0);
    t.test_eq_f64("Custom mode ±5V (min=-5.0)", analog_min(), -5.0);
    t.test_eq_f64("Custom mode ±5V (max=+5.0)", analog_max(), 5.0);

    reset_analog_mode();
    t.test_eq_f64("Reset returns to Int201", analog_min(), -100.0);

    println!();

    // ═══════════════════════════════════════════════════════════════════
    // SATURATION - THE HEART OF ANALOG
    // ═══════════════════════════════════════════════════════════════════
    t.section("📈 SATURATION BEHAVIOR (Core Paradigm)");

    reset_analog_mode();
    set_analog_mode(AnalogMode::Int201);

    // Positive saturation
    t.test_eq_f64("80 + 50 saturates to 100", analog_add(80.0, 50.0), 100.0);
    t.test_eq_f64("50 * 50 saturates to 100", analog_mul(50.0, 50.0), 100.0);
    t.test_eq_f64("Clamp 999 → 100", clamp_analog(999.0), 100.0);

    // Negative saturation
    t.test_eq_f64("-80 + -50 saturates to -100", analog_add(-80.0, -50.0), -100.0);
    t.test_eq_f64("-50 * 50 saturates to -100", analog_mul(-50.0, 50.0), -100.0);
    t.test_eq_f64("Clamp -999 → -100", clamp_analog(-999.0), -100.0);

    // Within range - no saturation
    t.test_eq_f64("30 + 40 = 70 (no saturation)", analog_add(30.0, 40.0), 70.0);
    t.test_eq_f64("5 * 10 = 50 (no saturation)", analog_mul(5.0, 10.0), 50.0);

    // Boundary values
    t.test_eq_f64("100 + 0 stays at 100", analog_add(100.0, 0.0), 100.0);
    t.test_eq_f64("-100 - 0 stays at -100", analog_sub(-100.0, 0.0), -100.0);

    println!();

    // ═══════════════════════════════════════════════════════════════════
    // BASIC ARITHMETIC
    // ═══════════════════════════════════════════════════════════════════
    t.section("🔢 BASIC ARITHMETIC (Int201 mode)");

    reset_analog_mode();
    set_analog_mode(AnalogMode::Int201);

    t.test_eq_f64("Addition: 25 + 30 = 55", analog_add(25.0, 30.0), 55.0);
    t.test_eq_f64("Subtraction: 50 - 30 = 20", analog_sub(50.0, 30.0), 20.0);
    t.test_eq_f64("Multiplication: 5 × 10 = 50", analog_mul(5.0, 10.0), 50.0);
    t.test_eq_f64("Division: 100 ÷ 4 = 25", analog_div(100.0, 4.0), 25.0);
    t.test_eq_f64("Modulo: 17 % 5 = 2", analog_mod(17.0, 5.0), 2.0);
    t.test_eq_f64("Negation: -42 → 42 (abs)", analog_abs(-42.0), 42.0);
    t.test_eq_f64("Negation: neg(30) = -30", analog_neg(30.0), -30.0);
    t.test_eq_f64("Square root: √49 = 7", analog_sqrt(49.0), 7.0);
    t.test_eq_f64("Power: 2^6 = 64", analog_pow(2.0, 6.0), 64.0);
    t.test_eq_f64("Power saturation: 2^10 = 100", analog_pow(2.0, 10.0), 100.0);

    println!();

    // ═══════════════════════════════════════════════════════════════════
    // EDGE CASES & SAFETY
    // ═══════════════════════════════════════════════════════════════════
    t.section("🛡️  EDGE CASES & SAFETY");

    reset_analog_mode();
    set_analog_mode(AnalogMode::Int201);

    t.test_eq_f64("Division by zero → 0 (midpoint)", analog_div(50.0, 0.0), 0.0);
    t.test_eq_f64("Sqrt of negative → 0 (safe)", analog_sqrt(-25.0), 0.0);
    t.test_eq_f64("Modulo by zero → clamped input", analog_mod(42.0, 0.0), 42.0);
    t.test_eq_f64("0 + 0 = 0", analog_add(0.0, 0.0), 0.0);
    t.test_eq_f64("0 * anything = 0", analog_mul(0.0, 999.0), 0.0);
    t.test_eq_f64("neg(0) = 0", analog_neg(0.0), 0.0);
    t.test_eq_f64("Sqrt(0) = 0", analog_sqrt(0.0), 0.0);

    println!();

    // ═══════════════════════════════════════════════════════════════════
    // FLOAT UNIT MODE [-1.0, +1.0]
    // ═══════════════════════════════════════════════════════════════════
    t.section("🔬 FLOAT UNIT MODE [-1.0, +1.0]");

    reset_analog_mode();
    set_analog_mode(AnalogMode::FloatUnit);

    t.test_eq_f64("0.5 + 0.3 = 0.8", analog_add(0.5, 0.3), 0.8);
    t.test_eq_f64("0.8 + 0.5 saturates to 1.0", analog_add(0.8, 0.5), 1.0);
    t.test_eq_f64("-0.8 - 0.5 saturates to -1.0", analog_add(-0.8, -0.5), -1.0);
    t.test_eq_f64("0.5 * 0.5 = 0.25", analog_mul(0.5, 0.5), 0.25);
    t.test_eq_f64("2.0 * 3.0 saturates to 1.0", analog_mul(2.0, 3.0), 1.0);

    println!();

    // ═══════════════════════════════════════════════════════════════════
    // CONFIG-BASED OPERATIONS
    // ═══════════════════════════════════════════════════════════════════
    t.section("🔧 CONFIG-BASED OPERATIONS");

    let config = AnalogConfig::new(AnalogMode::Int201);

    t.test_eq_f64("Config add 80+50 = 100", config.add(80.0, 50.0), 100.0);
    t.test_eq_f64("Config mul 50*50 = 100", config.mul(50.0, 50.0), 100.0);
    t.test_eq_f64("Config div 100/0 = 0", config.div(100.0, 0.0), 0.0);
    t.test_eq_f64("Config lerp(0.0) = -100", config.lerp(-100.0, 100.0, 0.0), -100.0);
    t.test_eq_f64("Config lerp(0.5) = 0", config.lerp(-100.0, 100.0, 0.5), 0.0);
    t.test_eq_f64("Config lerp(1.0) = 100", config.lerp(-100.0, 100.0, 1.0), 100.0);
    t.test_eq_f64("Deadzone: 5 within 10 → 0", config.deadzone(5.0, 10.0), 0.0);
    t.test_eq_f64("Deadzone: 15 outside 10 → 15", config.deadzone(15.0, 10.0), 15.0);
    t.test_eq_f64("FMA: 10*5+20 = 70", config.fma(10.0, 5.0, 20.0), 70.0);
    t.test_eq_f64("FMA saturates: 50*50+0 = 100", config.fma(50.0, 50.0, 0.0), 100.0);

    println!();

    // ═══════════════════════════════════════════════════════════════════
    // NORMALIZATION & REMAPPING
    // ═══════════════════════════════════════════════════════════════════
    t.section("🔄 NORMALIZATION & REMAPPING");

    let config = AnalogConfig::new(AnalogMode::Int201);

    t.test_approx("Normalize 0 → 0.5", config.normalize(0.0), 0.5, 1e-10);
    t.test_approx("Normalize 100 → 1.0", config.normalize(100.0), 1.0, 1e-10);
    t.test_approx("Normalize -100 → 0.0", config.normalize(-100.0), 0.0, 1e-10);
    t.test_approx("Denormalize 0.5 → 0", config.denormalize(0.5), 0.0, 1e-10);
    t.test_approx("Denormalize 1.0 → 100", config.denormalize(1.0), 100.0, 1e-10);
    t.test_approx("Denormalize 0.0 → -100", config.denormalize(0.0), -100.0, 1e-10);

    // Remap: 0.5 from [0,1] → [-100,100] = 0
    t.test_approx(
        "Remap [0,1]→[-100,100]: 0.5 → 0",
        config.remap(0.5, 0.0, 1.0, -100.0, 100.0),
        0.0,
        1e-10,
    );
    // Remap: 75 from [0,100] → [0,1] = 0.75
    t.test_approx(
        "Remap [0,100]→[0,1]: 75 → 0.75",
        config.remap(75.0, 0.0, 100.0, 0.0, 1.0),
        0.75,
        1e-10,
    );

    println!();

    // ═══════════════════════════════════════════════════════════════════
    // TRIGONOMETRY
    // ═══════════════════════════════════════════════════════════════════
    t.section("📐 TRIGONOMETRY (Analog-clamped)");

    reset_analog_mode();
    set_analog_mode(AnalogMode::Int201);

    t.test_approx("sin(0) = 0", analog_sin(0.0), 0.0, 1e-10);
    t.test_approx("cos(0) = 1", analog_cos(0.0), 1.0, 1e-10);
    t.test_approx("sin(π/2) = 1", analog_sin(std::f64::consts::FRAC_PI_2), 1.0, 1e-10);
    t.test_approx("cos(π) = -1", analog_cos(std::f64::consts::PI), -1.0, 1e-10);
    t.test_approx("sin(π/6) ≈ 0.5", analog_sin(std::f64::consts::FRAC_PI_6), 0.5, 0.01);
    t.test_approx("tanh(0) = 0", analog_tanh(0.0), 0.0, 1e-10);

    // Tan near π/2 should saturate to ±100 in Int201 mode
    let tan_big = analog_tan(1.5); // close to π/2
    t.test("tan(~π/2) saturates to range", tan_big >= -100.0 && tan_big <= 100.0);

    // Sigmoid and ReLU (neural activation functions)
    // Sigmoid maps (0,1) → analog range via denormalize
    // sigmoid(0)=0.5 → denormalize(0.5) = midpoint = 0 in Int201
    t.test_approx("sigmoid(0) → midpoint (0 in Int201)", analog_sigmoid(0.0), 0.0, 0.01);
    t.test("sigmoid(large) → max", analog_sigmoid(10.0) > 90.0);
    t.test("sigmoid(-large) → min", analog_sigmoid(-10.0) < -90.0);
    t.test_eq_f64("ReLU(-5) = 0", analog_relu(-5.0), 0.0);
    t.test_eq_f64("ReLU(5) = 5", analog_relu(5.0), 5.0);

    println!();

    // ═══════════════════════════════════════════════════════════════════
    // ANGLE CONVERSIONS
    // ═══════════════════════════════════════════════════════════════════
    t.section("🧭 ANGLE CONVERSIONS");

    set_analog_mode(AnalogMode::Int201);

    t.test_approx("0° → 0 rad", deg_to_rad(0.0), 0.0, 1e-10);
    // Note: 180° = π ≈ 3.14 which fits in [-100, 100]
    t.test_approx("180° → π rad", deg_to_rad(180.0), std::f64::consts::PI, 0.01);
    // rad_to_deg is a pure conversion (no analog clamping)
    t.test_approx("π rad → 180°", rad_to_deg(std::f64::consts::PI), 180.0, 0.01);
    // 180° in degrees is clamped to 100 in Int201 mode!
    // But 1 rad → ~57.3° which fits
    t.test_approx("1 rad → ~57.3°", rad_to_deg(1.0), 57.2957, 0.01);

    // Wrapping
    t.test_approx(
        "Wrap 720° → 0°",
        wrap_degrees(720.0),
        0.0,
        0.01,
    );

    println!();

    // ═══════════════════════════════════════════════════════════════════
    // LINEAR ALGEBRA 2D
    // ═══════════════════════════════════════════════════════════════════
    t.section("📊 LINEAR ALGEBRA 2D");

    reset_analog_mode();
    set_analog_mode(AnalogMode::Int201);

    t.test_eq_f64(
        "Dot 2D: (3,4)·(1,2) = 11",
        analog_dot_2d(3.0, 4.0, 1.0, 2.0),
        11.0,
    );
    t.test_eq_f64(
        "Magnitude 2D: |(3,4)| = 5",
        analog_magnitude_2d(3.0, 4.0),
        5.0,
    );
    t.test_eq_f64(
        "Distance 2D: (0,0)→(3,4) = 5",
        analog_distance_2d(0.0, 0.0, 3.0, 4.0),
        5.0,
    );

    // Normalization
    let (nx, ny) = analog_normalize_2d(3.0, 4.0);
    t.test_approx("Normalize 2D x: 3/5 = 0.6", nx, 0.6, 0.01);
    t.test_approx("Normalize 2D y: 4/5 = 0.8", ny, 0.8, 0.01);

    // Saturation in dot product
    t.test_eq_f64(
        "Dot 2D saturates: (100,100)·(100,100) → 100",
        analog_dot_2d(100.0, 100.0, 100.0, 100.0),
        100.0,
    );

    println!();

    // ═══════════════════════════════════════════════════════════════════
    // LINEAR ALGEBRA 3D
    // ═══════════════════════════════════════════════════════════════════
    t.section("📊 LINEAR ALGEBRA 3D");

    t.test_eq_f64(
        "Dot 3D: (1,2,3)·(4,5,6) = 32",
        analog_dot_3d(1.0, 2.0, 3.0, 4.0, 5.0, 6.0),
        32.0,
    );
    t.test_approx(
        "Magnitude 3D: |(1,2,2)| = 3",
        analog_magnitude_3d(1.0, 2.0, 2.0),
        3.0,
        0.01,
    );
    t.test_approx(
        "Distance 3D: (0,0,0)→(1,2,2) = 3",
        analog_distance_3d(0.0, 0.0, 0.0, 1.0, 2.0, 2.0),
        3.0,
        0.01,
    );

    let (nx, ny, nz) = analog_normalize_3d(1.0, 2.0, 2.0);
    t.test_approx("Normalize 3D x: 1/3", nx, 1.0 / 3.0, 0.01);
    t.test_approx("Normalize 3D y: 2/3", ny, 2.0 / 3.0, 0.01);
    t.test_approx("Normalize 3D z: 2/3", nz, 2.0 / 3.0, 0.01);

    println!();

    // ═══════════════════════════════════════════════════════════════════
    // BATCH OPERATIONS
    // ═══════════════════════════════════════════════════════════════════
    t.section("📦 BATCH OPERATIONS");

    reset_analog_mode();
    set_analog_mode(AnalogMode::Int201);

    use crate::math::{batch_add, batch_mul, batch_scale, batch_clamp};

    let a = vec![10.0, 20.0, 80.0, -90.0];
    let b = vec![5.0, 10.0, 50.0, -30.0];

    let result = batch_add(&a, &b);
    t.test_eq_f64("Batch add [0]: 10+5=15", result[0], 15.0);
    t.test_eq_f64("Batch add [1]: 20+10=30", result[1], 30.0);
    t.test_eq_f64("Batch add [2]: 80+50=100 (sat)", result[2], 100.0);
    t.test_eq_f64("Batch add [3]: -90-30=-100 (sat)", result[3], -100.0);

    let result = batch_mul(&a, &b);
    t.test_eq_f64("Batch mul [0]: 10*5=50", result[0], 50.0);
    t.test_eq_f64("Batch mul [2]: 80*50=100 (sat)", result[2], 100.0);

    let scaled = batch_scale(&[10.0, 50.0, 80.0], 2.0);
    t.test_eq_f64("Batch scale: 10*2=20", scaled[0], 20.0);
    t.test_eq_f64("Batch scale: 50*2=100 (sat)", scaled[1], 100.0);
    t.test_eq_f64("Batch scale: 80*2=100 (sat)", scaled[2], 100.0);

    let clamped = batch_clamp(&[50.0, 150.0, -200.0, 0.0]);
    t.test_eq_f64("Batch clamp: 50 → 50", clamped[0], 50.0);
    t.test_eq_f64("Batch clamp: 150 → 100", clamped[1], 100.0);
    t.test_eq_f64("Batch clamp: -200 → -100", clamped[2], -100.0);
    t.test_eq_f64("Batch clamp: 0 → 0", clamped[3], 0.0);

    println!();

    // ═══════════════════════════════════════════════════════════════════
    // EURORACK ±5V SYNTHESIS USE CASE
    // ═══════════════════════════════════════════════════════════════════
    t.section("🎛️  EURORACK ±5V SYNTH SIMULATION");

    let eurorack = AnalogConfig::new_custom(-5.0, 5.0);

    // LFO sine wave stays in range
    let lfo = eurorack.sin(std::f64::consts::PI / 4.0);
    t.test("LFO sine within ±5V", lfo >= -5.0 && lfo <= 5.0);
    t.test_approx("LFO sin(π/4) ≈ 0.707", lfo, 0.7071, 0.01);

    // Modulation: base + LFO * depth
    let base_cutoff = 2.0;
    let mod_depth = 3.0;
    let modulated = eurorack.add(base_cutoff, eurorack.mul(lfo, mod_depth));
    t.test(
        "Modulated CV within ±5V",
        modulated >= -5.0 && modulated <= 5.0,
    );

    // Hot signal clips at rails
    let hot_signal = eurorack.add(4.0, 3.0);
    t.test_eq_f64("Hot signal 4+3 clips to +5V", hot_signal, 5.0);
    let cold_signal = eurorack.add(-4.0, -3.0);
    t.test_eq_f64("Cold signal -4-3 clips to -5V", cold_signal, -5.0);

    // VCA: signal * level
    let signal = 3.0;
    let level = 0.5;
    let vca_out = eurorack.mul(signal, level);
    t.test_eq_f64("VCA: 3V * 0.5 = 1.5V", vca_out, 1.5);

    // Mixer: sum of signals saturates
    let osc1 = 3.0;
    let osc2 = 2.5;
    let osc3 = 1.0;
    let mix = eurorack.add(eurorack.add(osc1, osc2), osc3);
    t.test_eq_f64("Mixer 3+2.5+1 clips to 5V", mix, 5.0);

    println!();

    // ═══════════════════════════════════════════════════════════════════
    // NEURAL NETWORK ACTIVATION USE CASE
    // ═══════════════════════════════════════════════════════════════════
    t.section("🧠 NEURAL NETWORK ACTIVATION (FloatUnit)");

    let nn = AnalogConfig::new(AnalogMode::FloatUnit);

    // Neuron: weighted_sum → activation
    let input = 0.7;
    let weight = 1.5;
    let bias = -0.3;

    let weighted = nn.mul(input, weight); // 0.7 * 1.5 = 1.05 → clamp to 1.0
    t.test_eq_f64("Weighted 0.7*1.5 clamps to 1.0", weighted, 1.0);

    let biased = nn.add(weighted, bias); // 1.0 + (-0.3) = 0.7
    t.test_eq_f64("Biased: 1.0 + (-0.3) = 0.7", biased, 0.7);

    let activated = nn.tanh(biased); // tanh(0.7) ≈ 0.604
    t.test_approx("tanh(0.7) ≈ 0.604", activated, 0.604, 0.01);

    // ReLU activation (using global state set to FloatUnit)
    set_analog_mode(AnalogMode::FloatUnit);
    t.test_eq_f64("ReLU(0.5) = 0.5", analog_relu(0.5), 0.5);
    t.test_eq_f64("ReLU(-0.3) = 0.0", analog_relu(-0.3), 0.0);
    t.test_eq_f64("ReLU(5.0) clamps to 1.0", analog_relu(5.0), 1.0);

    println!();

    // ═══════════════════════════════════════════════════════════════════
    // CROSS-MODE CONSISTENCY
    // ═══════════════════════════════════════════════════════════════════
    t.section("🔀 CROSS-MODE CONSISTENCY");

    // Same operation, different modes - saturation scales correctly
    let int201 = AnalogConfig::new(AnalogMode::Int201);
    let int2001 = AnalogConfig::new(AnalogMode::Int2001);
    let unit = AnalogConfig::new(AnalogMode::FloatUnit);

    // All modes: large * large → saturate to max
    t.test_eq_f64("Int201: 99*99 → 100", int201.mul(99.0, 99.0), 100.0);
    t.test_eq_f64("Int2001: 999*999 → 1000", int2001.mul(999.0, 999.0), 1000.0);
    t.test_eq_f64("FloatUnit: 0.99*2.0 → 1.0", unit.mul(0.99, 2.0), 1.0);

    // Midpoint consistency
    t.test_eq_f64("Int201 midpoint = 0", int201.midpoint(), 0.0);
    t.test_eq_f64("Int2001 midpoint = 0", int2001.midpoint(), 0.0);
    t.test_eq_f64("FloatUnit midpoint = 0", unit.midpoint(), 0.0);

    println!();

    // ═══════════════════════════════════════════════════════════════════
    // PERFORMANCE MICRO-BENCHMARK
    // ═══════════════════════════════════════════════════════════════════
    t.section("⚡ PERFORMANCE MICRO-BENCHMARK");

    reset_analog_mode();
    set_analog_mode(AnalogMode::Int201);

    let iters = 100_000;

    // Arithmetic throughput
    let bench_start = Instant::now();
    let mut acc = 0.0f64;
    for i in 0..iters {
        acc = analog_add(acc, 0.001);
        if i % 1000 == 0 {
            acc = analog_sub(acc, acc * 0.5);
        }
    }
    let arith_ns = bench_start.elapsed().as_nanos() as f64 / iters as f64;
    println!(
        "   Arithmetic: {:.1} ns/op ({} ops, result={:.2})",
        arith_ns, iters, acc
    );
    t.test("Arithmetic < 100ns/op", arith_ns < 100.0);

    // Trig throughput
    let bench_start = Instant::now();
    let mut acc = 0.0f64;
    for i in 0..iters {
        let phase = (i as f64) * 0.001;
        acc += analog_sin(phase);
    }
    let trig_ns = bench_start.elapsed().as_nanos() as f64 / iters as f64;
    println!(
        "   Trig (sin): {:.1} ns/op ({} ops, checksum={:.2})",
        trig_ns, iters, acc
    );
    t.test("Trig < 200ns/op", trig_ns < 200.0);

    // Batch throughput
    let big_a: Vec<f64> = (0..10_000).map(|i| (i as f64) * 0.01).collect();
    let big_b: Vec<f64> = (0..10_000).map(|i| (10_000 - i) as f64 * 0.01).collect();
    let bench_start = Instant::now();
    let _result = batch_add(&big_a, &big_b);
    let batch_us = bench_start.elapsed().as_micros();
    println!("   Batch add (10k): {} µs", batch_us);
    t.test("Batch 10k < 1000µs", batch_us < 1000);

    println!();

    // ═══════════════════════════════════════════════════════════════════
    // SUMMARY
    // ═══════════════════════════════════════════════════════════════════
    let elapsed = start.elapsed();
    println!("Total time: {:.2?}", elapsed);
    t.summary();
}
