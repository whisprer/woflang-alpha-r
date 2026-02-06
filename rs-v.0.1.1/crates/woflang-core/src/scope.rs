//! Scope and variable management.
//!
//! Woflang uses lexical scoping with a scope stack. Each block can
//! introduce a new scope, and variables are looked up from innermost
//! to outermost scope.

use crate::{BlockId, WofValue, WofError, Result};
use std::collections::HashMap;

/// A unique identifier for a scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScopeId(pub u32);

impl ScopeId {
    /// The global scope ID.
    pub const GLOBAL: Self = Self(0);
}

/// A single scope containing variable bindings.
#[derive(Debug, Clone)]
pub struct Scope {
    /// Unique identifier for this scope.
    pub id: ScopeId,
    /// The block that introduced this scope.
    pub block_id: BlockId,
    /// Parent scope (for lookup chain).
    pub parent: Option<ScopeId>,
    /// Variable bindings in this scope.
    variables: HashMap<String, WofValue>,
}

impl Scope {
    /// Create a new scope.
    #[must_use]
    pub fn new(id: ScopeId, block_id: BlockId, parent: Option<ScopeId>) -> Self {
        Self {
            id,
            block_id,
            parent,
            variables: HashMap::new(),
        }
    }

    /// Define a variable in this scope.
    pub fn define(&mut self, name: String, value: WofValue) {
        self.variables.insert(name, value);
    }

    /// Get a variable from this scope only (not parent).
    #[must_use]
    pub fn get_local(&self, name: &str) -> Option<&WofValue> {
        self.variables.get(name)
    }

    /// Get a mutable reference to a variable in this scope only.
    pub fn get_local_mut(&mut self, name: &str) -> Option<&mut WofValue> {
        self.variables.get_mut(name)
    }

    /// Check if a variable exists in this scope (not parent).
    #[must_use]
    pub fn has_local(&self, name: &str) -> bool {
        self.variables.contains_key(name)
    }

    /// Remove a variable from this scope.
    pub fn undefine(&mut self, name: &str) -> Option<WofValue> {
        self.variables.remove(name)
    }

    /// Get all variable names in this scope.
    #[must_use]
    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.variables.keys().map(String::as_str)
    }

    /// Get the number of variables in this scope.
    #[must_use]
    pub fn len(&self) -> usize {
        self.variables.len()
    }

    /// Check if this scope has no variables.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.variables.is_empty()
    }
}

/// A stack of scopes for variable resolution.
///
/// The `ScopeStack` maintains the chain of active scopes during execution.
/// Variable lookups traverse from the innermost scope outward until a
/// binding is found.
#[derive(Debug)]
pub struct ScopeStack {
    /// All scopes, indexed by ID.
    scopes: Vec<Scope>,
    /// Stack of active scope IDs (innermost last).
    active: Vec<ScopeId>,
    /// Next scope ID to assign.
    next_id: u32,
}

impl Default for ScopeStack {
    fn default() -> Self {
        Self::new()
    }
}

impl ScopeStack {
    /// Create a new scope stack with a global scope.
    #[must_use]
    pub fn new() -> Self {
        let global = Scope::new(ScopeId::GLOBAL, BlockId::ROOT, None);
        Self {
            scopes: vec![global],
            active: vec![ScopeId::GLOBAL],
            next_id: 1,
        }
    }

    /// Push a new scope onto the stack.
    pub fn push(&mut self, block_id: BlockId) -> ScopeId {
        let id = ScopeId(self.next_id);
        self.next_id += 1;
        
        let parent = self.current_id();
        let scope = Scope::new(id, block_id, Some(parent));
        
        self.scopes.push(scope);
        self.active.push(id);
        id
    }

    /// Pop the current scope from the stack.
    ///
    /// Returns the popped scope ID, or `None` if only the global scope remains.
    pub fn pop(&mut self) -> Option<ScopeId> {
        if self.active.len() > 1 {
            self.active.pop()
        } else {
            None
        }
    }

    /// Get the current (innermost) scope ID.
    #[must_use]
    pub fn current_id(&self) -> ScopeId {
        self.active.last().copied().unwrap_or(ScopeId::GLOBAL)
    }

    /// Get a reference to the current scope.
    #[must_use]
    pub fn current(&self) -> &Scope {
        let id = self.current_id();
        self.get(id).expect("current scope must exist")
    }

    /// Get a mutable reference to the current scope.
    pub fn current_mut(&mut self) -> &mut Scope {
        let id = self.current_id();
        self.get_mut(id).expect("current scope must exist")
    }

    /// Get a scope by ID.
    #[must_use]
    pub fn get(&self, id: ScopeId) -> Option<&Scope> {
        self.scopes.iter().find(|s| s.id == id)
    }

    /// Get a mutable reference to a scope by ID.
    pub fn get_mut(&mut self, id: ScopeId) -> Option<&mut Scope> {
        self.scopes.iter_mut().find(|s| s.id == id)
    }

    /// Define a variable in the current scope.
    pub fn define(&mut self, name: impl Into<String>, value: WofValue) {
        self.current_mut().define(name.into(), value);
    }

    /// Look up a variable, searching from innermost to outermost scope.
    #[must_use]
    pub fn lookup(&self, name: &str) -> Option<&WofValue> {
        let mut scope_id = Some(self.current_id());
        
        while let Some(id) = scope_id {
            if let Some(scope) = self.get(id) {
                if let Some(value) = scope.get_local(name) {
                    return Some(value);
                }
                scope_id = scope.parent;
            } else {
                break;
            }
        }
        
        None
    }

    /// Look up a variable and get a mutable reference.
    ///
    /// Note: This is more complex because we need to find which scope
    /// contains the variable first, then get a mutable reference.
    pub fn lookup_mut(&mut self, name: &str) -> Option<&mut WofValue> {
        // First, find which scope contains the variable
        let mut target_scope_id = None;
        let mut scope_id = Some(self.current_id());
        
        while let Some(id) = scope_id {
            if let Some(scope) = self.get(id) {
                if scope.has_local(name) {
                    target_scope_id = Some(id);
                    break;
                }
                scope_id = scope.parent;
            } else {
                break;
            }
        }
        
        // Then get a mutable reference to that scope and the variable
        target_scope_id
            .and_then(|id| self.get_mut(id))
            .and_then(|scope| scope.get_local_mut(name))
    }

    /// Set a variable's value, searching through scopes.
    ///
    /// If the variable exists in any enclosing scope, update it there.
    /// Otherwise, define it in the current scope.
    pub fn set(&mut self, name: impl Into<String>, value: WofValue) {
        let name = name.into();
        
        // Check if the variable exists in any scope
        if self.lookup(&name).is_some() {
            // Update existing variable
            if let Some(var) = self.lookup_mut(&name) {
                *var = value;
            }
        } else {
            // Define new variable in current scope
            self.define(name, value);
        }
    }

    /// Get a variable's value, returning an error if not found.
    pub fn get_var(&self, name: &str) -> Result<WofValue> {
        self.lookup(name)
            .cloned()
            .ok_or_else(|| WofError::undefined_variable(name))
    }

    /// Set a variable's value, returning an error if not defined.
    pub fn set_var(&mut self, name: &str, value: WofValue) -> Result<()> {
        if let Some(var) = self.lookup_mut(name) {
            *var = value;
            Ok(())
        } else {
            Err(WofError::undefined_variable(name))
        }
    }

    /// Check if a variable is defined in any enclosing scope.
    #[must_use]
    pub fn is_defined(&self, name: &str) -> bool {
        self.lookup(name).is_some()
    }

    /// Get the depth of scope nesting.
    #[must_use]
    pub fn depth(&self) -> usize {
        self.active.len()
    }

    /// Check if we're at the global scope.
    #[must_use]
    pub fn is_global(&self) -> bool {
        self.active.len() == 1
    }

    /// Reset to just the global scope.
    pub fn reset(&mut self) {
        // Clear the global scope's variables
        if let Some(global) = self.scopes.first_mut() {
            global.variables.clear();
        }
        // Keep only the global scope
        self.scopes.truncate(1);
        self.active.truncate(1);
        self.next_id = 1;
    }

    /// Get all variable names visible from the current scope.
    pub fn all_visible_names(&self) -> Vec<String> {
        let mut names = Vec::new();
        let mut scope_id = Some(self.current_id());
        
        while let Some(id) = scope_id {
            if let Some(scope) = self.get(id) {
                for name in scope.names() {
                    if !names.contains(&name.to_string()) {
                        names.push(name.to_string());
                    }
                }
                scope_id = scope.parent;
            } else {
                break;
            }
        }
        
        names
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scope_stack_basics() {
        let mut scopes = ScopeStack::new();
        
        // Define in global scope
        scopes.define("x", WofValue::integer(42));
        assert_eq!(scopes.lookup("x").map(|v| v.as_integer()), Some(Some(42)));
        
        // Push a new scope
        scopes.push(BlockId::new(1));
        
        // Can still see x from parent
        assert_eq!(scopes.lookup("x").map(|v| v.as_integer()), Some(Some(42)));
        
        // Shadow x in inner scope
        scopes.define("x", WofValue::integer(100));
        assert_eq!(scopes.lookup("x").map(|v| v.as_integer()), Some(Some(100)));
        
        // Pop scope, x reverts to original
        scopes.pop();
        assert_eq!(scopes.lookup("x").map(|v| v.as_integer()), Some(Some(42)));
    }

    #[test]
    fn set_variable_in_enclosing_scope() {
        let mut scopes = ScopeStack::new();
        
        scopes.define("counter", WofValue::integer(0));
        scopes.push(BlockId::new(1));
        
        // Modify the outer variable
        scopes.set("counter", WofValue::integer(5));
        
        // Check it's modified in the outer scope
        scopes.pop();
        assert_eq!(scopes.lookup("counter").map(|v| v.as_integer()), Some(Some(5)));
    }

    #[test]
    fn undefined_variable_error() {
        let scopes = ScopeStack::new();
        
        let result = scopes.get_var("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn scope_depth() {
        let mut scopes = ScopeStack::new();
        
        assert_eq!(scopes.depth(), 1);
        assert!(scopes.is_global());
        
        scopes.push(BlockId::new(1));
        assert_eq!(scopes.depth(), 2);
        assert!(!scopes.is_global());
        
        scopes.push(BlockId::new(2));
        assert_eq!(scopes.depth(), 3);
        
        scopes.pop();
        scopes.pop();
        assert_eq!(scopes.depth(), 1);
        assert!(scopes.is_global());
    }
}
