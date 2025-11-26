// ==================================================
// mirror_ops.cpp - Reverse Stack View Mode
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
#include <algorithm>
#include <iostream>

using woflang::WoflangInterpreter;
using woflang::WofValue;

namespace {
std::atomic<bool> mirror_mode{false};

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

    // Toggle mirror mode and physically reverse current stack
    interp.register_op(":mirror", [](WoflangInterpreter& ip) {
        WofStackAdapter S{ip.stack};
        bool now = !mirror_mode.load(std::memory_order_relaxed);
        mirror_mode.store(now, std::memory_order_relaxed);

        std::reverse(ip.stack.begin(), ip.stack.end());

        std::cout << "\n🪞 Reverse-stack mode "
                  << (now ? "enabled" : "disabled")
                  << ". Top and bottom have swapped stories.\n\n";

        // OLD: WofValue v{}; v.d = now ? 1.0 : 0.0;
        WofValue v = WofValue::make_double(now ? 1.0 : 0.0);
        S.push(v);
    });
}
