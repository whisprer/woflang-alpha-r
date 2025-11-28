// =================================================
// quantum_ops.cpp - your very own quantum computer!
// =================================================

#ifndef WOFLANG_PLUGIN_EXPORT
# ifdef _WIN32
#  define WOFLANG_PLUGIN_EXPORT __declspec(dllexport)
# else
#  define WOFLANG_PLUGIN_EXPORT __attribute__((visibility("default")))
# endif
#endif

#include "woflang.hpp"

#include <iostream>
#include <random>
#include <cstdint>

using woflang::WoflangInterpreter;
using woflang::WofValue;
using woflang::WofType;

// ---- local helpers -------------------------------------------------

static std::mt19937& rng() {
    static std::mt19937 gen{std::random_device{}()};
    return gen;
}

static int rand_bit() {
    static std::uniform_int_distribution<int> dist(0, 1);
    return dist(rng());
}

static bool is_integer(const WofValue& v) {
    return v.type == WofType::Integer;
}

static std::int64_t as_int(const WofValue& v) {
    return std::get<std::int64_t>(v.value);
}

static WofValue make_int_value(std::int64_t n) {
    WofValue v;
    v.type  = WofType::Integer;
    v.value = n;
    return v;
}

// |ψ⟩  -> push random qubit {0,1}
static void op_qubit_superposition(WoflangInterpreter& ip) {
    int bit = rand_bit();
    ip.push(make_int_value(bit));
    std::cout << "[quantum] |ψ⟩ superposition -> pushed qubit " << bit << "\n";
}

// H gate: discard input qubit, return fresh random qubit
static void op_hadamard(WoflangInterpreter& ip) {
    if (!ip.stack.empty()) {
        ip.stack.pop_back();
    }
    int bit = rand_bit();
    ip.push(make_int_value(bit));
    std::cout << "[quantum] H gate -> new qubit " << bit << "\n";
}

// X gate: bit flip 0 <-> 1
static void op_pauli_x(WoflangInterpreter& ip) {
    if (ip.stack.empty()) {
        std::cout << "[quantum] X gate: empty stack\n";
        return;
    }
    WofValue q = ip.stack.back();
    ip.stack.pop_back();

    if (!is_integer(q)) {
        std::cout << "[quantum] X gate: non-integer value, ignoring\n";
        ip.push(q);
        return;
    }

    std::int64_t v       = as_int(q);
    std::int64_t flipped = (v == 0) ? 1 : 0;
    ip.push(make_int_value(flipped));
    std::cout << "[quantum] X gate: " << v << " -> " << flipped << "\n";
}

// measure: print and push classical result
static void op_measure(WoflangInterpreter& ip) {
    if (ip.stack.empty()) {
        std::cout << "[quantum] measure: empty stack\n";
        return;
    }

    WofValue q = ip.stack.back();
    ip.stack.pop_back();

    std::int64_t v = 0;
    if (is_integer(q)) {
        v = as_int(q);
    }
    std::cout << "[quantum] measured: " << v << "\n";
    ip.push(make_int_value(v));
}

extern "C" WOFLANG_PLUGIN_EXPORT void register_plugin(WoflangInterpreter& interp) {
    interp.register_op("|ψ⟩", [](WoflangInterpreter& ip) {
        op_qubit_superposition(ip);
    });

    interp.register_op("H", [](WoflangInterpreter& ip) {
        op_hadamard(ip);
    });

    interp.register_op("X", [](WoflangInterpreter& ip) {
        op_pauli_x(ip);
    });

    interp.register_op("measure", [](WoflangInterpreter& ip) {
        op_measure(ip);
    });

    std::cout << "[quantum_ops] registered |ψ⟩, H, X, measure\n";
}
