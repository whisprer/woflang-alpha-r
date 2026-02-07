//! Instruction representation for compiled Woflang programs.
//!
//! Instructions are the compiled form of tokens, carrying an opcode,
//! optional operands, and source location for error reporting.

use crate::{Opcode, Span, WofValue};
use std::fmt;

/// A single instruction in a compiled Woflang program.
///
/// Instructions consist of an opcode, optional operand, and source span.
/// They are designed to be compact for cache efficiency during execution.
#[derive(Debug, Clone)]
pub struct Instruction {
    /// The operation to perform.
    pub opcode: Opcode,
    /// Optional operand (for push, jump, call, etc.).
    pub operand: Operand,
    /// Source location for error reporting.
    pub span: Span,
}

impl Instruction {
    /// Create a simple instruction with no operand.
    #[must_use]
    pub const fn simple(opcode: Opcode, span: Span) -> Self {
        Self {
            opcode,
            operand: Operand::None,
            span,
        }
    }

    /// Create an instruction that pushes a value.
    #[must_use]
    pub fn push_value(value: WofValue, span: Span) -> Self {
        Self {
            opcode: Opcode::PushLiteral,
            operand: Operand::Value(value),
            span,
        }
    }

    /// Create an instruction that references a symbol.
    #[must_use]
    pub fn symbol(name: String, span: Span) -> Self {
        Self {
            opcode: Opcode::PushSymbol,
            operand: Operand::Symbol(name),
            span,
        }
    }

    /// Create a jump instruction.
    #[must_use]
    pub fn jump(target: usize, span: Span) -> Self {
        Self {
            opcode: Opcode::Jump,
            operand: Operand::Address(target),
            span,
        }
    }

    /// Create a call instruction.
    #[must_use]
    pub fn call(name: String, span: Span) -> Self {
        Self {
            opcode: Opcode::Call,
            operand: Operand::Symbol(name),
            span,
        }
    }

    /// Create a variable definition instruction.
    #[must_use]
    pub fn define_var(name: String, span: Span) -> Self {
        Self {
            opcode: Opcode::DefineVar,
            operand: Operand::Symbol(name),
            span,
        }
    }

    /// Create a variable read instruction.
    #[must_use]
    pub fn read_var(name: String, span: Span) -> Self {
        Self {
            opcode: Opcode::ReadVar,
            operand: Operand::Symbol(name),
            span,
        }
    }

    /// Create a variable set instruction.
    #[must_use]
    pub fn set_var(name: String, span: Span) -> Self {
        Self {
            opcode: Opcode::Set,
            operand: Operand::Symbol(name),
            span,
        }
    }

    /// Create a label instruction.
    #[must_use]
    pub fn label(name: String, span: Span) -> Self {
        Self {
            opcode: Opcode::Label,
            operand: Operand::Symbol(name),
            span,
        }
    }

    /// Create a repeat instruction with a count.
    #[must_use]
    pub fn repeat(count: i64, span: Span) -> Self {
        Self {
            opcode: Opcode::Repeat,
            operand: Operand::Count(count),
            span,
        }
    }

    /// Check if this instruction has no operand.
    #[must_use]
    pub const fn is_simple(&self) -> bool {
        matches!(self.operand, Operand::None)
    }

    /// Get the symbol operand, if any.
    #[must_use]
    pub fn symbol_operand(&self) -> Option<&str> {
        match &self.operand {
            Operand::Symbol(s) => Some(s),
            _ => None,
        }
    }

    /// Get the value operand, if any.
    #[must_use]
    pub fn value_operand(&self) -> Option<&WofValue> {
        match &self.operand {
            Operand::Value(v) => Some(v),
            _ => None,
        }
    }

    /// Get the address operand, if any.
    #[must_use]
    pub fn address_operand(&self) -> Option<usize> {
        match &self.operand {
            Operand::Address(a) => Some(*a),
            _ => None,
        }
    }

    /// Get the count operand, if any.
    #[must_use]
    pub fn count_operand(&self) -> Option<i64> {
        match &self.operand {
            Operand::Count(c) => Some(*c),
            _ => None,
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.operand {
            Operand::None => write!(f, "{}", self.opcode),
            Operand::Value(v) => write!(f, "push {v}"),
            Operand::Symbol(s) => write!(f, "{} {s}", self.opcode),
            Operand::Address(a) => write!(f, "{} @{a}", self.opcode),
            Operand::Count(c) => write!(f, "{} {c}", self.opcode),
        }
    }
}

/// The operand of an instruction.
#[derive(Debug, Clone, Default)]
pub enum Operand {
    /// No operand.
    #[default]
    None,
    /// A literal value to push.
    Value(WofValue),
    /// A symbol name (variable, function, label).
    Symbol(String),
    /// An instruction address (for jumps).
    Address(usize),
    /// A count (for repeat).
    Count(i64),
}

impl Operand {
    /// Check if this is a None operand.
    #[must_use]
    pub const fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }
}

/// A compiled program consisting of instructions.
#[derive(Debug, Clone, Default)]
pub struct Program {
    /// The instruction sequence.
    pub instructions: Vec<Instruction>,
    /// Labels mapping names to instruction indices.
    pub labels: std::collections::HashMap<String, usize>,
    /// The source code (for error context).
    pub source: Option<String>,
}

impl Program {
    /// Create a new empty program.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a program with source code.
    #[must_use]
    pub fn with_source(source: String) -> Self {
        Self {
            instructions: Vec::new(),
            labels: std::collections::HashMap::new(),
            source: Some(source),
        }
    }

    /// Add an instruction to the program.
    pub fn push(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }

    /// Get the current instruction pointer (next instruction index).
    #[must_use]
    pub fn ip(&self) -> usize {
        self.instructions.len()
    }

    /// Define a label at the current position.
    pub fn define_label(&mut self, name: String) {
        self.labels.insert(name, self.ip());
    }

    /// Look up a label by name.
    #[must_use]
    pub fn lookup_label(&self, name: &str) -> Option<usize> {
        self.labels.get(name).copied()
    }

    /// Get an instruction by index.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&Instruction> {
        self.instructions.get(index)
    }

    /// Get the number of instructions.
    #[must_use]
    pub fn len(&self) -> usize {
        self.instructions.len()
    }

    /// Check if the program is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.instructions.is_empty()
    }

    /// Extract source context for a span.
    #[must_use]
    pub fn source_context(&self, span: Span, context_lines: usize) -> Option<String> {
        let source = self.source.as_ref()?;
        
        if span.is_synthetic() {
            return None;
        }
        
        let lines: Vec<&str> = source.lines().collect();
        let line_idx = (span.line() as usize).saturating_sub(1);
        
        if line_idx >= lines.len() {
            return None;
        }
        
        let start = line_idx.saturating_sub(context_lines);
        let end = (line_idx + context_lines + 1).min(lines.len());
        
        let mut result = String::new();
        for (i, line) in lines[start..end].iter().enumerate() {
            let line_no = start + i + 1;
            let marker = if line_no == span.line() as usize { ">" } else { " " };
            result.push_str(&format!("{marker} {line_no:4} | {line}\n"));
            
            if line_no == span.line() as usize {
                // Add caret pointing to the column
                let col = (span.column() as usize).saturating_sub(1);
                result.push_str(&format!("       | {:>width$}^\n", "", width = col));
            }
        }
        
        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instruction_creation() {
        let span = Span::new(1, 1, 0);
        
        let add = Instruction::simple(Opcode::Add, span);
        assert!(add.is_simple());
        
        let push = Instruction::push_value(WofValue::integer(42), span);
        assert!(!push.is_simple());
        assert_eq!(push.value_operand().map(|v| v.as_integer()), Some(Some(42)));
        
        let call = Instruction::call("test".to_string(), span);
        assert_eq!(call.symbol_operand(), Some("test"));
    }

    #[test]
    fn program_labels() {
        let mut program = Program::new();
        
        program.push(Instruction::simple(Opcode::Nop, Span::synthetic()));
        program.define_label("start".to_string());
        program.push(Instruction::simple(Opcode::Add, Span::synthetic()));
        
        assert_eq!(program.lookup_label("start"), Some(1));
        assert_eq!(program.lookup_label("nonexistent"), None);
    }

    #[test]
    fn source_context_extraction() {
        let source = "line 1\nline 2\nline 3 with error\nline 4\nline 5";
        let program = Program::with_source(source.to_string());
        
        let span = Span::with_length(3, 7, 20, 5);
        let context = program.source_context(span, 1).unwrap();
        
        assert!(context.contains("line 2"));
        assert!(context.contains("line 3 with error"));
        assert!(context.contains("line 4"));
        assert!(context.contains("^"));
    }
}
