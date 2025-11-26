// ==================================================
// wofl_sigil_ops.cpp - Hidden Glyph Totem
// ==================================================

#ifndef WOFLANG_PLUGIN_EXPORT
# ifdef _WIN32
#  define WOFLANG_PLUGIN_EXPORT __declspec(dllexport)
# else
#  define WOFLANG_PLUGIN_EXPORT __attribute__((visibility("default")))
# endif
#endif

#include "woflang.hpp"

#include <iostream>

using woflang::WoflangInterpreter;

extern "C" WOFLANG_PLUGIN_EXPORT void
register_plugin(WoflangInterpreter& interp) {
    using namespace woflang;

    interp.register_op(":wofsigil", [](WoflangInterpreter&) {
        std::cout << R"SIGIL(

            ╭────────────────────────────╮
            │        W O F L A N G      │
            │      glyph totem v1.0     │
            ╰────────────────────────────╯
                   ⟁  ◬  𓂀  ☍  ₪
                 stack  •  sigil  •  code

        )SIGIL";
        std::cout << "\n";
    });
}
