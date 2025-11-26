// ==================================================
// dreamlog_ops.cpp - Surreal Glyph Debug Stream
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
#include <exception>
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

static std::mt19937& rng() {
    static std::mt19937 gen(
        std::chrono::steady_clock::now().time_since_epoch().count()
    );
    return gen;
}
} // namespace

extern "C" WOFLANG_PLUGIN_EXPORT void
register_plugin(WoflangInterpreter& interp) {
    using namespace woflang;

    interp.register_op(":dreamlog", [](WoflangInterpreter& ip) {
        WofStackAdapter S{ip.stack};

        static const std::vector<std::string> glyphs = {
            "⟁", "◬", "𓂀", "₪", "⚚", "⌘", "☍", "⧖", "ᚠ", "ᚨ", "ᛟ"
        };
        static const std::vector<std::string> verbs = {
            "rearranges", "whispers to", "devours", "mirrors", "weaves through",
            "annotates", "relabels", "erases", "reflects inside"
        };

        std::uniform_int_distribution<> gdis(0, (int)glyphs.size() - 1);
        std::uniform_int_distribution<> vdis(0, (int)verbs.size() - 1);

        std::cout << "\n☁ Surreal Dreamlog Trace\n";
        std::cout << "----------------------------------------\n";

        for (int i = 0; i < 4; ++i) {
            const auto& g1 = glyphs[gdis(rng())];
            const auto& g2 = glyphs[gdis(rng())];
            const auto& v  = verbs[vdis(rng())];

            std::cout << "  " << g1 << "  " << v << "  " << g2 << "\n";
        }

        if (!S.empty()) {
            const auto& top = S.top();
            double approx = 0.0;
            try {
                approx = top.as_numeric();
            } catch (const std::exception&) {
                // non-numeric top-of-stack, leave approx = 0
            }

            std::cout << "\n  top-of-stack drifts as " << glyphs[gdis(rng())]
                      << " ≈ " << approx << "\n";
        }

        std::cout << "----------------------------------------\n\n";

        WofValue v = WofValue::make_double(0.0);
        S.push(v);
    });
}
