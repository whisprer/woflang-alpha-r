//! Operation registry for Woflang.
//!
//! The [`Registry`] manages the mapping from operation names to their
//! handler functions. It supports both compile-time registration (the
//! preferred path) and runtime registration for plugins.
//!
//! ## Design
//!
//! Operations are stored as boxed trait objects to allow heterogeneous
//! handler types while maintaining a uniform dispatch interface. The
//! registry uses a `HashMap` for O(1) lookup during interpretation.

use std::collections::HashMap;
use std::sync::Arc;
use woflang_core::{InterpreterContext, Result};

/// Function type for operation handlers.
///
/// Handlers receive a mutable reference to an interpreter context
/// (anything implementing [`InterpreterContext`]) and return a result
/// indicating success or failure.
pub type OpFn<Ctx> = fn(&mut Ctx) -> Result<()>;

/// A reference-counted operation handler for dynamic dispatch.
///
/// Using `Arc` instead of `Box` allows us to clone handlers when needed,
/// which is essential for avoiding borrow conflicts during dispatch.
pub type BoxedOp<Ctx> = Arc<dyn Fn(&mut Ctx) -> Result<()> + Send + Sync>;

/// Operation registry mapping names to handlers.
///
/// The registry is generic over the interpreter context type, allowing
/// reuse with different interpreter implementations.
pub struct Registry<Ctx: InterpreterContext> {
    ops: HashMap<String, BoxedOp<Ctx>>,
    aliases: HashMap<String, String>,
}

impl<Ctx: InterpreterContext> Default for Registry<Ctx> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Ctx: InterpreterContext> Registry<Ctx> {
    /// Create a new empty registry.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            ops: HashMap::new(),
            aliases: HashMap::new(),
        }
    }

    /// Create a registry with pre-allocated capacity.
    #[inline]
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            ops: HashMap::with_capacity(capacity),
            aliases: HashMap::new(),
        }
    }

    /// Register an operation handler.
    ///
    /// If an operation with the same name already exists, it is replaced.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// registry.register("double", |interp| {
    ///     let val = interp.stack_mut().pop_numeric()?;
    ///     interp.stack_mut().push(WofValue::double(val * 2.0));
    ///     Ok(())
    /// });
    /// ```
    pub fn register<F>(&mut self, name: impl Into<String>, handler: F)
    where
        F: Fn(&mut Ctx) -> Result<()> + Send + Sync + 'static,
    {
        self.ops.insert(name.into(), Arc::new(handler));
    }

    /// Register an operation with a function pointer (zero-overhead).
    pub fn register_fn(&mut self, name: impl Into<String>, handler: OpFn<Ctx>)
    where
        Ctx: 'static,
    {
        self.ops.insert(name.into(), Arc::new(handler));
    }

    /// Register an alias for an existing operation.
    ///
    /// Aliases are resolved at lookup time, not registration time.
    pub fn alias(&mut self, alias: impl Into<String>, target: impl Into<String>) {
        self.aliases.insert(alias.into(), target.into());
    }

    /// Look up an operation by name.
    ///
    /// Returns `None` if the operation is not registered.
    #[must_use]
    pub fn get(&self, name: &str) -> Option<&BoxedOp<Ctx>> {
        // Check for alias first
        let resolved = self.aliases.get(name).map_or(name, String::as_str);
        self.ops.get(resolved)
    }

    /// Look up an operation by name and clone it.
    ///
    /// This is the preferred method when you need to call the handler
    /// while mutably borrowing the interpreter, as it avoids borrow conflicts.
    #[must_use]
    pub fn get_cloned(&self, name: &str) -> Option<BoxedOp<Ctx>> {
        self.get(name).cloned()
    }

    /// Check if an operation is registered.
    #[must_use]
    pub fn contains(&self, name: &str) -> bool {
        let resolved = self.aliases.get(name).map_or(name, String::as_str);
        self.ops.contains_key(resolved)
    }

    /// Get the number of registered operations.
    #[must_use]
    pub fn len(&self) -> usize {
        self.ops.len()
    }

    /// Check if the registry is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.ops.is_empty()
    }

    /// Iterate over all registered operation names.
    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.ops.keys().map(String::as_str)
    }

    /// Remove an operation from the registry.
    pub fn remove(&mut self, name: &str) -> bool {
        self.ops.remove(name).is_some()
    }

    /// Merge another registry into this one.
    ///
    /// Operations from `other` will overwrite existing operations
    /// with the same name.
    pub fn merge(&mut self, other: Self) {
        self.ops.extend(other.ops);
        self.aliases.extend(other.aliases);
    }
}

impl<Ctx: InterpreterContext> std::fmt::Debug for Registry<Ctx> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Registry")
            .field("ops_count", &self.ops.len())
            .field("aliases_count", &self.aliases.len())
            .field("ops", &self.ops.keys().collect::<Vec<_>>())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use woflang_core::{WofStack, WofValue};

    // Minimal test context
    struct TestCtx {
        stack: WofStack,
    }

    impl InterpreterContext for TestCtx {
        fn push(&mut self, value: WofValue) {
            self.stack.push(value);
        }

        fn pop(&mut self) -> Result<WofValue> {
            self.stack.pop()
        }

        fn peek(&self) -> Result<&WofValue> {
            self.stack.peek()
        }

        fn has(&self, n: usize) -> bool {
            self.stack.has(n)
        }

        fn stack(&self) -> &WofStack {
            &self.stack
        }

        fn stack_mut(&mut self) -> &mut WofStack {
            &mut self.stack
        }

        fn clear(&mut self) {
            self.stack.clear();
        }
    }

    #[test]
    fn register_and_call() {
        let mut registry: Registry<TestCtx> = Registry::new();
        registry.register("inc", |ctx| {
            let val = ctx.stack_mut().pop_integer()?;
            ctx.push(WofValue::integer(val + 1));
            Ok(())
        });

        let mut ctx = TestCtx {
            stack: WofStack::new(),
        };
        ctx.push(WofValue::integer(41));

        let op = registry.get("inc").unwrap();
        op(&mut ctx).unwrap();

        assert_eq!(ctx.stack.pop_integer().unwrap(), 42);
    }

    #[test]
    fn alias_resolution() {
        let mut registry: Registry<TestCtx> = Registry::new();
        registry.register("duplicate", |ctx| ctx.stack_mut().dup());
        registry.alias("dup", "duplicate");

        assert!(registry.contains("dup"));
        assert!(registry.get("dup").is_some());
    }
}
