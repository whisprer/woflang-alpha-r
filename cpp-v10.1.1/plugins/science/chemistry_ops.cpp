// ============================================================
// chemistry_ops.cpp - All operations currently just log that they are unimplemented.
// Auto-generated Stub to ensure build success with the new Woflang core.
// ============================================================

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

    interp.register_op("atomic_weight", [](WoflangInterpreter& ip) {
        std::cout << "[chemistry_ops] op \"atomic_weight\" is not yet implemented."
                  << " Stack size: " << ip.stack.size() << "\n";
    });

    interp.register_op("avogadro", [](WoflangInterpreter& ip) {
        std::cout << "[chemistry_ops] op \"avogadro\" is not yet implemented."
                  << " Stack size: " << ip.stack.size() << "\n";
    });

    interp.register_op("element_info", [](WoflangInterpreter& ip) {
        std::cout << "[chemistry_ops] op \"element_info\" is not yet implemented."
                  << " Stack size: " << ip.stack.size() << "\n";
    });

    interp.register_op("pH_from_conc", [](WoflangInterpreter& ip) {
        std::cout << "[chemistry_ops] op \"pH_from_conc\" is not yet implemented."
                  << " Stack size: " << ip.stack.size() << "\n";
    });

}
