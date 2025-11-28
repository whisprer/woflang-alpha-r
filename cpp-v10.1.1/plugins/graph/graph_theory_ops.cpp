// plugins/graph/graph_theory_ops.cpp
#include "woflang.hpp"

#include <map>
#include <string>
#include <vector>
#include <stdexcept>
#include <cstdint>

#ifndef WOFLANG_PLUGIN_EXPORT
#define WOFLANG_PLUGIN_EXPORT extern "C"
#endif

using namespace woflang;

namespace {

struct Graph {
    bool directed = false;
    std::vector<std::vector<int>> adj;
};

std::map<std::string, Graph>& graphs() {
    static std::map<std::string, Graph> g;
    return g;
}

Graph& get_graph(const std::string& name) {
    auto& gmap = graphs();
    auto it = gmap.find(name);
    if (it == gmap.end()) {
        throw std::runtime_error("graph: unknown graph name \"" + name + "\"");
    }
    return it->second;
}

// Helpers that only touch WofValue::type and .value
std::int64_t require_int(const WofValue& v, const char* ctx) {
    if (v.type == WofType::Integer) {
        return std::get<std::int64_t>(v.value);
    }
    if (v.type == WofType::Double) {
        double d = std::get<double>(v.value);
        return static_cast<std::int64_t>(d);
    }
    throw std::runtime_error(std::string(ctx) + ": expected integer");
}

std::string require_string(const WofValue& v, const char* ctx) {
    if (v.type == WofType::String) {
        return std::get<std::string>(v.value);
    }
    throw std::runtime_error(std::string(ctx) + ": expected string");
}

void require_stack_size(const WoflangInterpreter& ip, std::size_t n, const char* ctx) {
    if (ip.stack.size() < n) {
        throw std::runtime_error(std::string(ctx) + ": stack underflow");
    }
}

void register_graph_theory_ops(WoflangInterpreter& interp) {
    // Create / reset a graph: (num_nodes:int, name:string) -> ()
    interp.register_op("graph_new", [](WoflangInterpreter& ip) {
        require_stack_size(ip, 2, "graph_new");

        WofValue name_v = ip.stack.back();
        ip.stack.pop_back();
        WofValue n_v = ip.stack.back();
        ip.stack.pop_back();

        std::string name = require_string(name_v, "graph_new: graph name");
        std::int64_t n = require_int(n_v, "graph_new: num_nodes");
        if (n < 0) {
            throw std::runtime_error("graph_new: num_nodes must be non-negative");
        }

        Graph& g = graphs()[name];
        g.directed = false;
        g.adj.assign(static_cast<std::size_t>(n), std::vector<int>{});
    });

    // Add an undirected edge: (u:int, v:int, name:string) -> ()
    interp.register_op("graph_add_edge", [](WoflangInterpreter& ip) {
        require_stack_size(ip, 3, "graph_add_edge");

        WofValue name_v = ip.stack.back();
        ip.stack.pop_back();
        WofValue v_v = ip.stack.back();
        ip.stack.pop_back();
        WofValue u_v = ip.stack.back();
        ip.stack.pop_back();

        std::string name = require_string(name_v, "graph_add_edge: graph name");
        std::int64_t u = require_int(u_v, "graph_add_edge: u");
        std::int64_t v = require_int(v_v, "graph_add_edge: v");

        Graph& g = get_graph(name);
        auto n = static_cast<std::int64_t>(g.adj.size());
        if (u < 0 || v < 0 || u >= n || v >= n) {
            throw std::runtime_error("graph_add_edge: node index out of range");
        }

        auto ui = static_cast<std::size_t>(u);
        auto vi = static_cast<std::size_t>(v);

        g.adj[ui].push_back(static_cast<int>(v));
        if (!g.directed && u != v) {
            g.adj[vi].push_back(static_cast<int>(u));
        }
    });

    // Degree of a node: (node:int, name:string) -> (degree:int)
    interp.register_op("graph_degree", [](WoflangInterpreter& ip) {
        require_stack_size(ip, 2, "graph_degree");

        WofValue name_v = ip.stack.back();
        ip.stack.pop_back();
        WofValue node_v = ip.stack.back();
        ip.stack.pop_back();

        std::string name = require_string(name_v, "graph_degree: graph name");
        std::int64_t node = require_int(node_v, "graph_degree: node");

        Graph& g = get_graph(name);
        auto n = static_cast<std::int64_t>(g.adj.size());
        if (node < 0 || node >= n) {
            throw std::runtime_error("graph_degree: node index out of range");
        }

        auto deg = static_cast<std::int64_t>(
            g.adj[static_cast<std::size_t>(node)].size()
        );

        WofValue out;
        out.type  = WofType::Integer;
        out.value = deg;
        ip.stack.push_back(out);
    });

    // Clear graph: (name:string) -> ()
    interp.register_op("graph_clear", [](WoflangInterpreter& ip) {
        require_stack_size(ip, 1, "graph_clear");

        WofValue name_v = ip.stack.back();
        ip.stack.pop_back();

        std::string name = require_string(name_v, "graph_clear: graph name");
        graphs().erase(name);
    });
}

} // namespace

WOFLANG_PLUGIN_EXPORT void register_plugin(WoflangInterpreter& interp) {
    register_graph_theory_ops(interp);
}
