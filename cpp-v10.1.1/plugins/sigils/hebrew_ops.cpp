// plugins/sigils/hebrew_ops.cpp
//
// A small fun module:
//   - hebrew_mode_on / hebrew_mode_off: toggle RTL "Hebrew" display mode
//   - hebrew_echo: echo top-of-stack as (pseudo) Hebrew (RTL-mirrored)
//   - hebrews_it: tell the Moses tea joke (and push it as a string)

#include "woflang.hpp"

#include <algorithm>
#include <iostream>
#include <string>
#include <variant>

using namespace woflang;

// Export macro if not already defined by core
#ifndef WOFLANG_PLUGIN_EXPORT
# ifdef _WIN32
#  define WOFLANG_PLUGIN_EXPORT extern "C" __declspec(dllexport)
# else
#  define WOFLANG_PLUGIN_EXPORT extern "C"
# endif
#endif

// Simple helper: stringify a WofValue for display/echo
static std::string value_to_string(const WofValue &v) {
    switch (v.type) {
        case WofType::Integer:
            return std::to_string(std::get<int64_t>(v.value));
        case WofType::Double:
            return std::to_string(std::get<double>(v.value));
        case WofType::String:
            return std::get<std::string>(v.value);
        default:
            return "<unknown>";
    }
}

// Global (per-process) "Hebrew mode" flag for this plugin
static bool g_hebrew_mode = false;

// Tiny trick: prepend RLM (U+200F) and reverse the string
// Many terminals/UIs will then render it right-to-left visually.
static std::string to_pseudo_hebrew(const std::string &s) {
    std::string reversed(s.rbegin(), s.rend());

    std::string result;
    // Encode U+200F = 0xE2 0x80 0x8F as UTF-8
    result.push_back(static_cast<char>(0xE2));
    result.push_back(static_cast<char>(0x80));
    result.push_back(static_cast<char>(0x8F));

    result += reversed;
    return result;
}

static void op_hebrew_mode_on(WoflangInterpreter &interp) {
    (void)interp;
    g_hebrew_mode = true;
    std::cout << "[hebrew_ops] Hebrew mode: ON (RTL mirroring enabled)\n";
}

static void op_hebrew_mode_off(WoflangInterpreter &interp) {
    (void)interp;
    g_hebrew_mode = false;
    std::cout << "[hebrew_ops] Hebrew mode: OFF\n";
}

// Echo the top-of-stack as a (pseudo) Hebrew string
static void op_hebrew_echo(WoflangInterpreter &interp) {
    auto &st = interp.stack;
    if (st.empty()) {
        std::cout << "[hebrew_ops] hebrew_echo: stack is empty.\n";
        return;
    }

    WofValue v = st.back();
    st.pop_back();

    std::string s   = value_to_string(v);
    std::string out = g_hebrew_mode ? to_pseudo_hebrew(s) : s;

    std::cout << out << "\n";

    // Push the echoed text as a string value
    WofValue res;
    res.type  = WofType::String;
    res.value = out;
    interp.stack.push_back(res);
}

// Tell the Moses tea joke, honoring Hebrew mode, and push it as a string
static void op_hebrews_it(WoflangInterpreter &interp) {
    const std::string joke = "How does Moses take his tea? He brews it!";

    std::string out = g_hebrew_mode ? to_pseudo_hebrew(joke) : joke;

    std::cout << out << "\n";

    WofValue v;
    v.type  = WofType::String;
    v.value = out;
    interp.stack.push_back(v);
}

WOFLANG_PLUGIN_EXPORT void register_plugin(WoflangInterpreter &interp) {
    interp.register_op("hebrew_mode_on",  op_hebrew_mode_on);
    interp.register_op("hebrew_mode_off", op_hebrew_mode_off);
    interp.register_op("hebrew_echo",     op_hebrew_echo);
    interp.register_op("hebrews_it",      op_hebrews_it);
    std::cout << "[hebrew_ops] Plugin loaded.\n";
}
