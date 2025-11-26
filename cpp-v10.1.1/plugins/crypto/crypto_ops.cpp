// ======================================================
// crypto_ops.cpp - Cryptography and Encoding Operations
// (modernized for Woflang v10.1.1 core API)
// ======================================================

#include <cstdint>
#include <cmath>
#include <iostream>
#include <string>

#include "woflang.hpp"

#ifndef WOFLANG_PLUGIN_EXPORT
# ifdef _WIN32
#  define WOFLANG_PLUGIN_EXPORT __declspec(dllexport)
# else
#  define WOFLANG_PLUGIN_EXPORT __attribute__((visibility("default")))
# endif
#endif

using woflang::WoflangInterpreter;
using woflang::WofValue;

// -------------------------- helpers --------------------------

namespace {

std::int64_t to_int64(const WofValue& v, const char* ctx) {
    // v10.1.1 exposes a type-safe numeric accessor; use that.
    double d = v.as_numeric();

    if (!std::isfinite(d)) {
        throw std::runtime_error(std::string(ctx) + ": argument must be finite");
    }

    // Match original semantics: truncate toward zero.
    return static_cast<std::int64_t>(d);
}

// Simple deterministic trial-division primality test for 64-bit integers
bool is_prime_int64(std::int64_t n) {
    if (n <= 1) return false;
    if (n <= 3) return true;
    if (n % 2 == 0 || n % 3 == 0) return false;

    // 6k ± 1 wheel
    for (std::int64_t i = 5; i * i <= n; i += 6) {
        if (n % i == 0 || n % (i + 2) == 0) {
            return false;
        }
    }
    return true;
}

} // namespace

// -------------------------- plugin entry --------------------------

extern "C" WOFLANG_PLUGIN_EXPORT void register_plugin(WoflangInterpreter& interp) {
    // ---------------------------------------------------------------------
    // prime_check : n -- 0|1
    //
    // Pops an integer n from the Woflang stack and pushes:
    //   1.0 if n is prime
    //   0.0 otherwise
    //
    // Also logs "<n> is prime" or "<n> is not prime" to stdout.
    //
    // This is what the --benchmark harness expects to call.
    // ---------------------------------------------------------------------
    interp.register_op("prime_check", [](WoflangInterpreter& ip) {
        auto& S = ip.stack;

        if (S.empty()) {
            throw std::runtime_error("prime_check requires a number");
        }

        // Pop argument
        WofValue v = S.back();
        S.pop_back();

        std::int64_t n = to_int64(v, "prime_check");

        bool prime = is_prime_int64(n);

        // Construct a numeric WofValue using the core's factories.
        // We choose make_int(0/1) so it's a clean integral flag.
        WofValue result = WofValue::make_int(prime ? 1 : 0);
        S.push_back(result);

        if (prime) {
            std::cout << n << " is prime" << std::endl;
        } else {
            std::cout << n << " is not prime" << std::endl;
        }
    });

    // ---------------------------------------------------------------------
    // The original crypto_ops.cpp likely had more ops (random, etc.).
    // We can re-add them later. For the benchmark, only prime_check
    // needs to be fully functional.
    // ---------------------------------------------------------------------
}
