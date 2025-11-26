// ==================================================
// stack_ops.cpp - Stack Benchmark Helper (v10.1.1 API)
// stubbed
// ==================================================

#ifndef WOFLANG_PLUGIN_EXPORT
# ifdef _WIN32
#  define WOFLANG_PLUGIN_EXPORT __declspec(dllexport)
# else
#  define WOFLANG_PLUGIN_EXPORT __attribute__((visibility("default")))
# endif
#endif

#include "../../src/core/woflang.hpp"

#include <iostream>

using woflang::WoflangInterpreter;

extern "C" WOFLANG_PLUGIN_EXPORT void
register_plugin(WoflangInterpreter& interp) {
    interp.register_op("stack_slayer", [](WoflangInterpreter&) {
        // No-op: used for benchmarking overhead
    });

    std::cout << "[stack] Stack helper plugin loaded (stack_slayer ready).\n";
}
