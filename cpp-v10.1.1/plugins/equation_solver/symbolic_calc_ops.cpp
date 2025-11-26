// ==================================================
// symbolic_calc_ops.cpp - Symbolic Calculus (v10.1.1 API)
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
#include <memory>
#include <string>
#include <stdexcept>

using woflang::WoflangInterpreter;
using woflang::WofValue;

namespace symbolic {

struct Expr {
    virtual std::string str() const = 0;
    virtual std::unique_ptr<Expr> diff(const std::string& var) const = 0;
    virtual std::unique_ptr<Expr> clone() const = 0;
    virtual ~Expr() = default;
};

struct Const : public Expr {
    double val;
    explicit Const(double v) : val(v) {}

    std::string str() const override {
        return std::to_string(val);
    }

    std::unique_ptr<Expr> diff(const std::string&) const override {
        return std::make_unique<Const>(0.0);
    }

    std::unique_ptr<Expr> clone() const override {
        return std::make_unique<Const>(val);
    }
};

struct Var : public Expr {
    std::string name;
    explicit Var(std::string n) : name(std::move(n)) {}

    std::string str() const override {
        return name;
    }

    std::unique_ptr<Expr> diff(const std::string& var) const override {
        // d/dx x = 1, d/dx y = 0 for y != x
        return std::make_unique<Const>(name == var ? 1.0 : 0.0);
    }

    std::unique_ptr<Expr> clone() const override {
        return std::make_unique<Var>(name);
    }
};

struct Add : public Expr {
    std::unique_ptr<Expr> lhs;
    std::unique_ptr<Expr> rhs;

    Add(std::unique_ptr<Expr> l, std::unique_ptr<Expr> r)
        : lhs(std::move(l)), rhs(std::move(r)) {}

    std::string str() const override {
        return "(" + lhs->str() + " + " + rhs->str() + ")";
    }

    std::unique_ptr<Expr> diff(const std::string& var) const override {
        return std::make_unique<Add>(lhs->diff(var), rhs->diff(var));
    }

    std::unique_ptr<Expr> clone() const override {
        return std::make_unique<Add>(lhs->clone(), rhs->clone());
    }
};

struct Mul : public Expr {
    std::unique_ptr<Expr> lhs;
    std::unique_ptr<Expr> rhs;

    Mul(std::unique_ptr<Expr> l, std::unique_ptr<Expr> r)
        : lhs(std::move(l)), rhs(std::move(r)) {}

    std::string str() const override {
        return "(" + lhs->str() + " * " + rhs->str() + ")";
    }

    std::unique_ptr<Expr> diff(const std::string& var) const override {
        // Product rule: (f*g)' = f'*g + f*g'
        auto f_prime = lhs->diff(var);
        auto g_prime = rhs->diff(var);

        return std::make_unique<Add>(
            std::make_unique<Mul>(std::move(f_prime), rhs->clone()),
            std::make_unique<Mul>(lhs->clone(), std::move(g_prime))
        );
    }

    std::unique_ptr<Expr> clone() const override {
        return std::make_unique<Mul>(lhs->clone(), rhs->clone());
    }
};

} // namespace symbolic

extern "C" WOFLANG_PLUGIN_EXPORT void register_plugin(WoflangInterpreter& interp) {
    using namespace woflang;

    interp.register_op("symbolic_diff", [](WoflangInterpreter& ip) {
        // Demo: differentiate x*x with respect to x
        auto expr = std::make_unique<symbolic::Mul>(
            std::make_unique<symbolic::Var>("x"),
            std::make_unique<symbolic::Var>("x")
        );
        auto deriv = expr->diff("x");

        std::cout << "\n[calculus] Expression: " << expr->str() << "\n";
        std::cout << "[calculus] Derivative: " << deriv->str() << "\n\n";

        // Push a dummy numeric to keep the stack discipline happy
        ip.stack.push_back(WofValue::make_double(1.0));
    });

    std::cout << "[calculus] Symbolic calculus plugin loaded.\n";
}
