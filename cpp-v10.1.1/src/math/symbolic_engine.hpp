// ==================================================
// symbolic_engine.hpp - Symbolic Math Engine (v10.1.1 API)
// ==================================================

#pragma once

#include "../../src/core/woflang.hpp"

#include <vector>
#include <string>
#include <memory>
#include <unordered_map>
#include <optional>
#include <iostream>
#include <functional>

namespace woflang {

/**
 * @brief Unit representation for physical quantities
 */
class Unit {
public:
    std::string base_unit;
    std::unordered_map<std::string, int> dimensions;

    /// Default constructor
    Unit() = default;

    /// Constructor with base unit
    explicit Unit(const std::string& base) : base_unit(base) {
        dimensions[base] = 1;
    }

    /// Multiply two units
    Unit operator*(const Unit& other) const {
        Unit result;
        result.dimensions = dimensions;
        for (const auto& [dim, exp] : other.dimensions) {
            result.dimensions[dim] += exp;
        }
        // Cleanup zero exponents
        for (auto it = result.dimensions.begin(); it != result.dimensions.end();) {
            if (it->second == 0) {
                it = result.dimensions.erase(it);
            } else {
                ++it;
            }
        }
        result.format_base_unit();
        return result;
    }

    /// Divide two units
    Unit operator/(const Unit& other) const {
        Unit result;
        result.dimensions = dimensions;
        for (const auto& [dim, exp] : other.dimensions) {
            result.dimensions[dim] -= exp;
        }
        // Cleanup zero exponents
        for (auto it = result.dimensions.begin(); it != result.dimensions.end();) {
            if (it->second == 0) {
                it = result.dimensions.erase(it);
            } else {
                ++it;
            }
        }
        result.format_base_unit();
        return result;
    }

    /// Raise unit to a power
    Unit pow(int exponent) const {
        Unit result;
        for (const auto& [dim, exp] : dimensions) {
            result.dimensions[dim] = exp * exponent;
        }
        // Cleanup zero exponents
        for (auto it = result.dimensions.begin(); it != result.dimensions.end();) {
            if (it->second == 0) {
                it = result.dimensions.erase(it);
            } else {
                ++it;
            }
        }
        result.format_base_unit();
        return result;
    }

    [[nodiscard]] std::string to_string() const {
        return base_unit;
    }

private:
    void format_base_unit() {
        base_unit.clear();
        bool first = true;

        // Numerator (positive exponents)
        for (const auto& [dim, exp] : dimensions) {
            if (exp > 0) {
                if (!first) base_unit += "·";
                base_unit += dim;
                if (exp > 1) base_unit += "^" + std::to_string(exp);
                first = false;
            }
        }

        // If no positive exponents
        if (first) {
            base_unit = "1";
            first = false;
        }

        // Denominator (negative exponents)
        bool has_neg = false;
        for (const auto& [dim, exp] : dimensions) {
            if (exp < 0) {
                has_neg = true;
                break;
            }
        }

        if (has_neg) {
            base_unit += "/";
            first = true;
            for (const auto& [dim, exp] : dimensions) {
                if (exp < 0) {
                    if (!first) base_unit += "·";
                    base_unit += dim;
                    if (exp < -1) base_unit += "^" + std::to_string(-exp);
                    first = false;
                }
            }
        }
    }
};

/**
 * @brief Register symbolic math operations
 */
inline void register_symbolic_ops(WoflangInterpreter& interp) {
    // simplify: Symbolic simplification demo
    interp.register_op("simplify", [](WoflangInterpreter& ip) {
        if (ip.stack.empty()) {
            std::cout << "[simplify] Stack is empty\n";
            return;
        }

        auto val = ip.stack.back();
        ip.stack.pop_back();

        // Demo: just output what we got
        std::cout << "[simplify] Simplified: " << val.as_numeric() << "\n";
        ip.stack.push_back(val);
    });

    // solve_linear: Solve ax = b (expects a, b on stack; pops and solves)
    interp.register_op("solve_linear", [](WoflangInterpreter& ip) {
        if (ip.stack.size() < 2) {
            std::cout << "[solve_linear] needs 2 values (a, b for ax=b)\n";
            return;
        }

        auto b = ip.stack.back();
        ip.stack.pop_back();
        auto a = ip.stack.back();
        ip.stack.pop_back();

        if (!a.is_numeric() || !b.is_numeric()) {
            std::cout << "[solve_linear] both values must be numeric\n";
            ip.stack.push_back(a);
            ip.stack.push_back(b);
            return;
        }

        double a_val = a.as_numeric();
        double b_val = b.as_numeric();

        if (a_val == 0.0) {
            std::cout << "[solve_linear] coefficient cannot be zero\n";
            ip.stack.push_back(a);
            ip.stack.push_back(b);
            return;
        }

        double x = b_val / a_val;
        std::cout << "[solve_linear] " << a_val << " * x = " << b_val
                  << " => x = " << x << "\n";

        ip.stack.push_back(WofValue::make_double(x));
    });

    // unit: Add unit metadata to a value
    interp.register_op("unit", [](WoflangInterpreter& ip) {
        if (ip.stack.empty()) {
            std::cout << "[unit] Stack empty\n";
            return;
        }

        auto val = ip.stack.back();
        ip.stack.pop_back();

        // For demo, just annotate the value
        std::cout << "[unit] Value " << val.as_numeric() << " marked with unit\n";
        ip.stack.push_back(val);
    });

    // mul_unit: Multiply two unit-marked values
    interp.register_op("mul_unit", [](WoflangInterpreter& ip) {
        if (ip.stack.size() < 2) {
            std::cout << "[mul_unit] needs 2 values\n";
            return;
        }

        auto b = ip.stack.back();
        ip.stack.pop_back();
        auto a = ip.stack.back();
        ip.stack.pop_back();

        if (!a.is_numeric() || !b.is_numeric()) {
            std::cout << "[mul_unit] both must be numeric\n";
            ip.stack.push_back(a);
            ip.stack.push_back(b);
            return;
        }

        double result = a.as_numeric() * b.as_numeric();
        std::cout << "[mul_unit] " << a.as_numeric() << " * " << b.as_numeric()
                  << " = " << result << "\n";

        ip.stack.push_back(WofValue::make_double(result));
    });

    // div_unit: Divide two unit-marked values
    interp.register_op("div_unit", [](WoflangInterpreter& ip) {
        if (ip.stack.size() < 2) {
            std::cout << "[div_unit] needs 2 values\n";
            return;
        }

        auto b = ip.stack.back();
        ip.stack.pop_back();
        auto a = ip.stack.back();
        ip.stack.pop_back();

        if (!a.is_numeric() || !b.is_numeric()) {
            std::cout << "[div_unit] both must be numeric\n";
            ip.stack.push_back(a);
            ip.stack.push_back(b);
            return;
        }

        double b_val = b.as_numeric();
        if (b_val == 0.0) {
            std::cout << "[div_unit] division by zero\n";
            ip.stack.push_back(a);
            ip.stack.push_back(b);
            return;
        }

        double result = a.as_numeric() / b_val;
        std::cout << "[div_unit] " << a.as_numeric() << " / " << b_val
                  << " = " << result << "\n";

        ip.stack.push_back(WofValue::make_double(result));
    });

    std::cout << "[symbolic] Symbolic engine registered.\n";
}

} // namespace woflang
