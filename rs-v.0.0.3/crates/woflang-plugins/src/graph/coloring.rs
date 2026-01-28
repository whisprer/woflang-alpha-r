//! Graph coloring operations for Woflang.
//!
//! Provides greedy graph coloring using Welsh-Powell heuristic.

use woflang_core::{InterpreterContext, WofValue};
use woflang_runtime::Interpreter;

use super::core::get_graph;

/// Greedy graph coloring using Welsh-Powell heuristic.
/// Returns (coloring, num_colors_used).
fn greedy_coloring(adj: &[Vec<usize>]) -> (Vec<usize>, usize) {
    let n = adj.len();
    if n == 0 {
        return (Vec::new(), 0);
    }

    // Sort vertices by decreasing degree (Welsh-Powell heuristic)
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&a, &b| adj[b].len().cmp(&adj[a].len()));

    let mut color = vec![usize::MAX; n];
    let mut max_color = 0;

    for &u in &order {
        // Find colors used by neighbors
        let mut used = vec![false; n];
        for &v in &adj[u] {
            if v < n && color[v] != usize::MAX {
                if color[v] < used.len() {
                    used[color[v]] = true;
                }
            }
        }

        // Find smallest available color
        let mut c = 0;
        while c < n && used[c] {
            c += 1;
        }

        color[u] = c;
        if c > max_color {
            max_color = c;
        }
    }

    (color, max_color + 1)
}

/// Check if a coloring is valid (no adjacent nodes have same color).
fn is_valid_coloring(adj: &[Vec<usize>], color: &[usize]) -> bool {
    for (u, neighbors) in adj.iter().enumerate() {
        for &v in neighbors {
            if v < color.len() && color[u] == color[v] && u != v {
                return false;
            }
        }
    }
    true
}

/// Compute chromatic bound (lower bound on chromatic number).
/// Uses clique size as lower bound.
fn chromatic_lower_bound(adj: &[Vec<usize>]) -> usize {
    let n = adj.len();
    if n == 0 {
        return 0;
    }

    // Simple heuristic: max degree + 1 is upper bound
    // For lower bound, we use max clique heuristic (greedy)
    let mut max_clique = 1;
    
    for u in 0..n {
        // Try to extend clique starting from u
        let mut clique = vec![u];
        for &v in &adj[u] {
            if v > u && v < n {
                // Check if v is adjacent to all nodes in current clique
                let mut adjacent_to_all = true;
                for &c in &clique {
                    if !adj[v].contains(&c) && !adj[c].contains(&v) {
                        adjacent_to_all = false;
                        break;
                    }
                }
                if adjacent_to_all {
                    clique.push(v);
                }
            }
        }
        max_clique = max_clique.max(clique.len());
    }

    max_clique
}

/// Register graph coloring operations.
pub fn register(interp: &mut Interpreter) {
    // ═══════════════════════════════════════════════════════════════
    // GRAPH COLORING
    // ═══════════════════════════════════════════════════════════════
    
    // Greedy coloring (Welsh-Powell)
    // Stack: name → summary_string num_colors
    interp.register("graph_color_greedy", |interp| {
        let name = interp.stack_mut().pop()?.as_string()?;
        let graph = get_graph(&name)?;
        let n = graph.node_count();
        
        if n == 0 {
            interp.stack_mut().push(WofValue::string(format!("graph {} is empty; no colours needed", name)));
            interp.stack_mut().push(WofValue::integer(0));
            return Ok(());
        }
        
        let (coloring, num_colors) = greedy_coloring(&graph.adj);
        
        // Build summary string
        let mut summary = format!("graph {} colouring (greedy):\n", name);
        for (i, &c) in coloring.iter().enumerate() {
            summary.push_str(&format!("  {} -> c{}\n", i, c));
        }
        summary.push_str(&format!("total colours used: {}", num_colors));
        
        interp.stack_mut().push(WofValue::string(summary));
        interp.stack_mut().push(WofValue::integer(num_colors as i64));
        Ok(())
    });

    // Just get the number of colors needed
    // Stack: name → num_colors
    interp.register("graph_chromatic", |interp| {
        let name = interp.stack_mut().pop()?.as_string()?;
        let graph = get_graph(&name)?;
        
        if graph.node_count() == 0 {
            interp.stack_mut().push(WofValue::integer(0));
            return Ok(());
        }
        
        let (_, num_colors) = greedy_coloring(&graph.adj);
        interp.stack_mut().push(WofValue::integer(num_colors as i64));
        Ok(())
    });

    // Check if graph is k-colorable
    // Stack: k name → 1|0
    interp.register("graph_k_colorable?", |interp| {
        let name = interp.stack_mut().pop()?.as_string()?;
        let k = interp.stack_mut().pop()?.as_integer()? as usize;
        
        let graph = get_graph(&name)?;
        
        if graph.node_count() == 0 {
            interp.stack_mut().push(WofValue::integer(1));
            return Ok(());
        }
        
        let (_, num_colors) = greedy_coloring(&graph.adj);
        let is_colorable = num_colors <= k;
        interp.stack_mut().push(WofValue::integer(if is_colorable { 1 } else { 0 }));
        Ok(())
    });

    // Check if graph is bipartite (2-colorable)
    // Stack: name → 1|0
    interp.register("graph_bipartite?", |interp| {
        let name = interp.stack_mut().pop()?.as_string()?;
        let graph = get_graph(&name)?;
        let n = graph.node_count();
        
        if n == 0 {
            interp.stack_mut().push(WofValue::integer(1));
            return Ok(());
        }
        
        // BFS-based bipartite check
        let mut color = vec![None; n];
        let mut is_bipartite = true;
        
        for start in 0..n {
            if color[start].is_some() {
                continue;
            }
            
            let mut queue = std::collections::VecDeque::new();
            queue.push_back(start);
            color[start] = Some(0);
            
            while let Some(u) = queue.pop_front() {
                let c = color[u].unwrap_or(0);
                for &v in graph.neighbors(u) {
                    if v >= n {
                        continue;
                    }
                    match color[v] {
                        Some(vc) if vc == c => {
                            is_bipartite = false;
                            break;
                        }
                        None => {
                            color[v] = Some(1 - c);
                            queue.push_back(v);
                        }
                        _ => {}
                    }
                }
                if !is_bipartite {
                    break;
                }
            }
            if !is_bipartite {
                break;
            }
        }
        
        interp.stack_mut().push(WofValue::integer(if is_bipartite { 1 } else { 0 }));
        Ok(())
    });

    // Get chromatic number lower bound
    // Stack: name → lower_bound
    interp.register("graph_chromatic_lower", |interp| {
        let name = interp.stack_mut().pop()?.as_string()?;
        let graph = get_graph(&name)?;
        let bound = chromatic_lower_bound(&graph.adj);
        interp.stack_mut().push(WofValue::integer(bound as i64));
        Ok(())
    });
}
