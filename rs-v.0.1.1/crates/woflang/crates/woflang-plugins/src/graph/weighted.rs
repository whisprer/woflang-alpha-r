//! Weighted graph operations for Woflang.
//!
//! Provides weighted graph creation and Dijkstra's shortest path algorithm.

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::sync::{Arc, RwLock};
use woflang_core::{WofError, WofValue, InterpreterContext};
use woflang_runtime::Interpreter;

/// A weighted edge.
#[derive(Debug, Clone)]
pub struct Edge {
    pub to: usize,
    pub weight: f64,
}

/// A weighted graph.
#[derive(Debug, Clone, Default)]
pub struct WeightedGraph {
    pub directed: bool,
    pub adj: Vec<Vec<Edge>>,
}

impl WeightedGraph {
    /// Create a new weighted graph with n nodes.
    pub fn new(n: usize, directed: bool) -> Self {
        Self {
            directed,
            adj: vec![Vec::new(); n],
        }
    }

    /// Number of nodes.
    pub fn node_count(&self) -> usize {
        self.adj.len()
    }

    /// Add a weighted edge from u to v.
    pub fn add_edge(&mut self, u: usize, v: usize, weight: f64) {
        if u < self.adj.len() && v < self.adj.len() {
            self.adj[u].push(Edge { to: v, weight });
            if !self.directed {
                self.adj[v].push(Edge { to: u, weight });
            }
        }
    }
}

/// State for Dijkstra's priority queue.
#[derive(Debug, Clone)]
struct State {
    cost: f64,
    node: usize,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost && self.node == other.node
    }
}

impl Eq for State {}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for min-heap
        other.cost.partial_cmp(&self.cost).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Run Dijkstra's algorithm, returns (distances, parents).
fn dijkstra(graph: &WeightedGraph, start: usize) -> (Vec<f64>, Vec<Option<usize>>) {
    let n = graph.node_count();
    let mut dist = vec![f64::INFINITY; n];
    let mut parent: Vec<Option<usize>> = vec![None; n];
    let mut heap = BinaryHeap::new();

    dist[start] = 0.0;
    heap.push(State { cost: 0.0, node: start });

    while let Some(State { cost, node: u }) = heap.pop() {
        if cost > dist[u] {
            continue; // Skip outdated entry
        }

        for edge in &graph.adj[u] {
            let v = edge.to;
            if v >= n {
                continue;
            }
            let new_dist = cost + edge.weight;
            if new_dist < dist[v] {
                dist[v] = new_dist;
                parent[v] = Some(u);
                heap.push(State { cost: new_dist, node: v });
            }
        }
    }

    (dist, parent)
}

/// Reconstruct path from parent array.
fn reconstruct_path(parent: &[Option<usize>], start: usize, end: usize) -> Vec<usize> {
    let mut path = Vec::new();
    let mut current = end;
    
    while current != start {
        path.push(current);
        match parent[current] {
            Some(p) => current = p,
            None => return Vec::new(), // No path
        }
    }
    path.push(start);
    path.reverse();
    path
}

/// Global weighted graph storage.
type WGraphStore = Arc<RwLock<HashMap<String, WeightedGraph>>>;

fn get_store() -> WGraphStore {
    use std::sync::OnceLock;
    static STORE: OnceLock<WGraphStore> = OnceLock::new();
    STORE.get_or_init(|| Arc::new(RwLock::new(HashMap::new()))).clone()
}

fn get_wgraph(name: &str) -> Result<WeightedGraph, WofError> {
    let store = get_store();
    let guard = store.read().map_err(|_| WofError::Runtime("wgraph lock poisoned".into()))?;
    guard.get(name).cloned().ok_or_else(|| {
        WofError::Runtime(format!("weighted graph: unknown graph '{}'", name))
    })
}

fn set_wgraph(name: &str, graph: WeightedGraph) -> Result<(), WofError> {
    let store = get_store();
    let mut guard = store.write().map_err(|_| WofError::Runtime("wgraph lock poisoned".into()))?;
    guard.insert(name.to_string(), graph);
    Ok(())
}

fn with_wgraph_mut<F, R>(name: &str, f: F) -> Result<R, WofError>
where
    F: FnOnce(&mut WeightedGraph) -> Result<R, WofError>,
{
    let store = get_store();
    let mut guard = store.write().map_err(|_| WofError::Runtime("wgraph lock poisoned".into()))?;
    let graph = guard.get_mut(name).ok_or_else(|| {
        WofError::Runtime(format!("weighted graph: unknown graph '{}'", name))
    })?;
    f(graph)
}

/// Register weighted graph operations.
pub fn register(interp: &mut Interpreter) {
    // ═══════════════════════════════════════════════════════════════
    // WEIGHTED GRAPH CREATION
    // ═══════════════════════════════════════════════════════════════
    
    // Create a new weighted undirected graph
    // Stack: num_nodes name → ()
    interp.register("graph_w_new", |interp| {
        let name = interp.stack_mut().pop()?.as_string()?;
        let n = interp.stack_mut().pop()?.as_integer()?;
        
        if n < 0 {
            return Err(WofError::Runtime("graph_w_new: num_nodes must be >= 0".into()));
        }
        
        let graph = WeightedGraph::new(n as usize, false);
        set_wgraph(&name, graph)?;
        Ok(())
    });

    // Create a new weighted directed graph
    // Stack: num_nodes name → ()
    interp.register("digraph_w_new", |interp| {
        let name = interp.stack_mut().pop()?.as_string()?;
        let n = interp.stack_mut().pop()?.as_integer()?;
        
        if n < 0 {
            return Err(WofError::Runtime("digraph_w_new: num_nodes must be >= 0".into()));
        }
        
        let graph = WeightedGraph::new(n as usize, true);
        set_wgraph(&name, graph)?;
        Ok(())
    });

    // ═══════════════════════════════════════════════════════════════
    // WEIGHTED EDGE OPERATIONS
    // ═══════════════════════════════════════════════════════════════
    
    // Add a weighted edge
    // Stack: weight v u name → ()
    interp.register("graph_w_add_edge", |interp| {
        let name = interp.stack_mut().pop()?.as_string()?;
        let u = interp.stack_mut().pop()?.as_integer()? as usize;
        let v = interp.stack_mut().pop()?.as_integer()? as usize;
        let weight = interp.stack_mut().pop()?.as_double()?;
        
        if weight < 0.0 {
            return Err(WofError::Runtime("graph_w_add_edge: negative weights not allowed for Dijkstra".into()));
        }
        
        with_wgraph_mut(&name, |g| {
            let n = g.node_count();
            if u >= n || v >= n {
                return Err(WofError::Runtime("graph_w_add_edge: node index out of range".into()));
            }
            g.add_edge(u, v, weight);
            Ok(())
        })
    });

    // ═══════════════════════════════════════════════════════════════
    // DIJKSTRA'S ALGORITHM
    // ═══════════════════════════════════════════════════════════════
    
    // Find shortest path (Dijkstra)
    // Stack: dst start name → path_string distance
    interp.register("graph_w_shortest", |interp| {
        let name = interp.stack_mut().pop()?.as_string()?;
        let start = interp.stack_mut().pop()?.as_integer()? as usize;
        let dst = interp.stack_mut().pop()?.as_integer()? as usize;
        
        let graph = get_wgraph(&name)?;
        let n = graph.node_count();
        
        if n == 0 {
            interp.stack_mut().push(WofValue::string(format!("graph {} is empty; no path", name)));
            interp.stack_mut().push(WofValue::double(-1.0));
            return Ok(());
        }
        
        if start >= n || dst >= n {
            return Err(WofError::Runtime("graph_w_shortest: node index out of range".into()));
        }
        
        let (dist, parent) = dijkstra(&graph, start);
        let d = dist[dst];
        
        if d.is_infinite() {
            interp.stack_mut().push(WofValue::string(format!("no path from {} to {} in graph {}", start, dst, name)));
            interp.stack_mut().push(WofValue::double(-1.0));
            return Ok(());
        }
        
        let path = reconstruct_path(&parent, start, dst);
        let path_str = format!(
            "shortest path in {}: {} (dist={})",
            name,
            path.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" -> "),
            d
        );
        
        interp.stack_mut().push(WofValue::string(path_str));
        interp.stack_mut().push(WofValue::double(d));
        Ok(())
    });

    // Get just the shortest distance
    // Stack: dst start name → distance
    interp.register("graph_w_distance", |interp| {
        let name = interp.stack_mut().pop()?.as_string()?;
        let start = interp.stack_mut().pop()?.as_integer()? as usize;
        let dst = interp.stack_mut().pop()?.as_integer()? as usize;
        
        let graph = get_wgraph(&name)?;
        let n = graph.node_count();
        
        if start >= n || dst >= n {
            return Err(WofError::Runtime("graph_w_distance: node index out of range".into()));
        }
        
        let (dist, _) = dijkstra(&graph, start);
        let d = dist[dst];
        
        if d.is_infinite() {
            interp.stack_mut().push(WofValue::double(-1.0));
        } else {
            interp.stack_mut().push(WofValue::double(d));
        }
        Ok(())
    });

    // Clear weighted graph
    // Stack: name → ()
    interp.register("graph_w_clear", |interp| {
        let name = interp.stack_mut().pop()?.as_string()?;
        let store = get_store();
        let mut guard = store.write().map_err(|_| WofError::Runtime("wgraph lock poisoned".into()))?;
        guard.remove(&name);
        Ok(())
    });
}
