//! Block tracking for structured control flow.
//!
//! Woflang uses block-structured execution where code is organized into
//! nested blocks delimited by `⺆` (open) and `⺘` (close). This module
//! provides the infrastructure to track block boundaries during parsing
//! and execution.

use crate::Span;
use std::fmt;

/// A unique identifier for a block.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockId(pub u32);

impl BlockId {
    /// The root/global block ID.
    pub const ROOT: Self = Self(0);

    /// Create a new block ID.
    #[must_use]
    pub const fn new(id: u32) -> Self {
        Self(id)
    }
}

impl fmt::Display for BlockId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "block#{}", self.0)
    }
}

/// The type of a block, determining its execution semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlockType {
    /// The global/root scope.
    Global,
    /// A function definition block.
    Function,
    /// A conditional if block.
    If,
    /// A then-branch block.
    Then,
    /// An else-branch block.
    Else,
    /// An else-if branch block.
    ElseIf,
    /// A loop block.
    Loop,
    /// A for-loop block.
    For,
    /// A repeat-N-times block.
    Repeat,
    /// A generic code block.
    Generic,
}

impl BlockType {
    /// Check if this block type creates a new scope for variables.
    #[must_use]
    pub const fn creates_scope(&self) -> bool {
        matches!(self, Self::Function | Self::Loop | Self::For | Self::Repeat)
    }

    /// Check if this block type can be exited with `break`.
    #[must_use]
    pub const fn is_loop(&self) -> bool {
        matches!(self, Self::Loop | Self::For | Self::Repeat)
    }

    /// Check if this block type is part of a conditional chain.
    #[must_use]
    pub const fn is_conditional(&self) -> bool {
        matches!(self, Self::If | Self::Then | Self::Else | Self::ElseIf)
    }
}

impl fmt::Display for BlockType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Global => "global",
            Self::Function => "func",
            Self::If => "if",
            Self::Then => "then",
            Self::Else => "else",
            Self::ElseIf => "elif",
            Self::Loop => "loop",
            Self::For => "for",
            Self::Repeat => "repeat",
            Self::Generic => "block",
        };
        write!(f, "{name}")
    }
}

/// Information about a single block in the program.
#[derive(Debug, Clone)]
pub struct BlockInfo {
    /// Unique identifier for this block.
    pub id: BlockId,
    /// The type of block.
    pub block_type: BlockType,
    /// Instruction pointer where this block starts.
    pub start_ip: usize,
    /// Instruction pointer where this block ends (inclusive).
    pub end_ip: usize,
    /// ID of the parent block (None for global).
    pub parent: Option<BlockId>,
    /// Source span of the block opening.
    pub span: Span,
    /// Optional name (for functions).
    pub name: Option<String>,
}

impl BlockInfo {
    /// Create a new block info.
    #[must_use]
    pub fn new(
        id: BlockId,
        block_type: BlockType,
        start_ip: usize,
        parent: Option<BlockId>,
        span: Span,
    ) -> Self {
        Self {
            id,
            block_type,
            start_ip,
            end_ip: start_ip,
            parent,
            span,
            name: None,
        }
    }

    /// Create a named function block.
    #[must_use]
    pub fn function(id: BlockId, name: String, start_ip: usize, parent: Option<BlockId>, span: Span) -> Self {
        Self {
            id,
            block_type: BlockType::Function,
            start_ip,
            end_ip: start_ip,
            parent,
            span,
            name: Some(name),
        }
    }

    /// Check if an instruction pointer is within this block.
    #[must_use]
    pub fn contains(&self, ip: usize) -> bool {
        ip >= self.start_ip && ip <= self.end_ip
    }

    /// Get the length of this block in instructions.
    #[must_use]
    pub fn len(&self) -> usize {
        self.end_ip.saturating_sub(self.start_ip) + 1
    }

    /// Check if the block is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.end_ip < self.start_ip
    }
}

/// Registry of all blocks in a program.
///
/// The `BlockRegistry` tracks all blocks discovered during parsing,
/// enabling efficient lookup of block boundaries during execution.
#[derive(Debug, Default)]
pub struct BlockRegistry {
    /// All registered blocks, indexed by ID.
    blocks: Vec<BlockInfo>,
    /// Next block ID to assign.
    next_id: u32,
}

impl BlockRegistry {
    /// Create a new empty block registry.
    #[must_use]
    pub fn new() -> Self {
        let mut registry = Self {
            blocks: Vec::new(),
            next_id: 1,
        };
        // Register the global block
        registry.blocks.push(BlockInfo::new(
            BlockId::ROOT,
            BlockType::Global,
            0,
            None,
            Span::synthetic(),
        ));
        registry
    }

    /// Register a new block and return its ID.
    pub fn register(
        &mut self,
        block_type: BlockType,
        start_ip: usize,
        parent: Option<BlockId>,
        span: Span,
    ) -> BlockId {
        let id = BlockId::new(self.next_id);
        self.next_id += 1;
        self.blocks.push(BlockInfo::new(id, block_type, start_ip, parent, span));
        id
    }

    /// Register a named function block.
    pub fn register_function(
        &mut self,
        name: String,
        start_ip: usize,
        parent: Option<BlockId>,
        span: Span,
    ) -> BlockId {
        let id = BlockId::new(self.next_id);
        self.next_id += 1;
        self.blocks.push(BlockInfo::function(id, name, start_ip, parent, span));
        id
    }

    /// Close a block by setting its end IP.
    pub fn close(&mut self, id: BlockId, end_ip: usize) {
        if let Some(block) = self.get_mut(id) {
            block.end_ip = end_ip;
        }
    }

    /// Get a block by ID.
    #[must_use]
    pub fn get(&self, id: BlockId) -> Option<&BlockInfo> {
        self.blocks.iter().find(|b| b.id == id)
    }

    /// Get a mutable reference to a block by ID.
    pub fn get_mut(&mut self, id: BlockId) -> Option<&mut BlockInfo> {
        self.blocks.iter_mut().find(|b| b.id == id)
    }

    /// Find the innermost block containing an instruction pointer.
    #[must_use]
    pub fn block_containing(&self, ip: usize) -> Option<&BlockInfo> {
        // Find the smallest block containing this IP
        self.blocks
            .iter()
            .filter(|b| b.contains(ip))
            .min_by_key(|b| b.len())
    }

    /// Find the innermost loop block containing an instruction pointer.
    #[must_use]
    pub fn enclosing_loop(&self, ip: usize) -> Option<&BlockInfo> {
        self.blocks
            .iter()
            .filter(|b| b.contains(ip) && b.block_type.is_loop())
            .min_by_key(|b| b.len())
    }

    /// Get all blocks.
    #[must_use]
    pub fn all_blocks(&self) -> &[BlockInfo] {
        &self.blocks
    }

    /// Get the number of registered blocks.
    #[must_use]
    pub fn len(&self) -> usize {
        self.blocks.len()
    }

    /// Check if the registry is empty (only has global block).
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.blocks.len() <= 1
    }

    /// Find a function by name.
    #[must_use]
    pub fn find_function(&self, name: &str) -> Option<&BlockInfo> {
        self.blocks.iter().find(|b| {
            b.block_type == BlockType::Function && b.name.as_deref() == Some(name)
        })
    }

    /// Get all direct children of a block.
    #[must_use]
    pub fn children_of(&self, id: BlockId) -> Vec<&BlockInfo> {
        self.blocks
            .iter()
            .filter(|b| b.parent == Some(id))
            .collect()
    }

    /// Clear all blocks except the global block.
    pub fn reset(&mut self) {
        self.blocks.truncate(1);
        self.next_id = 1;
        if let Some(global) = self.blocks.first_mut() {
            global.end_ip = 0;
        }
    }
}

/// A stack for tracking nested block context during parsing/execution.
#[derive(Debug, Default)]
pub struct BlockStack {
    stack: Vec<BlockId>,
}

impl BlockStack {
    /// Create a new block stack starting at the global scope.
    #[must_use]
    pub fn new() -> Self {
        Self {
            stack: vec![BlockId::ROOT],
        }
    }

    /// Push a new block onto the stack.
    pub fn push(&mut self, id: BlockId) {
        self.stack.push(id);
    }

    /// Pop the top block from the stack.
    pub fn pop(&mut self) -> Option<BlockId> {
        // Never pop the global block
        if self.stack.len() > 1 {
            self.stack.pop()
        } else {
            None
        }
    }

    /// Get the current (innermost) block ID.
    #[must_use]
    pub fn current(&self) -> BlockId {
        self.stack.last().copied().unwrap_or(BlockId::ROOT)
    }

    /// Get the parent of the current block.
    #[must_use]
    pub fn parent(&self) -> Option<BlockId> {
        if self.stack.len() >= 2 {
            self.stack.get(self.stack.len() - 2).copied()
        } else {
            None
        }
    }

    /// Get the depth of nesting.
    #[must_use]
    pub fn depth(&self) -> usize {
        self.stack.len()
    }

    /// Check if we're at the global scope.
    #[must_use]
    pub fn is_global(&self) -> bool {
        self.stack.len() == 1
    }

    /// Clear the stack back to just the global scope.
    pub fn reset(&mut self) {
        self.stack.truncate(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn block_registry_basics() {
        let mut registry = BlockRegistry::new();
        
        // Global block should already exist
        assert_eq!(registry.len(), 1);
        
        let func_id = registry.register_function(
            "test".to_string(),
            10,
            Some(BlockId::ROOT),
            Span::synthetic(),
        );
        registry.close(func_id, 20);
        
        let block = registry.get(func_id).unwrap();
        assert_eq!(block.name.as_deref(), Some("test"));
        assert!(block.contains(15));
        assert!(!block.contains(25));
    }

    #[test]
    fn block_stack_operations() {
        let mut stack = BlockStack::new();
        
        assert!(stack.is_global());
        assert_eq!(stack.depth(), 1);
        
        stack.push(BlockId::new(1));
        assert!(!stack.is_global());
        assert_eq!(stack.current(), BlockId::new(1));
        
        stack.push(BlockId::new(2));
        assert_eq!(stack.current(), BlockId::new(2));
        assert_eq!(stack.parent(), Some(BlockId::new(1)));
        
        stack.pop();
        assert_eq!(stack.current(), BlockId::new(1));
        
        stack.pop();
        assert!(stack.is_global());
        
        // Cannot pop the global block
        assert!(stack.pop().is_none());
    }

    #[test]
    fn find_enclosing_loop() {
        let mut registry = BlockRegistry::new();
        
        let loop_id = registry.register(
            BlockType::Loop,
            5,
            Some(BlockId::ROOT),
            Span::synthetic(),
        );
        registry.close(loop_id, 15);
        
        let inner = registry.register(
            BlockType::If,
            8,
            Some(loop_id),
            Span::synthetic(),
        );
        registry.close(inner, 12);
        
        // From inside the if, we should find the enclosing loop
        let found = registry.enclosing_loop(10).unwrap();
        assert_eq!(found.id, loop_id);
    }
}
