// graph_shortest_path_ops.cpp
//
// Weighted shortest path (Dijkstra) helpers for Woflang.
//
// Public ops:
//
//   graph_w_new
//     Stack: (num_nodes:int, name:string) -> ()
//       Creates/overwrites an undirected weighted graph with the given name
//       and node count. Nodes are indexed 0..num_nodes-1.
//
//   graph_w_add_edge
//     Stack: (weight:double, v:int, u:int, name:string) -> ()
//       Adds an undirected edge (u <-> v) with the given non-negative weight
//       to the named graph.
//
//   graph_w_shortest
//     Stack: (dst:int, start:int, name:string) -> (path_string:string, distance:double)
//       Runs Dijkstra from `start` to `dst` in the named graph.
//       If there is no path, distance = -1.0 and the path_string explains why.
//

#include "woflang.hpp"

#include <map>
#include <string>
#include <vector>
#include <queue>
#include <limits>
#include <stdexcept>
#include <cstdint>
#include <sstream>
#include <algorithm>
#include <iostream> 

#ifndef WOFLANG_PLUGIN_EXPORT
#define WOFLANG_PLUGIN_EXPORT extern "C"
#endif

using namespace woflang;

// ---------- Minimal WofValue helpers (aligned with other v10 plugins) --------

static void require_stack_size(const WoflangInterpreter &ip,
                               std::size_t               n,
                               const char               *ctx) {
    if (ip.stack.size() < n) {
        throw std::runtime_error(std::string(ctx) + ": stack underflow");
    }
}

static WofValue make_int64(std::int64_t v) {
    WofValue out;
    out.type  = WofType::Integer;
    out.value = v;
    return out;
}

static WofValue make_double(double v) {
    WofValue out;
    out.type  = WofType::Double;
    out.value = v;
    return out;
}

static WofValue make_string(const std::string &s) {
    WofValue out;
    out.type  = WofType::String;
    out.value = s;
    return out;
}

static void push_int(WoflangInterpreter &ip, std::int64_t v) {
    ip.stack.push_back(make_int64(v));
}

static void push_double(WoflangInterpreter &ip, double v) {
    ip.stack.push_back(make_double(v));
}

static void push_string(WoflangInterpreter &ip, const std::string &s) {
    ip.stack.push_back(make_string(s));
}

static std::int64_t require_int(const WofValue &v, const char *ctx) {
    if (v.type == WofType::Integer) {
        return std::get<std::int64_t>(v.value);
    }
    if (v.type == WofType::Double) {
        double d = std::get<double>(v.value);
        return static_cast<std::int64_t>(d);
    }
    throw std::runtime_error(std::string(ctx) + ": expected integer");
}

static double require_double(const WofValue &v, const char *ctx) {
    if (v.type == WofType::Double) {
        return std::get<double>(v.value);
    }
    if (v.type == WofType::Integer) {
        return static_cast<double>(std::get<std::int64_t>(v.value));
    }
    throw std::runtime_error(std::string(ctx) + ": expected number");
}

static std::string require_string(const WofValue &v, const char *ctx) {
    if (v.type == WofType::String) {
        return std::get<std::string>(v.value);
    }
    throw std::runtime_error(std::string(ctx) + ": expected string");
}

// ---------- Graph storage ----------------------------------------------------

struct GraphEdge {
    int    to;
    double w;
};

struct Graph {
    bool directed = false;
    std::vector<std::vector<GraphEdge>> adj;
};

static std::map<std::string, Graph> &graphs() {
    static std::map<std::string, Graph> g;
    return g;
}

static Graph &get_graph(const std::string &name) {
    auto &gmap = graphs();
    auto  it   = gmap.find(name);
    if (it == gmap.end()) {
        throw std::runtime_error("graph_weighted: unknown graph \"" + name + "\"");
    }
    return it->second;
}

// ---------- Dijkstra ---------------------------------------------------------

static void dijkstra(const Graph        &g,
                     int                 start,
                     std::vector<double> &dist,
                     std::vector<int>    &parent) {
    const int n = static_cast<int>(g.adj.size());
    dist.assign(n, std::numeric_limits<double>::infinity());
    parent.assign(n, -1);

    using Node = std::pair<double, int>;
    auto cmp   = [](const Node &a, const Node &b) {
        return a.first > b.first;
    };
    std::priority_queue<Node, std::vector<Node>, decltype(cmp)> pq(cmp);

    dist[start] = 0.0;
    pq.push(Node{0.0, start});

    while (!pq.empty()) {
        auto [d, u] = pq.top();
        pq.pop();

        if (d > dist[u]) {
            continue;
        }

        for (const GraphEdge &e : g.adj[static_cast<std::size_t>(u)]) {
            int v = e.to;
            if (v < 0 || v >= n) {
                continue;
            }
            double nd = d + e.w;
            if (nd < dist[v]) {
                dist[v]   = nd;
                parent[v] = u;
                pq.push(Node{nd, v});
            }
        }
    }
}

// ---------- Plugin ops -------------------------------------------------------

WOFLANG_PLUGIN_EXPORT void register_plugin(WoflangInterpreter &interp) {
    // (num_nodes:int, name:string) -> ()
    interp.register_op("graph_w_new", [](WoflangInterpreter &ip) {
        const char *ctx = "graph_w_new";
        require_stack_size(ip, 2, ctx);

        // Stack top: name, below: num_nodes
        WofValue name_v = ip.stack.back();
        ip.stack.pop_back();
        WofValue n_v = ip.stack.back();
        ip.stack.pop_back();

        std::string  name = require_string(name_v, "graph_w_new: graph name");
        std::int64_t n    = require_int(n_v, "graph_w_new: num_nodes");
        if (n < 0) {
            throw std::runtime_error("graph_w_new: num_nodes must be non-negative");
        }

        Graph &g = graphs()[name];
        g.directed = false;
        g.adj.assign(static_cast<std::size_t>(n), std::vector<GraphEdge>{});
    });

    // (weight:double, v:int, u:int, name:string) -> ()
    interp.register_op("graph_w_add_edge", [](WoflangInterpreter &ip) {
        const char *ctx = "graph_w_add_edge";
        require_stack_size(ip, 4, ctx);

        // Stack top: name, then u, then v, then weight
        WofValue name_v = ip.stack.back();
        ip.stack.pop_back();
        WofValue u_v = ip.stack.back();
        ip.stack.pop_back();
        WofValue v_v = ip.stack.back();
        ip.stack.pop_back();
        WofValue w_v = ip.stack.back();
        ip.stack.pop_back();

        std::string  name = require_string(name_v, "graph_w_add_edge: graph name");
        std::int64_t u    = require_int(u_v, "graph_w_add_edge: u");
        std::int64_t v    = require_int(v_v, "graph_w_add_edge: v");
        double       w    = require_double(w_v, "graph_w_add_edge: weight");

        if (w < 0.0) {
            throw std::runtime_error("graph_w_add_edge: negative weights not allowed for Dijkstra");
        }

        Graph &g = get_graph(name);
        auto   n_nodes = static_cast<std::int64_t>(g.adj.size());
        if (u < 0 || v < 0 || u >= n_nodes || v >= n_nodes) {
            throw std::runtime_error("graph_w_add_edge: node index out of range");
        }

        std::size_t ui = static_cast<std::size_t>(u);
        std::size_t vi = static_cast<std::size_t>(v);

        g.adj[ui].push_back(GraphEdge{static_cast<int>(v), w});
        if (!g.directed) {
            g.adj[vi].push_back(GraphEdge{static_cast<int>(u), w});
        }
    });

    // (dst:int, start:int, name:string) -> (path_string:string, distance:double)
    interp.register_op("graph_w_shortest", [](WoflangInterpreter &ip) {
        const char *ctx = "graph_w_shortest";
        require_stack_size(ip, 3, ctx);

        // Stack top: name, then start, then dst
        WofValue name_v  = ip.stack.back();
        ip.stack.pop_back();
        WofValue start_v = ip.stack.back();
        ip.stack.pop_back();
        WofValue dst_v   = ip.stack.back();
        ip.stack.pop_back();

        std::string  name  = require_string(name_v,  "graph_w_shortest: graph name");
        std::int64_t start = require_int(start_v, "graph_w_shortest: start");
        std::int64_t dst   = require_int(dst_v,   "graph_w_shortest: dst");

        Graph &g = get_graph(name);
        auto   n_nodes = static_cast<std::int64_t>(g.adj.size());

        if (n_nodes == 0) {
            std::ostringstream oss;
            oss << "graph " << name << " is empty; no path";
            push_string(ip, oss.str());
            push_double(ip, -1.0);
            return;
        }
        if (start < 0 || start >= n_nodes || dst < 0 || dst >= n_nodes) {
            throw std::runtime_error("graph_w_shortest: node index out of range");
        }

        int s = static_cast<int>(start);
        int t = static_cast<int>(dst);

        std::vector<double> dist;
        std::vector<int>    parent;
        dijkstra(g, s, dist, parent);

        double d = dist[t];
        if (d == std::numeric_limits<double>::infinity()) {
            std::ostringstream oss;
            oss << "no path from " << s << " to " << t << " in graph " << name;
            push_string(ip, oss.str());
            push_double(ip, -1.0);
            return;
        }

        // Reconstruct path.
        std::vector<int> path;
        for (int cur = t; cur != -1; cur = parent[cur]) {
            path.push_back(cur);
        }
        std::reverse(path.begin(), path.end());

        std::ostringstream oss;
        oss << "shortest path in " << name << ": ";
        for (std::size_t i = 0; i < path.size(); ++i) {
            if (i > 0) {
                oss << " -> ";
            }
            oss << path[i];
        }
        oss << " (dist=" << d << ")";

        push_string(ip, oss.str());
        push_double(ip, d);
    });

    std::cout
        << "[graph] Weighted shortest-path ops loaded: "
        << "graph_w_new graph_w_add_edge graph_w_shortest\n";
}
