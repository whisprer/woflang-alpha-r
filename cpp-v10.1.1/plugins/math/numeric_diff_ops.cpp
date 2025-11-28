// numeric_diff_ops.cpp
// Unified finite-difference numerical differentiation plugin for WofLang v10.x
//
// Provides a small set of robust finite-difference operators that operate purely
// on numeric values already on the stack. This keeps the implementation generic
// and lets user code decide how to compute f(x±h) etc.
//
// Ops and stack conventions (bottom ... top):
//
//   diff_forward:
//     Stack before:   f(x)   f(x+h)   h
//     Stack after:    f'(x)      (single numeric)
//     Formula: (f(x+h) - f(x)) / h
//
//   diff_backward:
//     Stack before:   f(x-h)   f(x)   h
//     Stack after:    f'(x)
//     Formula: (f(x) - f(x-h)) / h
//
//   diff_central:
//     Stack before:   f(x-h)   f(x+h)   h
//     Stack after:    f'(x)
//     Formula: (f(x+h) - f(x-h)) / (2 h)
//
//   diff_second:
//     Stack before:   f(x-h)   f(x)   f(x+h)   h
//     Stack after:    f''(x)
//     Formula: (f(x+h) - 2 f(x) + f(x-h)) / (h^2)

#include <cmath>
#include <stdexcept>
#include <string>
#include <cerrno>

#include "woflang.hpp"

using namespace woflang;

#ifndef WOFLANG_PLUGIN_EXPORT
# ifdef _WIN32
#  define WOFLANG_PLUGIN_EXPORT extern "C" __declspec(dllexport)
# else
#  define WOFLANG_PLUGIN_EXPORT extern "C"
# endif
#endif

namespace {

// Convert WofValue to double with diagnostics.
double to_numeric(const WofValue& v, const char* ctx) {
    switch (v.type) {
        case WofType::Integer:
            return static_cast<double>(std::get<std::int64_t>(v.value));
        case WofType::Double:
            return std::get<double>(v.value);
        case WofType::String: {
            const auto& s = std::get<std::string>(v.value);
            if (s.empty()) {
                throw std::runtime_error(std::string(ctx) + ": empty string is not numeric");
            }
            char* end = nullptr;
            errno = 0;
            double val = std::strtod(s.c_str(), &end);
            if (end == s.c_str() || errno == ERANGE) {
                throw std::runtime_error(std::string(ctx) + ": non-numeric string \"" + s + "\"");
            }
            return val;
        }
        default:
            throw std::runtime_error(std::string(ctx) + ": unsupported type for numeric conversion");
    }
}

// Small helper to pop a numeric value from the stack with good diagnostics.
double pop_numeric(WoflangInterpreter& interp, const char* op_name, const char* what) {
    if (interp.stack.empty()) {
        throw std::runtime_error(std::string(op_name) + ": stack underflow while popping " + what);
    }
    WofValue v = interp.stack.back();
    interp.stack.pop_back();
    return to_numeric(v, op_name);
}

WofValue make_double(double x) {
    WofValue v;
    v.type  = WofType::Double;
    v.value = x;
    return v;
}

void op_diff_forward(WoflangInterpreter& interp) {
    constexpr const char* OP = "diff_forward";

    double h     = pop_numeric(interp, OP, "step h");
    double f_xph = pop_numeric(interp, OP, "f(x+h)");
    double f_x   = pop_numeric(interp, OP, "f(x)");

    if (h == 0.0) {
        throw std::runtime_error(std::string(OP) + ": step h must be non-zero");
    }

    double deriv = (f_xph - f_x) / h;
    interp.stack.push_back(make_double(deriv));
}

void op_diff_backward(WoflangInterpreter& interp) {
    constexpr const char* OP = "diff_backward";

    double h     = pop_numeric(interp, OP, "step h");
    double f_x   = pop_numeric(interp, OP, "f(x)");
    double f_xmh = pop_numeric(interp, OP, "f(x-h)");

    if (h == 0.0) {
        throw std::runtime_error(std::string(OP) + ": step h must be non-zero");
    }

    double deriv = (f_x - f_xmh) / h;
    interp.stack.push_back(make_double(deriv));
}

void op_diff_central(WoflangInterpreter& interp) {
    constexpr const char* OP = "diff_central";

    double h     = pop_numeric(interp, OP, "step h");
    double f_xph = pop_numeric(interp, OP, "f(x+h)");
    double f_xmh = pop_numeric(interp, OP, "f(x-h)");

    if (h == 0.0) {
        throw std::runtime_error(std::string(OP) + ": step h must be non-zero");
    }

    double deriv = (f_xph - f_xmh) / (2.0 * h);
    interp.stack.push_back(make_double(deriv));
}

void op_diff_second(WoflangInterpreter& interp) {
    constexpr const char* OP = "diff_second";

    double h     = pop_numeric(interp, OP, "step h");
    double f_xph = pop_numeric(interp, OP, "f(x+h)");
    double f_x   = pop_numeric(interp, OP, "f(x)");
    double f_xmh = pop_numeric(interp, OP, "f(x-h)");

    if (h == 0.0) {
        throw std::runtime_error(std::string(OP) + ": step h must be non-zero");
    }

    double second = (f_xph - 2.0 * f_x + f_xmh) / (h * h);
    interp.stack.push_back(make_double(second));
}

} // namespace

WOFLANG_PLUGIN_EXPORT void register_plugin(WoflangInterpreter& interp) {
    interp.register_op("diff_forward",  [](WoflangInterpreter& ip) { op_diff_forward(ip);  });
    interp.register_op("diff_backward", [](WoflangInterpreter& ip) { op_diff_backward(ip); });
    interp.register_op("diff_central",  [](WoflangInterpreter& ip) { op_diff_central(ip);  });
    interp.register_op("diff_second",   [](WoflangInterpreter& ip) { op_diff_second(ip);   });
}
