// ================================================================
// neural_chess_ops.cpp - All operations currently just log that they are unimplemented.
// Auto-generated Stub to ensure build success with the new Woflang core.
// ================================================================

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

    interp.register_op("chess_neural_eval", [](WoflangInterpreter& ip) {
        std::cout << "[neural_chess_ops] op \"chess_neural_eval\" is not yet implemented."
                  << " Stack size: " << ip.stack.size() << "\n";
    });

    interp.register_op("chess_neural_move", [](WoflangInterpreter& ip) {
        std::cout << "[neural_chess_ops] op \"chess_neural_move\" is not yet implemented."
                  << " Stack size: " << ip.stack.size() << "\n";
    });

    interp.register_op("chess_new", [](WoflangInterpreter& ip) {
        std::cout << "[neural_chess_ops] op \"chess_new\" is not yet implemented."
                  << " Stack size: " << ip.stack.size() << "\n";
    });

    interp.register_op("chess_quick_train", [](WoflangInterpreter& ip) {
        std::cout << "[neural_chess_ops] op \"chess_quick_train\" is not yet implemented."
                  << " Stack size: " << ip.stack.size() << "\n";
    });

    interp.register_op("chess_show", [](WoflangInterpreter& ip) {
        std::cout << "[neural_chess_ops] op \"chess_show\" is not yet implemented."
                  << " Stack size: " << ip.stack.size() << "\n";
    });

}
