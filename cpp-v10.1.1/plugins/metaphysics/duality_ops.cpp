// ==================================================
// duality_ops.cpp - concrete dual semantics
// (Woflang v10.1.1 plugin)
// ==================================================

#ifndef WOFLANG_PLUGIN_EXPORT
# ifdef _WIN32
#  define WOFLANG_PLUGIN_EXPORT __declspec(dllexport)
# else
#  define WOFLANG_PLUGIN_EXPORT __attribute__((visibility("default")))
# endif
#endif

#include "woflang.hpp"

#include <algorithm>
#include <cctype>
#include <iostream>
#include <stdexcept>
#include <string>
#include <variant>
#include <cmath> 

using woflang::WoflangInterpreter;
using woflang::WofValue;
using woflang::WofType;

namespace {

bool g_duality_enabled = false;

double to_double(const WofValue& v, const char* ctx) {
    double d = v.as_numeric();
    if (!std::isfinite(d)) {
        throw std::runtime_error(std::string(ctx) + ": expected finite number");
    }
    return d;
}

std::int64_t to_int(const WofValue& v, const char* ctx) {
    return static_cast<std::int64_t>(to_double(v, ctx));
}

bool to_bool(const WofValue& v, const char* ctx) {
    double d = to_double(v, ctx);
    return d != 0.0;
}

std::string to_string_value(const WofValue& v, const char* ctx) {
    if (v.type == WofType::String) {
        return std::get<std::string>(v.value);
    }
    double d = v.as_numeric();
    if (!std::isfinite(d)) {
        throw std::runtime_error(std::string(ctx) + ": expected string or finite number");
    }
    return std::to_string(d);
}

// Lowercase copy (for simple token handling)
std::string to_lower_copy(std::string s) {
    std::transform(s.begin(), s.end(), s.begin(),
                   [](unsigned char c){ return static_cast<char>(std::tolower(c)); });
    return s;
}

// Simple textual duality: swap "and" <-> "or", "true" <-> "false" (case-insensitive)
std::string dualize_formula(const std::string& in) {
    std::string lower = to_lower_copy(in);
    std::string out   = in; // we'll modify in-place-ish using indices

    auto replace_token = [&](const std::string& token,
                             const std::string& replacement) {
        std::size_t pos = 0;
        while (true) {
            pos = lower.find(token, pos);
            if (pos == std::string::npos) break;

            bool left_ok = (pos == 0 ||
                            !std::isalnum(static_cast<unsigned char>(lower[pos - 1])));
            bool right_ok = (pos + token.size() >= lower.size() ||
                             !std::isalnum(static_cast<unsigned char>(lower[pos + token.size()])));

            if (left_ok && right_ok) {
                out.replace(pos, token.size(), replacement);
                lower.replace(pos, token.size(), replacement);
                pos += replacement.size();
            } else {
                pos += token.size();
            }
        }
    };

    // Use ordering that avoids accidental re-rewrites
    replace_token("true",  "##DUAL_TRUE##");
    replace_token("false", "##DUAL_FALSE##");
    replace_token("and",   "##DUAL_AND##");
    replace_token("or",    "##DUAL_OR##");

    replace_token("##DUAL_TRUE##",  "false");
    replace_token("##DUAL_FALSE##", "true");
    replace_token("##DUAL_AND##",   "or");
    replace_token("##DUAL_OR##",    "and");

    return out;
}

} // namespace

extern "C" WOFLANG_PLUGIN_EXPORT void register_plugin(WoflangInterpreter& interp) {
    // -------------------------------------------------------------
    // duality_on / duality_off / duality_toggle
    // -------------------------------------------------------------
    interp.register_op("duality_on", [](WoflangInterpreter&) {
        g_duality_enabled = true;
        std::cout << "☯️  Duality mode: ON\n";
    });

    interp.register_op("duality_off", [](WoflangInterpreter&) {
        g_duality_enabled = false;
        std::cout << "☯️  Duality mode: OFF\n";
    });

    interp.register_op("duality_toggle", [](WoflangInterpreter&) {
        g_duality_enabled = !g_duality_enabled;
        std::cout << "☯️  Duality mode toggled to: "
                  << (g_duality_enabled ? "ON" : "OFF") << "\n";
    });

    // Backwards-compatible name "duality" as a toggle
    interp.register_op("duality", [](WoflangInterpreter& ip) {
        (void)ip;
        g_duality_enabled = !g_duality_enabled;
        std::cout << "☯️  duality: mode is now "
                  << (g_duality_enabled ? "ON" : "OFF") << "\n";
    });

    // -------------------------------------------------------------
    // dual_add : a b -- (a+b) or (a-b)
    //
    // If duality OFF:    returns a + b
    // If duality ON:     returns a - b
    // -------------------------------------------------------------
    interp.register_op("dual_add", [](WoflangInterpreter& ip) {
        auto& S = ip.stack;
        if (S.size() < 2) {
            throw std::runtime_error("dual_add requires two numbers");
        }

        WofValue vb = S.back(); S.pop_back();
        WofValue va = S.back(); S.pop_back();

        double b = to_double(vb, "dual_add(b)");
        double a = to_double(va, "dual_add(a)");

        double res = g_duality_enabled ? (a - b) : (a + b);
        S.push_back(WofValue::make_double(res));
    });

    // -------------------------------------------------------------
    // dual_and : a b -- result
    //
    // If duality OFF: boolean AND
    // If duality ON:  boolean OR  (logical dual)
    // Inputs: non-zero => true, zero => false
    // -------------------------------------------------------------
    interp.register_op("dual_and", [](WoflangInterpreter& ip) {
        auto& S = ip.stack;
        if (S.size() < 2) {
            throw std::runtime_error("dual_and requires two booleans");
        }

        WofValue vb = S.back(); S.pop_back();
        WofValue va = S.back(); S.pop_back();

        bool b = to_bool(vb, "dual_and(b)");
        bool a = to_bool(va, "dual_and(a)");

        bool res;
        if (g_duality_enabled) {
            // dual: OR
            res = (a || b);
        } else {
            // normal: AND
            res = (a && b);
        }

        S.push_back(WofValue::make_int(res ? 1 : 0));
    });

    // -------------------------------------------------------------
    // dual_or : a b -- result
    //
    // If duality OFF: boolean OR
    // If duality ON:  boolean AND  (logical dual)
    // -------------------------------------------------------------
    interp.register_op("dual_or", [](WoflangInterpreter& ip) {
        auto& S = ip.stack;
        if (S.size() < 2) {
            throw std::runtime_error("dual_or requires two booleans");
        }

        WofValue vb = S.back(); S.pop_back();
        WofValue va = S.back(); S.pop_back();

        bool b = to_bool(vb, "dual_or(b)");
        bool a = to_bool(va, "dual_or(a)");

        bool res;
        if (g_duality_enabled) {
            // dual: AND
            res = (a && b);
        } else {
            // normal: OR
            res = (a || b);
        }

        S.push_back(WofValue::make_int(res ? 1 : 0));
    });

    // -------------------------------------------------------------
    // dual_not : a -- result
    //
    // Logical NOT; logs current duality mode for flavor.
    // -------------------------------------------------------------
    interp.register_op("dual_not", [](WoflangInterpreter& ip) {
        auto& S = ip.stack;
        if (S.empty()) {
            throw std::runtime_error("dual_not requires one boolean");
        }

        WofValue v = S.back(); S.pop_back();
        bool a = to_bool(v, "dual_not(a)");

        bool res = !a;
        std::cout << "☯️  dual_not (duality "
                  << (g_duality_enabled ? "ON" : "OFF")
                  << "): " << (a ? "true" : "false")
                  << " -> " << (res ? "true" : "false") << "\n";

        S.push_back(WofValue::make_int(res ? 1 : 0));
    });

    // -------------------------------------------------------------
    // dual_logic : formula_str -- dual_formula_str
    //
    // Textual dualization:
    //   - swaps "and" <-> "or"
    //   - swaps "true" <-> "false"
    // Case-insensitive token matching.
    // -------------------------------------------------------------
    interp.register_op("dual_logic", [](WoflangInterpreter& ip) {
        auto& S = ip.stack;
        if (S.empty()) {
            throw std::runtime_error("dual_logic requires a formula string");
        }

        WofValue v = S.back(); S.pop_back();
        std::string formula = to_string_value(v, "dual_logic");

        std::string dual = dualize_formula(formula);

        std::cout << "☯️  dual_logic: \"" << formula
                  << "\" -> \"" << dual << "\"\n";

        S.push_back(WofValue::make_string(dual));
    });

    std::cout << "[duality_ops] Duality plugin loaded (logical & numeric duals available).\n";
}
