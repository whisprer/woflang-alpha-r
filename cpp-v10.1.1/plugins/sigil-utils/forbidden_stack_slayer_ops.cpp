// =======================================================
// forbidden_stack_slayer_ops.cpp - special stack slaying
// =======================================================

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

static void op_stack_slayer(WoflangInterpreter& interp) {
    std::cout << "[Forbidden] You have slain the entire stack! (void consumes all...)\n";
    interp.clear_stack();
}

extern "C" WOFLANG_PLUGIN_EXPORT void register_plugin(WoflangInterpreter& interp) {
    interp.register_op("stack_slayer", [](WoflangInterpreter& ip) {
        op_stack_slayer(ip);
    });

    std::cout << "[forbidden_stack_slayer_ops] Plugin loaded.\n";
}
