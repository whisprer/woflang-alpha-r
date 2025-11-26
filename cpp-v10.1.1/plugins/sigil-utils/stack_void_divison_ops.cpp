// ===================================================================
// stack_void_division_ops.cpp - beware division by zero in the stack
// ===================================================================

#include "../../src/core/woflang.hpp"
#include <iostream>

class GlyphProphecyStackVoidDivisionPlugin : public WoflangPlugin {
public:
    void register_ops(WoflangInterpreter& interp) override {
        interp.register_op("glyph_prophecy", [](WoflangInterpreter&) {
            std::cout << "[Forbidden] The encrypted glyph prophecy divides the stack void. Beware division by zero!" << std::endl;
        });
    }
};

WOFLANG_PLUGIN_EXPORT void register_plugin(WoflangInterpreter& interp) {
    static GlyphProphecyStackVoidDivisionPlugin plugin;
    plugin.register_ops(interp);
}
