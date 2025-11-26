// ==================================================
// expo_log_ops.cpp - Exponential & Log Operations (v10.1.1)
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
#include math>
#include <stdexcept>

using woflang::WoflangInterpreter;
using woflang::WofValue;

class MathlibExponentialsPlugin : public woflang::WoflangPlugin {
public:
    void register_ops(WoflangInterpreter& interp) override {
        interp.register_op("exp", [](WoflangInterpreter& interp) {
            if (interp.stack.empty()) {
                throw std::runtime_error("exp: stack underflow");
            }

            auto x = interp.stack.back();
            interp.stack.pop_back();

            if (!x.is_numeric()) {
                throw std::runtime_error("exp: expected numeric value");
            }

            double val = x.as_numeric();
            double result = std::exp(val);

            interp.stack.push_back(WofValue::make_double(result));
            std::cout << "[mathlib] exp(" << val << ") = " << result << "\n";
        });

        interp.register_op("ln", [](WoflangInterpreter& interp) {
            if (interp.stack.empty()) {
                throw std::runtime_error("ln: stack underflow");
            }

            auto x = interp.stack.back();
            interp.stack.pop_back();

            if (!x.is_numeric()) {
                throw std::runtime_error("ln: expected numeric value");
            }

            double val = x.as_numeric();
            if (val <= 0.0) {
                throw std::runtime_error("ln: domain error (x must be > 0)");
            }

            double result = std::log(val);
            interp.stack.push_back(WofValue::make_double(result));
            std::cout << "[mathlib] ln(" << val << ") = " << result << "\n";
        });

        interp.register_op("log10", [](WoflangInterpreter& interp) {
            if (interp.stack.empty()) {
                throw std::runtime_error("log10: stack underflow");
            }

            auto x = interp.stack.back();
            interp.stack.pop_back();

            if (!x.is_numeric()) {
                throw std::runtime_error("log10: expected numeric value");
            }

            double val = x.as_numeric();
            if (val <= 0.0) {
                throw std::runtime_error("log10: domain error (x must be > 0)");
            }

            double result = std::log10(val);
            interp.stack.push_back(WofValue::make_double(result));
            std::cout << "[mathlib] log10(" << val << ") = " << result << "\n";
        });

        interp.register_op("log2", [](WoflangInterpreter& interp) {
            if (interp.stack.empty()) {
                throw std::runtime_error("log2: stack underflow");
            }

            auto x = interp.stack.back();
            interp.stack.pop_back();

            if (!x.is_numeric()) {
                throw std::runtime_error("log2: expected numeric value");
            }

            double val = x.as_numeric();
            if (val <= 0.0) {
                throw std::runtime_error("log2: domain error (x must be > 0)");
            }

            double result = std::log2(val);
            interp.stack.push_back(WofValue::make_double(result));
            std::cout << "[mathlib] log2(" << val << ") = " << result << "\n";
        });

        std::cout << "[exponentials] Plugin loaded.\n";
    }
};

extern "C" WOFLANG_PLUGIN_EXPORT void
register_plugin(WoflangInterpreter& interp) {
    static MathlibExponentialsPlugin plugin;
    plugin.register_ops(interp);
}
