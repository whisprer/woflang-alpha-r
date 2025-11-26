// ============================================================
// markov_suuggest_math_ops.cpp - markov based math autocmplete
// ============================================================

#include <iostream>
#include <string>
#include <vector>
#include <random>
#include "woflang.hpp"

#ifndef WOFLANG_PLUGIN_EXPORT
# ifdef _WIN32
#  define WOFLANG_PLUGIN_EXPORT __declspec(dllexport)
# else
#  define WOFLANG_PLUGIN_EXPORT __attribute__((visibility("default")))
# endif
#endif

using woflang::WoflangInterpreter;

static std::mt19937& rng() {
    static std::mt19937 gen{std::random_device{}()};
    return gen;
}

extern "C" WOFLANG_PLUGIN_EXPORT void register_plugin(WoflangInterpreter& interp) {
    interp.register_op("markov_suggest", [](WoflangInterpreter& ip) {
        (void)ip;
        std::vector<std::string> suggestions = {
            "Try: X X +",
            "Try: pi * radius radius *",
            "Try: X X *",
            "Try: sum i = 1 to N"
        };
        if (suggestions.empty()) {
            return;
        }
        std::uniform_int_distribution<std::size_t> dist(0, suggestions.size() - 1);
        std::size_t idx = dist(rng());
        std::cout << "[Markov Suggestion] " << suggestions[idx] << "\n";
    });

    std::cout << "[wof_markov_math_suggestions] registered markov_suggest\n";
}
