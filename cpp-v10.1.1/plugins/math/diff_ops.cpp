// ====================================================
// diff_ops.cpp - Numerical Differentiation Operations
// ====================================================
//
// Stack-based finite difference operators for WofLang:
// 
// 1) Forward difference:
//
//    Stack BEFORE (bottom .. top):
//        f(x)   f(x+h)   h
//    Code:
//        diff.forward
//    Stack AFTER:
//        df_dx
//
// 2) Backward difference:
//
//    Stack BEFORE:
//        f(x-h)   f(x)   h
//    Code:
//        diff.backward
//    Stack AFTER:
//        df_dx
//
// 3) Central difference:
//
//    Stack BEFORE:
//        f(x-h)   f(x+h)   h
//    Code:
//        diff.central
//    Stack AFTER:
//        df_dx
//
// All values must be numeric; h must be non-zero.
//
// ====================================================

#ifndef WOFLANG_PLUGIN_EXPORT
# ifdef _WIN32
#  define WOFLANG_PLUGIN_EXPORT __declspec(dllexport)
# else
#  define WOFLANG_PLUGIN_EXPORT __attribute__((visibility("default")))
# endif
#endif

#include "../src/core/woflang.hpp"
#include <vector>
#include <stdexcept>
#include <string>
#include <cmath>

using woflang::WofValue;
using woflang::WoflangInterpreter;

namespace {

/// Lightweight adapter that gives vector-backed stack semantics
template <typename Container>
class WofStackAdapter {
public:
    explicit WofStackAdapter(Container& data) : data_(data) {}

    bool empty() const noexcept { return data_.empty(); }
    std::size_t size() const noexcept { return data_.size(); }

    WofValue& top() {
        if (data_.empty()) {
            throw std::runtime_error("Stack underflow: top() on empty stack");
        }
        return data_.back();
    }

    const WofValue& top() const {
        if (data_.empty()) {
            throw std::runtime_error("Stack underflow: top() on empty stack");
        }
        return data_.back();
    }

    void push(const WofValue& v) {
        data_.push_back(v);
    }

    /// Pop and return the top value
    WofValue pop_value(const char* context) {
        if (data_.empty()) {
            throw std::runtime_error(
                std::string("Stack underflow in ") + context
            );
        }
        WofValue v = data_.back();
        data_.pop_back();
        return v;
    }

private:
    Container& data_;
};

inline double require_numeric(const WofValue& v, const char* what) {
    // Relies on your current WofValue::as_numeric() throwing if not numeric
    try {
        return v.as_numeric();
    } catch (const std::exception& e) {
        std::string msg = "Expected numeric value for ";
        msg += what;
        msg += ": ";
        msg += e.what();
        throw std::runtime_error(msg);
    }
}

inline void check_nonzero(double h, const char* what) {
    if (h == 0.0) {
        throw std::runtime_error(
            std::string("Step size h must be non-zero in ") + what
        );
    }
}

} // anonymous namespace

extern "C" WOFLANG_PLUGIN_EXPORT void register_plugin(WoflangInterpreter& interp) {
    using StackType = WofStackAdapter<decltype(interp.stack)>;

    auto register_op = [&](const std::string& name, auto fn) {
        interp.register_op(name, [fn](WoflangInterpreter& ip) {
            StackType S{ip.stack};
            fn(S);
        });
    };

    // Forward difference:
    // df/dx ≈ (f(x+h) - f(x)) / h
    //
    // Stack BEFORE: f(x), f(x+h), h   (h on top)
    // Stack AFTER: df_dx
    register_op("diff.forward", [](StackType& S) {
        const char* ctx = "diff.forward";

        // Pop in reverse order of push: h, f(x+h), f(x)
        WofValue h_v    = S.pop_value(ctx);
        WofValue fxph_v = S.pop_value(ctx);
        WofValue fx_v   = S.pop_value(ctx);

        double h    = require_numeric(h_v, "h (forward)");
        double fxph = require_numeric(fxph_v, "f(x+h)");
        double fx   = require_numeric(fx_v, "f(x)");

        check_nonzero(h, ctx);

        double df_dx = (fxph - fx) / h;

        S.push(WofValue::make_double(df_dx));
    });

    // Backward difference:
    // df/dx ≈ (f(x) - f(x-h)) / h
    //
    // Stack BEFORE: f(x-h), f(x), h
    // Stack AFTER: df_dx
    register_op("diff.backward", [](StackType& S) {
        const char* ctx = "diff.backward";

        WofValue h_v   = S.pop_value(ctx);
        WofValue fx_v  = S.pop_value(ctx);
        WofValue fxmh_v= S.pop_value(ctx);

        double h    = require_numeric(h_v, "h (backward)");
        double fx   = require_numeric(fx_v, "f(x)");
        double fxmh = require_numeric(fxmh_v, "f(x-h)");

        check_nonzero(h, ctx);

        double df_dx = (fx - fxmh) / h;

        S.push(WofValue::make_double(df_dx));
    });

    // Central difference:
    // df/dx ≈ (f(x+h) - f(x-h)) / (2h)
    //
    // Stack BEFORE: f(x-h), f(x+h), h
    // Stack AFTER: df_dx
    register_op("diff.central", [](StackType& S) {
        const char* ctx = "diff.central";

        WofValue h_v    = S.pop_value(ctx);
        WofValue fxph_v = S.pop_value(ctx);
        WofValue fxmh_v = S.pop_value(ctx);

        double h    = require_numeric(h_v, "h (central)");
        double fxph = require_numeric(fxph_v, "f(x+h)");
        double fxmh = require_numeric(fxmh_v, "f(x-h)");

        check_nonzero(h, ctx);

        double df_dx = (fxph - fxmh) / (2.0 * h);

        S.push(WofValue::make_double(df_dx));
    });
}
