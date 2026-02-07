//! Run the comprehensive analog test suite.
//!
//! Usage: cargo run -p woflang-analog --example analog_tests

fn main() {
    woflang_analog::test_suite::run_analog_test_suite();
}
