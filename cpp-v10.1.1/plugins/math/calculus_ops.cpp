// plugins/math/calculus_ops.cpp
//
// Numerical calculus helpers for WofLang.
//
// Provides basic derivative and integral primitives that operate
// purely on numeric stack values, without needing any function
// objects or extra types.
//
// Stack conventions (top is rightmost):
//
//   slope:
//     [x1, y1, x2, y2] -> [m]
//       m = (y2 - y1) / (x2 - x1)
//
//   derivative_forward:
//     [f(x), f(x+h), h] -> [df/dx]
//       df/dx ≈ (f(x+h) - f(x)) / h
//
//   derivative_central:
//     [f(x-h), f(x+h), h] -> [df/dx]
//       df/dx ≈ (f(x+h) - f(x-h)) / (2h)
//
//   integral_trapezoid:
//     [a, b, n, f(x0), f(x1), ..., f(xn)] -> [approx]
//       a, b: integration bounds
//       n:    number of subintervals (integer, ≥ 1)
//       f(x0..xn): function samples at the n+1 grid points
//
//   integral_simpson:
//     [a, b, n, f(x0), f(x1), ..., f(xn)] -> [approx]
//       n must be even (Simpson's rule requirement).
//

#include "../../src/core/woflang.hpp"

#include <cmath>
#include <cstddef>
#include <iostream>
#include <stdexcept>
#include <vector>

using woflang::WofValue;
using woflang::WoflangInterpreter;

namespace {

// Helper to pull a numeric from a WofValue with context in error messages.
double to_double(const WofValue &v, const char *ctx) {
    try {
        return v.as_numeric();
    } catch (const std::exception &e) {
        throw std::runtime_error(std::string("[calculus] ") + ctx + ": " + e.what());
    }
}

// Pop from stack with bounds check.
WofValue pop_checked(std::vector<WofValue> &st, const char *ctx) {
    if (st.empty()) {
        throw std::runtime_error(std::string("[calculus] stack underflow in ") + ctx);
    }
    WofValue v = st.back();
    st.pop_back();
    return v;
}

// Ensure there are at least n items on stack.
void ensure_stack_size(const std::vector<WofValue> &st, std::size_t n, const char *ctx) {
    if (st.size() < n) {
        throw std::runtime_error(
            std::string("[calculus] need at least ") +
            std::to_string(n) +
            " stack values in " +
            ctx +
            ", have " +
            std::to_string(st.size())
        );
    }
}

// Numerical calculus plugin registering ops with the interpreter.
class MathlibCalculusPlugin {
public:
    void register_ops(WoflangInterpreter &interp) {
        // Slope between two points: [x1, y1, x2, y2] -> [m]
        interp.register_op("slope", [](WoflangInterpreter &ip) {
            auto &st = ip.stack;
            ensure_stack_size(st, 4, "slope");

            WofValue y2v = pop_checked(st, "slope(y2)");
            WofValue x2v = pop_checked(st, "slope(x2)");
            WofValue y1v = pop_checked(st, "slope(y1)");
            WofValue x1v = pop_checked(st, "slope(x1)");

            double y2 = to_double(y2v, "slope(y2)");
            double x2 = to_double(x2v, "slope(x2)");
            double y1 = to_double(y1v, "slope(y1)");
            double x1 = to_double(x1v, "slope(x1)");

            double dx = x2 - x1;
            if (dx == 0.0) {
                throw std::runtime_error("[calculus] slope: x2 - x1 == 0");
            }

            double m = (y2 - y1) / dx;
            ip.stack.emplace_back(m);
        });

        // Forward difference derivative: [f(x), f(x+h), h] -> [df/dx]
        interp.register_op("derivative_forward", [](WoflangInterpreter &ip) {
            auto &st = ip.stack;
            ensure_stack_size(st, 3, "derivative_forward");

            WofValue hv      = pop_checked(st, "derivative_forward(h)");
            WofValue fxph_v  = pop_checked(st, "derivative_forward(fx+h)");
            WofValue fx_v    = pop_checked(st, "derivative_forward(fx)");

            double h    = to_double(hv, "derivative_forward(h)");
            double fxph = to_double(fxph_v, "derivative_forward(fx+h)");
            double fx   = to_double(fx_v, "derivative_forward(fx)");

            if (h == 0.0) {
                throw std::runtime_error("[calculus] derivative_forward: h == 0");
            }

            double d = (fxph - fx) / h;
            ip.stack.emplace_back(d);
        });

        // Central difference derivative: [f(x-h), f(x+h), h] -> [df/dx]
        interp.register_op("derivative_central", [](WoflangInterpreter &ip) {
            auto &st = ip.stack;
            ensure_stack_size(st, 3, "derivative_central");

            WofValue hv      = pop_checked(st, "derivative_central(h)");
            WofValue fxph_v  = pop_checked(st, "derivative_central(fx+h)");
            WofValue fxmh_v  = pop_checked(st, "derivative_central(fx-h)");

            double h    = to_double(hv, "derivative_central(h)");
            double fxph = to_double(fxph_v, "derivative_central(fx+h)");
            double fxmh = to_double(fxmh_v, "derivative_central(fx-h)");

            if (h == 0.0) {
                throw std::runtime_error("[calculus] derivative_central: h == 0");
            }

            double d = (fxph - fxmh) / (2.0 * h);
            ip.stack.emplace_back(d);
        });

        // Trapezoidal rule:
        //   [a, b, n, f(x0), f(x1), ..., f(xn)] -> [approx]
        interp.register_op("integral_trapezoid", [](WoflangInterpreter &ip) {
            auto &st = ip.stack;
            ensure_stack_size(st, 3, "integral_trapezoid");

            // First pop n, then b, then a (reverse of push order).
            WofValue nv = pop_checked(st, "integral_trapezoid(n)");
            WofValue bv = pop_checked(st, "integral_trapezoid(b)");
            WofValue av = pop_checked(st, "integral_trapezoid(a)");

            double n_d = to_double(nv, "integral_trapezoid(n)");
            long long n_ll = static_cast<long long>(std::llround(n_d));
            if (n_ll <= 0) {
                throw std::runtime_error("[calculus] integral_trapezoid: n must be >= 1");
            }

            double a = to_double(av, "integral_trapezoid(a)");
            double b = to_double(bv, "integral_trapezoid(b)");

            std::size_t n = static_cast<std::size_t>(n_ll);

            // Now need n+1 samples f(x0..xn) still on stack.
            ensure_stack_size(st, n + 1, "integral_trapezoid(samples)");

            // Samples are assumed pushed in order x0..xn, so x_n is on top.
            std::vector<double> f(n + 1);
            for (std::size_t i = 0; i < n + 1; ++i) {
                WofValue fv = pop_checked(st, "integral_trapezoid(sample)");
                f[n - i] = to_double(fv, "integral_trapezoid(sample)"); // reverse
            }

            double h = (b - a) / static_cast<double>(n);
            double sum = 0.0;
            for (std::size_t i = 1; i < n; ++i) {
                sum += f[i];
            }

            double approx = h * (0.5 * f[0] + sum + 0.5 * f[n]);
            ip.stack.emplace_back(approx);
        });

        // Simpson's rule:
        //   [a, b, n, f(x0), f(x1), ..., f(xn)] -> [approx], n even
        interp.register_op("integral_simpson", [](WoflangInterpreter &ip) {
            auto &st = ip.stack;
            ensure_stack_size(st, 3, "integral_simpson");

            WofValue nv = pop_checked(st, "integral_simpson(n)");
            WofValue bv = pop_checked(st, "integral_simpson(b)");
            WofValue av = pop_checked(st, "integral_simpson(a)");

            double n_d = to_double(nv, "integral_simpson(n)");
            long long n_ll = static_cast<long long>(std::llround(n_d));
            if (n_ll <= 0 || (n_ll % 2) != 0) {
                throw std::runtime_error("[calculus] integral_simpson: n must be even and >= 2");
            }

            double a = to_double(av, "integral_simpson(a)");
            double b = to_double(bv, "integral_simpson(b)");

            std::size_t n = static_cast<std::size_t>(n_ll);

            ensure_stack_size(st, n + 1, "integral_simpson(samples)");

            std::vector<double> f(n + 1);
            for (std::size_t i = 0; i < n + 1; ++i) {
                WofValue fv = pop_checked(st, "integral_simpson(sample)");
                f[n - i] = to_double(fv, "integral_simpson(sample)");
            }

            double h = (b - a) / static_cast<double>(n);
            double sum_odd = 0.0;
            double sum_even = 0.0;

            for (std::size_t i = 1; i < n; ++i) {
                if (i % 2 == 0) {
                    sum_even += f[i];
                } else {
                    sum_odd += f[i];
                }
            }

            double approx = (h / 3.0) * (f[0] + f[n] + 4.0 * sum_odd + 2.0 * sum_even);
            ip.stack.emplace_back(approx);
        });

        std::cout << "[calculus] calculus_ops: registered slope/derivative/integral ops\n";
    }
};

} // anonymous namespace

// Exported entry point for the dynamic loader on all platforms.
// Must be unmangled C linkage so that GetProcAddress/dlsym("register_plugin")
// can find it.
extern "C" WOFLANG_PLUGIN_EXPORT void register_plugin(woflang::WoflangInterpreter& interp) {
    static MathlibCalculusPlugin plugin;
    plugin.register_ops(interp);
    std::cout << "[calculus] Numerical calculus plugin loaded.\n";
}
