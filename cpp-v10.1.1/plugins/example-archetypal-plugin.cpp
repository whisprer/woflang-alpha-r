#ifndef WOFLANG_PLUGIN_EXPORT
# ifdef _WIN32
#  define WOFLANG_PLUGIN_EXPORT __declspec(dllexport)
# else
#  define WOFLANG_PLUGIN_EXPORT __attribute__((visibility("default")))
# endif
#endif

#include "woflang.hpp"
#include <...needed std headers...>

using woflang::WoflangInterpreter;
using woflang::WofValue;

extern "C" WOFLANG_PLUGIN_EXPORT void register_plugin(WoflangInterpreter& interp) {
    interp.register_op("op_name", [](WoflangInterpreter& ip) {
        // use ip.pop_numeric(), ip.push(...), WofValue::make_double, etc.
    });

    // as many ops as needed
}

// single exported `register_plugin()` function
//
// helpers:
// `ip.pop_numeric()` / `ip.require_stack_size(n)`
// `WofValue::make_integer`, `make_double`, `make_string`
// `value.as_numeric()` / `value.as_string()`
//
// Ensure correct #include set: woflang.hpp, <cmath>, <string>, <vector>, etc.
//
// Strip any global state that touches the interpreter on destruction (no segfaults on shutdown).
//
// [Kept all the original behavior / semantics from v9 wherever possible.]
// 
// organized into a `plugins/<category>/whatever_ops.cpp` tree.
// 
// Since woflang_add_plugin now links against woflang_core, new plugins will “just link” as long as we name the target/paths to match.
// 
// Use the pattern that now compiles and runs for existing plugins:
// `#include "../../src/core/woflang.hpp"` (or `woflang.hpp` where appropriate)
// `class <NiceName>Plugin : public WoflangPlugin { ... }`;
// `WOFLANG_PLUGIN_EXPORT void register_plugin(WoflangInterpreter& interp)` that constructs the plugin and calls `register_ops(interp)`.
// `Register ops via interp.register_op("op_name", [](WoflangInterpreter& ip) { ... })` just like the ones now working and logging in your `--test` run.
//
// Adapt stack/value access to the new WofValue usage
// Use helper methods / accessors that do exist in the new core (e.g. as_number / as_bool / whatever we’ve already relied on for the working plugins), or
// Fall back to simple “pop to double” helpers in the plugin if necessary, but without calling old missing stuff like WofValue::make_integer, make_double, etc.
// Ensure everything does clean error handling and pushes a value (or at least leaves the stack in a consistent state) even on bad input.
//
// 