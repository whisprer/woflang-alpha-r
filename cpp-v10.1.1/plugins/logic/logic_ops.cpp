// =============================================================
// logic_ops.cpp - All operations currently just log that they are unimplemented.
// Auto-generated Stub to ensure build success with the new Woflang core.
// =============================================================

#include <iostream>
#include <string>
#include "woflang.hpp"

#ifndef WOFLANG_PLUGIN_EXPORT
# ifdef _WIN32
#  define WOFLANG_PLUGIN_EXPORT __declspec(dllexport)
# else
#  define WOFLANG_PLUGIN_EXPORT __attribute__((visibility("default")))
# endif
#endif

using woflang::WoflangInterpreter;

extern "C" WOFLANG_PLUGIN_EXPORT void register_plugin(WoflangInterpreter& interp) {

    interp.register_op("and", [](WoflangInterpreter& ip) {
        std::cout << "[logic_ops] op \"and\" is not yet implemented."
                  << " Stack size: " << ip.stack.size() << "\n";
    });

    interp.register_op("contradiction", [](WoflangInterpreter& ip) {
        std::cout << "[logic_ops] op \"contradiction\" is not yet implemented."
                  << " Stack size: " << ip.stack.size() << "\n";
    });

    interp.register_op("equivalent", [](WoflangInterpreter& ip) {
        std::cout << "[logic_ops] op \"equivalent\" is not yet implemented."
                  << " Stack size: " << ip.stack.size() << "\n";
    });

    interp.register_op("implies", [](WoflangInterpreter& ip) {
        std::cout << "[logic_ops] op \"implies\" is not yet implemented."
                  << " Stack size: " << ip.stack.size() << "\n";
    });

    interp.register_op("nand", [](WoflangInterpreter& ip) {
        std::cout << "[logic_ops] op \"nand\" is not yet implemented."
                  << " Stack size: " << ip.stack.size() << "\n";
    });

    interp.register_op("nor", [](WoflangInterpreter& ip) {
        std::cout << "[logic_ops] op \"nor\" is not yet implemented."
                  << " Stack size: " << ip.stack.size() << "\n";
    });

    interp.register_op("not", [](WoflangInterpreter& ip) {
        std::cout << "[logic_ops] op \"not\" is not yet implemented."
                  << " Stack size: " << ip.stack.size() << "\n";
    });

    interp.register_op("or", [](WoflangInterpreter& ip) {
        std::cout << "[logic_ops] op \"or\" is not yet implemented."
                  << " Stack size: " << ip.stack.size() << "\n";
    });

    interp.register_op("tautology", [](WoflangInterpreter& ip) {
        std::cout << "[logic_ops] op \"tautology\" is not yet implemented."
                  << " Stack size: " << ip.stack.size() << "\n";
    });

    interp.register_op("xor", [](WoflangInterpreter& ip) {
        std::cout << "[logic_ops] op \"xor\" is not yet implemented."
                  << " Stack size: " << ip.stack.size() << "\n";
    });

}
