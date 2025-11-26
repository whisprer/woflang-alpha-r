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

using woflang::WofValue;
using woflang::WoflangInterpreter;

namespace {

// Simple stack adapter for Woflang's value stack
template <typename Container>
struct WofStackAdapter {
    Container& v;
    auto size() const { return v.size(); }
    bool empty() const { return v.empty(); }
    WofValue& top() { return v.back(); }
    const WofValue& top() const { return v.back(); }
    void pop() { v.pop_back(); }
    void push(const WofValue& x) { v.push_back(x); }
};

// Extract numeric value or throw with a nice message
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

    // RPN: "a b op"  → stack: [..., a, b]
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

} // anonymous namespace

extern "C" WOFLANG_PLUGIN_EXPORT void register_plugin(WoflangInterpreter& interp) {
    // Basic arithmetic
    interp.register_op("+", [](WoflangInterpreter& ip) {
        binary_op(ip, "+", [](double a, double b) { return a + b; });
    });

    interp.register_op("-", [](WoflangInterpreter& ip) {
        binary_op(ip, "-", [](double a, double b) { return a - b; });
    });

    interp.register_op("*", [](WoflangInterpreter& ip) {
        binary_op(ip, "*", [](double a, double b) { return a * b; });
    });

    interp.register_op("/", [](WoflangInterpreter& ip) {
        binary_op(ip, "/", [](double a, double b) {
            if (b == 0.0) {
                throw std::runtime_error("/: division by zero");
            }
            return a / b;
        });
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

    // You used to have constants / extra stuff here; for v10.1.1
    // we leave pi/e to trig_ops.cpp so trig tests are clean.
}
