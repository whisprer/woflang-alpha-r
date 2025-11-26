// ==================================================
// symbolic_logic_ops.cpp - Logical Operations (v10.1.1 API)
// ==================================================

#ifndef WOFLANG_PLUGIN_EXPORT
# ifdef _WIN32
#  define WOFLANG_PLUGIN_EXPORT __declspec(dllexport)
# else
#  define WOFLANG_PLUGIN_EXPORT __attribute__((visibility("default")))
# endif
#endif

#include "../../src/core/woflang.hpp"

#include <iostream>
#include <stdexcept>

using woflang::WoflangInterpreter;
using woflang::WofValue;

namespace {
template <typename Container>
struct WofStackAdapter {
    Container& v;
    auto size() const { return v.size(); }
    bool empty() const { return v.empty(); }
    woflang::WofValue& top() { return v.back(); }
    const woflang::WofValue& top() const { return v.back(); }
    void pop() { v.pop_back(); }
    void push(const woflang::WofValue& x) { v.push_back(x); }
};

static inline bool to_bool(const WofValue& val) {
    return val.as_numeric() != 0.0;
}
}

extern "C" WOFLANG_PLUGIN_EXPORT void
register_plugin(WoflangInterpreter& interp) {
    using namespace woflang;

    interp.register_op("forall", [](WoflangInterpreter&) {
        throw std::runtime_error("forall quantifier not yet implemented");
    });

    interp.register_op("exists", [](WoflangInterpreter&) {
        throw std::runtime_error("exists quantifier not yet implemented");
    });

    interp.register_op("implies", [](WoflangInterpreter& ip) {
        WofStackAdapter S{ip.stack};
        if (S.size() < 2) throw std::runtime_error("implies: needs 2 values");
        auto b = S.top(); S.pop();
        auto a = S.top(); S.pop();
        bool result = !to_bool(a) || to_bool(b);
        S.push(WofValue::make_double(result ? 1.0 : 0.0));
    });

    interp.register_op("iff", [](WoflangInterpreter& ip) {
        WofStackAdapter S{ip.stack};
        if (S.size() < 2) throw std::runtime_error("iff: needs 2 values");
        auto b = S.top(); S.pop();
        auto a = S.top(); S.pop();
        bool result = to_bool(a) == to_bool(b);
        S.push(WofValue::make_double(result ? 1.0 : 0.0));
    });

    interp.register_op("and", [](WoflangInterpreter& ip) {
        WofStackAdapter S{ip.stack};
        if (S.size() < 2) throw std::runtime_error("and: needs 2 values");
        auto b = S.top(); S.pop();
        auto a = S.top(); S.pop();
        bool result = to_bool(a) && to_bool(b);
        S.push(WofValue::make_double(result ? 1.0 : 0.0));
    });

    interp.register_op("or", [](WoflangInterpreter& ip) {
        WofStackAdapter S{ip.stack};
        if (S.size() < 2) throw std::runtime_error("or: needs 2 values");
        auto b = S.top(); S.pop();
        auto a = S.top(); S.pop();
        bool result = to_bool(a) || to_bool(b);
        S.push(WofValue::make_double(result ? 1.0 : 0.0));
    });

    interp.register_op("not", [](WoflangInterpreter& ip) {
        WofStackAdapter S{ip.stack};
        if (S.empty()) throw std::runtime_error("not: stack underflow");
        auto a = S.top(); S.pop();
        bool result = !to_bool(a);
        S.push(WofValue::make_double(result ? 1.0 : 0.0));
    });

    interp.register_op("tautology_demo", [](WoflangInterpreter& ip) {
        WofStackAdapter S{ip.stack};
        std::cout << "\n🎓 Tautology Demo: A OR NOT A\n";
        for (int i = 0; i < 2; ++i) {
            bool a = (i == 1);
            bool not_a = !a;
            bool result = a || not_a;
            std::cout << "  A=" << (a ? "T" : "F") << " | ¬A=" << (not_a ? "T" : "F")
                      << " | A ∨ ¬A=" << (result ? "T" : "F") << "\n";
        }
        std::cout << "  Result: Always TRUE (tautology!)\n\n";
        S.push(WofValue::make_double(1.0));
    });

    interp.register_op("contradiction_demo", [](WoflangInterpreter& ip) {
        WofStackAdapter S{ip.stack};
        std::cout << "\n🎓 Contradiction Demo: A AND NOT A\n";
        for (int i = 0; i < 2; ++i) {
            bool a = (i == 1);
            bool not_a = !a;
            bool result = a && not_a;
            std::cout << "  A=" << (a ? "T" : "F") << " | ¬A=" << (not_a ? "T" : "F")
                      << " | A ∧ ¬A=" << (result ? "T" : "F") << "\n";
        }
        std::cout << "  Result: Always FALSE (contradiction!)\n\n";
        S.push(WofValue::make_double(0.0));
    });

    std::cout << "[logic] Symbolic logic plugin loaded.\n";
}
