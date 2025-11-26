// =======================================================
// prophecy_ops.cpp - [All operations currently just log that they are unimplemented.]
// Auto-generated Stub to ensure build success with the new Woflang core.
// =======================================================

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

    interp.register_op("oracle", [](WoflangInterpreter& ip) {
        std::cout << "[prophecy_ops] op \"oracle\" is not yet implemented."
                  << " Stack size: " << ip.stack.size() << "\n";
    });

    interp.register_op("prophecy", [](WoflangInterpreter& ip) {
        std::cout << "[prophecy_ops] op \"prophecy\" is not yet implemented."
                  << " Stack size: " << ip.stack.size() << "\n";
    });

}
