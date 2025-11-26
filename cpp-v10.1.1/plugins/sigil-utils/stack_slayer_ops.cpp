// ==================================================
// stack_slayer_ops.cpp - Dramatic Stack Demolition (v10.1.1 API)
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
#include <chrono>
#include <thread>

using woflang::WoflangInterpreter;
using woflang::WofValue;

namespace {

template <typename Container>
struct WofStackAdapter {
    Container& v;
    auto size() const { return v.size(); }
    bool empty() const { return v.empty(); }
    WofValue& top() { return v.back(); }
    const WofValue& top() const { return v.back(); }
    void pop() { v.pop_back(); }
    void push(const WofValue& x) { v.push_back(x); }
};

} // namespace

extern "C" WOFLANG_PLUGIN_EXPORT void register_plugin(WoflangInterpreter& interp) {
    using namespace woflang;

    interp.register_op("stack_slayer", [](WoflangInterpreter& ip) {
        WofStackAdapter S{ip.stack};

        if (S.empty()) {
            std::cout << "⚔️  The Stack Slayer finds nothing to slay.\n";
            return;
        }

        std::cout << "⚔️  THE STACK SLAYER AWAKENS! ⚔️\n";
        std::this_thread::sleep_for(std::chrono::milliseconds(500));

        std::size_t victims = S.size();

        // Dramatically clear the stack with effect
        for (std::size_t i = 0; i < victims; ++i) {
            std::cout << "💀 ";
            std::this_thread::sleep_for(std::chrono::milliseconds(100));
        }

        std::cout << "\n";

        while (!S.empty()) {
            S.pop();
        }

        std::cout << "⚰️  The Stack Slayer has claimed "
                  << victims << " victims. The stack lies empty.\n\n";
    });

    interp.register_op("resurrect", [](WoflangInterpreter& ip) {
        WofStackAdapter S{ip.stack};

        std::cout << "✨ Attempting resurrection ritual...\n";

        // Resurrect with mystical constants
        S.push(WofValue::make_double(3.14159));   // π
        S.push(WofValue::make_double(2.71828));   // e
        S.push(WofValue::make_double(1.61803));   // φ (golden ratio)

        std::cout << "✨ Three sacred constants have risen from the void.\n";
        std::cout << "   π ≈ 3.14159\n";
        std::cout << "   e ≈ 2.71828\n";
        std::cout << "   φ ≈ 1.61803\n\n";
    });

    std::cout << "[slayer] Stack Slayer plugin loaded. ⚔️\n";
}
