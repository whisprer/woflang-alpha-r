// ===================================================
// io_debug_ops.cpp - core i/o operations integration
// ===================================================

#include <iostream>
#include <iomanip>
#include <string>

#include "woflang.hpp"

#ifndef WOFLANG_PLUGIN_EXPORT
#  ifdef _WIN32
#    define WOFLANG_PLUGIN_EXPORT __declspec(dllexport)
#  else
#    define WOFLANG_PLUGIN_EXPORT __attribute__((visibility("default")))
#  endif
#endif

using woflang::WofValue;
using woflang::WoflangInterpreter;

namespace {

/**
 * Small helper to describe a WofValue nicely.
 */
std::string describe_value(const WofValue& v) {
    // We rely only on the public API of WofValue.
    std::string s = v.to_string();
    if (v.is_numeric()) {
        s += " (numeric)";
    }
    return s;
}

} // namespace

extern "C" WOFLANG_PLUGIN_EXPORT void register_plugin(WoflangInterpreter& interp) {
    using namespace std;
    using woflang::WofValue;

    // -------------------------------------------------------------------------
    // print  : pop top of stack and print it
    // -------------------------------------------------------------------------
    interp.register_op("print", [](WoflangInterpreter& ip) {
        if (!ip.stack_has(1)) {
            std::cerr << "[io_debug::print] stack underflow: need 1 value\n";
            return;
        }
        WofValue v = ip.pop();
        std::cout << v.to_string() << std::endl;
    });

    // -------------------------------------------------------------------------
    // stack_dump : dump entire stack (bottom -> top) without modifying it
    // -------------------------------------------------------------------------
    interp.register_op("stack_dump", [](WoflangInterpreter& ip) {
        const auto& st = ip.stack;
        std::cout << "[io_debug::stack_dump] size = " << st.size() << '\n';

        if (st.empty()) {
            std::cout << "  (stack is empty)\n";
            return;
        }

        for (std::size_t i = 0; i < st.size(); ++i) {
            const auto& v = st[i];
            std::cout << "  [" << std::setw(3) << i << "] "
                      << describe_value(v) << '\n';
        }
        std::cout.flush();
    });

    // -------------------------------------------------------------------------
    // stack_top : print top-of-stack (without popping)
    // -------------------------------------------------------------------------
    interp.register_op("stack_top", [](WoflangInterpreter& ip) {
        const auto& st = ip.stack;
        if (st.empty()) {
            std::cerr << "[io_debug::stack_top] stack is empty\n";
            return;
        }
        const auto& v = st.back();
        std::cout << "[io_debug::stack_top] "
                  << describe_value(v) << std::endl;
    });

    // -------------------------------------------------------------------------
    // stack_size : print current stack depth
    // -------------------------------------------------------------------------
    interp.register_op("stack_size", [](WoflangInterpreter& ip) {
        std::cout << "[io_debug::stack_size] "
                  << ip.stack.size() << std::endl;
    });

    // -------------------------------------------------------------------------
    // stack_clear : clear the stack completely
    // -------------------------------------------------------------------------
    interp.register_op("stack_clear", [](WoflangInterpreter& ip) {
        std::cout << "[io_debug::stack_clear] clearing "
                  << ip.stack.size() << " values\n";
        ip.clear_stack();
    });
}
