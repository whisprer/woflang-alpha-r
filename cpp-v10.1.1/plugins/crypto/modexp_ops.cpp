// modexp_ops.cpp - Modular exponentiation and helpers for Woflang
//
// Provides basic modular arithmetic helpers:
//   - modexp:      (base exp mod -- result)
//   - modinv:      (a m -- inv) where a * inv ≡ 1 (mod m), if it exists
//   - modexp_demo: pushes a small demo value onto the stack.
//
// All operations are integer-only and operate on the WofLang data stack.
// They do not depend on any WofValue::make_* helpers to avoid link issues.

#include <cstdint>
#include <cmath>
#include <stdexcept>
#include <string>
#include <variant>
#include <limits>

#include "woflang.hpp"

#ifndef WOFLANG_PLUGIN_EXPORT
#define WOFLANG_PLUGIN_EXPORT extern "C"
#endif

using namespace woflang;

// ---- Small WofValue helpers -------------------------------------------------

static double to_double_checked(const WofValue &v, const char *op_name) {
    if (v.type == WofType::Integer) {
        return static_cast<double>(std::get<int64_t>(v.value));
    }
    if (v.type == WofType::Double) {
        return std::get<double>(v.value);
    }
    throw std::runtime_error(std::string(op_name) + ": expected numeric value");
}

static int64_t to_int_checked(const WofValue &v, const char *op_name) {
    double d = to_double_checked(v, op_name);
    if (!std::isfinite(d)) {
        throw std::runtime_error(std::string(op_name) + ": non-finite numeric value");
    }
    if (d > static_cast<double>(std::numeric_limits<int64_t>::max()) ||
        d < static_cast<double>(std::numeric_limits<int64_t>::min())) {
        throw std::runtime_error(std::string(op_name) + ": integer overflow");
    }
    return static_cast<int64_t>(d);
}

static WofValue make_int64(int64_t v) {
    WofValue out;
    out.type  = WofType::Integer;
    out.value = v;
    return out;
}

static void push_int(WoflangInterpreter &ip, int64_t v) {
    ip.stack.push_back(make_int64(v));
}

static WofValue pop_value(WoflangInterpreter &ip, const char *op_name) {
    if (ip.stack.empty()) {
        throw std::runtime_error(std::string(op_name) + ": stack underflow");
    }
    WofValue v = ip.stack.back();
    ip.stack.pop_back();
    return v;
}

// ---- Core math helpers ------------------------------------------------------

static int64_t mod_floor(int64_t a, int64_t m) {
    int64_t r = a % m;
    if (r < 0) r += m;
    return r;
}

static int64_t modexp_int(int64_t base, int64_t exp, int64_t mod) {
    if (mod <= 0) {
        throw std::runtime_error("modexp: modulus must be positive");
    }
    base = mod_floor(base, mod);
    int64_t result = 1 % mod;
    while (exp > 0) {
        if (exp & 1) {
            result = mod_floor(result * base, mod);
        }
        base = mod_floor(base * base, mod);
        exp >>= 1;
    }
    return result;
}

static int64_t egcd(int64_t a, int64_t b, int64_t &x, int64_t &y) {
    if (b == 0) {
        x = 1;
        y = 0;
        return a;
    }
    int64_t x1 = 0, y1 = 0;
    int64_t g = egcd(b, a % b, x1, y1);
    x = y1;
    y = x1 - (a / b) * y1;
    return g;
}

static int64_t modinv_int(int64_t a, int64_t m) {
    int64_t x = 0, y = 0;
    int64_t g = egcd(a, m, x, y);
    if (g != 1 && g != -1) {
        throw std::runtime_error("modinv: inverse does not exist (numbers not coprime)");
    }
    int64_t inv = mod_floor(x, m);
    return inv;
}

// ---- Stack-level operations -------------------------------------------------

static void op_modexp(WoflangInterpreter &ip) {
    // Stack: base exponent modulus  --  result
    const char *op = "modexp";

    int64_t mod   = to_int_checked(pop_value(ip, op), op);
    int64_t exp   = to_int_checked(pop_value(ip, op), op);
    int64_t base  = to_int_checked(pop_value(ip, op), op);

    if (exp < 0) {
        throw std::runtime_error("modexp: negative exponent not supported (use modinv if needed)");
    }

    int64_t result = modexp_int(base, exp, mod);
    push_int(ip, result);
}

static void op_modinv(WoflangInterpreter &ip) {
    // Stack: a m  --  inv
    const char *op = "modinv";

    int64_t m = to_int_checked(pop_value(ip, op), op);
    int64_t a = to_int_checked(pop_value(ip, op), op);

    if (m <= 0) {
        throw std::runtime_error("modinv: modulus must be positive");
    }

    int64_t inv = modinv_int(a, m);
    push_int(ip, inv);
}

static void op_modexp_demo(WoflangInterpreter &ip) {
    // Push a small demo: 7^128 mod 101
    int64_t result = modexp_int(7, 128, 101);
    push_int(ip, result);
}

// ---- Plugin entry point -----------------------------------------------------

WOFLANG_PLUGIN_EXPORT void register_plugin(WoflangInterpreter &interp) {
    interp.register_op("modexp", [](WoflangInterpreter &ip) {
        op_modexp(ip);
    });

    interp.register_op("modinv", [](WoflangInterpreter &ip) {
        op_modinv(ip);
    });

    interp.register_op("modexp_demo", [](WoflangInterpreter &ip) {
        op_modexp_demo(ip);
    });
}
