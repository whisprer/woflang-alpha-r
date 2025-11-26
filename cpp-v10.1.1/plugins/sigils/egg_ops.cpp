// ==================================================
// egg_ops.cpp - Cryptic Glyph Haiku Plugin
// ==================================================

#ifndef WOFLANG_PLUGIN_EXPORT
# ifdef _WIN32
#  define WOFLANG_PLUGIN_EXPORT __declspec(dllexport)
# else
#  define WOFLANG_PLUGIN_EXPORT __attribute__((visibility("default")))
# endif
#endif

#include "woflang.hpp"

#include <vector>
#include <string>
#include <random>
#include <chrono>
#include <iostream>

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
} // namespace

extern "C" WOFLANG_PLUGIN_EXPORT void
register_plugin(WoflangInterpreter& interp) {
    using namespace woflang;

    interp.register_op(":egg", [](WoflangInterpreter& ip) {
        WofStackAdapter S{ip.stack};

        static const std::vector<std::vector<std::string>> haiku = {
            {
                "  𐌼  sigils whisper",
                "  ∴  under the heap’s cold moon",
                "  ₪  stacks dream of return"
            },
            {
                "  ☍  glyphs fall like snow",
                "  ⇌  pointers trace forgotten paths",
                "  𐌀  null sings quietly"
            },
            {
                "  ☯  void drinks all symbols",
                "  Ϟ  sparks of undefined dance",
                "  ◬  main never returns"
            },
            {
                "  𓂀  eye of the opcode",
                "  ʘ  watches spins of fate and ints",
                "  ⌘  breakpoints in the dark"
            }
        };

        static std::mt19937 gen(
            std::chrono::steady_clock::now().time_since_epoch().count()
        );
        std::uniform_int_distribution<> dis(0, static_cast<int>(haiku.size()) - 1);

        const auto& poem = haiku[dis(gen)];

        std::cout << "\n🥚 Cryptic Glyph Haiku:\n";
        for (const auto& line : poem) {
            std::cout << line << "\n";
        }
        std::cout << "\n";

        // Line count as numeric value
        WofValue v = WofValue::make_int(3);
        S.push(v);
    });
}
