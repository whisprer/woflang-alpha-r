//! Graph search algorithms for Woflang.
//!
//! Provides BFS, DFS, reachability, and path existence queries.

use std::collections::VecDeque;
use woflang_core::{WofError, WofValue, InterpreterContext};
use woflang_runtime::Interpreter;

use super::core::{get_graph, Graph};

/// BFS traversal, returns number of reachable nodes.
fn bfs_reach(graph: &Graph, start: usize) -> usize {
    let n = graph.node_count();
    if n == 0 || start >= n {
        return 0;
    }

    let mut visited = vec![false; n];
    let mut queue = VecDeque::new();
    
    visited[start] = true;
    queue.push_back(start);
    let mut count = 0;

    while let Some(u) = queue.pop_front() {
        count += 1;
        for &v in graph.neighbors(u) {
            if v < n && !visited[v] {
                visited[v] = true;
                queue.push_back(v);
            }
        }
    }

    count
}

/// BFS shortest path (unweighted), returns distance or -1 if unreachable.
fn bfs_distance(graph: &Graph, start: usize, end: usize) -> i64 {
    let n = graph.node_count();
    if n == 0 || start >= n || end >= n {
        return -1;
    }
    if start == end {
        return 0;
    }

    let mut dist = vec![-1i64; n];
    let mut queue = VecDeque::new();
    
    dist[start] = 0;
    queue.push_back(start);

    while let Some(u) = queue.pop_front() {
        for &v in graph.neighbors(u) {
            if v < n && dist[v] == -1 {
                dist[v] = dist[u] + 1;
                if v == end {
                    return dist[v];
                }
                queue.push_back(v);
            }
        }
    }

    -1 // Unreachable
}

/// DFS traversal, returns nodes in DFS order.
fn dfs_order(graph: &Graph, start: usize) -> Vec<usize> {
    let n = graph.node_count();
    if n == 0 || start >= n {
        return Vec::new();
    }

    let mut visited = vec![false; n];
    let mut order = Vec::new();
    let mut stack = vec![start];

    while let Some(u) = stack.pop() {
        if visited[u] {
            continue;
        }
        visited[u] = true;
        order.push(u);

        // Add neighbors in reverse order for correct DFS order
        for &v in graph.neighbors(u).iter().rev() {
            if v < n && !visited[v] {
                stack.push(v);
            }
        }
    }

    order
}

/// Check if graph is connected (for undirected) or weakly connected (ignoring direction).
fn is_connected(graph: &Graph) -> bool {
    let n = graph.node_count();
    if n == 0 {
        return true;
    }
    bfs_reach(graph, 0) == n
}

/// Find all connected components, returns vector of component sizes.
fn connected_components(graph: &Graph) -> Vec<usize> {
    let n = graph.node_count();
    if n == 0 {
        return Vec::new();
    }

    let mut visited = vec![false; n];
    let mut components = Vec::new();

    for start in 0..n {
        if visited[start] {
            continue;
        }

        let mut queue = VecDeque::new();
        queue.push_back(start);
        visited[start] = true;
        let mut size = 0;

        while let Some(u) = queue.pop_front() {
            size += 1;
            for &v in graph.neighbors(u) {
                if v < n && !visited[v] {
                    visited[v] = true;
                    queue.push_back(v);
                }
            }
        }

        components.push(size);
    }

    components
}

/// Register graph search operations.
pub fn register(interp: &mut Interpreter) {
    // ═══════════════════════════════════════════════════════════════
    // BFS OPERATIONS
    // ═══════════════════════════════════════════════════════════════
    
    // Count reachable nodes from start via BFS
    // Stack: start name → count
    interp.register("graph_bfs_reach", |interp| {
        let name = interp.stack_mut().pop()?.as_string()?;
        let start = interp.stack_mut().pop()?.as_integer()? as usize;
        
        let graph = get_graph(&name)?;
        if start >= graph.node_count() {
            return Err(WofError::Runtime("graph_bfs_reach: start index out of range".into()));
        }
        
        let count = bfs_reach(&graph, start);
        interp.stack_mut().push(WofValue::integer(count as i64));
        Ok(())
    });

    // Check if path exists between two nodes
    // Stack: dst start name → 1|0
    interp.register("graph_path_exists", |interp| {
        let name = interp.stack_mut().pop()?.as_string()?;
        let start = interp.stack_mut().pop()?.as_integer()? as usize;
        let dst = interp.stack_mut().pop()?.as_integer()? as usize;
        
        let graph = get_graph(&name)?;
        let n = graph.node_count();
        if start >= n || dst >= n {
            return Err(WofError::Runtime("graph_path_exists: node index out of range".into()));
        }
        
        let dist = bfs_distance(&graph, start, dst);
        interp.stack_mut().push(WofValue::integer(if dist >= 0 { 1 } else { 0 }));
        Ok(())
    });

    // Get shortest path length (unweighted, in edges)
    // Stack: dst start name → distance (-1 if unreachable)
    interp.register("graph_shortest_path_len", |interp| {
        let name = interp.stack_mut().pop()?.as_string()?;
        let start = interp.stack_mut().pop()?.as_integer()? as usize;
        let dst = interp.stack_mut().pop()?.as_integer()? as usize;
        
        let graph = get_graph(&name)?;
        let n = graph.node_count();
        if start >= n || dst >= n {
            return Err(WofError::Runtime("graph_shortest_path_len: node index out of range".into()));
        }
        
        let dist = bfs_distance(&graph, start, dst);
        interp.stack_mut().push(WofValue::integer(dist));
        Ok(())
    });

    // ═══════════════════════════════════════════════════════════════
    // DFS OPERATIONS
    // ═══════════════════════════════════════════════════════════════
    
    // Get DFS traversal order, push count of visited nodes
    // Stack: start name → count
    interp.register("graph_dfs_reach", |interp| {
        let name = interp.stack_mut().pop()?.as_string()?;
        let start = interp.stack_mut().pop()?.as_integer()? as usize;
        
        let graph = get_graph(&name)?;
        if start >= graph.node_count() {
            return Err(WofError::Runtime("graph_dfs_reach: start index out of range".into()));
        }
        
        let order = dfs_order(&graph, start);
        interp.stack_mut().push(WofValue::integer(order.len() as i64));
        Ok(())
    });

    // ═══════════════════════════════════════════════════════════════
    // CONNECTIVITY
    // ═══════════════════════════════════════════════════════════════
    
    // Check if graph is connected
    // Stack: name → 1|0
    interp.register("graph_connected?", |interp| {
        let name = interp.stack_mut().pop()?.as_string()?;
        let graph = get_graph(&name)?;
        let connected = is_connected(&graph);
        interp.stack_mut().push(WofValue::integer(if connected { 1 } else { 0 }));
        Ok(())
    });

    // Count connected components
    // Stack: name → count
    interp.register("graph_components", |interp| {
        let name = interp.stack_mut().pop()?.as_string()?;
        let graph = get_graph(&name)?;
        let components = connected_components(&graph);
        interp.stack_mut().push(WofValue::integer(components.len() as i64));
        Ok(())
    });
}
