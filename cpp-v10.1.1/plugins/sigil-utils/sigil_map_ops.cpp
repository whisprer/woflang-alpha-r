// ======================================
// sigil_map_ops.cpp - what lies where?
// ======================================

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

static void op_sigil_map(WoflangInterpreter& /*interp*/) {
    std::cout << "Sacred Sigils:\n"
              << "prophecy (🔮): Cryptic stack fate message\n"
              << "stack_slayer (☠️): Destroys the stack (forbidden)\n"
              << "egg (🥚): Joy Easter Egg\n"
              << "glyph_prophecy (🜄): Secret, full-moon only\n"
              << "omega (Ω): Session ending\n"
              << "sigil_map (🗺️): This map\n";
}

extern "C" WOFLANG_PLUGIN_EXPORT void register_plugin(WoflangInterpreter& interp) {
    interp.register_op("sigil_map", [](WoflangInterpreter& ip) {
        op_sigil_map(ip);
    });

    std::cout << "[sigil_map_ops] Plugin loaded.\n";
}
