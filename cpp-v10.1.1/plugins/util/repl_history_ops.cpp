// =====================================================
// repl_history_ops.cpp - stores/recalls string history
// =====================================================

#include <iostream>
#include <string>
#include <vector>
#include "woflang.hpp"

#ifndef WOFLANG_PLUGIN_EXPORT
# ifdef _WIN32
#  define WOFLANG_PLUGIN_EXPORT __declspec(dllexport)
# else
#  define WOFLANG_PLUGIN_EXPORT __attribute__((visibility("default")))
# endif
#endif

using woflang::WoflangInterpreter;
using woflang::WofValue;
using woflang::WofType;

static std::vector<std::string> g_repl_history;

static bool is_text(const WofValue& v) {
    return v.type == WofType::String || v.type == WofType::Symbol;
}

static std::string as_text(const WofValue& v) {
    return std::get<std::string>(v.value);
}

static void op_add_history(WoflangInterpreter& ip) {
    if (ip.stack.empty()) {
        return;
    }
    WofValue v = ip.stack.back();
    ip.stack.pop_back();

    if (is_text(v)) {
        g_repl_history.push_back(as_text(v));
    }
}

static void op_show_history(WoflangInterpreter&) {
    std::cout << "REPL History:\n";
    for (std::size_t i = 0; i < g_repl_history.size(); ++i) {
        std::cout << i << ": " << g_repl_history[i] << "\n";
    }
}

static void op_clear_history(WoflangInterpreter&) {
    g_repl_history.clear();
    std::cout << "REPL history cleared.\n";
}

extern "C" WOFLANG_PLUGIN_EXPORT void register_plugin(WoflangInterpreter& interp) {
    interp.register_op("add_history", [](WoflangInterpreter& ip) {
        op_add_history(ip);
    });

    interp.register_op("show_history", [](WoflangInterpreter& ip) {
        op_show_history(ip);
    });

    interp.register_op("clear_history", [](WoflangInterpreter& ip) {
        op_clear_history(ip);
    });

    std::cout << "[repl_history_commands] registered add_history, show_history, clear_history\n";
}
