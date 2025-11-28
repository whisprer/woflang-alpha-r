// ===================================================
// markov_suggest_ops.cpp - basic markov autocomplete
// ===================================================

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
#include <iostream>
#include <cstdlib>

using woflang::WoflangInterpreter;

// Demo op: just prints a random math suggestion.
// (Does not modify stack, same as the original behaviour.)
static void op_suggest_math(WoflangInterpreter& interp) {
    (void)interp; // unused for now

    std::vector<std::string> sugg = {
        "Try: X X +",
        "Try: pi * radius radius *",
        "Try: a b + c +",
        "Try: X X *",
        "Try: sqrt Y"
    };

    if (sugg.empty()) {
        return;
    }

    int idx = std::rand() % static_cast<int>(sugg.size());
    std::cout << "[Markov Suggest] " << sugg[idx] << std::endl;
}

extern "C" WOFLANG_PLUGIN_EXPORT void register_plugin(WoflangInterpreter& interp) {
    interp.register_op("suggest_math", [](WoflangInterpreter& ip) {
        op_suggest_math(ip);
    });

    std::cout << "[markov_suggest_ops] Plugin loaded.\n";
}
