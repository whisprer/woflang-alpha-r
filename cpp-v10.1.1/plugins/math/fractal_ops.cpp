// plugins/math/fractal_ops.cpp
//
// Fractal mathematics plugin for WofLang v10.1.1
// Implements:
//   - mandelbrot     : escape-time iteration count for a complex point
//   - sierpinski     : ASCII Sierpinski triangle (bitwise pattern)
//   - hausdorff_dim  : generic self-similar Hausdorff dimension
//
// Updated to v10 core API (no WoflangPlugin base, no as_numeric/make_double),
// behaviour preserved.

#ifndef WOFLANG_PLUGIN_EXPORT
# ifdef _WIN32
#  define WOFLANG_PLUGIN_EXPORT extern "C" __declspec(dllexport)
# else
#  define WOFLANG_PLUGIN_EXPORT extern "C"
# endif
#endif

#include "woflang.hpp"

#include <cmath>
#include <iostream>
#include <limits>
#include <string>
#include <cstdlib>
#include <cerrno>

using namespace woflang;

// ---------- Helpers ----------

// Compute Mandelbrot escape iterations for c = cr + i*ci.
static int mandelbrot_escape(double cr, double ci, int max_iter) {
    double zr = 0.0;
    double zi = 0.0;
    int iter  = 0;

    while (iter < max_iter) {
        double zr2 = zr * zr;
        double zi2 = zi * zi;

        if (zr2 + zi2 > 4.0) {
            break;
        }

        double new_zr = zr2 - zi2 + cr;
        double new_zi = 2.0 * zr * zi + ci;

        zr = new_zr;
        zi = new_zi;
        ++iter;
    }
    return iter;
}

// Print ASCII Sierpinski triangle.
static void print_sierpinski(int depth) {
    if (depth < 1) depth = 1;
    if (depth > 8) depth = 8; // avoid insane output

    const int size = 1 << depth;
    std::cout << "[fractal_ops] Sierpinski triangle (depth " << depth << ")\n";
    for (int y = 0; y < size; ++y) {
        // crude centering
        for (int s = 0; s < size - y; ++s) {
            std::cout << ' ';
        }
        for (int x = 0; x < size; ++x) {
            if ((x & y) == 0) std::cout << '*';
            else              std::cout << ' ';
        }
        std::cout << '\n';
    }
}

// Hausdorff/self-similar dimension D = log(N) / log(scale).
static double hausdorff_dimension(double n, double scale) {
    if (n <= 0.0 || scale <= 0.0 || scale == 1.0) {
        return std::numeric_limits<double>::quiet_NaN();
    }
    return std::log(n) / std::log(scale);
}

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

// ---------- Plugin entry ----------

WOFLANG_PLUGIN_EXPORT void register_plugin(WoflangInterpreter& interp) {
    // mandelbrot: (real imag max_iter) -> iterations
    interp.register_op("mandelbrot", [](WoflangInterpreter& ip) {
        if (ip.stack.size() < 3) {
            std::cerr << "[fractal_ops] mandelbrot requires: real imag max_iter\n";
            return;
        }

        WofValue v_max = ip.stack.back(); ip.stack.pop_back();
        WofValue v_im  = ip.stack.back(); ip.stack.pop_back();
        WofValue v_re  = ip.stack.back(); ip.stack.pop_back();

        double max_d = to_numeric(v_max, "[fractal_ops] mandelbrot max_iter");
        if (max_d < 1.0)      max_d = 1.0;
        if (max_d > 10000.0)  max_d = 10000.0; // sanity clamp
        int max_iter = static_cast<int>(max_d);

        double imag = to_numeric(v_im, "[fractal_ops] mandelbrot imag");
        double real = to_numeric(v_re, "[fractal_ops] mandelbrot real");

        int iters = mandelbrot_escape(real, imag, max_iter);

        std::cout << "[fractal_ops] mandelbrot("
                  << real << " + " << imag << "i, max_iter=" << max_iter
                  << ") -> iters=" << iters
                  << (iters == max_iter ? " (likely in set)\n" : " (escaped)\n");

        ip.stack.push_back(make_num(static_cast<double>(iters)));
    });

    // sierpinski: (depth:int) -> ()
    interp.register_op("sierpinski", [](WoflangInterpreter& ip) {
        if (ip.stack.empty()) {
            std::cerr << "[fractal_ops] sierpinski requires: depth\n";
            return;
        }
        WofValue v_depth = ip.stack.back();
        ip.stack.pop_back();

        int depth = static_cast<int>(to_numeric(v_depth, "[fractal_ops] sierpinski depth"));
        print_sierpinski(depth);
    });

    // hausdorff_dim: (N, scale) -> dimension
    interp.register_op("hausdorff_dim", [](WoflangInterpreter& ip) {
        if (ip.stack.size() < 2) {
            std::cerr << "[fractal_ops] hausdorff_dim requires: N scale\n";
            return;
        }

        WofValue v_scale = ip.stack.back(); ip.stack.pop_back();
        WofValue v_n     = ip.stack.back(); ip.stack.pop_back();

        double scale = to_numeric(v_scale, "[fractal_ops] hausdorff_dim scale");
        double n     = to_numeric(v_n,     "[fractal_ops] hausdorff_dim N");

        double d = hausdorff_dimension(n, scale);

        std::cout << "[fractal_ops] hausdorff_dim(N=" << n
                  << ", scale=" << scale << ") = " << d << "\n";

        ip.stack.push_back(make_num(d));
    });

    std::cout << "[fractal_ops] Fractal mathematics plugin loaded.\n";
}
