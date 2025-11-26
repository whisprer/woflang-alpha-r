// ================================================
// prime_hecc_ops.cpp - oops, you classic typo'd
// ================================================

#include <iostream>

#include "woflang.hpp"

#ifndef WOFLANG_PLUGIN_EXPORT
#  ifdef _WIN32
#    define WOFLANG_PLUGIN_EXPORT __declspec(dllexport)
#  else
#    define WOFLANG_PLUGIN_EXPORT __attribute__((visibility("default")))
#  endif
#endif

using woflang::WoflangInterpreter;

extern "C" WOFLANG_PLUGIN_EXPORT void register_plugin(WoflangInterpreter& interp) {
    interp.register_op("prime_hecc", [](WoflangInterpreter& ip) {
        static const char* banner = R"(==============================================
P R I M E   H E C C
==============================================
PRIME HECK OP: emptying stack!

    .--. 
   |o_o |
   |:_/ |
  //   \ \
 (|     | )
/'\_   _/`\
\___)=(___/

A typo, or a summons from the deep?
You typed 'pime_heck' instead of 'prime_check'.
==============================================
)";
        
        std::cout << banner;
        std::cout << "[prime_hecc_ops] clearing " << ip.stack.size() << " values from stack\n";
        ip.stack.clear();
        std::cout.flush();
    });
}
