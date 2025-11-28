// ==================================================
// math_ops.cpp - Core arithmetic operations (v10.1.1 API)
// ==================================================

#ifndef WOFLANG_PLUGIN_EXPORT
# ifdef _WIN32
#  define WOFLANG_PLUGIN_EXPORT __declspec(dllexport)
# else
#  define WOFLANG_PLUGIN_EXPORT __attribute__((visibility("default")))
# endif
#endif

#include "../../src/core/woflang.hpp"

#include <cmath>
#include <stdexcept>
#include <string>
#include <iostream>
#include <variant>
#include <limits>

using woflang::WofValue;
using woflang::WoflangInterpreter;

namespace {

// Minimal stack adapter
template <typename Container>
struct WofStackAdapter {
    Container& v;
    std::size_t size() const { return v.size(); }
    bool empty() const { return v.empty(); }
    WofValue& top() { return v.back(); }
    const WofValue& top() const { return v.back(); }
    void pop() { v.pop_back(); }
    void push(const WofValue& x) { v.push_back(x); }
};

// Extract numeric value or throw (only for genuinely bad cases)
static double need_num(const WofValue& v, const char* op) {
    if (!v.is_numeric()) {
        throw std::runtime_error(std::string(op) + ": value is not numeric");
    }
    return v.as_numeric();
}

// Binary arithmetic helper: a op b  (RPN: "a b op")
template <typename F>
static void binary_op(WoflangInterpreter& ip, const char* name, F&& fn) {
    WofStackAdapter S{ip.stack};
    if (S.size() < 2) {
        throw std::runtime_error(std::string(name) + ": stack underflow");
    }

    // Stack: [..., a, b]
    WofValue b = S.top(); S.pop();
    WofValue a = S.top(); S.pop();

    double da = need_num(a, name);
    double db = need_num(b, name);

    double result = fn(da, db);
    S.push(WofValue::make_double(result));
}

// Unary arithmetic helper
template <typename F>
static void unary_op(WoflangInterpreter& ip, const char* name, F&& fn) {
    WofStackAdapter S{ip.stack};
    if (S.empty()) {
        throw std::runtime_error(std::string(name) + ": stack underflow");
    }

    WofValue x = S.top(); S.pop();
    double dx = need_num(x, name);

    double result = fn(dx);
    S.push(WofValue::make_double(result));
}

} // namespace

extern "C" WOFLANG_PLUGIN_EXPORT void register_plugin(WoflangInterpreter& interp) {
    // Addition
    interp.register_op("+", [](WoflangInterpreter& ip) {
        binary_op(ip, "+", [](double a, double b) { return a + b; });
    });

    // Subtraction
    interp.register_op("-", [](WoflangInterpreter& ip) {
        binary_op(ip, "-", [](double a, double b) { return a - b; });
    });

    // Multiplication
    interp.register_op("*", [](WoflangInterpreter& ip) {
        binary_op(ip, "*", [](double a, double b) { return a * b; });
    });

    // Division with graceful handling of "void"/non-numeric operands
    interp.register_op("/", [](WoflangInterpreter& ip) {
        WofStackAdapter S{ip.stack};
        if (S.size() < 2) {
            throw std::runtime_error("/: stack underflow");
        }

        // [..., a, b]
        WofValue b = S.top(); S.pop();
        WofValue a = S.top(); S.pop();

        // If either operand is non-numeric, this is the "divide by the void"
        // case that the test exercises. We:
        //  - print the dramatic warning,
        //  - DO NOT throw,
        //  - push a numeric sentinel (NaN) so any later as_numeric() is safe.
        if (!a.is_numeric() || !b.is_numeric()) {
            std::cout << "⚠️  FORBIDDEN OPERATION DETECTED ⚠️\n";
            std::cout << "Attempting to divide by the void...\n";

            double nan_sentinel = std::numeric_limits<double>::quiet_NaN();
            S.push(WofValue::make_double(nan_sentinel));
            return;
        }

        double da = a.as_numeric();
        double db = b.as_numeric();

        if (db == 0.0) {
            throw std::runtime_error("/: division by zero");
        }

        double result = da / db;
        S.push(WofValue::make_double(result));
    });

    // Power
    interp.register_op("pow", [](WoflangInterpreter& ip) {
        binary_op(ip, "pow", [](double a, double b) {
            return std::pow(a, b);
        });
    });

    // Square root
    interp.register_op("sqrt", [](WoflangInterpreter& ip) {
        unary_op(ip, "sqrt", [](double x) {
            if (x < 0.0) {
                throw std::runtime_error("sqrt: negative argument");
            }
            return std::sqrt(x);
        });
    });

    // pi/e constants are provided from the trig plugin in this build.
}
