// ===============================================================
// crypto_div_trial_ops.cpp - Cryptographic Division Trial Operations
// (modernized for Woflang v10.1.1 core API)
// ===============================================================

#include <cmath>
#include <cstdint>
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

namespace detail {

// Try to read a numeric value from WofValue.
// Prefer the new as_numeric(), but fall back to the old `.d` field if present.
template<typename T>
auto as_num(const T& v, int) -> decltype(v.as_numeric()) {
    return v.as_numeric();
}

template<typename T>
double as_num(const T& v, ...) {
    // Fallback for old WofValue that exposed a public `double d;`
    return v.d;
}

// Try to construct a numeric WofValue.
// We support three possible shapes:
//   1) static T::from_numeric(double)
//   2) T(double) constructor
//   3) public `.d` field as in older versions
template<typename T>
auto make_num(double x, int) -> decltype(T::from_numeric(x)) {
    return T::from_numeric(x);
}

template<typename T>
auto make_num(double x, long) -> decltype(T(x)) {
    return T(x);
}

template <typename T>
T make_num(double x, ...) {
    return WofValue::make_double(x);
}

inline std::int64_t to_int64(const WofValue& v) {
    double d = as_num(v, 0);
    if (!std::isfinite(d)) {
        throw std::runtime_error("prime_check: numeric argument must be finite");
    }
    return static_cast<std::int64_t>(d);
}

inline WofValue make_bool(bool b) {
    return WofValue::make_double(b ? 1.0 : 0.0);
}

// Deterministic trial-division primality test for 64-bit integers
inline bool is_prime_int64(std::int64_t n) {
    if (n <= 1) return false;
    if (n <= 3) return true;
    if (n % 2 == 0 || n % 3 == 0) return false;

    for (std::int64_t i = 5; i * i <= n; i += 6) {
        if (n % i == 0 || n % (i + 2) == 0) {
            return false;
        }
    }
    return true;
}

} // namespace detail

extern "C" WOFLANG_PLUGIN_EXPORT void register_plugin(WoflangInterpreter& interp) {
    // ---------------------------------------------------------------------
    // prime_check : n -- 0|1
    //
    // Pops an integer n and pushes:
    //   1.0 if n is prime
    //   0.0 otherwise
    //
    // Also logs "n is prime" or "n is not prime" to stdout.
    // ---------------------------------------------------------------------
    interp.register_op("prime_check", [](WoflangInterpreter& ip) {
        auto& st = ip.stack;

        if (st.empty()) {
            throw std::runtime_error("prime_check requires a number");
        }

        WofValue v = st.back();
        st.pop_back();

        std::int64_t n = detail::to_int64(v);
        bool prime = detail::is_prime_int64(n);

        WofValue result = detail::make_bool(prime);
        st.push_back(result);

        if (prime) {
            std::cout << n << " is prime" << std::endl;
        } else {
            std::cout << n << " is not prime" << std::endl;
        }
    });

    // If you later want diffie_hellman, random, etc. you can add them here.
    // For the benchmark, only prime_check needs to be functional.
}
