// plugins/graph/graph_search_ops.cpp
#include "woflang.hpp"

#include <map>
#include <string>
#include <vector>
#include <queue>
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

// Separate registry for unweighted graphs used by search ops
static std::map<std::string, Graph>& graphs() {
    static std::map<std::string, Graph> g;
    return g;
}

static Graph& get_graph(const std::string& name) {
    auto& gmap = graphs();
    auto it = gmap.find(name);
    if (it == gmap.end()) {
        throw std::runtime_error("graph_search: unknown graph \"" + name + "\"");
    }
    return it->second;
}

// Minimal helpers (v10 style)
static void require_stack_size(const WoflangInterpreter& ip, std::size_t n, const char* ctx) {
    if (ip.stack.size() < n) throw std::runtime_error(std::string(ctx) + ": stack underflow");
}
static std::int64_t require_int(const WofValue& v, const char* ctx) {
    if (v.type == WofType::Integer) return std::get<std::int64_t>(v.value);
    if (v.type == WofType::Double)  return static_cast<std::int64_t>(std::get<double>(v.value));
    throw std::runtime_error(std::string(ctx) + ": expected integer");
}
static std::string require_string(const WofValue& v, const char* ctx) {
    if (v.type == WofType::String) return std::get<std::string>(v.value);
    throw std::runtime_error(std::string(ctx) + ": expected string");
}

static int bfs_reach(const Graph& g, int start) {
    const int n = static_cast<int>(g.adj.size());
    if (n == 0) return 0;
    std::vector<bool> vis(n, false);
    std::queue<int> q;
    vis[start] = true;
    q.push(start);
    int count = 0;
    while (!q.empty()) {
        int u = q.front(); q.pop();
        ++count;
        for (int v : g.adj[static_cast<std::size_t>(u)]) {
            if (v < 0 || v >= n) continue;
            if (!vis[v]) { vis[v] = true; q.push(v); }
        }
    }
    return count;
}

static int bfs_shortest_path(const Graph& g, int start, int dst) {
    const int n = static_cast<int>(g.adj.size());
    if (n == 0) return -1;
    if (start == dst) return 0;
    std::vector<int> dist(n, -1);
    std::queue<int> q;
    dist[start] = 0;
    q.push(start);
    while (!q.empty()) {
        int u = q.front(); q.pop();
        for (int v : g.adj[static_cast<std::size_t>(u)]) {
            if (v < 0 || v >= n) continue;
            if (dist[v] == -1) {
                dist[v] = dist[u] + 1;
                if (v == dst) return dist[v];
                q.push(v);
            }
        }
    }
    return -1;
}

} // namespace

WOFLANG_PLUGIN_EXPORT void register_plugin(WoflangInterpreter& interp) {
    // BFS reachability count
    interp.register_op("graph_bfs_reach", [](WoflangInterpreter& ip) {
        const char* ctx = "graph_bfs_reach";
        require_stack_size(ip, 2, ctx);

        WofValue name_v  = ip.stack.back(); ip.stack.pop_back();
        WofValue start_v = ip.stack.back(); ip.stack.pop_back();

        std::string  name  = require_string(name_v,  "graph_bfs_reach: graph name");
        std::int64_t start = require_int(start_v,    "graph_bfs_reach: start");

        Graph& g = get_graph(name);
        auto n = static_cast<std::int64_t>(g.adj.size());
        if (n == 0) {
            WofValue out; out.type = WofType::Integer; out.value = static_cast<std::int64_t>(0);
            ip.stack.push_back(out);
            return;
        }
        if (start < 0 || start >= n) throw std::runtime_error("graph_bfs_reach: start index out of range");

        int count = bfs_reach(g, static_cast<int>(start));
        WofValue out; out.type = WofType::Integer; out.value = static_cast<std::int64_t>(count);
        ip.stack.push_back(out);
    });

    // Path existence (0/1)
    interp.register_op("graph_path_exists", [](WoflangInterpreter& ip) {
        const char* ctx = "graph_path_exists";
        require_stack_size(ip, 3, ctx);

        WofValue name_v  = ip.stack.back(); ip.stack.pop_back();
        WofValue start_v = ip.stack.back(); ip.stack.pop_back();
        WofValue dst_v   = ip.stack.back(); ip.stack.pop_back();

        std::string  name  = require_string(name_v,  "graph_path_exists: graph name");
        std::int64_t start = require_int(start_v,    "graph_path_exists: start");
        std::int64_t dst   = require_int(dst_v,      "graph_path_exists: dst");

        Graph& g = get_graph(name);
        auto n = static_cast<std::int64_t>(g.adj.size());
        if (start < 0 || start >= n || dst < 0 || dst >= n)
            throw std::runtime_error("graph_path_exists: node index out of range");

        int dist = bfs_shortest_path(g, static_cast<int>(start), static_cast<int>(dst));
        WofValue out; out.type = WofType::Integer; out.value = static_cast<std::int64_t>(dist >= 0 ? 1 : 0);
        ip.stack.push_back(out);
    });

    // Unweighted shortest-path length (edges)
    interp.register_op("graph_shortest_path_len", [](WoflangInterpreter& ip) {
        const char* ctx = "graph_shortest_path_len";
        require_stack_size(ip, 3, ctx);

        WofValue name_v  = ip.stack.back(); ip.stack.pop_back();
        WofValue start_v = ip.stack.back(); ip.stack.pop_back();
        WofValue dst_v   = ip.stack.back(); ip.stack.pop_back();

        std::string  name  = require_string(name_v,  "graph_shortest_path_len: graph name");
        std::int64_t start = require_int(start_v,    "graph_shortest_path_len: start");
        std::int64_t dst   = require_int(dst_v,      "graph_shortest_path_len: dst");

        Graph& g = get_graph(name);
        auto n = static_cast<std::int64_t>(g.adj.size());
        if (start < 0 || start >= n || dst < 0 || dst >= n)
            throw std::runtime_error("graph_shortest_path_len: node index out of range");

        int dist = bfs_shortest_path(g, static_cast<int>(start), static_cast<int>(dst));
        WofValue out; out.type = WofType::Integer; out.value = static_cast<std::int64_t>(dist);
        ip.stack.push_back(out);
    });
}
