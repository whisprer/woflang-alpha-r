// ===============================================================
// prime_ops.cpp - Prime utilities (is_prime, next_prime, factoring)
// (Woflang plugin, v10.1.1-compatible, old-style stack API)
// ===============================================================

#include "../../src/core/woflang.hpp"
#include <cstdint>
#include <cmath>
#include <iostream>
#include <vector>

using namespace woflang;

// ---------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------
namespace {

bool is_prime_int(int64_t n) {
    if (n < 2) return false;
    if (n == 2 || n == 3) return true;
    if (n % 2 == 0) return false;

    int64_t limit = static_cast<int64_t>(std::sqrt(static_cast<long double>(n)));
    for (int64_t d = 3; d <= limit; d += 2) {
        if (n % d == 0) return false;
    }
    return true;
}

int64_t next_prime_int(int64_t n) {
    if (n <= 2) return 2;
    int64_t candidate = (n % 2 == 0) ? n + 1 : n;
    while (!is_prime_int(candidate)) {
        candidate += 2;
    }
    return candidate;
}

// Basic trial-division factorisation; returns empty for n == 0/±1
std::vector<int64_t> factor_int(int64_t n) {
    std::vector<int64_t> factors;
    if (n == 0 || n == 1 || n == -1) {
        return factors;
    }

    if (n < 0) {
        factors.push_back(-1);
        n = -n;
    }

    while (n % 2 == 0) {
        factors.push_back(2);
        n /= 2;
    }

    int64_t d = 3;
    while (d * d <= n) {
        while (n % d == 0) {
            factors.push_back(d);
            n /= d;
        }
        d += 2;
    }

    if (n > 1) {
        factors.push_back(n);
    }

    return factors;
}

} // namespace

// ---------------------------------------------------------------
// Plugin
// ---------------------------------------------------------------

class PrimeOpsPlugin : public WoflangPlugin {
public:
    void register_ops(WoflangInterpreter& interp) override {
        // n -- 0|1
        interp.register_op("is_prime", [](WoflangInterpreter& interp) {
            auto& st = interp.stack;
            if (st.empty()) {
                std::cout << "is_prime: need 1 integer\n";
                return;
            }

            WofValue v = st.back(); st.pop_back();
            if (v.type != WofType::Integer) {
                std::cout << "is_prime: argument must be integer\n";
                return;
            }

            int64_t n = std::get<int64_t>(v.value);
            bool p = is_prime_int(n);

            st.emplace_back(static_cast<int64_t>(p ? 1 : 0));
        });

        // n -- next_prime(n)
        interp.register_op("next_prime", [](WoflangInterpreter& interp) {
            auto& st = interp.stack;
            if (st.empty()) {
                std::cout << "next_prime: need 1 integer\n";
                return;
            }

            WofValue v = st.back(); st.pop_back();
            if (v.type != WofType::Integer) {
                std::cout << "next_prime: argument must be integer\n";
                return;
            }

            int64_t n = std::get<int64_t>(v.value);
            int64_t p = next_prime_int(n);
            st.emplace_back(p);
        });

        // n -- f1 f2 ... fk  (prime factors)
        interp.register_op("prime_factors", [](WoflangInterpreter& interp) {
            auto& st = interp.stack;
            if (st.empty()) {
                std::cout << "prime_factors: need 1 integer\n";
                return;
            }

            WofValue v = st.back(); st.pop_back();
            if (v.type != WofType::Integer) {
                std::cout << "prime_factors: argument must be integer\n";
                return;
            }

            int64_t n = std::get<int64_t>(v.value);
            auto factors = factor_int(n);

            if (factors.empty()) {
                std::cout << "prime_factors: " << n
                          << " has no non-trivial factorisation (0, ±1, or prime)\n";
                // We *do not* push anything back in this case.
                return;
            }

            // Push factors in order; top of stack will be the last factor
            for (int64_t f : factors) {
                st.emplace_back(f);
            }
        });
    }
};

WOFLANG_PLUGIN_EXPORT void register_plugin(WoflangInterpreter& interp) {
    static PrimeOpsPlugin plugin;
    plugin.register_ops(interp);
    std::cout << "[prime_ops] Plugin loaded (is_prime, next_prime, prime_factors)\n";
}
