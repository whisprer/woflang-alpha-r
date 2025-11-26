// ==================================================
// symbolic_linear_solve_ops.cpp - Linear Equation Solver (v10.1.1)
// ==================================================

#ifndef WOFLANG_PLUGIN_EXPORT
# ifdef _WIN32
#  define WOFLANG_PLUGIN_EXPORT __declspec(dllexport)
# else
#  define WOFLANG_PLUGIN_EXPORT __attribute__((visibility("default")))
# endif
#endif

#include "../../src/core/woflang.hpp"

#include <iostream>
#include <stdexcept>

using woflang::WoflangInterpreter;
using woflang::WofValue;
using woflang::WofType;

class SymbolicSolveLinearPlugin : public woflang::WoflangPlugin {
public:
    void register_ops(WoflangInterpreter& interp) override {
        // solve_linear: expects coeff, var, "=", rhs on stack
        // Solves: coeff * var = rhs => var = rhs / coeff
        interp.register_op("solve_linear", [](WoflangInterpreter& interp) {
            if (interp.stack.size() < 4) {
                std::cout << "[solve_linear] needs: coeff var '=' rhs\n";
                return;
            }

            auto rhs = interp.stack.back();
            interp.stack.pop_back();

            auto eq_op = interp.stack.back();
            interp.stack.pop_back();

            auto var = interp.stack.back();
            interp.stack.pop_back();

            auto coeff = interp.stack.back();
            interp.stack.pop_back();

            // Validate types
            if (!coeff.is_numeric()) {
                std::cout << "[solve_linear] coefficient must be numeric\n";
                interp.stack.push_back(coeff);
                interp.stack.push_back(var);
                interp.stack.push_back(eq_op);
                interp.stack.push_back(rhs);
                return;
            }

            if (var.type != WofType::Symbol) {
                std::cout << "[solve_linear] variable must be a symbol\n";
                interp.stack.push_back(coeff);
                interp.stack.push_back(var);
                interp.stack.push_back(eq_op);
                interp.stack.push_back(rhs);
                return;
            }

            if (eq_op.type != WofType::Symbol ||
                std::get<std::string>(eq_op.value) != "=") {
                std::cout << "[solve_linear] expected '=' operator\n";
                interp.stack.push_back(coeff);
                interp.stack.push_back(var);
                interp.stack.push_back(eq_op);
                interp.stack.push_back(rhs);
                return;
            }

            if (!rhs.is_numeric()) {
                std::cout << "[solve_linear] RHS must be numeric\n";
                interp.stack.push_back(coeff);
                interp.stack.push_back(var);
                interp.stack.push_back(eq_op);
                interp.stack.push_back(rhs);
                return;
            }

            double coeff_val = coeff.as_numeric();
            double rhs_val = rhs.as_numeric();

            if (coeff_val == 0.0) {
                std::cout << "[solve_linear] coefficient cannot be zero\n";
                interp.stack.push_back(coeff);
                interp.stack.push_back(var);
                interp.stack.push_back(eq_op);
                interp.stack.push_back(rhs);
                return;
            }

            double solution = rhs_val / coeff_val;
            std::string var_name = std::get<std::string>(var.value);

            std::cout << "[solve_linear] " << coeff_val << " * " << var_name
                      << " = " << rhs_val << " => " << var_name << " = " << solution
                      << "\n";

            interp.stack.push_back(WofValue::make_double(solution));
        });

        std::cout << "[solve_linear] Plugin loaded.\n";
    }
};

extern "C" WOFLANG_PLUGIN_EXPORT void
register_plugin(WoflangInterpreter& interp) {
    static SymbolicSolveLinearPlugin plugin;
    plugin.register_ops(interp);
}
