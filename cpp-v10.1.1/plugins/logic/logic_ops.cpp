// plugins/logic/logic_ops.cpp
// Boolean and comparison logic operators for WofLang v10.x

#ifndef WOFLANG_PLUGIN_EXPORT
# ifdef _WIN32
#  define WOFLANG_PLUGIN_EXPORT extern "C" __declspec(dllexport)
# else
#  define WOFLANG_PLUGIN_EXPORT extern "C"
# endif
#endif

#include "woflang.hpp"

#include <iostream>
#include <string>
#include <variant>
#include <stdexcept>
#include <cstdlib>   // std::strtod
#include <cerrno>
#include <cmath>

using namespace woflang;

// Helpers for converting and pushing boolean-like values.
namespace {

// ---------- Core stack helpers ----------

void require_stack_size(const WoflangInterpreter &ip,
                        std::size_t               n,
                        const char               *ctx) {
    if (ip.stack.size() < n) {
        throw std::runtime_error(std::string(ctx) + ": stack underflow");
    }
}

WofValue pop_raw(WoflangInterpreter &ip, const char *ctx) {
    require_stack_size(ip, 1, ctx);
    WofValue v = ip.stack.back();
    ip.stack.pop_back();
    return v;
}

void push_value(WoflangInterpreter &ip, const WofValue &v) {
    ip.stack.push_back(v);
}

// ---------- Type helpers ----------

bool is_string(const WofValue &v) {
    return v.type == WofType::String;
}

bool is_integer(const WofValue &v) {
    return v.type == WofType::Integer;
}

bool is_double(const WofValue &v) {
    return v.type == WofType::Double;
}

// Convert to numeric (Integer / Double / numeric String).
double to_numeric(const WofValue &v, const char *ctx = "[logic_ops] to_numeric") {
    if (is_integer(v)) {
        return static_cast<double>(std::get<std::int64_t>(v.value));
    }
    if (is_double(v)) {
        return std::get<double>(v.value);
    }
    if (is_string(v)) {
        const std::string &s = std::get<std::string>(v.value);
        if (s.empty()) {
            return 0.0;
        }
        char *end = nullptr;
        errno = 0;
        double val = std::strtod(s.c_str(), &end);
        if (end == s.c_str() || errno == ERANGE) {
            throw std::runtime_error(std::string(ctx) + ": non-numeric string \"" + s + "\"");
        }
        return val;
    }
    throw std::runtime_error(std::string(ctx) + ": unsupported type for numeric conversion");
}

// Truthiness: non-zero numeric is true, zero is false.
bool to_bool_like(const WofValue &v) {
    return to_numeric(v, "[logic_ops] to_bool_like") != 0.0;
}

// Represent booleans as numeric 1.0 / 0.0 so they compose with numeric ops.
WofValue make_bool_value(bool b) {
    WofValue out;
    out.type  = WofType::Double;
    out.value = b ? 1.0 : 0.0;
    return out;
}

// Equality: strings compare by content; otherwise compare numeric value.
bool values_equal(const WofValue &a, const WofValue &b) {
    if (is_string(a) && is_string(b)) {
        return std::get<std::string>(a.value) == std::get<std::string>(b.value);
    }
    return to_numeric(a, "[logic_ops] eq-lhs") ==
           to_numeric(b, "[logic_ops] eq-rhs");
}

// Generic numeric comparison helper.
template <typename Cmp>
void binary_compare(WoflangInterpreter &ip, Cmp cmp, const char *ctx) {
    require_stack_size(ip, 2, ctx);
    WofValue rhs = pop_raw(ip, ctx); // top
    WofValue lhs = pop_raw(ip, ctx); // next
    double   a   = to_numeric(lhs, ctx);
    double   b   = to_numeric(rhs, ctx);
    bool     result = cmp(a, b);
    push_value(ip, make_bool_value(result));
}

} // namespace

// ---------- Plugin entry: register ops on interpreter ----------

WOFLANG_PLUGIN_EXPORT void register_plugin(WoflangInterpreter &interp) {
    // Logical AND
    interp.register_op("and", [](WoflangInterpreter &ip) {
        const char *ctx = "[logic_ops] 'and'";
        require_stack_size(ip, 2, ctx);
        WofValue rhs = pop_raw(ip, ctx);
        WofValue lhs = pop_raw(ip, ctx);
        bool result = to_bool_like(lhs) && to_bool_like(rhs);
        push_value(ip, make_bool_value(result));
    });

    // Logical OR
    interp.register_op("or", [](WoflangInterpreter &ip) {
        const char *ctx = "[logic_ops] 'or'";
        require_stack_size(ip, 2, ctx);
        WofValue rhs = pop_raw(ip, ctx);
        WofValue lhs = pop_raw(ip, ctx);
        bool result = to_bool_like(lhs) || to_bool_like(rhs);
        push_value(ip, make_bool_value(result));
    });

    // Logical XOR (exclusive or)
    interp.register_op("xor", [](WoflangInterpreter &ip) {
        const char *ctx = "[logic_ops] 'xor'";
        require_stack_size(ip, 2, ctx);
        WofValue rhs = pop_raw(ip, ctx);
        WofValue lhs = pop_raw(ip, ctx);
        bool result = to_bool_like(lhs) ^ to_bool_like(rhs);
        push_value(ip, make_bool_value(result));
    });

    // Logical NOT
    interp.register_op("not", [](WoflangInterpreter &ip) {
        const char *ctx = "[logic_ops] 'not'";
        require_stack_size(ip, 1, ctx);
        WofValue v = pop_raw(ip, ctx);
        bool result = !to_bool_like(v);
        push_value(ip, make_bool_value(result));
    });

    // Logical implication: a ⇒ b is (!a) OR b
    interp.register_op("implies", [](WoflangInterpreter &ip) {
        const char *ctx = "[logic_ops] 'implies'";
        require_stack_size(ip, 2, ctx);
        WofValue rhs = pop_raw(ip, ctx);
        WofValue lhs = pop_raw(ip, ctx);
        bool a = to_bool_like(lhs);
        bool b = to_bool_like(rhs);
        bool result = (!a) || b;
        push_value(ip, make_bool_value(result));
    });

    // Equality / inequality
    interp.register_op("eq", [](WoflangInterpreter &ip) {
        const char *ctx = "[logic_ops] 'eq'";
        require_stack_size(ip, 2, ctx);
        WofValue rhs = pop_raw(ip, ctx);
        WofValue lhs = pop_raw(ip, ctx);
        bool result = values_equal(lhs, rhs);
        push_value(ip, make_bool_value(result));
    });

    interp.register_op("neq", [](WoflangInterpreter &ip) {
        const char *ctx = "[logic_ops] 'neq'";
        require_stack_size(ip, 2, ctx);
        WofValue rhs = pop_raw(ip, ctx);
        WofValue lhs = pop_raw(ip, ctx);
        bool result = !values_equal(lhs, rhs);
        push_value(ip, make_bool_value(result));
    });

    // Numeric comparisons (using to_numeric)
    interp.register_op("gt", [](WoflangInterpreter &ip) {
        binary_compare(ip,
                       [](double a, double b) { return a > b; },
                       "[logic_ops] 'gt'");
    });

    interp.register_op("lt", [](WoflangInterpreter &ip) {
        binary_compare(ip,
                       [](double a, double b) { return a < b; },
                       "[logic_ops] 'lt'");
    });

    interp.register_op("gte", [](WoflangInterpreter &ip) {
        binary_compare(ip,
                       [](double a, double b) { return a >= b; },
                       "[logic_ops] 'gte'");
    });

    interp.register_op("lte", [](WoflangInterpreter &ip) {
        binary_compare(ip,
                       [](double a, double b) { return a <= b; },
                       "[logic_ops] 'lte'");
    });

    std::cout << "[logic_ops] logical and comparison operators registered\n";
}
