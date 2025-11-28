// geom_transform_ops.cpp
// 2D geometric transforms and coordinate conversions for WofLang v10.x

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

constexpr double PI = 3.14159265358979323846;

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

// translate2d:
//   in:  x, y, dx, dy
//   out: x', y' = x+dx, y+dy
void op_translate2d(WoflangInterpreter& interp) {
    constexpr const char* OP = "translate2d";

    double dy = pop_numeric(interp, OP, "dy");
    double dx = pop_numeric(interp, OP, "dx");
    double y  = pop_numeric(interp, OP, "y");
    double x  = pop_numeric(interp, OP, "x");

    double x_new = x + dx;
    double y_new = y + dy;

    interp.stack.push_back(make_double(x_new));
    interp.stack.push_back(make_double(y_new));
}

// scale2d:
//   in:  x, y, sx, sy
//   out: x', y' = x*sx, y*sy
void op_scale2d(WoflangInterpreter& interp) {
    constexpr const char* OP = "scale2d";

    double sy = pop_numeric(interp, OP, "sy");
    double sx = pop_numeric(interp, OP, "sx");
    double y  = pop_numeric(interp, OP, "y");
    double x  = pop_numeric(interp, OP, "x");

    double x_new = x * sx;
    double y_new = y * sy;

    interp.stack.push_back(make_double(x_new));
    interp.stack.push_back(make_double(y_new));
}

// rotate2d_rad:
//   in:  x, y, theta_rad
//   out: x', y'
void op_rotate2d_rad(WoflangInterpreter& interp) {
    constexpr const char* OP = "rotate2d_rad";

    double theta = pop_numeric(interp, OP, "theta_rad");
    double y     = pop_numeric(interp, OP, "y");
    double x     = pop_numeric(interp, OP, "x");

    double c = std::cos(theta);
    double s = std::sin(theta);

    double x_new = x * c - y * s;
    double y_new = x * s + y * c;

    interp.stack.push_back(make_double(x_new));
    interp.stack.push_back(make_double(y_new));
}

// rotate2d_deg:
//   in:  x, y, theta_deg
//   out: x', y'
void op_rotate2d_deg(WoflangInterpreter& interp) {
    constexpr const char* OP = "rotate2d_deg";

    double theta_deg = pop_numeric(interp, OP, "theta_deg");
    double y         = pop_numeric(interp, OP, "y");
    double x         = pop_numeric(interp, OP, "x");

    double theta = theta_deg * (PI / 180.0);

    double c = std::cos(theta);
    double s = std::sin(theta);

    double x_new = x * c - y * s;
    double y_new = x * s + y * c;

    interp.stack.push_back(make_double(x_new));
    interp.stack.push_back(make_double(y_new));
}

// cart_to_polar:
//   in:  x, y
//   out: r, theta_rad
void op_cart_to_polar(WoflangInterpreter& interp) {
    constexpr const char* OP = "cart_to_polar";

    double y = pop_numeric(interp, OP, "y");
    double x = pop_numeric(interp, OP, "x");

    double r     = std::hypot(x, y);
    double theta = std::atan2(y, x);

    interp.stack.push_back(make_double(r));
    interp.stack.push_back(make_double(theta));
}

// polar_to_cart:
//   in:  r, theta_rad
//   out: x, y
void op_polar_to_cart(WoflangInterpreter& interp) {
    constexpr const char* OP = "polar_to_cart";

    double theta = pop_numeric(interp, OP, "theta_rad");
    double r     = pop_numeric(interp, OP, "r");

    double x = r * std::cos(theta);
    double y = r * std::sin(theta);

    interp.stack.push_back(make_double(x));
    interp.stack.push_back(make_double(y));
}

} // namespace

WOFLANG_PLUGIN_EXPORT void register_plugin(WoflangInterpreter& interp) {
    interp.register_op("translate2d",   [](WoflangInterpreter& ip) { op_translate2d(ip);   });
    interp.register_op("scale2d",       [](WoflangInterpreter& ip) { op_scale2d(ip);       });
    interp.register_op("rotate2d_rad",  [](WoflangInterpreter& ip) { op_rotate2d_rad(ip);  });
    interp.register_op("rotate2d_deg",  [](WoflangInterpreter& ip) { op_rotate2d_deg(ip);  });
    interp.register_op("cart_to_polar", [](WoflangInterpreter& ip) { op_cart_to_polar(ip); });
    interp.register_op("polar_to_cart", [](WoflangInterpreter& ip) { op_polar_to_cart(ip); });
}
