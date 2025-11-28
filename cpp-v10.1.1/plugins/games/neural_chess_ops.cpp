// neural_chess_ops.cpp
//
// Woflang plugin shim for the "Neural Chess Ganglion Brain" engine.
//
// The original implementation in this file was actually a full Python
// program using PyTorch, GRU/LSTM and a differentiable CA to play and
// self-train at chess. That code has been moved to a proper .py file
// (e.g. plugins/games/neural_chess_ganglion.py).
//
// This C++ plugin provides lightweight stack ops that:
//
//   - Document what the neural chess engine does.
//   - Optionally spawn the external Python script and return its exit code.
//
// Stack contracts:
//
//   neural_chess_info
//     ( -- info-string )
//       Pushes a multi-line description of the neural chess engine.
//
//   neural_chess_run
//     ( [cmd-string] -- exit-code )
//       If the top of the stack is a string, it is taken as a shell
//       command and popped; otherwise a default command is used.
//       The command is passed to std::system(), and the resulting
//       process exit status (int) is pushed.
//
// Example Woflang usage:
//
//   neural_chess_info print
//
//   "python plugins/games/neural_chess_ganglion.py --mode human" neural_chess_run .  \
//       \  runs the neural engine in human-vs-AI mode and prints exit code.
//

#include <cstdlib>
#include <cstdint>
#include <string>
#include <variant>
#include <sstream>
#include <iostream>

#include "woflang.hpp"

#ifndef WOFLANG_PLUGIN_EXPORT
#define WOFLANG_PLUGIN_EXPORT extern "C"
#endif

using namespace woflang;

// ---- Minimal WofValue helpers (matched to other plugins) --------------------

static void ensure_stack_size(WoflangInterpreter &ip,
                              std::size_t         needed,
                              const char         *op_name) {
    if (ip.stack.size() < needed) {
        throw std::runtime_error(std::string(op_name) + ": stack underflow");
    }
}

static WofValue pop_raw(WoflangInterpreter &ip, const char *op_name) {
    ensure_stack_size(ip, 1, op_name);
    WofValue v = ip.stack.back();
    ip.stack.pop_back();
    return v;
}

static WofValue make_int64(int64_t v) {
    WofValue out;
    out.type  = WofType::Integer;
    out.value = v;
    return out;
}

static WofValue make_string(const std::string &s) {
    WofValue out;
    out.type  = WofType::String;
    out.value = s;
    return out;
}

static void push_int(WoflangInterpreter &ip, int64_t v) {
    ip.stack.push_back(make_int64(v));
}

static void push_string(WoflangInterpreter &ip, const std::string &s) {
    ip.stack.push_back(make_string(s));
}

static std::string to_string_value(const WofValue &v, const char *op_name) {
    if (v.type == WofType::String) {
        return std::get<std::string>(v.value);
    }
    if (v.type == WofType::Integer) {
        return std::to_string(std::get<int64_t>(v.value));
    }
    if (v.type == WofType::Double) {
        return std::to_string(std::get<double>(v.value));
    }
    throw std::runtime_error(std::string(op_name) + ": expected string or numeric");
}

// ---- Ops --------------------------------------------------------------------

static void op_neural_chess_info(WoflangInterpreter &ip) {
    (void)ip;

    std::ostringstream oss;
    oss
        << "Neural Chess \"Ganglion Brain\" overview:\n"
        << "\n"
        << "- Full chess rules (via python-chess in the original engine):\n"
        << "    castling, en passant, promotions, legal move generation.\n"
        << "- Brain is a synchronized trio:\n"
        << "    * CNN over board planes [12 x 8 x 8]\n"
        << "    * GRU over move history sequences\n"
        << "    * LSTM over a 2D Cellular Automaton (CA) grid\n"
        << "  coordinated by a 'Ganglion' fusion module.\n"
        << "- GAN-style pair:\n"
        << "    * Generator: GanglionBrain (policy + value)\n"
        << "    * Discriminator: judges (board, move) plausibility.\n"
        << "\n"
        << "Original Python entry point:\n"
        << "    neural_chess_ganglion.py --mode human|self-play\n"
        << "\n"
        << "Woflang bridge ops:\n"
        << "    neural_chess_info   ( -- description-string )\n"
        << "    neural_chess_run    ( [cmd-string] -- exit-code )\n"
        << "\n"
        << "Typical usage:\n"
        << "    \"python plugins/games/neural_chess_ganglion.py --mode human\" neural_chess_run .\n";

    push_string(ip, oss.str());
}

static void op_neural_chess_run(WoflangInterpreter &ip) {
    const char *op = "neural_chess_run";

    // Default command: adjust path/command as needed in your environment.
    std::string cmd =
        "python plugins/games/neural_chess_ganglion.py --mode human";

    // If top of stack is a string, treat it as the command instead.
    if (!ip.stack.empty() && ip.stack.back().type == WofType::String) {
        WofValue v = pop_raw(ip, op);
        cmd = to_string_value(v, op);
    }

    int status = std::system(cmd.c_str());
    push_int(ip, static_cast<int64_t>(status));
}

// ---- Plugin entry point -----------------------------------------------------

WOFLANG_PLUGIN_EXPORT void register_plugin(WoflangInterpreter &interp) {
    interp.register_op("neural_chess_info", [](WoflangInterpreter &ip) {
        op_neural_chess_info(ip);
    });

    interp.register_op("neural_chess_run", [](WoflangInterpreter &ip) {
        op_neural_chess_run(ip);
    });

    std::cout
        << "[neural_chess] Neural Chess shim loaded: "
        << "neural_chess_info neural_chess_run\n";
}
