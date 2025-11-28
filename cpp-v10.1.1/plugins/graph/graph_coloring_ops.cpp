// plugins/graph/graph_coloring_ops.cpp
#include "woflang.hpp"

#include <map>
#include <string>
#include <vector>
#include <stdexcept>
#include <cstdint>
#include <algorithm>
#include <sstream>

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
        throw std::runtime_error("graph_coloring: unknown graph \"" + name + "\"");
    }
    return it->second;
}

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

void register_graph_coloring_ops(WoflangInterpreter& interp) {
    // (num_nodes:int, name:string) -> ()
    interp.register_op("graph_col_new", [](WoflangInterpreter& ip) {
        require_stack_size(ip, 2, "graph_col_new");

        WofValue name_v = ip.stack.back();
        ip.stack.pop_back();
        WofValue n_v = ip.stack.back();
        ip.stack.pop_back();

        std::string name = require_string(name_v, "graph_col_new: graph name");
        std::int64_t n = require_int(n_v, "graph_col_new: num_nodes");
        if (n < 0) {
            throw std::runtime_error("graph_col_new: num_nodes must be non-negative");
        }

        Graph& g = graphs()[name];
        g.directed = false;
        g.adj.assign(static_cast<std::size_t>(n), std::vector<int>{});
    });

    // (u:int, v:int, name:string) -> ()
    interp.register_op("graph_col_add_edge", [](WoflangInterpreter& ip) {
        require_stack_size(ip, 3, "graph_col_add_edge");

        WofValue name_v = ip.stack.back();
        ip.stack.pop_back();
        WofValue v_v = ip.stack.back();
        ip.stack.pop_back();
        WofValue u_v = ip.stack.back();
        ip.stack.pop_back();

        std::string name = require_string(name_v, "graph_col_add_edge: graph name");
        std::int64_t u = require_int(u_v, "graph_col_add_edge: u");
        std::int64_t v = require_int(v_v, "graph_col_add_edge: v");

        Graph& g = get_graph(name);
        auto n = static_cast<std::int64_t>(g.adj.size());
        if (u < 0 || v < 0 || u >= n || v >= n) {
            throw std::runtime_error("graph_col_add_edge: node index out of range");
        }

        auto ui = static_cast<std::size_t>(u);
        auto vi = static_cast<std::size_t>(v);

        g.adj[ui].push_back(static_cast<int>(v));
        if (!g.directed && u != v) {
            g.adj[vi].push_back(static_cast<int>(u));
        }
    });

    // (name:string) -> (summary:string, num_colors:int)
    interp.register_op("graph_color_greedy", [](WoflangInterpreter& ip) {
        require_stack_size(ip, 1, "graph_color_greedy");

        WofValue name_v = ip.stack.back();
        ip.stack.pop_back();

        std::string name = require_string(name_v, "graph_color_greedy: graph name");
        Graph& g = get_graph(name);

        std::size_t n = g.adj.size();
        if (n == 0) {
            // summary string
            {
                WofValue s;
                s.type  = WofType::String;
                s.value = std::string("graph ") + name + " is empty; no colours needed";
                ip.stack.push_back(s);
            }
            // num_colors = 0
            {
                WofValue c;
                c.type  = WofType::Integer;
                c.value = static_cast<std::int64_t>(0);
                ip.stack.push_back(c);
            }
            return;
        }

        // Order vertices by decreasing degree (Welsh–Powell heuristic)
        std::vector<int> order;
        order.reserve(n);
        for (std::size_t i = 0; i < n; ++i) {
            order.push_back(static_cast<int>(i));
        }
        std::sort(order.begin(), order.end(),
                  [&g](int a, int b) {
                      return g.adj[static_cast<std::size_t>(a)].size() >
                             g.adj[static_cast<std::size_t>(b)].size();
                  });

        std::vector<int> color(n, -1);
        int max_color = -1;

        for (int u : order) {
            std::vector<bool> used(static_cast<std::size_t>(n), false);
            for (int v : g.adj[static_cast<std::size_t>(u)]) {
                if (v < 0 || static_cast<std::size_t>(v) >= n) continue;
                int c = color[static_cast<std::size_t>(v)];
                if (c >= 0 && static_cast<std::size_t>(c) < used.size()) {
                    used[static_cast<std::size_t>(c)] = true;
                }
            }

            int c = 0;
            while (c < static_cast<int>(n) && used[static_cast<std::size_t>(c)]) {
                ++c;
            }
            color[static_cast<std::size_t>(u)] = c;
            if (c > max_color) max_color = c;
        }

        int num_colors = max_color + 1;

        std::ostringstream oss;
        oss << "graph " << name << " colouring (greedy):\n";
        for (std::size_t i = 0; i < n; ++i) {
            oss << "  " << i << " -> c" << color[i] << "\n";
        }
        oss << "total colours used: " << num_colors;

        // Push summary string
        {
            WofValue s;
            s.type  = WofType::String;
            s.value = oss.str();
            ip.stack.push_back(s);
        }
        // Push num_colors
        {
            WofValue c;
            c.type  = WofType::Integer;
            c.value = static_cast<std::int64_t>(num_colors);
            ip.stack.push_back(c);
        }
    });

    // (name:string) -> ()
    interp.register_op("graph_col_clear", [](WoflangInterpreter& ip) {
        require_stack_size(ip, 1, "graph_col_clear");

        WofValue name_v = ip.stack.back();
        ip.stack.pop_back();

        std::string name = require_string(name_v, "graph_col_clear: graph name");
        graphs().erase(name);
    });
}

} // namespace

WOFLANG_PLUGIN_EXPORT void register_plugin(WoflangInterpreter& interp) {
    register_graph_coloring_ops(interp);
}
