// ==================================================
// whitexmas_ops.cpp - Sigil Snow / Matrix Rain
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
#include <thread>
#include <iostream>

using woflang::WoflangInterpreter;

namespace {
static std::mt19937& rng() {
    static std::mt19937 gen(
        std::chrono::steady_clock::now().time_since_epoch().count()
    );
    return gen;
}

std::string random_sigil() {
    static const std::vector<std::string> sigils = {
        "⟁","◬","𓂀","₪","⚚","⌘","☍","✶","✺","✦","ᚠ","ᛟ"
    };
    std::uniform_int_distribution<> d(0, (int)sigils.size() - 1);
    return sigils[d(rng())];
}
} // namespace

extern "C" WOFLANG_PLUGIN_EXPORT void
register_plugin(WoflangInterpreter& interp) {
    using namespace woflang;

    interp.register_op(":whitexmas", [](WoflangInterpreter&) {
        std::cout << "\n❄ Sigil storm begins...\n\n";

        const int width  = 40;
        const int height = 16;

        for (int row = 0; row < height; ++row) {
            for (int col = 0; col < width; ++col) {
                if ((rng()() & 7) == 0) {
                    std::cout << random_sigil();
                } else {
                    std::cout << " ";
                }
            }
            std::cout << "\n";
            std::this_thread::sleep_for(std::chrono::milliseconds(60));
        }

        std::cout << "\nThe sigils melt back into the heap.\n\n";
    });
}
