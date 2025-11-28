#include "woflang.hpp"

#include <algorithm>
#include <cmath>
#include <iostream>
#include <random>
#include <string>
#include <unordered_map>
#include <variant>

using namespace woflang;

// Export macro for this plugin (matches CMake target: entropy_ops)
#ifdef _WIN32
#  ifdef entropy_ops_EXPORTS
#    define WOFLANG_PLUGIN_EXP __declspec(dllexport)
#  else
#    define WOFLANG_PLUGIN_EXP __declspec(dllimport)
#  endif
#else
#  define WOFLANG_PLUGIN_EXP
#endif

namespace {

// Turn a WofValue into a canonical key string for entropy counting
static std::string make_entropy_key(const WofValue& v)
{
    switch (v.type) {
    case WofType::Integer: {
        auto iv = std::get<int64_t>(v.value);
        return "i:" + std::to_string(iv);
    }
    case WofType::Double: {
        auto dv = std::get<double>(v.value);
        return "d:" + std::to_string(dv);
    }
    case WofType::String: {
        const auto& s = std::get<std::string>(v.value);
        return "s:" + s;
    }
    default:
        // Fallback for any future types
        return "x";
    }
}

// Best-effort numeric interpretation for ordering.
// Non-numeric values just get 0.0 but are kept after numerics in ordering.
static bool is_numeric(const WofValue& v)
{
    return v.type == WofType::Integer || v.type == WofType::Double;
}

static double to_numeric(const WofValue& v)
{
    if (v.type == WofType::Integer) {
        return static_cast<double>(std::get<int64_t>(v.value));
    }
    if (v.type == WofType::Double) {
        return std::get<double>(v.value);
    }
    // Strings / others: we could try stod, but "entropy" is already fuzzy,
    // and for ordering we just want a stable, harmless default.
    return 0.0;
}

static WofValue make_double(double x)
{
    WofValue v;
    v.type  = WofType::Double;
    v.value = x;
    return v;
}

} // namespace

extern "C" {
WOFLANG_PLUGIN_EXP void register_plugin(WoflangInterpreter& interp)
{
    // ENTROPY: Shannon entropy over all stack values, push result in bits
    interp.register_op("entropy", [](WoflangInterpreter& ip) {
        const auto n = ip.stack.size();
        if (n == 0) {
            // Empty stack => entropy 0
            ip.stack.push_back(make_double(0.0));
            std::cout << "[entropy_ops] entropy: empty stack => H = 0 bits\n";
            return;
        }

        std::unordered_map<std::string, std::size_t> counts;
        counts.reserve(n);

        for (const auto& v : ip.stack) {
            const std::string key = make_entropy_key(v);
            ++counts[key];
        }

        const double total = static_cast<double>(n);
        double H = 0.0;

        for (const auto& [key, c] : counts) {
            (void)key; // key unused except for counting
            const double p = static_cast<double>(c) / total;
            if (p > 0.0) {
                H -= p * std::log2(p);
            }
        }

        ip.stack.push_back(make_double(H));
        std::cout << "[entropy_ops] entropy over " << n
                  << " values, " << counts.size()
                  << " unique symbols => H = " << H << " bits\n";
    });

    // CHAOS: randomly shuffle the stack
    interp.register_op("chaos", [](WoflangInterpreter& ip) {
        if (ip.stack.empty()) {
            std::cout << "[entropy_ops] chaos: stack already empty, nothing to shuffle\n";
            return;
        }

        // Local RNG; no need for cryptographic quality here.
        std::random_device rd;
        std::mt19937 gen(rd());

        std::shuffle(ip.stack.begin(), ip.stack.end(), gen);

        std::cout << "[entropy_ops] chaos: stack has been randomly permuted (size = "
                  << ip.stack.size() << ")\n";
    });

    // ORDER: stable sort with numeric values first, ascending
    interp.register_op("order", [](WoflangInterpreter& ip) {
        if (ip.stack.empty()) {
            std::cout << "[entropy_ops] order: stack already empty, nothing to sort\n";
            return;
        }

        auto& st = ip.stack;
        std::stable_sort(st.begin(), st.end(),
                         [](const WofValue& a, const WofValue& b) {
                             const bool an = is_numeric(a);
                             const bool bn = is_numeric(b);

                             if (an && bn) {
                                 return to_numeric(a) < to_numeric(b);
                             }
                             if (an != bn) {
                                 // numeric values first
                                 return an;
                             }
                             // non-numeric relative order preserved by stable_sort
                             return false;
                         });

        std::cout << "[entropy_ops] order: stack sorted; numeric values promoted (size = "
                  << st.size() << ")\n";
    });

    std::cout << "[entropy_ops] Plugin loaded (entropy, chaos, order)\n";
}
}
