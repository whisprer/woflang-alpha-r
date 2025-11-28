// =====================================================
// prophecy_chain_ops.cpp - chain triggers prophecy egg!
// =====================================================

#ifndef WOFLANG_PLUGIN_EXPORT
# ifdef _WIN32
#  define WOFLANG_PLUGIN_EXPORT __declspec(dllexport)
# else
#  define WOFLANG_PLUGIN_EXPORT __attribute__((visibility("default")))
# endif
#endif

#include "woflang.hpp"

#include <iostream>
#include <vector>
#include <string>
#include <cstdlib>

using woflang::WoflangInterpreter;

static std::vector<std::string> prophecy_chain;

static void op_prophecy(WoflangInterpreter& /*interp*/) {
    std::vector<std::string> prophecies = {
        "A stack unbalanced is a prophecy unfulfilled.",
        "Beware the glyph echoing twice.",
        "The void grows with each lost symbol."
    };

    if (prophecies.empty()) {
        return;
    }

    int idx = std::rand() % static_cast<int>(prophecies.size());
    std::string msg = prophecies[idx];
    prophecy_chain.push_back(msg);

    std::cout << "[Prophecy] " << msg << std::endl;
}

static void op_prophecy_chain(WoflangInterpreter& /*interp*/) {
    std::cout << "🔗  Prophecy Chain:\n";
    for (const auto& p : prophecy_chain) {
        std::cout << "  " << p << "\n";
    }
}

extern "C" WOFLANG_PLUGIN_EXPORT void register_plugin(WoflangInterpreter& interp) {
    interp.register_op("prophecy", [](WoflangInterpreter& ip) {
        op_prophecy(ip);
    });

    interp.register_op("prophecy_chain", [](WoflangInterpreter& ip) {
        op_prophecy_chain(ip);
    });

    std::cout << "[prophecy_chain_ops] Plugin loaded.\n";
}
