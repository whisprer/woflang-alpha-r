// plugins/sigils/prophecy_ops.cpp

#include "woflang.hpp"

#include <iostream>
#include <random>
#include <string>
#include <vector>

using namespace woflang;

#ifndef WOFLANG_PLUGIN_EXPORT
# ifdef _WIN32
#  define WOFLANG_PLUGIN_EXPORT extern "C" __declspec(dllexport)
# else
#  define WOFLANG_PLUGIN_EXPORT extern "C"
# endif
#endif

static std::mt19937 &prophecy_rng() {
    static std::mt19937 rng{std::random_device{}()};
    return rng;
}

static void op_prophecy(WoflangInterpreter &interp) {
    static const std::vector<std::string> prophecies = {
        "In the glyph’s shadow, your stack’s fate is sealed.",
        "Beware: the next push may tip the void.",
        "The stack will echo your intent, not your command.",
        "A silent glyph is the most powerful of all.",
        "When the top is light, the bottom bears the weight.",
        "Three swaps from now, a revelation will surface.",
        "Between ∅ and ∞, your next op chooses the path.",
        "The slayer sleeps… for now."
    };

    auto &rng = prophecy_rng();
    std::uniform_int_distribution<std::size_t> dist(0, prophecies.size() - 1);
    const std::string &chosen = prophecies[dist(rng)];

    std::cout << "[Prophecy] " << chosen << "\n";

    // Also push the prophecy text onto the stack as a String value
    WofValue v;
    v.type  = WofType::String;
    v.value = chosen;
    interp.stack.push_back(v);
}

WOFLANG_PLUGIN_EXPORT void register_plugin(WoflangInterpreter &interp) {
    interp.register_op("prophecy", op_prophecy);
    std::cout << "[prophecy_ops] Plugin loaded.\n";
}
