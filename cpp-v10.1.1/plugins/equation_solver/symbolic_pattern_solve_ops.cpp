// symbolic_pattern_solve_ops.cpp - heuristic pattern-based equation solver
//
// This plugin provides a lightweight pattern solver that operates on
// *string* equations such as:
//   "2x + 3 = 7"
//   "x^2 - 5x + 6 = 0"
//
// It is intentionally simple and safe. For now it recognises a few common
// patterns and returns solutions as strings pushed onto the WofLang stack.
//
// Stack conventions:
//   pattern_solve   (equation-string -- solution-string)
//                    For quadratic equations, both roots are returned as a
//                    single string like "x = 2, x = 3".
//
// The implementation avoids WofValue::make_* helpers to stay compatible with
// the v10 core.

#include <cmath>
#include <cstdint>
#include <regex>
#include <stdexcept>
#include <string>
#include <variant>

#include "woflang.hpp"

#ifndef WOFLANG_PLUGIN_EXPORT
#define WOFLANG_PLUGIN_EXPORT extern "C"
#endif

using namespace woflang;

// ---- WofValue helpers -------------------------------------------------------

static void ensure_stack_size(WoflangInterpreter &ip,
                              std::size_t         needed,
                              const char         *op_name) {
    if (ip.stack.size() < needed) {
        throw std::runtime_error(std::string(op_name) + ": stack underflow");
    }
}

static WofValue pop_raw(WoflangInterpreter &ip, const char *op_name) {
    ensure_stack_size(ip, 1, op_name);
    WofValue v = ip.stack.back();
    ip.stack.pop_back();
    return v;
}

static WofValue make_string(const std::string &s) {
    WofValue out;
    out.type  = WofType::String;
    out.value = s;
    return out;
}

static void push_string(WoflangInterpreter &ip, const std::string &s) {
    ip.stack.push_back(make_string(s));
}

static std::string to_string_value(const WofValue &v, const char *op_name) {
    if (v.type == WofType::String) {
        return std::get<std::string>(v.value);
    }
    if (v.type == WofType::Integer) {
        return std::to_string(std::get<int64_t>(v.value));
    }
    if (v.type == WofType::Double) {
        return std::to_string(std::get<double>(v.value));
    }
    throw std::runtime_error(std::string(op_name) + ": expected string or numeric");
}

// ---- Pattern solvers --------------------------------------------------------

struct LinearSolution {
    bool   ok;
    double x;
};

struct QuadraticSolution {
    bool   ok;
    double x1;
    double x2;
    bool   complex_roots;
};

static LinearSolution solve_linear(double a, double b, double c) {
    // ax + b = c  => x = (c - b) / a
    if (std::abs(a) < 1e-12) {
        return {false, 0.0};
    }
    return {true, (c - b) / a};
}

static QuadraticSolution solve_quadratic(double a, double b, double c) {
    QuadraticSolution sol{false, 0.0, 0.0, false};
    if (std::abs(a) < 1e-12) {
        // Degenerates to linear
        auto lin = solve_linear(b, c, 0.0);
        if (lin.ok) {
            sol.ok = true;
            sol.x1 = lin.x;
            sol.x2 = lin.x;
        }
        return sol;
    }

    double disc = b * b - 4.0 * a * c;
    if (disc < 0.0) {
        sol.ok            = true;
        sol.complex_roots = true;
        double real = -b / (2.0 * a);
        double imag = std::sqrt(-disc) / (2.0 * a);
        sol.x1 = real; // store real part
        sol.x2 = imag; // store imag part
    } else {
        sol.ok  = true;
        double rdisc = std::sqrt(disc);
        sol.x1 = (-b + rdisc) / (2.0 * a);
        sol.x2 = (-b - rdisc) / (2.0 * a);
    }
    return sol;
}

// Try to match "ax + b = c" where a,b,c are doubles.
// Examples:
//   "2x + 3 = 7"
//   "x + 1 = 5"  (a defaults to 1)
//   "-3x-9=0"
static bool try_linear_pattern(const std::string &eq, std::string &out) {
    std::regex re(
        R"(^\s*([+-]?\d*(?:\.\d+)?)\s*x\s*([+-]?\s*\d+(?:\.\d+)?)\s*=\s*([+-]?\s*\d+(?:\.\d+)?)\s*$)",
        std::regex::ECMAScript);
    std::smatch m;
    if (!std::regex_match(eq, m, re) || m.size() != 4) {
        return false;
    }

    auto parse = [](const std::string &s) -> double {
        std::string t;
        for (char ch : s) {
            if (!std::isspace(static_cast<unsigned char>(ch))) t.push_back(ch);
        }
        if (t.empty() || t == "+" || t == "-") {
            return (t == "-") ? -1.0 : 1.0;
        }
        return std::stod(t);
    };

    double a = parse(m[1].str());
    double b = parse(m[2].str());
    double c = parse(m[3].str());

    auto sol = solve_linear(a, b, c);
    if (!sol.ok) return false;

    out = "x = " + std::to_string(sol.x);
    return true;
}

// Try to match "ax^2 + bx + c = 0"
// Coefficients may be omitted (1x^2, x^2, -x^2, etc).
static bool try_quadratic_pattern(const std::string &eq, std::string &out) {
    std::regex re(
        R"(^\s*([+-]?\d*(?:\.\d+)?)\s*x\^2\s*([+-]?\s*\d*(?:\.\d+)?)\s*x\s*([+-]?\s*\d+(?:\.\d+)?)\s*=\s*0\s*$)",
        std::regex::ECMAScript);
    std::smatch m;
    if (!std::regex_match(eq, m, re) || m.size() != 5) {
        return false;
    }

    auto parse = [](const std::string &s, bool allow_empty = true) -> double {
        std::string t;
        for (char ch : s) {
            if (!std::isspace(static_cast<unsigned char>(ch))) t.push_back(ch);
        }
        if (t.empty()) {
            if (allow_empty) return 1.0;
            return 0.0;
        }
        if (t == "+" || t == "-") {
            return (t == "-") ? -1.0 : 1.0;
        }
        return std::stod(t);
    };

    double a = parse(m[1].str(), true);
    double b = parse(m[2].str(), true);
    double c = parse(m[3].str(), false);

    auto sol = solve_quadratic(a, b, c);
    if (!sol.ok) return false;

    if (sol.complex_roots) {
        double real = sol.x1;
        double imag = sol.x2;
        out = "x = " + std::to_string(real) + " ± " + std::to_string(imag) + "i";
    } else {
        out = "x = " + std::to_string(sol.x1);
        if (std::abs(sol.x2 - sol.x1) > 1e-9) {
            out += ", x = " + std::to_string(sol.x2);
        }
    }
    return true;
}

// ---- Stack op ---------------------------------------------------------------

static void op_pattern_solve(WoflangInterpreter &ip) {
    const char *op = "pattern_solve";
    std::string eq = to_string_value(pop_raw(ip, op), op);

    std::string solution;
    bool matched = false;

    if (!matched) {
        matched = try_linear_pattern(eq, solution);
    }
    if (!matched) {
        matched = try_quadratic_pattern(eq, solution);
    }

    if (!matched) {
        solution =
            "pattern_solve: no recognised pattern in equation '" + eq +
            "'. Supported forms include ax + b = c and ax^2 + bx + c = 0.";
    }

    push_string(ip, solution);
}

// ---- Plugin entry point -----------------------------------------------------

WOFLANG_PLUGIN_EXPORT void register_plugin(WoflangInterpreter &interp) {
    interp.register_op("pattern_solve", [](WoflangInterpreter &ip) {
        op_pattern_solve(ip);
    });
}
