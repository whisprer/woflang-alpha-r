// gradient_hessian_ops.cpp
// 2D numerical gradient & Hessian via central finite differences for WofLang v10.x

#include <cmath>
#include <stdexcept>
#include <string>
#include <cerrno>

#include "woflang.hpp"

using namespace woflang;

#ifndef WOFLANG_PLUGIN_EXPORT
# ifdef _WIN32
#  define WOFLANG_PLUGIN_EXPORT extern "C" __declspec(dllexport)
# else
#  define WOFLANG_PLUGIN_EXPORT extern "C"
# endif
#endif

namespace {

double to_numeric(const WofValue& v, const char* ctx) {
    switch (v.type) {
        case WofType::Integer:
            return static_cast<double>(std::get<std::int64_t>(v.value));
        case WofType::Double:
            return std::get<double>(v.value);
        case WofType::String: {
            const auto& s = std::get<std::string>(v.value);
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

double pop_numeric(WoflangInterpreter& interp, const char* op_name, const char* what) {
    if (interp.stack.empty()) {
        throw std::runtime_error(std::string(op_name) + ": stack underflow while popping " + what);
    }
    WofValue v = interp.stack.back();
    interp.stack.pop_back();
    return to_numeric(v, op_name);
}

WofValue make_double(double x) {
    WofValue v;
    v.type  = WofType::Double;
    v.value = x;
    return v;
}

// grad2_central:
//   in (bottom -> top):
//     f(x-h,y), f(x+h,y), f(x,y-h), f(x,y+h), h
//   out: grad_x, grad_y
void op_grad2_central(WoflangInterpreter& interp) {
    constexpr const char* OP = "grad2_central";

    double h        = pop_numeric(interp, OP, "step h");
    double f_x_y_ph = pop_numeric(interp, OP, "f(x, y+h)");
    double f_x_y_mh = pop_numeric(interp, OP, "f(x, y-h)");
    double f_xph_y  = pop_numeric(interp, OP, "f(x+h, y)");
    double f_xmh_y  = pop_numeric(interp, OP, "f(x-h, y)");

    if (h == 0.0) {
        throw std::runtime_error(std::string(OP) + ": step h must be non-zero");
    }

    double gx = (f_xph_y  - f_xmh_y)  / (2.0 * h);
    double gy = (f_x_y_ph - f_x_y_mh) / (2.0 * h);

    interp.stack.push_back(make_double(gx));
    interp.stack.push_back(make_double(gy));
}

// hess2_central:
//   in (bottom -> top):
//     f(x-h,y-h),
//     f(x-h,y),
//     f(x-h,y+h),
//     f(x,y-h),
//     f(x,y),
//     f(x,y+h),
//     f(x+h,y-h),
//     f(x+h,y),
//     f(x+h,y+h),
//     h
//   out: f_xx, f_yy, f_xy
void op_hess2_central(WoflangInterpreter& interp) {
    constexpr const char* OP = "hess2_central";

    double h          = pop_numeric(interp, OP, "step h");
    double f_xph_y_ph = pop_numeric(interp, OP, "f(x+h, y+h)");
    double f_xph_y    = pop_numeric(interp, OP, "f(x+h, y)");
    double f_xph_y_mh = pop_numeric(interp, OP, "f(x+h, y-h)");
    double f_x_y_ph   = pop_numeric(interp, OP, "f(x, y+h)");
    double f_x_y      = pop_numeric(interp, OP, "f(x, y)");
    double f_x_y_mh   = pop_numeric(interp, OP, "f(x, y-h)");
    double f_xmh_y_ph = pop_numeric(interp, OP, "f(x-h, y+h)");
    double f_xmh_y    = pop_numeric(interp, OP, "f(x-h, y)");
    double f_xmh_y_mh = pop_numeric(interp, OP, "f(x-h, y-h)");

    if (h == 0.0) {
        throw std::runtime_error(std::string(OP) + ": step h must be non-zero");
    }

    double h2    = h * h;
    double invh2 = 1.0 / h2;

    double f_xx = (f_xph_y  - 2.0 * f_x_y + f_xmh_y ) * invh2;
    double f_yy = (f_x_y_ph - 2.0 * f_x_y + f_x_y_mh) * invh2;
    double f_xy = (f_xph_y_ph - f_xph_y_mh - f_xmh_y_ph + f_xmh_y_mh) / (4.0 * h2);

    interp.stack.push_back(make_double(f_xx));
    interp.stack.push_back(make_double(f_yy));
    interp.stack.push_back(make_double(f_xy));
}

} // namespace

WOFLANG_PLUGIN_EXPORT void register_plugin(WoflangInterpreter& interp) {
    interp.register_op("grad2_central", [](WoflangInterpreter& ip) {
        op_grad2_central(ip);
    });

    interp.register_op("hess2_central", [](WoflangInterpreter& ip) {
        op_hess2_central(ip);
    });
}
