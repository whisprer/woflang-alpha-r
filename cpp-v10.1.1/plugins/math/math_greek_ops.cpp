// =================================================================
// math_greek_ops.cpp - All operations currently just log that they are unimplemented.
// Auto-generated Stub to ensure build success with the new Woflang core.
// =================================================================

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

    interp.register_op("PI", [](WoflangInterpreter& ip) {
        std::cout << "[math_greek_ops] op \"PI\" is not yet implemented."
                  << " Stack size: " << ip.stack.size() << "\n";
    });

    interp.register_op("delta", [](WoflangInterpreter& ip) {
        std::cout << "[math_greek_ops] op \"delta\" is not yet implemented."
                  << " Stack size: " << ip.stack.size() << "\n";
    });

    interp.register_op("empty", [](WoflangInterpreter& ip) {
        std::cout << "[math_greek_ops] op \"empty\" is not yet implemented."
                  << " Stack size: " << ip.stack.size() << "\n";
    });

    interp.register_op("inf", [](WoflangInterpreter& ip) {
        std::cout << "[math_greek_ops] op \"inf\" is not yet implemented."
                  << " Stack size: " << ip.stack.size() << "\n";
    });

    interp.register_op("infinity", [](WoflangInterpreter& ip) {
        std::cout << "[math_greek_ops] op \"infinity\" is not yet implemented."
                  << " Stack size: " << ip.stack.size() << "\n";
    });

    interp.register_op("product", [](WoflangInterpreter& ip) {
        std::cout << "[math_greek_ops] op \"product\" is not yet implemented."
                  << " Stack size: " << ip.stack.size() << "\n";
    });

    interp.register_op("sum", [](WoflangInterpreter& ip) {
        std::cout << "[math_greek_ops] op \"sum\" is not yet implemented."
                  << " Stack size: " << ip.stack.size() << "\n";
    });

    interp.register_op("void", [](WoflangInterpreter& ip) {
        std::cout << "[math_greek_ops] op \"void\" is not yet implemented."
                  << " Stack size: " << ip.stack.size() << "\n";
    });

    interp.register_op("Δ", [](WoflangInterpreter& ip) {
        std::cout << "[math_greek_ops] op \"Δ\" is not yet implemented."
                  << " Stack size: " << ip.stack.size() << "\n";
    });

    interp.register_op("Π", [](WoflangInterpreter& ip) {
        std::cout << "[math_greek_ops] op \"Π\" is not yet implemented."
                  << " Stack size: " << ip.stack.size() << "\n";
    });

    interp.register_op("Σ", [](WoflangInterpreter& ip) {
        std::cout << "[math_greek_ops] op \"Σ\" is not yet implemented."
                  << " Stack size: " << ip.stack.size() << "\n";
    });

    interp.register_op("π", [](WoflangInterpreter& ip) {
        std::cout << "[math_greek_ops] op \"π\" is not yet implemented."
                  << " Stack size: " << ip.stack.size() << "\n";
    });

    interp.register_op("∅", [](WoflangInterpreter& ip) {
        std::cout << "[math_greek_ops] op \"∅\" is not yet implemented."
                  << " Stack size: " << ip.stack.size() << "\n";
    });

    interp.register_op("√", [](WoflangInterpreter& ip) {
        std::cout << "[math_greek_ops] op \"√\" is not yet implemented."
                  << " Stack size: " << ip.stack.size() << "\n";
    });

    interp.register_op("∞", [](WoflangInterpreter& ip) {
        std::cout << "[math_greek_ops] op \"∞\" is not yet implemented."
                  << " Stack size: " << ip.stack.size() << "\n";
    });

}
