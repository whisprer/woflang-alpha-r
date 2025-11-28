// ==================================================
// stack_ops.cpp - Stack utility operations (v10.1.1)
// ==================================================

#ifndef WOFLANG_PLUGIN_EXPORT
# ifdef _WIN32
#  define WOFLANG_PLUGIN_EXPORT __declspec(dllexport)
# else
#  define WOFLANG_PLUGIN_EXPORT __attribute__((visibility("default")))
# endif
#endif

#include "../../src/core/woflang.hpp"

#include <stdexcept>
#include <iostream>
#include <cstdint>

using woflang::WoflangInterpreter;
using woflang::WofValue;

namespace {

// Duplicate the top stack value
void op_stack_dup(WoflangInterpreter& ip) {
    if (ip.stack.empty()) {
        throw std::runtime_error("stack_dup: stack is empty");
    }
    ip.push(ip.stack.back());
}

// Swap the top two stack values
void op_stack_swap(WoflangInterpreter& ip) {
    if (ip.stack.size() < 2) {
        throw std::runtime_error("stack_swap: need at least two values on the stack");
    }
    auto n = ip.stack.size();
    std::swap(ip.stack[n - 1], ip.stack[n - 2]);
}

// Drop (pop and discard) the top value
void op_stack_drop(WoflangInterpreter& ip) {
    if (ip.stack.empty()) {
        throw std::runtime_error("stack_drop: stack is empty");
    }
    (void)ip.pop();
}

// Clear the entire stack
void op_stack_clear(WoflangInterpreter& ip) {
    ip.clear_stack();
}

// Push the current stack depth as a numeric value
void op_stack_depth(WoflangInterpreter& ip) {
    double depth = static_cast<double>(ip.stack.size());
    ip.push(WofValue::make_double(depth));
}

} // namespace

extern "C" WOFLANG_PLUGIN_EXPORT void register_plugin(WoflangInterpreter& interp) {
    interp.register_op("stack_dup",    op_stack_dup);
    interp.register_op("stack_swap",   op_stack_swap);
    interp.register_op("stack_drop",   op_stack_drop);
    interp.register_op("stack_clear",  op_stack_clear);
    interp.register_op("stack_depth",  op_stack_depth);

    std::cout << "[stack_ops] Plugin loaded: stack_dup, stack_swap, "
                 "stack_drop, stack_clear, stack_depth\n";
}
