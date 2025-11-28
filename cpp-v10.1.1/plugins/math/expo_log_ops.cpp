// plugins/math/expo_log_ops.cpp
// Exponential & logarithmic operations for WofLang v10.x
//
// Provides:
//   exp(x)    : e^x
//   ln(x)     : natural log
//   log(x)    : alias of ln(x)
//   log10(x)  : base-10 log
//   log2(x)   : base-2 log
//
// Behaviour matches the original plugin, just updated to the v10 core API.

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
#include <stdexcept>
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
    // e^x
    interp.register_op("exp", [](WoflangInterpreter& ip) {
        if (ip.stack.empty()) {
            throw std::runtime_error("[exponentials] 'exp' requires one operand");
        }
        WofValue v = ip.stack.back();
        ip.stack.pop_back();

        double x      = to_numeric(v, "[exponentials] 'exp'");
        double result = std::exp(x);
        ip.stack.push_back(make_num(result));
    });

    // ln(x)
    interp.register_op("ln", [](WoflangInterpreter& ip) {
        if (ip.stack.empty()) {
            throw std::runtime_error("[exponentials] 'ln' requires one operand");
        }
        WofValue v = ip.stack.back();
        ip.stack.pop_back();

        double x = to_numeric(v, "[exponentials] 'ln'");
        if (x <= 0.0) {
            throw std::runtime_error("[exponentials] ln(x) domain error: x must be > 0");
        }
        double result = std::log(x);
        ip.stack.push_back(make_num(result));
    });

    // log(x) = ln(x)
    interp.register_op("log", [](WoflangInterpreter& ip) {
        if (ip.stack.empty()) {
            throw std::runtime_error("[exponentials] 'log' requires one operand");
        }
        WofValue v = ip.stack.back();
        ip.stack.pop_back();

        double x = to_numeric(v, "[exponentials] 'log'");
        if (x <= 0.0) {
            throw std::runtime_error("[exponentials] log(x) domain error: x must be > 0");
        }
        double result = std::log(x);
        ip.stack.push_back(make_num(result));
    });

    // log10(x)
    interp.register_op("log10", [](WoflangInterpreter& ip) {
        if (ip.stack.empty()) {
            throw std::runtime_error("[exponentials] 'log10' requires one operand");
        }
        WofValue v = ip.stack.back();
        ip.stack.pop_back();

        double x = to_numeric(v, "[exponentials] 'log10'");
        if (x <= 0.0) {
            throw std::runtime_error("[exponentials] log10(x) domain error: x must be > 0");
        }
        double result = std::log10(x);
        ip.stack.push_back(make_num(result));
    });

    // log2(x)
    interp.register_op("log2", [](WoflangInterpreter& ip) {
        if (ip.stack.empty()) {
            throw std::runtime_error("[exponentials] 'log2' requires one operand");
        }
        WofValue v = ip.stack.back();
        ip.stack.pop_back();

        double x = to_numeric(v, "[exponentials] 'log2'");
        if (x <= 0.0) {
            throw std::runtime_error("[exponentials] log2(x) domain error: x must be > 0");
        }
        double result = std::log2(x);
        ip.stack.push_back(make_num(result));
    });

    std::cout << "[exponentials] Plugin loaded.\n";
}
