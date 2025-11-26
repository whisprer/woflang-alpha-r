// ==================================================
// deity_ops.cpp - Unsafe Divine Recursion Mode
// ==================================================

#ifndef WOFLANG_PLUGIN_EXPORT
# ifdef _WIN32
#  define WOFLANG_PLUGIN_EXPORT __declspec(dllexport)
# else
#  define WOFLANG_PLUGIN_EXPORT __attribute__((visibility("default")))
# endif
#endif

#include "woflang.hpp"

#include <atomic>
#include <iostream>

using woflang::WoflangInterpreter;
using woflang::WofValue;

namespace {
std::atomic<bool> deity_mode{false};

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
} // namespace

extern "C" WOFLANG_PLUGIN_EXPORT void
register_plugin(WoflangInterpreter& interp) {
    using namespace woflang;

    interp.register_op(":deity", [](WoflangInterpreter& ip) {
        WofStackAdapter S{ip.stack};
        bool now = !deity_mode.load(std::memory_order_relaxed);
        deity_mode.store(now, std::memory_order_relaxed);

        std::cout << "\n👁  Deity mode "
                  << (now ? "ENABLED" : "DISABLED")
                  << ".\n";
        if (now) {
            std::cout << "    Recursion guards are ignored where possible.\n"
                         "    The call stack gazes back.\n\n";
        } else {
            std::cout << "    Mortal limits restored.\n\n";
        }

        // If you expose recursion limits in core, you’d toggle them here.

        WofValue v = WofValue::make_double(now ? 1.0 : 0.0);
        S.push(v);
    });
}
