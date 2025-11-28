// ==================================================
// symbolic_linear_solve_ops.cpp - Linear Equation Solver (v10.1.1)
// ==================================================

#include <iostream>
#include <stdexcept>
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

// Solve: coeff * var = rhs  =>  var = rhs / coeff
// Stack (top last): [coeff var "=" rhs]
static void op_symbolic_linear_solve(WoflangInterpreter& interp) {
    if (interp.stack.size() < 4) {
        std::cout << "[symbolic_linear_solve_ops] needs: coeff var '=' rhs\n";
        return;
    }

    // Pop in reverse order (top of stack last)
    WofValue rhs   = interp.stack.back(); interp.stack.pop_back();
    WofValue eq_op = interp.stack.back(); interp.stack.pop_back();
    WofValue var   = interp.stack.back(); interp.stack.pop_back();
    WofValue coeff = interp.stack.back(); interp.stack.pop_back();

    // Validate types
    if (!coeff.is_numeric()) {
        std::cout << "[symbolic_linear_solve_ops] coefficient must be numeric\n";
        interp.stack.push_back(coeff);
        interp.stack.push_back(var);
        interp.stack.push_back(eq_op);
        interp.stack.push_back(rhs);
        return;
    }

    if (var.type != WofType::Symbol) {
        std::cout << "[symbolic_linear_solve_ops] variable must be a symbol\n";
        interp.stack.push_back(coeff);
        interp.stack.push_back(var);
        interp.stack.push_back(eq_op);
        interp.stack.push_back(rhs);
        return;
    }

    if (eq_op.type != WofType::Symbol ||
        std::get<std::string>(eq_op.value) != "=") {
        std::cout << "[symbolic_linear_solve_ops] expected '=' operator\n";
        interp.stack.push_back(coeff);
        interp.stack.push_back(var);
        interp.stack.push_back(eq_op);
        interp.stack.push_back(rhs);
        return;
    }

    if (!rhs.is_numeric()) {
        std::cout << "[symbolic_linear_solve_ops] RHS must be numeric\n";
        interp.stack.push_back(coeff);
        interp.stack.push_back(var);
        interp.stack.push_back(eq_op);
        interp.stack.push_back(rhs);
        return;
    }

    double coeff_val = coeff.as_numeric();
    double rhs_val   = rhs.as_numeric();

    if (coeff_val == 0.0) {
        std::cout << "[symbolic_linear_solve_ops] coefficient cannot be zero\n";
        interp.stack.push_back(coeff);
        interp.stack.push_back(var);
        interp.stack.push_back(eq_op);
        interp.stack.push_back(rhs);
        return;
    }

    double solution      = rhs_val / coeff_val;
    std::string var_name = std::get<std::string>(var.value);

    std::cout << "[symbolic_linear_solve_ops] "
              << coeff_val << " * " << var_name
              << " = " << rhs_val
              << " => " << var_name << " = " << solution
              << "\n";

    // Push numeric solution back to stack
    interp.stack.push_back(WofValue::make_double(solution));
}

extern "C" WOFLANG_PLUGIN_EXPORT void register_plugin(WoflangInterpreter& interp) {
    interp.register_op("symbolic_linear_solve_ops", [](WoflangInterpreter& ip) {
        try {
            op_symbolic_linear_solve(ip);
        } catch (const std::exception& e) {
            std::cout << "[symbolic_linear_solve_ops] error: " << e.what() << "\n";
        }
    });

    std::cout << "[symbolic_linear_solve_ops] Plugin loaded.\n";
}
