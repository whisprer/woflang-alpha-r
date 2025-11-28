// ===================================================================
// stack_void_division_ops.cpp - beware division by zero in the stack
// ===================================================================

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

static void op_glyph_prophecy(WoflangInterpreter& /*interp*/) {
    std::cout << "[Forbidden] The encrypted glyph prophecy divides the stack void. "
                 "Beware division by zero!"
              << std::endl;
}

extern "C" WOFLANG_PLUGIN_EXPORT void register_plugin(WoflangInterpreter& interp) {
    interp.register_op("glyph_prophecy", [](WoflangInterpreter& ip) {
        op_glyph_prophecy(ip);
    });

    std::cout << "[stack_void_division_ops] Plugin loaded.\n";
}
