// ========================================================
// music.cpp - All operations currently just log that they are unimplemented.
// Auto-generated Stub to ensure build success with the new Woflang core.
// ========================================================

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

    interp.register_op("bpm", [](WoflangInterpreter& ip) {
        std::cout << "[music] op \"bpm\" is not yet implemented."
                  << " Stack size: " << ip.stack.size() << "\n";
    });

    interp.register_op("major", [](WoflangInterpreter& ip) {
        std::cout << "[music] op \"major\" is not yet implemented."
                  << " Stack size: " << ip.stack.size() << "\n";
    });

}
