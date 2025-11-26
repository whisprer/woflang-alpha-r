// ==================================================
// chaos_unlock_ops.cpp - Forbidden Glyph Unlocker
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
std::atomic<bool> chaos_unlocked{false};

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

    interp.register_op(":unlock", [](WoflangInterpreter& ip) {
        WofStackAdapter S{ip.stack};
        chaos_unlocked.store(true, std::memory_order_relaxed);

        std::cout << "\n⚡ Forbidden glyphs unlocked for this session.\n";
        std::cout << "   Use with reverence; the stack remembers.\n\n";

        WofValue v = WofValue::make_double(1.0);
        S.push(v);
    });

    // query: pushes 1.0 if chaos unlocked, else 0.0
    interp.register_op(":chaos?", [](WoflangInterpreter& ip) {
        WofStackAdapter S{ip.stack};
        bool on = chaos_unlocked.load(std::memory_order_relaxed);
        WofValue v = WofValue::make_double(on ? 1.0 : 0.0);
        S.push(v);
        std::cout << "[chaos] " << (on ? "unleashed\n" : "sleeping\n");
    });
}
