// ============================================================================
// assert_ops.cpp - currently, All operations log that they are 'unimplemented'
// [Auto-generated to ensure build success with the new Woflang core.]
// ============================================================================

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

    interp.register_op("expect_approx", [](WoflangInterpreter& ip) {
        std::cout << "[assert_ops] op \"expect_approx\" is not yet implemented."
                  << " Stack size: " << ip.stack.size() << "\n";
    });

    interp.register_op("expect_eq", [](WoflangInterpreter& ip) {
        std::cout << "[assert_ops] op \"expect_eq\" is not yet implemented."
                  << " Stack size: " << ip.stack.size() << "\n";
    });

    interp.register_op("expect_true", [](WoflangInterpreter& ip) {
        std::cout << "[assert_ops] op \"expect_true\" is not yet implemented."
                  << " Stack size: " << ip.stack.size() << "\n";
    });

    interp.register_op("note", [](WoflangInterpreter& ip) {
        std::cout << "[assert_ops] op \"note\" is not yet implemented."
                  << " Stack size: " << ip.stack.size() << "\n";
    });

}
