// =======================================================================
// geom_transform_ops.cpp - All operations currently just log that they are unimplemented.
// Auto-generated Stub to ensure build success with the new Woflang core.
// =======================================================================

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

    interp.register_op("reflect_x", [](WoflangInterpreter& ip) {
        std::cout << "[geom_transform_ops] op \"reflect_x\" is not yet implemented."
                  << " Stack size: " << ip.stack.size() << "\n";
    });

    interp.register_op("reflect_y", [](WoflangInterpreter& ip) {
        std::cout << "[geom_transform_ops] op \"reflect_y\" is not yet implemented."
                  << " Stack size: " << ip.stack.size() << "\n";
    });

    interp.register_op("rotate2d", [](WoflangInterpreter& ip) {
        std::cout << "[geom_transform_ops] op \"rotate2d\" is not yet implemented."
                  << " Stack size: " << ip.stack.size() << "\n";
    });

    interp.register_op("scale2d", [](WoflangInterpreter& ip) {
        std::cout << "[geom_transform_ops] op \"scale2d\" is not yet implemented."
                  << " Stack size: " << ip.stack.size() << "\n";
    });

    interp.register_op("translate2d", [](WoflangInterpreter& ip) {
        std::cout << "[geom_transform_ops] op \"translate2d\" is not yet implemented."
                  << " Stack size: " << ip.stack.size() << "\n";
    });

}
