// ==================================================
// glitchmode_ops.cpp - Fun Glyph Glitch Mode
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
#include <random>
#include <chrono>
#include <iostream>

using woflang::WoflangInterpreter;
using woflang::WofValue;

namespace {
std::atomic<bool> glitch_mode{false};

static std::mt19937& rng() {
    static std::mt19937 gen(
        std::chrono::steady_clock::now().time_since_epoch().count()
    );
    return gen;
}

char random_glyph_char() {
    static const char* glyphs = "!@#$%^&*+=?/\\|~";
    std::uniform_int_distribution<> d(0, 14);
    return glyphs[d(rng())];
}
} // namespace

extern "C" WOFLANG_PLUGIN_EXPORT void
register_plugin(WoflangInterpreter& interp) {
    using namespace woflang;

    interp.register_op(":glitchmode", [](WoflangInterpreter&) {
        glitch_mode.store(!glitch_mode.load(std::memory_order_relaxed),
                          std::memory_order_relaxed);

        bool now = glitch_mode.load(std::memory_order_relaxed);
        std::cout << "\n⚠ Glitch mode "
                  << (now ? "ONLINE" : "OFFLINE")
                  << ". Random glyph substitutions "
                  << (now ? "may occur.\n\n" : "cease.\n\n");
    });

    // Optional: fun op that prints a glitched echo
    interp.register_op(":glitch-echo", [](WoflangInterpreter&) {
        if (!glitch_mode.load(std::memory_order_relaxed)) {
            std::cout << "(no glitches today)\n";
            return;
        }

        std::string base = "woflang glyph stream";
        for (char& c : base) {
            if (c != ' ' && (rng()() & 3) == 0) {
                c = random_glyph_char();
            }
        }
        std::cout << base << "\n";
    });
}
