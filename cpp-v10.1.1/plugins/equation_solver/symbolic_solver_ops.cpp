// ==================================================
// symbolic_solver_ops.cpp - Basic Symbolic Solver (v10.1.1)
// ==================================================

#ifndef WOFLANG_PLUGIN_EXPORT
# ifdef _WIN32
#  define WOFLANG_PLUGIN_EXPORT __declspec(dllexport)
# else
#  define WOFLANG_PLUGIN_EXPORT __attribute__((visibility("default")))
# endif
#endif

#include "woflang.hpp"

#include <iostream>
#include <stdexcept>
#include <string>
#include <cmath>

using woflang::WoflangInterpreter;
using woflang::WofValue;
using woflang::WofType;

// -------- solve: basic demo numeric "solver" --------

static void op_solve(WoflangInterpreter& interp) {
    if (interp.stack.empty()) {
        std::cout << "[solve] stack is empty\n";
        return;
    }

    auto expr = interp.stack.back();
    interp.stack.pop_back();

    if (!expr.is_numeric()) {
        std::cout << "[solve] expected numeric expression\n";
        interp.stack.push_back(expr);
        return;
    }

    double val = expr.as_numeric();
    std::cout << "[solve] Solving for expression = " << val << "\n";
    std::cout << "[solve] (Demo: would need symbolic AST for full solving)\n";

    // For demo, just echo back
    interp.stack.push_back(expr);
}

// -------- solve_quadratic: ax^2 + bx + c = 0 --------

static void op_solve_quadratic(WoflangInterpreter& interp) {
    if (interp.stack.size() < 3) {
        std::cout << "[solve_quadratic] needs: a b c (for ax^2+bx+c=0)\n";
        return;
    }

    auto c = interp.stack.back(); interp.stack.pop_back();
    auto b = interp.stack.back(); interp.stack.pop_back();
    auto a = interp.stack.back(); interp.stack.pop_back();

    if (!a.is_numeric() || !b.is_numeric() || !c.is_numeric()) {
        std::cout << "[solve_quadratic] all coefficients must be numeric\n";
        interp.stack.push_back(a);
        interp.stack.push_back(b);
        interp.stack.push_back(c);
        return;
    }

    double a_val = a.as_numeric();
    double b_val = b.as_numeric();
    double c_val = c.as_numeric();

    if (a_val == 0.0) {
        std::cout << "[solve_quadratic] 'a' cannot be zero\n";
        interp.stack.push_back(a);
        interp.stack.push_back(b);
        interp.stack.push_back(c);
        return;
    }

    double discriminant = b_val * b_val - 4.0 * a_val * c_val;

    std::cout << "[solve_quadratic] " << a_val << "x^2 + " << b_val
              << "x + " << c_val << " = 0\n";
    std::cout << "[solve_quadratic] Discriminant = " << discriminant << "\n";

    if (discriminant < 0.0) {
        std::cout << "[solve_quadratic] No real solutions (complex roots)\n";
        interp.stack.push_back(WofValue::make_double(discriminant));
    } else if (discriminant == 0.0) {
        double x = -b_val / (2.0 * a_val);
        std::cout << "[solve_quadratic] One solution: x = " << x << "\n";
        interp.stack.push_back(WofValue::make_double(x));
    } else {
        double sqrt_disc = std::sqrt(discriminant);
        double x1 = (-b_val + sqrt_disc) / (2.0 * a_val);
        double x2 = (-b_val - sqrt_disc) / (2.0 * a_val);
        std::cout << "[solve_quadratic] Two solutions:\n";
        std::cout << "  x1 = " << x1 << "\n";
        std::cout << "  x2 = " << x2 << "\n";
        interp.stack.push_back(WofValue::make_double(x1));
        interp.stack.push_back(WofValue::make_double(x2));
    }
}

extern "C" WOFLANG_PLUGIN_EXPORT void register_plugin(WoflangInterpreter& interp) {
    interp.register_op("solve", [](WoflangInterpreter& ip) {
        try {
            op_solve(ip);
        } catch (const std::exception& e) {
            std::cout << "[solve] error: " << e.what() << "\n";
        }
    });

    interp.register_op("solve_quadratic", [](WoflangInterpreter& ip) {
        try {
            op_solve_quadratic(ip);
        } catch (const std::exception& e) {
            std::cout << "[solve_quadratic] error: " << e.what() << "\n";
        }
    });

    std::cout << "[symbolic_solve] Plugin loaded.\n";
}
