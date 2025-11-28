// plugins/sigils/moses_ops.cpp

#include "woflang.hpp"

#include <iostream>
#include <string>
#include <vector>

using namespace woflang;

#ifndef WOFLANG_PLUGIN_EXPORT
# ifdef _WIN32
#  define WOFLANG_PLUGIN_EXPORT extern "C" __declspec(dllexport)
# else
#  define WOFLANG_PLUGIN_EXPORT extern "C"
# endif
#endif

// Helper: format a single WofValue as a short description
static std::string describe_value(const WofValue &v) {
    switch (v.type) {
        case WofType::Integer:
            return std::to_string(std::get<int64_t>(v.value));
        case WofType::Double:
            return std::to_string(std::get<double>(v.value));
        case WofType::String:
            return "\"" + std::get<std::string>(v.value) + "\"";
        default:
            return "<unknown>";
    }
}

// Non-destructive “part the sea” visualization
static void op_moses(WoflangInterpreter &interp) {
    auto &st = interp.stack;
    const std::size_t n = st.size();

    if (n == 0) {
        std::cout << "[moses] The sea is dry. The stack is empty.\n";
        return;
    }

    if (n == 1) {
        std::cout << "[moses] Only one value in the sea; nothing to part:\n";
        std::cout << "        top → " << describe_value(st.back()) << "\n";
        return;
    }

    std::size_t mid = n / 2; // lower indices = bottom of stack

    std::cout << "🌊 [moses] Parting the stack-sea of " << n << " values...\n";
    std::cout << "    left (" << mid << " values, bottom side):\n";
    for (std::size_t i = 0; i < mid; ++i) {
        std::cout << "      [" << i << "] " << describe_value(st[i]) << "\n";
    }

    std::cout << "    ───────────────  ⟡  ───────────────\n";

    std::cout << "    right (" << (n - mid) << " values, including top):\n";
    for (std::size_t i = mid; i < n; ++i) {
        std::cout << "      [" << i << "] " << describe_value(st[i])
                  << (i + 1 == n ? "   ← top" : "")
                  << "\n";
    }
}

// Destructive variant: re-lay out the stack with an explicit separator marker.
// Bottom ... left_half ... "⟡-SEA-SPLIT-⟡" ... right_half ... top
static void op_moses_split(WoflangInterpreter &interp) {
    auto &st = interp.stack;
    const std::size_t n = st.size();

    if (n < 2) {
        std::cout << "[moses_split] Need at least two values to part the sea.\n";
        return;
    }

    std::size_t mid = n / 2;

    std::vector<WofValue> left;
    std::vector<WofValue> right;
    left.reserve(mid);
    right.reserve(n - mid);

    for (std::size_t i = 0; i < mid; ++i) {
        left.push_back(st[i]);
    }
    for (std::size_t i = mid; i < n; ++i) {
        right.push_back(st[i]);
    }

    st.clear();

    // Bottom: left half in original order
    for (const auto &v : left) {
        st.push_back(v);
    }

    // Separator marker as a plain string value
    WofValue sep;
    sep.type  = WofType::String;
    sep.value = std::string("⟡-SEA-SPLIT-⟡");
    st.push_back(sep);

    // Then right half in original order
    for (const auto &v : right) {
        st.push_back(v);
    }

    std::cout << "🌊 [moses_split] The stack-sea has been parted.\n";
    std::cout << "    Left side size:  " << left.size() << "\n";
    std::cout << "    Right side size: " << right.size() << "\n";
    std::cout << "    Marker value:    \"⟡-SEA-SPLIT-⟡\" (in the middle of the stack)\n";
}

WOFLANG_PLUGIN_EXPORT void register_plugin(WoflangInterpreter &interp) {
    interp.register_op("moses",       op_moses);
    interp.register_op("moses_split", op_moses_split);
    std::cout << "[moses_ops] Plugin loaded.\n";
}
