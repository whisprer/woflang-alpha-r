// =================================================
// over_unity_ops.cpp - wait, is this an easter egg?
// =================================================

#ifndef WOFLANG_PLUGIN_EXPORT
# ifdef _WIN32
#  define WOFLANG_PLUGIN_EXPORT __declspec(dllexport)
# else
#  define WOFLANG_PLUGIN_EXPORT __attribute__((visibility("default")))
# endif
#endif

#include "woflang.hpp"
#include <iostream>

using woflang::WoflangInterpreter;

static void op_over_unity(WoflangInterpreter& interp) {
    (void)interp; // unused for now

    std::cout
        << "⚡  Over Unity! Energy out exceeds energy in. "
        << "Next op will be disabled... (just kidding, demo only)\n";
    // Optional: set a global "disable-next-op" flag (advanced extension)
}

extern "C" WOFLANG_PLUGIN_EXPORT void register_plugin(WoflangInterpreter& interp) {
    interp.register_op("over_unity", [](WoflangInterpreter& ip) {
        op_over_unity(ip);
    });

    std::cout << "[over_unity_ops] Plugin loaded.\n";
}
