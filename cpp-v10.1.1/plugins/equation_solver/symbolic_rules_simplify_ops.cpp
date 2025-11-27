// ======================================================
// symbolic_rules_simplify_ops.cpp - boils down symbolic logic
// ======================================================

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
using woflang::WofType;

// ---------- Local helpers to construct WofValue safely ----------

static WofValue make_int_value(std::int64_t n) {
    WofValue v;
    v.type  = WofType::Integer;
    v.value = n;                 // matches as_int: std::get<std::int64_t>(v.value)
    return v;
}

static WofValue make_string_value(const std::string& s) {
    WofValue v;
    v.type  = WofType::String;
    v.value = s;                 // matches as_text: std::get<std::string>(v.value)
    return v;
}

// Helpers for symbolic/syntactic manipulation
static bool is_symbol(const WofValue& v) {
    return v.type == WofType::Symbol;
}

static bool is_string(const WofValue& v) {
    return v.type == WofType::String;
}

static bool is_integer(const WofValue& v) {
    return v.type == WofType::Integer;
}

static std::string as_text(const WofValue& v) {
    if (v.type == WofType::Symbol || v.type == WofType::String) {
        return std::get<std::string>(v.value);
    }
    return {};
}

static std::int64_t as_int(const WofValue& v) {
    return std::get<std::int64_t>(v.value);
}

// X X +  ->  2 X *
static void op_simplify_sum(WoflangInterpreter& interp) {
    if (interp.stack.size() < 3) {
        return;
    }

    WofValue c = interp.stack.back(); interp.stack.pop_back(); // operator
    WofValue b = interp.stack.back(); interp.stack.pop_back(); // rhs
    WofValue a = interp.stack.back(); interp.stack.pop_back(); // lhs

    const bool match =
        is_symbol(a) &&
        is_symbol(b) &&
        as_text(a) == as_text(b) &&
        is_string(c) &&
        as_text(c) == "+";

    if (match) {
        // X X +  ->  2 X *
        interp.push(make_int_value(2));
        interp.push(a); // same symbol
        interp.push(make_string_value("*"));
    } else {
        // No change: restore original triple
        interp.push(a);
        interp.push(b);
        interp.push(c);
    }
}

// X * 1  ->  X
// 1 * X  ->  X
static void op_simplify_mul_one(WoflangInterpreter& interp) {
    if (interp.stack.size() < 3) {
        return;
    }

    WofValue c = interp.stack.back(); interp.stack.pop_back(); // operator
    WofValue b = interp.stack.back(); interp.stack.pop_back();
    WofValue a = interp.stack.back(); interp.stack.pop_back();

    if (is_string(c) && as_text(c) == "*") {
        // a * 1
        if (is_integer(b) && as_int(b) == 1) {
            interp.push(a);
            return;
        }

        // 1 * b
        if (is_integer(a) && as_int(a) == 1) {
            interp.push(b);
            return;
        }
    }

    // No match -> restore
    interp.push(a);
    interp.push(b);
    interp.push(c);
}

extern "C" WOFLANG_PLUGIN_EXPORT void register_plugin(WoflangInterpreter& interp) {
    interp.register_op("simplify_sum", [](WoflangInterpreter& ip) {
        try {
            op_simplify_sum(ip);
        } catch (const std::exception& e) {
            ip.push(make_string_value(std::string("simplify_sum error: ") + e.what()));
        }
    });

    interp.register_op("simplify_mul_one", [](WoflangInterpreter& ip) {
        try {
            op_simplify_mul_one(ip);
        } catch (const std::exception& e) {
            ip.push(make_string_value(std::string("simplify_mul_one error: ") + e.what()));
        }
    });

    std::cout << "[symbolic_simplify_rules] registered simplify_sum, simplify_mul_one\n";
}
