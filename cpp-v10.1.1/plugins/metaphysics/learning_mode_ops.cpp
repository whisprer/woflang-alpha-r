// =====================================================
// learning_mode_ops.cpp - let's take lessons in Woflang
// =====================================================

#ifndef WOFLANG_PLUGIN_EXPORT
# ifdef _WIN32
#  define WOFLANG_PLUGIN_EXPORT __declspec(dllexport)
# else
#  define WOFLANG_PLUGIN_EXPORT __attribute__((visibility("default")))
# endif
#endif

#include "woflang.hpp"
#include <iostream>
#include <vector>
#include <string>
#include <cstdlib>

using woflang::WoflangInterpreter;
using woflang::WofValue;

// lesson: print a random learning tip
static void op_lesson(WoflangInterpreter& interp) {
    (void)interp; // stack not used here

    std::vector<std::string> lessons = {
        "Lesson 1: To add numbers, use: 2 3 + print",
        "Lesson 2: To duplicate stack top, use: dup",
        "Lesson 3: To print pi, use: pi print",
        "Lesson 4: To use a plugin, type its op name (e.g., π print)",
        "Lesson 5: Try a chemistry op: 2 mol"
    };

    if (lessons.empty()) {
        return;
    }

    int idx = std::rand() % static_cast<int>(lessons.size());
    std::cout << "[Learning Mode] " << lessons[idx] << std::endl;
}

// hint: inspect stack and print a hint
static void op_hint(WoflangInterpreter& interp) {
    if (interp.stack.empty()) {
        std::cout << "Hint: The stack is empty! Try pushing a value.\n";
    } else {
        std::cout << "Hint: Use 'print' to see the top of the stack.\n";
    }
}

// quiz: ask a fixed quiz question
static void op_quiz(WoflangInterpreter& interp) {
    (void)interp; // no stack use yet
    std::cout << "[Quiz] What does 'Δ' do in Greek Math mode?\n";
    std::cout << "A) Adds two numbers\n";
    std::cout << "B) Subtracts top from next\n";
    std::cout << "C) Multiplies two numbers\n";
}

extern "C" WOFLANG_PLUGIN_EXPORT void register_plugin(WoflangInterpreter& interp) {
    interp.register_op("lesson", [](WoflangInterpreter& ip) {
        op_lesson(ip);
    });

    interp.register_op("hint", [](WoflangInterpreter& ip) {
        op_hint(ip);
    });

    interp.register_op("quiz", [](WoflangInterpreter& ip) {
        op_quiz(ip);
    });

    std::cout << "[learning_mode_ops] Plugin loaded.\n";
}
