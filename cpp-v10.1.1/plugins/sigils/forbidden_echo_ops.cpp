// ===================================================
// forbidden_echo_ops.cpp - whoops, now it's echoing
// ===================================================

#ifndef WOFLANG_PLUGIN_EXPORT
# ifdef _WIN32
#  define WOFLANG_PLUGIN_EXPORT __declspec(dllexport)
# else
#  define WOFLANG_PLUGIN_EXPORT __attribute__((visibility("default")))
# endif
#endif

#include "woflang.hpp"

#include <iostream>
#include <string>
#include <cstdint>

using woflang::WoflangInterpreter;
using woflang::WofValue;
using woflang::WofType;

static std::string last_forbidden_message;

// void_division:
//
// - Records a "forbidden" message.
// - If the stack is empty OR top is integer 0:
//     -> prints message and clears the stack.
// - Otherwise prints a warning that only zero can echo the void.
static void op_void_division(WoflangInterpreter& interp) {
    last_forbidden_message = "You have peered into the void.";

    if (interp.stack.empty()) {
        std::cout << "∅  " << last_forbidden_message << " (stack erased)" << std::endl;
        interp.clear_stack();
        return;
    }

    const WofValue& top = interp.stack.back();

    bool is_zero_int =
        (top.type == WofType::Integer) &&
        (std::get<std::int64_t>(top.value) == 0);

    if (is_zero_int) {
        std::cout << "∅  " << last_forbidden_message << " (stack erased)" << std::endl;
        interp.clear_stack();
    } else {
        std::cout << "∅  Only the zero can echo the void.\n";
    }
}

// forbidden_echo:
//
// - If there is a stored forbidden message, prints the "echo".
// - Otherwise says there's nothing to echo.
static void op_forbidden_echo(WoflangInterpreter& /*interp*/) {
    if (!last_forbidden_message.empty()) {
        std::cout << "∅∅  Forbidden echo (inverted): "
                  << last_forbidden_message
                  << " (now returned to you)\n";
    } else {
        std::cout << "∅∅  No forbidden op to echo.\n";
    }
}

extern "C" WOFLANG_PLUGIN_EXPORT void register_plugin(WoflangInterpreter& interp) {
    interp.register_op("void_division", [](WoflangInterpreter& ip) {
        op_void_division(ip);
    });

    interp.register_op("forbidden_echo", [](WoflangInterpreter& ip) {
        op_forbidden_echo(ip);
    });

    std::cout << "[forbidden_echo_ops] Plugin loaded.\n";
}
