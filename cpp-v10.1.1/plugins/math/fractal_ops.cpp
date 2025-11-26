// ==================================================================
// fractal_ops.cpp - All operations currently just log that they are unimplemented.
// Auto-generated Stub to ensure build success with the new Woflang core.
// ================================================================== 

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

    interp.register_op("hausdorff", [](WoflangInterpreter& ip) {
        std::cout << "[fractal_ops] op \"hausdorff\" is not yet implemented."
                  << " Stack size: " << ip.stack.size() << "\n";
    });

    interp.register_op("julia", [](WoflangInterpreter& ip) {
        std::cout << "[fractal_ops] op \"julia\" is not yet implemented."
                  << " Stack size: " << ip.stack.size() << "\n";
    });

    interp.register_op("mandelbrot", [](WoflangInterpreter& ip) {
        std::cout << "[fractal_ops] op \"mandelbrot\" is not yet implemented."
                  << " Stack size: " << ip.stack.size() << "\n";
    });

    interp.register_op("menger_square", [](WoflangInterpreter& ip) {
        std::cout << "[fractal_ops] op \"menger_square\" is not yet implemented."
                  << " Stack size: " << ip.stack.size() << "\n";
    });

    interp.register_op("sierpinski", [](WoflangInterpreter& ip) {
        std::cout << "[fractal_ops] op \"sierpinski\" is not yet implemented."
                  << " Stack size: " << ip.stack.size() << "\n";
    });

}
