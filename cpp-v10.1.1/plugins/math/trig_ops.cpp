// ==================================================
// trig_ops.cpp - Trigonometric operations for WofLang
// ==================================================

#ifndef WOFLANG_PLUGIN_EXPORT
#  ifdef _WIN32
#    define WOFLANG_PLUGIN_EXPORT __declspec(dllexport)
#  else
#    define WOFLANG_PLUGIN_EXPORT __attribute__((visibility("default")))
#  endif
#endif

#include "../../src/core/woflang.hpp"

#include <cmath>
#include <stdexcept>
#include <string>

using namespace woflang;

namespace {

// Pop a numeric value from the interpreter stack and return it as double.
double pop_numeric(WoflangInterpreter& ip, const char* ctx) {
    auto& st = ip.stack;
    if (st.empty()) {
        throw std::runtime_error(std::string(ctx) + ": empty stack");
    }

    WofValue v = st.back();
    st.pop_back();

    try {
        return v.as_numeric();
    } catch (const std::exception& e) {
        throw std::runtime_error(std::string(ctx) + ": " + e.what());
    }
}

// Push a double as a numeric WofValue.
void push_double(WoflangInterpreter& ip, double x) {
    ip.stack.push_back(WofValue::make_double(x));
}

// Helper to register a unary trig op: name, std::sin/std::cos/etc
template <typename F>
void register_unary(WoflangInterpreter& interp,
                    const char* name,
                    F func) {
    interp.register_op(name, [func, name](WoflangInterpreter& ip) {
        double x = pop_numeric(ip, name);
        double y = func(x);
        push_double(ip, y);
    });
}

} // namespace

extern "C" WOFLANG_PLUGIN_EXPORT
void register_plugin(WoflangInterpreter& interp) {
    // Numeric constants
    interp.register_op("pi", [](WoflangInterpreter& ip) {
        // Use high-precision PI if available
        constexpr double pi_val =
        #ifdef M_PI
            M_PI;
        #else
            3.14159265358979323846;
        #endif
        push_double(ip, pi_val);
    });

    interp.register_op("e", [](WoflangInterpreter& ip) {
        constexpr double e_val = 2.71828182845904523536;
        push_double(ip, e_val);
    });

    // Basic trig (radians)
    register_unary(interp, "sin",
        static_cast<double(*)(double)>(std::sin));
    register_unary(interp, "cos",
        static_cast<double(*)(double)>(std::cos));
    register_unary(interp, "tan",
        static_cast<double(*)(double)>(std::tan));

    // Inverse trig
    register_unary(interp, "asin",
        static_cast<double(*)(double)>(std::asin));
    register_unary(interp, "acos",
        static_cast<double(*)(double)>(std::acos));
    register_unary(interp, "atan",
        static_cast<double(*)(double)>(std::atan));

    // atan2(y, x): note order
    interp.register_op("atan2", [](WoflangInterpreter& ip) {
        double x = pop_numeric(ip, "atan2 x");
        double y = pop_numeric(ip, "atan2 y");
        push_double(ip, std::atan2(y, x));
    });

    // Hyperbolic
    register_unary(interp, "sinh",
        static_cast<double(*)(double)>(std::sinh));
    register_unary(interp, "cosh",
        static_cast<double(*)(double)>(std::cosh));
    register_unary(interp, "tanh",
        static_cast<double(*)(double)>(std::tanh));
}
