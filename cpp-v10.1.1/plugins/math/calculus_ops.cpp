// plugins/math/calculus_ops.cpp
//
// Numerical calculus helpers for WofLang v10.1.1
// Implements finite-difference derivative estimators and a basic slope op.
//
// Provided ops:
//   derivative_central   : [f(x+h) f(x-h) h] -> df/dx (central difference)
//   derivative_forward   : [f(x+h) f(x)   h] -> df/dx (forward difference)
//   derivative_backward  : [f(x)   f(x-h) h] -> df/dx (backward difference)
//   slope                : [y2 y1 x2 x1]     -> (y2 - y1) / (x2 - x1)
//
// Compatibility stubs (unchanged semantics):
//   derivative           : prints hint, no stack change
//   integral             : prints hint, no stack change

#ifndef WOFLANG_PLUGIN_EXPORT
#  ifdef _WIN32
#    define WOFLANG_PLUGIN_EXPORT __declspec(dllexport)
#  else
#    define WOFLANG_PLUGIN_EXPORT __attribute__((visibility("default")))
#  endif
#endif

#include "woflang.hpp"

#include <cmath>
#include <iostream>
#include <limits>
#include <string>
#include <cstdlib>
#include <cerrno>

using namespace woflang;

namespace {

double to_numeric(const WofValue& v, const char* ctx) {
    switch (v.type) {
        case WofType::Integer:
            return static_cast<double>(std::get<std::int64_t>(v.value));
        case WofType::Double:
            return std::get<double>(v.value);
        case WofType::String: {
            const std::string& s = std::get<std::string>(v.value);
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

WofValue make_num(double x) {
    WofValue out;
    out.type  = WofType::Double;
    out.value = x;
    return out;
}

} // namespace

WOFLANG_PLUGIN_EXPORT void register_plugin(WoflangInterpreter& interp) {
    // Central difference: df/dx ≈ (f(x+h) - f(x-h)) / (2h)
    // Stack: f(x+h) f(x-h) h
    interp.register_op("derivative_central", [](WoflangInterpreter& ip) {
        if (ip.stack.size() < 3) {
            std::cerr << "[calculus] derivative_central requires: f(x+h) f(x-h) h\n";
            return;
        }

        WofValue v_h  = ip.stack.back(); ip.stack.pop_back();
        WofValue v_fm = ip.stack.back(); ip.stack.pop_back();
        WofValue v_fp = ip.stack.back(); ip.stack.pop_back();

        double h  = to_numeric(v_h,  "[calculus] derivative_central h");
        double fp = to_numeric(v_fp, "[calculus] derivative_central f(x+h)");
        double fm = to_numeric(v_fm, "[calculus] derivative_central f(x-h)");

        if (std::abs(h) <= std::numeric_limits<double>::epsilon()) {
            std::cerr << "[calculus] derivative_central: h too small or zero\n";
            ip.stack.push_back(make_num(std::numeric_limits<double>::quiet_NaN()));
            return;
        }

        double deriv = (fp - fm) / (2.0 * h);
        std::cout << "[calculus] derivative_central -> " << deriv << "\n";
        ip.stack.push_back(make_num(deriv));
    });

    // Forward difference: df/dx ≈ (f(x+h) - f(x)) / h
    // Stack: f(x+h) f(x) h
    interp.register_op("derivative_forward", [](WoflangInterpreter& ip) {
        if (ip.stack.size() < 3) {
            std::cerr << "[calculus] derivative_forward requires: f(x+h) f(x) h\n";
            return;
        }

        WofValue v_h  = ip.stack.back(); ip.stack.pop_back();
        WofValue v_f0 = ip.stack.back(); ip.stack.pop_back();
        WofValue v_fp = ip.stack.back(); ip.stack.pop_back();

        double h  = to_numeric(v_h,  "[calculus] derivative_forward h");
        double f0 = to_numeric(v_f0, "[calculus] derivative_forward f(x)");
        double fp = to_numeric(v_fp, "[calculus] derivative_forward f(x+h)");

        if (std::abs(h) <= std::numeric_limits<double>::epsilon()) {
            std::cerr << "[calculus] derivative_forward: h too small or zero\n";
            ip.stack.push_back(make_num(std::numeric_limits<double>::quiet_NaN()));
            return;
        }

        double deriv = (fp - f0) / h;
        std::cout << "[calculus] derivative_forward -> " << deriv << "\n";
        ip.stack.push_back(make_num(deriv));
    });

    // Backward difference: df/dx ≈ (f(x) - f(x-h)) / h
    // Stack: f(x) f(x-h) h
    interp.register_op("derivative_backward", [](WoflangInterpreter& ip) {
        if (ip.stack.size() < 3) {
            std::cerr << "[calculus] derivative_backward requires: f(x) f(x-h) h\n";
            return;
        }

        WofValue v_h  = ip.stack.back(); ip.stack.pop_back();
        WofValue v_fm = ip.stack.back(); ip.stack.pop_back();
        WofValue v_f0 = ip.stack.back(); ip.stack.pop_back();

        double h  = to_numeric(v_h,  "[calculus] derivative_backward h");
        double fm = to_numeric(v_fm, "[calculus] derivative_backward f(x-h)");
        double f0 = to_numeric(v_f0, "[calculus] derivative_backward f(x)");

        if (std::abs(h) <= std::numeric_limits<double>::epsilon()) {
            std::cerr << "[calculus] derivative_backward: h too small or zero\n";
            ip.stack.push_back(make_num(std::numeric_limits<double>::quiet_NaN()));
            return;
        }

        double deriv = (f0 - fm) / h;
        std::cout << "[calculus] derivative_backward -> " << deriv << "\n";
        ip.stack.push_back(make_num(deriv));
    });

    // Simple secant slope: (y2 - y1) / (x2 - x1)
    // Stack: y2 y1 x2 x1
    interp.register_op("slope", [](WoflangInterpreter& ip) {
        if (ip.stack.size() < 4) {
            std::cerr << "[calculus] slope requires: y2 y1 x2 x1\n";
            return;
        }

        WofValue v_x1 = ip.stack.back(); ip.stack.pop_back();
        WofValue v_x2 = ip.stack.back(); ip.stack.pop_back();
        WofValue v_y1 = ip.stack.back(); ip.stack.pop_back();
        WofValue v_y2 = ip.stack.back(); ip.stack.pop_back();

        double x1 = to_numeric(v_x1, "[calculus] slope x1");
        double x2 = to_numeric(v_x2, "[calculus] slope x2");
        double y1 = to_numeric(v_y1, "[calculus] slope y1");
        double y2 = to_numeric(v_y2, "[calculus] slope y2");

        double dx = x2 - x1;
        if (std::abs(dx) <= std::numeric_limits<double>::epsilon()) {
            std::cerr << "[calculus] slope: x2 == x1, vertical line\n";
            ip.stack.push_back(make_num(std::numeric_limits<double>::infinity()));
            return;
        }

        double m = (y2 - y1) / dx;
        std::cout << "[calculus] slope -> " << m << "\n";
        ip.stack.push_back(make_num(m));
    });

    // ---- Compatibility stubs for v9 names ----

    interp.register_op("derivative", [](WoflangInterpreter&) {
        std::cout << "[calculus] derivative stub: "
                     "use derivative_central / derivative_forward / derivative_backward "
                     "with pre-evaluated samples f(x±h).\n";
    });

    interp.register_op("integral", [](WoflangInterpreter&) {
        std::cout << "[calculus] integral stub: "
                     "numeric integration not yet implemented. "
                     "Consider trapezoidal/Simpson rules over sample grids.\n";
    });

WOFLANG_PLUGIN_EXPORT void register_plugin(WoflangInterpreter& interp) {
    std::cout << "[calculus] Symbolic calculus plugin loaded.\n";
    auto plugin = std::make_unique<CalculusOpsPlugin>();
    plugin->register_ops(interp);
    interp.add_plugin(std::move(plugin));
}
