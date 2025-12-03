//! Core graph structure and basic operations.
//!
//! Provides graph creation, edge addition, degree queries, and management.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use woflang_core::{WofError, WofValue};
use woflang_runtime::Interpreter;

/// An unweighted graph with adjacency list representation.
#[derive(Debug, Clone, Default)]
pub struct Graph {
    pub directed: bool,
    pub adj: Vec<Vec<usize>>,
}

impl Graph {
    /// Create a new graph with n nodes.
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

    /// Add an edge from u to v.
    pub fn add_edge(&mut self, u: usize, v: usize) {
        if u < self.adj.len() && v < self.adj.len() {
            self.adj[u].push(v);
            if !self.directed && u != v {
                self.adj[v].push(u);
            }
        }
    }

    /// Get degree of a node.
    pub fn degree(&self, node: usize) -> usize {
        self.adj.get(node).map(|v| v.len()).unwrap_or(0)
    }

    /// Get neighbors of a node.
    pub fn neighbors(&self, node: usize) -> &[usize] {
        self.adj.get(node).map(|v| v.as_slice()).unwrap_or(&[])
    }
}

/// Global graph storage (thread-safe).
type GraphStore = Arc<RwLock<HashMap<String, Graph>>>;

fn get_store() -> GraphStore {
    use std::sync::OnceLock;
    static STORE: OnceLock<GraphStore> = OnceLock::new();
    STORE.get_or_init(|| Arc::new(RwLock::new(HashMap::new()))).clone()
}

/// Get a graph by name (cloned).
pub fn get_graph(name: &str) -> Result<Graph, WofError> {
    let store = get_store();
    let guard = store.read().map_err(|_| WofError::Runtime("graph lock poisoned".into()))?;
    guard.get(name).cloned().ok_or_else(|| {
        WofError::Runtime(format!("graph: unknown graph '{}'", name))
    })
}

/// Store a graph by name.
pub fn set_graph(name: &str, graph: Graph) -> Result<(), WofError> {
    let store = get_store();
    let mut guard = store.write().map_err(|_| WofError::Runtime("graph lock poisoned".into()))?;
    guard.insert(name.to_string(), graph);
    Ok(())
}

/// Remove a graph by name.
pub fn remove_graph(name: &str) -> Result<(), WofError> {
    let store = get_store();
    let mut guard = store.write().map_err(|_| WofError::Runtime("graph lock poisoned".into()))?;
    guard.remove(name);
    Ok(())
}

/// Modify a graph in place.
pub fn with_graph_mut<F, R>(name: &str, f: F) -> Result<R, WofError>
where
    F: FnOnce(&mut Graph) -> Result<R, WofError>,
{
    let store = get_store();
    let mut guard = store.write().map_err(|_| WofError::Runtime("graph lock poisoned".into()))?;
    let graph = guard.get_mut(name).ok_or_else(|| {
        WofError::Runtime(format!("graph: unknown graph '{}'", name))
    })?;
    f(graph)
}

/// Register core graph operations.
pub fn register(interp: &mut Interpreter) {
    // ═══════════════════════════════════════════════════════════════
    // GRAPH CREATION
    // ═══════════════════════════════════════════════════════════════
    
    // Create a new undirected graph
    // Stack: num_nodes name → ()
    interp.register("graph_new", |interp| {
        let name = interp.stack_mut().pop()?.as_string()?;
        let n = interp.stack_mut().pop()?.as_integer()?;
        
        if n < 0 {
            return Err(WofError::Runtime("graph_new: num_nodes must be >= 0".into()));
        }
        
        let graph = Graph::new(n as usize, false);
        set_graph(&name, graph)?;
        Ok(())
    });

    // Create a new directed graph
    // Stack: num_nodes name → ()
    interp.register("digraph_new", |interp| {
        let name = interp.stack_mut().pop()?.as_string()?;
        let n = interp.stack_mut().pop()?.as_integer()?;
        
        if n < 0 {
            return Err(WofError::Runtime("digraph_new: num_nodes must be >= 0".into()));
        }
        
        let graph = Graph::new(n as usize, true);
        set_graph(&name, graph)?;
        Ok(())
    });

    // ═══════════════════════════════════════════════════════════════
    // EDGE OPERATIONS
    // ═══════════════════════════════════════════════════════════════
    
    // Add an edge
    // Stack: u v name → ()
    interp.register("graph_add_edge", |interp| {
        let name = interp.stack_mut().pop()?.as_string()?;
        let v = interp.stack_mut().pop()?.as_integer()? as usize;
        let u = interp.stack_mut().pop()?.as_integer()? as usize;
        
        with_graph_mut(&name, |g| {
            let n = g.node_count();
            if u >= n || v >= n {
                return Err(WofError::Runtime("graph_add_edge: node index out of range".into()));
            }
            g.add_edge(u, v);
            Ok(())
        })
    });

    // ═══════════════════════════════════════════════════════════════
    // QUERIES
    // ═══════════════════════════════════════════════════════════════
    
    // Get degree of a node
    // Stack: node name → degree
    interp.register("graph_degree", |interp| {
        let name = interp.stack_mut().pop()?.as_string()?;
        let node = interp.stack_mut().pop()?.as_integer()? as usize;
        
        let graph = get_graph(&name)?;
        if node >= graph.node_count() {
            return Err(WofError::Runtime("graph_degree: node index out of range".into()));
        }
        
        let deg = graph.degree(node);
        interp.stack_mut().push(WofValue::integer(deg as i64));
        Ok(())
    });

    // Get node count
    // Stack: name → count
    interp.register("graph_nodes", |interp| {
        let name = interp.stack_mut().pop()?.as_string()?;
        let graph = get_graph(&name)?;
        interp.stack_mut().push(WofValue::integer(graph.node_count() as i64));
        Ok(())
    });

    // Get edge count
    // Stack: name → count
    interp.register("graph_edges", |interp| {
        let name = interp.stack_mut().pop()?.as_string()?;
        let graph = get_graph(&name)?;
        let total: usize = graph.adj.iter().map(|v| v.len()).sum();
        let count = if graph.directed { total } else { total / 2 };
        interp.stack_mut().push(WofValue::integer(count as i64));
        Ok(())
    });

    // ═══════════════════════════════════════════════════════════════
    // MANAGEMENT
    // ═══════════════════════════════════════════════════════════════
    
    // Clear/delete a graph
    // Stack: name → ()
    interp.register("graph_clear", |interp| {
        let name = interp.stack_mut().pop()?.as_string()?;
        remove_graph(&name)?;
        Ok(())
    });

    // Check if graph exists
    // Stack: name → 1|0
    interp.register("graph_exists?", |interp| {
        let name = interp.stack_mut().pop()?.as_string()?;
        let exists = get_graph(&name).is_ok();
        interp.stack_mut().push(WofValue::integer(if exists { 1 } else { 0 }));
        Ok(())
    });
}
