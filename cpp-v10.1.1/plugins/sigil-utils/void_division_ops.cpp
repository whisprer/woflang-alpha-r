// ==========================================================
// void_division_ops.cpp - Forbidden Mathematical Operations
// ==========================================================

#ifndef WOFLANG_PLUGIN_EXPORT
# ifdef _WIN32
#  define WOFLANG_PLUGIN_EXPORT __declspec(dllexport)
# else
#  define WOFLANG_PLUGIN_EXPORT __attribute__((visibility("default")))
# endif
#endif

#include "woflang.hpp"

#include <iostream>
#include <limits>
#include <cstdlib>

using woflang::WoflangInterpreter;
using woflang::WofValue;

namespace {
template <typename Container>
struct WofStackAdapter {
    Container& v;
    auto size() const { return v.size(); }
    bool empty() const { return v.empty(); }
    woflang::WofValue& top() { return v.back(); }
    const woflang::WofValue& top() const { return v.back(); }
    void pop() { v.pop_back(); }
    void push(const woflang::WofValue& x) { v.push_back(x); }
};
}

extern "C" WOFLANG_PLUGIN_EXPORT void register_plugin(WoflangInterpreter& interp) {
    using namespace woflang;

    interp.register_op("void_division", [](WoflangInterpreter& ip) {
        WofStackAdapter S{ip.stack};

        std::cout << "⚠️  FORBIDDEN OPERATION DETECTED ⚠️\n";
        std::cout << "Attempting to divide by the void...\n";

        if (S.size() < 2) {
            std::cout << "The void requires a sacrifice.\n";
            return;
        }

        auto divisor = S.top();
        S.pop();
        auto dividend = S.top();
        S.pop();

        std::cout << "Dividing " << dividend.as_numeric()
                  << " by the essence of nothingness...\n";

        // The void consumes all
        while (!S.empty()) {
            S.pop();
        }

        // But leaves behind infinity
        WofValue result = WofValue::make_double(std::numeric_limits<double>::infinity());
        S.push(result);

        std::cout << "The operation succeeds. Infinity remains.\n";
        std::cout << "You have gazed into the abyss.\n";
    });

    interp.register_op("/0", [](WoflangInterpreter& ip) {
        WofStackAdapter S{ip.stack};

        if (S.empty()) {
            std::cout << "Even the void requires something to consume.\n";
            return;
        }

        auto value = S.top();
        S.pop();

        double numeric_val = value.as_numeric();
        std::cout << "÷0: " << numeric_val << " → ∞\n";

        WofValue result = WofValue::make_double(std::numeric_limits<double>::infinity());
        S.push(result);
    });
}
