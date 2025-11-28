// ==================================================
// assert_ops.cpp - Assertion utility operations (v10.1.1)
// ==================================================

#ifndef WOFLANG_PLUGIN_EXPORT
# ifdef _WIN32
#  define WOFLANG_PLUGIN_EXPORT __declspec(dllexport)
# else
#  define WOFLANG_PLUGIN_EXPORT __attribute__((visibility("default")))
# endif
#endif

#include "../../src/core/woflang.hpp"

#include <stdexcept>
#include <iostream>
#include <cmath>
#include <string>

using woflang::WoflangInterpreter;
using woflang::WofValue;

namespace {

constexpr double DEFAULT_EPSILON = 1e-9;

// assert_true: pop value, treat as numeric, require non-zero
void op_assert_true(WoflangInterpreter& ip) {
    if (ip.stack.empty()) {
        throw std::runtime_error("assert_true: stack is empty");
    }

    WofValue v = ip.pop();
    double x = v.as_numeric();

    if (x == 0.0) {
        throw std::runtime_error("assert_true failed: value is zero/false");
    }
}

// assert_eq: pop b, then a; assert a == b (numeric, within DEFAULT_EPSILON)
void op_assert_eq(WoflangInterpreter& ip) {
    if (ip.stack.size() < 2) {
        throw std::runtime_error("assert_eq: need at least two values on the stack");
    }

    WofValue vb = ip.pop();
    WofValue va = ip.pop();

    double b = vb.as_numeric();
    double a = va.as_numeric();

    double diff = std::fabs(a - b);
    if (diff > DEFAULT_EPSILON) {
        throw std::runtime_error("assert_eq failed: values differ (|a - b| = " + std::to_string(diff) + ")");
    }
}

// assert_near: pop epsilon, expected, value (top-first); assert |value - expected| <= epsilon
void op_assert_near(WoflangInterpreter& ip) {
    if (ip.stack.size() < 3) {
        throw std::runtime_error("assert_near: need at least three values on the stack (value, expected, epsilon)");
    }

    WofValue v_eps = ip.pop();
    WofValue v_exp = ip.pop();
    WofValue v_val = ip.pop();

    double eps = v_eps.as_numeric();
    double expected = v_exp.as_numeric();
    double value = v_val.as_numeric();

    if (eps < 0.0) {
        throw std::runtime_error("assert_near: epsilon must be non-negative");
    }

    double diff = std::fabs(value - expected);
    if (diff > eps) {
        throw std::runtime_error(
            "assert_near failed: |value - expected| = " + std::to_string(diff) +
            " > epsilon = " + std::to_string(eps)
        );
    }
}

} // namespace

extern "C" WOFLANG_PLUGIN_EXPORT void register_plugin(WoflangInterpreter& interp) {
    interp.register_op("assert_true", op_assert_true);
    interp.register_op("assert_eq",   op_assert_eq);
    interp.register_op("assert_near", op_assert_near);

    std::cout << "[assert_ops] Plugin loaded: assert_true, assert_eq, assert_near\n";
}
