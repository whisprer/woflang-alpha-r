// plugins/prime_heck_ops.cpp
//
// Prime-check operations for Woflang.
// Provides:
//   - prime_check : pops a numeric value N, pushes 1 if N is prime, 0 otherwise.

#include <cmath>
#include <cstdint>
#include <stdexcept>

#ifndef WOFLANG_PLUGIN_EXPORT
#  ifdef _WIN32
#    define WOFLANG_PLUGIN_EXPORT extern "C" __declspec(dllexport)
#  else
#    define WOFLANG_PLUGIN_EXPORT extern "C"
#  endif
#endif

#include "core/woflang.hpp"

using namespace woflang;

namespace {

bool is_probably_integer(double x) {
    double r = std::round(x);
    return std::fabs(x - r) < 0.5; // we're just sanitising inputs from scripts
}

bool is_prime_u64(std::uint64_t n) {
    if (n < 2) return false;
    if (n == 2 || n == 3) return true;
    if ((n & 1ull) == 0ull) return false;

    // simple deterministic trial division up to sqrt(n)
    std::uint64_t limit = static_cast<std::uint64_t>(std::sqrt(static_cast<long double>(n)));
    for (std::uint64_t d = 3; d <= limit; d += 2) {
        if (n % d == 0ull) return false;
    }
    return true;
}

void op_prime_check(WoflangInterpreter& interp) {
    if (interp.stack.empty()) {
        throw std::runtime_error("prime_check: stack underflow (need 1 value)");
    }

    // Top-of-stack is expected to be numeric. We allow both Integer and Double
    WofValue v = interp.pop();
    if (!v.is_numeric()) {
        throw std::runtime_error("prime_check: top of stack is not numeric");
    }

    double x = v.as_numeric();

    // Clamp/round to positive integer domain
    if (!is_probably_integer(x) || x < 0.0) {
        throw std::runtime_error("prime_check: expected non-negative integer");
    }

    std::uint64_t n = static_cast<std::uint64_t>(std::llround(x));
    bool prime = is_prime_u64(n);

    // Convention for the benchmark:
    //   1.0 => PRIME
    //   0.0 => COMPOSITE
    WofValue result(static_cast<int64_t>(prime ? 1 : 0));
    interp.push(result);
}

} // namespace

WOFLANG_PLUGIN_EXPORT void register_plugin(WoflangInterpreter& interp) {
    interp.register_op("prime_check",
        [](WoflangInterpreter& ctx) { op_prime_check(ctx); });
}
