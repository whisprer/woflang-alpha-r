//! Core interpreter for the Woflang stack machine.
//!
//! The [`Interpreter`] executes Woflang source code by tokenizing input
//! and dispatching operations through the registry. It maintains the
//! execution state (stack, scopes) and provides the context for operation handlers.

use crate::{KeyBindings, Registry, Token, TokenKind, Tokenizer};
use std::collections::{HashMap, VecDeque};
use std::fs;
use std::io::{self, BufRead, Write};
use std::path::Path;
use woflang_core::{
    BlockId, BlockRegistry, BlockStack, BlockType, InterpreterContext, 
    Result, ScopeStack, Span, WofError, WofStack, WofValue,
};

/// A user-defined function.
#[derive(Debug, Clone)]
pub struct FunctionDef {
    /// The function name.
    pub name: String,
    /// The function body as tokens.
    pub body: Vec<OwnedToken>,
    /// Number of parameters (taken from stack).
    pub arity: usize,
    /// Source location where defined.
    pub span: Span,
}

impl FunctionDef {
    /// Create a new function definition.
    pub fn new(name: impl Into<String>, body: Vec<OwnedToken>, span: Span) -> Self {
        Self {
            name: name.into(),
            body,
            arity: 0, // Default, can be set explicitly
            span,
        }
    }
    
    /// Set the function arity.
    pub fn with_arity(mut self, arity: usize) -> Self {
        self.arity = arity;
        self
    }
}

/// Context saved when calling a function.
#[derive(Debug, Clone)]
struct CallFrame {
    /// Tokens to resume after return.
    remaining_tokens: VecDeque<OwnedToken>,
    /// Block depth at call site.
    block_depth: usize,
}

/// The Woflang interpreter.
///
/// Manages the execution state and operation dispatch for a Woflang
/// program. The interpreter owns both the stack and the operation
/// registry, providing a complete execution environment.
///
/// # Examples
///
/// ```
/// use woflang_runtime::Interpreter;
///
/// let mut interp = Interpreter::new();
/// interp.exec_line("42 17 +").unwrap();
///
/// let result = interp.stack().peek().unwrap().as_integer().unwrap();
/// assert_eq!(result, 59);
/// ```
pub struct Interpreter {
    /// The data stack.
    stack: WofStack,
    /// The call stack (for function returns).
    call_stack: Vec<CallFrame>,
    /// Operation registry.
    registry: Registry<Self>,
    /// User-defined functions.
    functions: HashMap<String, FunctionDef>,
    /// Keybinding aliases.
    keybindings: KeyBindings,
    /// Variable scopes.
    scopes: ScopeStack,
    /// Block registry (for control flow).
    blocks: BlockRegistry,
    /// Block nesting stack.
    block_stack: BlockStack,
    /// Token buffer for lookahead/control flow.
    token_buffer: VecDeque<OwnedToken>,
    /// Current instruction pointer (for compiled mode).
    ip: usize,
    /// Skip mode depth (for skipping else branches etc).
    skip_depth: usize,
    /// Function definition mode: collecting body for this function name.
    defining_function: Option<String>,
    /// Tokens being collected for function body.
    function_body_buffer: Vec<OwnedToken>,
    /// Nesting depth inside function definition (to handle nested blocks).
    function_def_depth: usize,
    /// Loop body being collected.
    loop_body_buffer: Vec<OwnedToken>,
    /// Loop collection nesting depth.
    loop_collect_depth: usize,
    /// Type of loop being collected (for initial dispatch).
    collecting_loop: Option<LoopType>,
    /// Active loop frames (for nested loops).
    loop_stack: Vec<LoopFrame>,
    /// Break signal (exit innermost loop).
    break_signal: bool,
    /// Continue signal (restart innermost loop iteration).
    continue_signal: bool,
    /// Expand keybindings in input.
    pub expand_bindings: bool,
    /// Debug mode: print stack after each line.
    pub debug: bool,
}

/// Type of loop construct.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoopType {
    /// Infinite loop (⟳).
    Infinite,
    /// Repeat N times (⨯).
    Repeat(i64),
    /// While condition is true.
    While,
}

/// Active loop execution frame.
#[derive(Debug, Clone)]
struct LoopFrame {
    /// The loop body tokens.
    body: Vec<OwnedToken>,
    /// Type of loop.
    loop_type: LoopType,
    /// Current iteration (for repeat loops).
    iteration: i64,
    /// Maximum iterations (for repeat loops, 0 = infinite).
    max_iterations: i64,
}

/// An owned token for buffering during control flow.
#[derive(Debug, Clone)]
pub struct OwnedToken {
    /// The kind of token.
    pub kind: TokenKind,
    /// The token text (owned).
    pub text: String,
    /// Source location.
    pub span: Span,
}

impl<'a> From<Token<'a>> for OwnedToken {
    fn from(t: Token<'a>) -> Self {
        Self {
            kind: t.kind,
            text: t.text.to_string(),
            span: t.span,
        }
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl Interpreter {
    /// Create a new interpreter with an empty registry.
    #[must_use]
    pub fn new() -> Self {
        Self {
            stack: WofStack::with_capacity(64),
            call_stack: Vec::with_capacity(16),
            registry: Registry::new(),
            functions: HashMap::new(),
            keybindings: KeyBindings::with_defaults(),
            scopes: ScopeStack::new(),
            blocks: BlockRegistry::new(),
            block_stack: BlockStack::new(),
            token_buffer: VecDeque::new(),
            ip: 0,
            skip_depth: 0,
            defining_function: None,
            function_body_buffer: Vec::new(),
            function_def_depth: 0,
            loop_body_buffer: Vec::new(),
            loop_collect_depth: 0,
            collecting_loop: None,
            loop_stack: Vec::new(),
            break_signal: false,
            continue_signal: false,
            expand_bindings: true,
            debug: false,
        }
    }

    /// Create an interpreter with a pre-configured registry.
    #[must_use]
    pub fn with_registry(registry: Registry<Self>) -> Self {
        Self {
            stack: WofStack::with_capacity(64),
            call_stack: Vec::with_capacity(16),
            registry,
            functions: HashMap::new(),
            keybindings: KeyBindings::with_defaults(),
            scopes: ScopeStack::new(),
            blocks: BlockRegistry::new(),
            block_stack: BlockStack::new(),
            token_buffer: VecDeque::new(),
            ip: 0,
            skip_depth: 0,
            defining_function: None,
            function_body_buffer: Vec::new(),
            function_def_depth: 0,
            loop_body_buffer: Vec::new(),
            loop_collect_depth: 0,
            collecting_loop: None,
            loop_stack: Vec::new(),
            break_signal: false,
            continue_signal: false,
            expand_bindings: true,
            debug: false,
        }
    }

    /// Get a reference to the registry.
    #[must_use]
    pub fn registry(&self) -> &Registry<Self> {
        &self.registry
    }

    /// Get a mutable reference to the registry.
    #[must_use]
    pub fn registry_mut(&mut self) -> &mut Registry<Self> {
        &mut self.registry
    }

    /// Register an operation handler.
    pub fn register<F>(&mut self, name: impl Into<String>, handler: F)
    where
        F: Fn(&mut Self) -> Result<()> + Send + Sync + 'static,
    {
        self.registry.register(name, handler);
    }

    // ═══════════════════════════════════════════════════════════════
    // FUNCTION MANAGEMENT
    // ═══════════════════════════════════════════════════════════════

    /// Define a user function.
    pub fn define_function(&mut self, func: FunctionDef) {
        if self.debug {
            eprintln!("[debug] defined function: {} ({} tokens)", func.name, func.body.len());
        }
        self.functions.insert(func.name.clone(), func);
    }

    /// Check if a function is defined.
    #[must_use]
    pub fn has_function(&self, name: &str) -> bool {
        self.functions.contains_key(name)
    }

    /// Get a function definition.
    #[must_use]
    pub fn get_function(&self, name: &str) -> Option<&FunctionDef> {
        self.functions.get(name)
    }

    /// List all defined functions.
    #[must_use]
    pub fn function_names(&self) -> Vec<&str> {
        self.functions.keys().map(|s| s.as_str()).collect()
    }

    /// Call a user-defined function by name.
    pub fn call_function(&mut self, name: &str) -> Result<()> {
        // Get the function body (clone to avoid borrow issues)
        let func = self.functions.get(name)
            .ok_or_else(|| WofError::Runtime(format!("undefined function: '{name}'")))?
            .clone();

        if self.debug {
            eprintln!("[debug] calling function: {}", name);
        }

        // Save current execution context
        let frame = CallFrame {
            remaining_tokens: std::mem::take(&mut self.token_buffer),
            block_depth: self.block_stack.depth(),
        };
        self.call_stack.push(frame);

        // Create new scope for function
        self.push_scope(BlockType::Function);

        // Load function body into token buffer
        for token in &func.body {
            self.token_buffer.push_back(token.clone());
        }

        Ok(())
    }

    /// Return from the current function.
    pub fn return_from_function(&mut self) -> Result<()> {
        // Pop the function scope
        self.pop_scope();

        // Restore caller's execution context
        if let Some(frame) = self.call_stack.pop() {
            self.token_buffer = frame.remaining_tokens;
            if self.debug {
                eprintln!("[debug] returned from function");
            }
            Ok(())
        } else {
            // Return at top level - just clear tokens
            self.token_buffer.clear();
            Ok(())
        }
    }

    /// Check if we're currently inside a function call.
    #[must_use]
    pub fn in_function_call(&self) -> bool {
        !self.call_stack.is_empty()
    }

    // ═══════════════════════════════════════════════════════════════
    // KEYBINDINGS
    // ═══════════════════════════════════════════════════════════════

    /// Get a reference to the keybindings.
    #[must_use]
    pub fn keybindings(&self) -> &KeyBindings {
        &self.keybindings
    }

    /// Get mutable access to the keybindings.
    pub fn keybindings_mut(&mut self) -> &mut KeyBindings {
        &mut self.keybindings
    }

    /// Bind an alias to a glyph.
    pub fn bind(&mut self, alias: impl Into<String>, glyph: impl Into<String>) {
        self.keybindings.bind(alias, glyph);
    }

    /// Remove a binding.
    pub fn unbind(&mut self, alias: &str) -> bool {
        self.keybindings.unbind(alias)
    }

    /// Resolve an alias to its glyph.
    #[must_use]
    pub fn resolve_binding(&self, alias: &str) -> Option<&str> {
        self.keybindings.resolve(alias)
    }

    /// Load keybindings from the default file (~/.wofbinds).
    pub fn load_keybindings(&mut self) -> std::io::Result<usize> {
        self.keybindings.load_default()
    }

    /// Save keybindings to the default file (~/.wofbinds).
    pub fn save_keybindings(&self) -> std::io::Result<()> {
        self.keybindings.save_default()
    }

    // ═══════════════════════════════════════════════════════════════
    // VARIABLE ACCESS
    // ═══════════════════════════════════════════════════════════════

    /// Get the scope stack.
    #[must_use]
    pub fn scopes(&self) -> &ScopeStack {
        &self.scopes
    }

    /// Get mutable access to the scope stack.
    pub fn scopes_mut(&mut self) -> &mut ScopeStack {
        &mut self.scopes
    }

    /// Define a variable in the current scope.
    pub fn define_var(&mut self, name: impl Into<String>, value: WofValue) {
        self.scopes.define(name, value);
    }

    /// Get a variable's value.
    pub fn get_var(&self, name: &str) -> Result<WofValue> {
        self.scopes.get_var(name)
    }

    /// Set a variable's value (must already exist).
    pub fn set_var(&mut self, name: &str, value: WofValue) -> Result<()> {
        self.scopes.set_var(name, value)
    }

    /// Check if a variable is defined.
    #[must_use]
    pub fn has_var(&self, name: &str) -> bool {
        self.scopes.is_defined(name)
    }

    // ═══════════════════════════════════════════════════════════════
    // BLOCK & SCOPE MANAGEMENT
    // ═══════════════════════════════════════════════════════════════

    /// Push a new scope for a block.
    pub fn push_scope(&mut self, block_type: BlockType) -> BlockId {
        let block_id = self.blocks.register(
            block_type,
            self.ip,
            Some(self.block_stack.current()),
            Span::synthetic(),
        );
        self.block_stack.push(block_id);
        if block_type.creates_scope() {
            self.scopes.push(block_id);
        }
        block_id
    }

    /// Pop the current scope.
    pub fn pop_scope(&mut self) {
        if let Some(block_id) = self.block_stack.pop() {
            if let Some(block) = self.blocks.get(block_id) {
                if block.block_type.creates_scope() {
                    self.scopes.pop();
                }
            }
            self.blocks.close(block_id, self.ip);
        }
    }

    /// Get the current block depth.
    #[must_use]
    pub fn block_depth(&self) -> usize {
        self.block_stack.depth()
    }

    /// Get the current loop nesting depth.
    #[must_use]
    pub fn loop_depth(&self) -> usize {
        self.loop_stack.len()
    }

    /// Check if we're inside a loop.
    #[must_use]
    pub fn in_loop(&self) -> bool {
        !self.loop_stack.is_empty()
    }

    /// Get the current loop iteration (1-indexed), if in a loop.
    #[must_use]
    pub fn current_iteration(&self) -> Option<i64> {
        self.loop_stack.last().map(|f| f.iteration)
    }

    // ═══════════════════════════════════════════════════════════════
    // EXECUTION
    // ═══════════════════════════════════════════════════════════════

    /// Execute a single line of Woflang code.
    ///
    /// The line is tokenized and each token is dispatched through the
    /// interpreter. Errors are returned immediately; partial execution
    /// may have modified the stack.
    pub fn exec_line(&mut self, line: &str) -> Result<()> {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            return Ok(());
        }

        // Expand keybinding aliases if enabled
        let expanded = if self.expand_bindings {
            self.keybindings.expand_line(trimmed)
        } else {
            trimmed.to_string()
        };

        // Buffer all tokens for lookahead
        let tokenizer = Tokenizer::new(&expanded);
        self.token_buffer.clear();
        for token in tokenizer {
            self.token_buffer.push_back(token.into());
        }

        // Process tokens
        while let Some(token) = self.token_buffer.pop_front() {
            self.dispatch_owned_token(&token)?;
        }

        if self.debug {
            eprintln!("[debug] stack: {}", self.stack);
            eprintln!("[debug] scope depth: {}", self.scopes.depth());
        }

        Ok(())
    }

    /// Execute a script from a file.
    pub fn exec_file(&mut self, path: impl AsRef<Path>) -> Result<()> {
        let content = fs::read_to_string(path.as_ref()).map_err(WofError::from)?;
        for line in content.lines() {
            self.exec_line(line)?;
        }
        Ok(())
    }

    /// Run an interactive REPL (Read-Eval-Print Loop).
    ///
    /// Reads lines from stdin and executes them. Errors are printed
    /// but do not terminate the REPL.
    pub fn repl(&mut self) -> io::Result<()> {
        let stdin = io::stdin();
        let mut stdout = io::stdout();

        writeln!(stdout, "Woflang REPL v{}. Type 'exit' to quit.", woflang_core::VERSION)?;

        let reader = stdin.lock();
        for line in reader.lines() {
            let line = line?;
            let trimmed = line.trim();

            if trimmed == "exit" || trimmed == "quit" {
                writeln!(stdout, "Goodbye from woflang! 🐺")?;
                break;
            }

            if trimmed == ".s" || trimmed == "." {
                writeln!(stdout, "{}", self.stack)?;
                continue;
            }

            if trimmed == ":scope" || trimmed == ":vars" {
                let names = self.scopes.all_visible_names();
                writeln!(stdout, "Variables: {}", names.join(", "))?;
                continue;
            }

            if trimmed == ":funcs" || trimmed == ":functions" {
                let names = self.function_names();
                if names.is_empty() {
                    writeln!(stdout, "No functions defined")?;
                } else {
                    writeln!(stdout, "Functions: {}", names.join(", "))?;
                }
                continue;
            }

            // Keybinding commands
            if trimmed == ":binds" || trimmed == ":bindings" {
                let binds = self.keybindings.all();
                if binds.is_empty() {
                    writeln!(stdout, "No keybindings defined")?;
                } else {
                    writeln!(stdout, "Keybindings ({}):", binds.len())?;
                    for (alias, glyph) in binds {
                        writeln!(stdout, "  {} → {}", alias, glyph)?;
                    }
                }
                continue;
            }

            if let Some(rest) = trimmed.strip_prefix(":bind ") {
                let parts: Vec<&str> = rest.splitn(2, char::is_whitespace).collect();
                if parts.len() == 2 {
                    let alias = parts[0].trim();
                    let glyph = parts[1].trim();
                    self.bind(alias, glyph);
                    writeln!(stdout, "Bound: {} → {}", alias, glyph)?;
                } else {
                    writeln!(stdout, "Usage: :bind <alias> <glyph>")?;
                }
                continue;
            }

            if let Some(alias) = trimmed.strip_prefix(":unbind ") {
                let alias = alias.trim();
                if self.unbind(alias) {
                    writeln!(stdout, "Unbound: {}", alias)?;
                } else {
                    writeln!(stdout, "No binding for: {}", alias)?;
                }
                continue;
            }

            if trimmed == ":save-binds" {
                match self.save_keybindings() {
                    Ok(()) => writeln!(stdout, "Saved keybindings to ~/.wofbinds")?,
                    Err(e) => writeln!(stdout, "Failed to save: {}", e)?,
                }
                continue;
            }

            if trimmed == ":load-binds" {
                match self.load_keybindings() {
                    Ok(n) => writeln!(stdout, "Loaded {} keybindings from ~/.wofbinds", n)?,
                    Err(e) => writeln!(stdout, "Failed to load: {}", e)?,
                }
                continue;
            }

            if trimmed == ":help" {
                writeln!(stdout, "Woflang REPL Commands:")?;
                writeln!(stdout, "  .s, .          Show stack")?;
                writeln!(stdout, "  :vars          Show variables")?;
                writeln!(stdout, "  :funcs         Show functions")?;
                writeln!(stdout, "  :binds         Show keybindings")?;
                writeln!(stdout, "  :bind a g      Bind alias 'a' to glyph 'g'")?;
                writeln!(stdout, "  :unbind a      Remove binding for 'a'")?;
                writeln!(stdout, "  :save-binds    Save bindings to ~/.wofbinds")?;
                writeln!(stdout, "  :load-binds    Load bindings from ~/.wofbinds")?;
                writeln!(stdout, "  :help          Show this help")?;
                writeln!(stdout, "  exit, quit     Exit REPL")?;
                continue;
            }

            match self.exec_line(&line) {
                Ok(()) => {
                    if !self.stack.is_empty() {
                        if let Ok(top) = self.stack.peek() {
                            writeln!(stdout, "→ {top}")?;
                        }
                    }
                }
                Err(e) => {
                    writeln!(stdout, "Error: {e}")?;
                }
            }
        }

        Ok(())
    }

    /// Dispatch an owned token.
    fn dispatch_owned_token(&mut self, token: &OwnedToken) -> Result<()> {
        // If we're collecting a loop body, handle that first
        if self.collecting_loop.is_some() {
            return self.handle_loop_collect_mode(token);
        }

        // If we're defining a function, collect tokens
        if self.defining_function.is_some() {
            return self.handle_function_def_mode(token);
        }

        // If we're in skip mode, only process block delimiters
        if self.skip_depth > 0 {
            return self.handle_skip_mode(token);
        }

        // Check for break/continue signals
        if self.break_signal || self.continue_signal {
            // Skip tokens until we return to loop execution
            return Ok(());
        }

        match token.kind {
            TokenKind::Integer => {
                let value: i64 = token.text.parse()?;
                self.stack.push(WofValue::integer(value));
            }
            TokenKind::Float => {
                let value: f64 = token.text.parse()?;
                self.stack.push(WofValue::double(value));
            }
            TokenKind::String => {
                let value = crate::tokenizer::parse_string_literal(&token.text);
                self.stack.push(WofValue::string(value));
            }
            TokenKind::Symbol => {
                self.dispatch_symbol(&token.text, token.span)?;
            }
            TokenKind::Label => {
                // Label definition (:name) - register for jump targets
                let name = token.text.trim_start_matches(':');
                // Store current position as label target
                // For now, just acknowledge it
                if self.debug {
                    eprintln!("[debug] label defined: {name}");
                }
            }
            TokenKind::LabelRef => {
                // Label reference (@name) - for jumps
                let name = token.text.trim_start_matches('@');
                self.stack.push(WofValue::symbol(format!("@{name}")));
            }
            TokenKind::Eof => {}
        }
        Ok(())
    }

    /// Handle tokens while collecting a loop body.
    fn handle_loop_collect_mode(&mut self, token: &OwnedToken) -> Result<()> {
        match token.text.as_str() {
            "⺆" | "⟳" | "loop" => {
                // Nested block/loop - increase depth
                self.loop_collect_depth += 1;
                self.loop_body_buffer.push(token.clone());
            }
            "⺘" => {
                if self.loop_collect_depth == 0 {
                    // End of loop body - execute it
                    let loop_type = self.collecting_loop.take().unwrap();
                    let body = std::mem::take(&mut self.loop_body_buffer);
                    self.execute_loop(loop_type, body)?;
                } else {
                    // End of nested block
                    self.loop_collect_depth -= 1;
                    self.loop_body_buffer.push(token.clone());
                }
            }
            _ => {
                // Collect token into loop body
                self.loop_body_buffer.push(token.clone());
            }
        }
        Ok(())
    }

    /// Execute a loop with the given body.
    fn execute_loop(&mut self, loop_type: LoopType, body: Vec<OwnedToken>) -> Result<()> {
        let max_iterations = match loop_type {
            LoopType::Infinite => 0, // 0 = no limit
            LoopType::Repeat(n) => n,
            LoopType::While => 0, // Condition checked each iteration
        };

        if self.debug {
            eprintln!("[debug] executing loop: {:?}, body has {} tokens", loop_type, body.len());
        }

        // Push loop frame
        self.loop_stack.push(LoopFrame {
            body: body.clone(),
            loop_type,
            iteration: 0,
            max_iterations,
        });

        // Create scope for loop
        self.push_scope(BlockType::Loop);

        // Execute loop iterations
        loop {
            // Check iteration limit for repeat loops
            if let Some(frame) = self.loop_stack.last_mut() {
                if frame.max_iterations > 0 && frame.iteration >= frame.max_iterations {
                    break;
                }
                frame.iteration += 1;
            }

            // Execute loop body
            for token in &body {
                self.dispatch_owned_token(token)?;
                
                // Check for break
                if self.break_signal {
                    self.break_signal = false;
                    // Exit the loop
                    self.loop_stack.pop();
                    self.pop_scope();
                    return Ok(());
                }
                
                // Check for continue
                if self.continue_signal {
                    self.continue_signal = false;
                    break; // Break inner loop, continue outer
                }
            }

            // Safety limit for infinite loops (prevent runaway in REPL)
            if let Some(frame) = self.loop_stack.last() {
                if frame.loop_type == LoopType::Infinite && frame.iteration > 1_000_000 {
                    self.loop_stack.pop();
                    self.pop_scope();
                    return Err(WofError::Runtime("infinite loop safety limit reached (1M iterations)".into()));
                }
            }
        }

        // Normal loop completion
        self.loop_stack.pop();
        self.pop_scope();
        Ok(())
    }

    /// Handle tokens while collecting a function definition.
    fn handle_function_def_mode(&mut self, token: &OwnedToken) -> Result<()> {
        match token.text.as_str() {
            "⺆" => {
                // Opening a nested block inside function
                self.function_def_depth += 1;
                self.function_body_buffer.push(token.clone());
            }
            "⺘" => {
                if self.function_def_depth == 0 {
                    // End of function definition
                    let name = self.defining_function.take().unwrap();
                    let body = std::mem::take(&mut self.function_body_buffer);
                    let func = FunctionDef::new(name, body, token.span);
                    self.define_function(func);
                } else {
                    // End of nested block inside function
                    self.function_def_depth -= 1;
                    self.function_body_buffer.push(token.clone());
                }
            }
            _ => {
                // Collect token into function body
                self.function_body_buffer.push(token.clone());
            }
        }
        Ok(())
    }

    /// Handle tokens while in skip mode (skipping else branches etc).
    fn handle_skip_mode(&mut self, token: &OwnedToken) -> Result<()> {
        match token.text.as_str() {
            "⺆" | "若" | "loop" | "⟳" => {
                // Nested block - increase skip depth
                self.skip_depth += 1;
            }
            "⺘" => {
                // Block close - decrease skip depth
                self.skip_depth = self.skip_depth.saturating_sub(1);
            }
            "或" if self.skip_depth == 1 => {
                // We hit the else branch at our skip level - stop skipping
                self.skip_depth = 0;
            }
            _ => {
                // Skip this token
            }
        }
        Ok(())
    }

    /// Dispatch a symbol (operation or identifier).
    fn dispatch_symbol(&mut self, name: &str, span: Span) -> Result<()> {
        // ═══════════════════════════════════════════════════════════════
        // FUNCTION DEFINITION: ⊕name ⺆ ... ⺘
        // ═══════════════════════════════════════════════════════════════
        if name == "⊕" || name == "fn" || name == "func" || name == "def" {
            // Next token is function name, then ⺆
            if let Some(next) = self.token_buffer.pop_front() {
                if next.kind == TokenKind::Symbol {
                    let func_name = next.text.clone();
                    // Expect ⺆ next
                    if let Some(block_start) = self.token_buffer.pop_front() {
                        if block_start.text == "⺆" {
                            self.defining_function = Some(func_name);
                            self.function_body_buffer.clear();
                            self.function_def_depth = 0;
                            return Ok(());
                        }
                        self.token_buffer.push_front(block_start);
                    }
                }
                self.token_buffer.push_front(next);
            }
            return Err(WofError::Runtime("⊕ requires: ⊕ name ⺆ body ⺘".into()));
        }

        // ═══════════════════════════════════════════════════════════════
        // FUNCTION CALL: 巡 name
        // ═══════════════════════════════════════════════════════════════
        if name == "巡" || name == "call" {
            if let Some(next) = self.token_buffer.pop_front() {
                if next.kind == TokenKind::Symbol {
                    return self.call_function(&next.text);
                }
                self.token_buffer.push_front(next);
            }
            return Err(WofError::Runtime("巡 requires a function name".into()));
        }

        // ═══════════════════════════════════════════════════════════════
        // RETURN: 至
        // ═══════════════════════════════════════════════════════════════
        if name == "至" || name == "return" || name == "ret" {
            return self.return_from_function();
        }

        // ═══════════════════════════════════════════════════════════════
        // INFINITE LOOP: ⟳ ⺆ ... ⺘
        // ═══════════════════════════════════════════════════════════════
        if name == "⟳" || name == "loop" {
            // Expect ⺆ next
            if let Some(block_start) = self.token_buffer.pop_front() {
                if block_start.text == "⺆" {
                    self.collecting_loop = Some(LoopType::Infinite);
                    self.loop_body_buffer.clear();
                    self.loop_collect_depth = 0;
                    return Ok(());
                }
                self.token_buffer.push_front(block_start);
            }
            return Err(WofError::Runtime("⟳ requires: ⟳ ⺆ body ⺘".into()));
        }

        // ═══════════════════════════════════════════════════════════════
        // REPEAT N TIMES: N ⨯ ⺆ ... ⺘  or  ⨯ ⺆ ... ⺘ (N from stack)
        // ═══════════════════════════════════════════════════════════════
        if name == "⨯" || name == "times" || name == "repeat" {
            // Get count from stack
            let count = self.stack.pop()?.as_integer()?;
            
            // Expect ⺆ next
            if let Some(block_start) = self.token_buffer.pop_front() {
                if block_start.text == "⺆" {
                    self.collecting_loop = Some(LoopType::Repeat(count));
                    self.loop_body_buffer.clear();
                    self.loop_collect_depth = 0;
                    return Ok(());
                }
                self.token_buffer.push_front(block_start);
            }
            return Err(WofError::Runtime("⨯ requires: N ⨯ ⺆ body ⺘".into()));
        }

        // ═══════════════════════════════════════════════════════════════
        // BREAK: 🛑 (exit innermost loop)
        // ═══════════════════════════════════════════════════════════════
        if name == "🛑" || name == "break" {
            if self.loop_stack.is_empty() {
                return Err(WofError::Runtime("🛑 (break) outside of loop".into()));
            }
            self.break_signal = true;
            return Ok(());
        }

        // ═══════════════════════════════════════════════════════════════
        // CONTINUE: ↻ (restart loop iteration)
        // ═══════════════════════════════════════════════════════════════
        if name == "↻" || name == "continue" {
            if self.loop_stack.is_empty() {
                return Err(WofError::Runtime("↻ (continue) outside of loop".into()));
            }
            self.continue_signal = true;
            return Ok(());
        }

        // ═══════════════════════════════════════════════════════════════
        // VARIABLE READ: 読 varname
        // ═══════════════════════════════════════════════════════════════
        if name == "読" || name == "load" || name == "get" {
            if let Some(next) = self.token_buffer.pop_front() {
                if next.kind == TokenKind::Symbol {
                    let value = self.get_var(&next.text)?;
                    self.stack.push(value);
                    return Ok(());
                }
                self.token_buffer.push_front(next);
            }
            return Err(WofError::Runtime("読 requires a variable name".into()));
        }

        // ═══════════════════════════════════════════════════════════════
        // VARIABLE DEFINE: 字 varname (value from stack)
        // ═══════════════════════════════════════════════════════════════
        if name == "字" || name == "define" || name == "let" {
            if let Some(next) = self.token_buffer.pop_front() {
                if next.kind == TokenKind::Symbol {
                    let var_name = next.text.clone();
                    let value = self.stack.pop()?;
                    self.define_var(var_name, value);
                    return Ok(());
                }
                self.token_buffer.push_front(next);
            }
            return Err(WofError::Runtime("字 requires a variable name".into()));
        }

        // ═══════════════════════════════════════════════════════════════
        // VARIABLE SET: 支 varname (value from stack)
        // ═══════════════════════════════════════════════════════════════
        if name == "支" || name == "set" || name == "store" {
            if let Some(next) = self.token_buffer.pop_front() {
                if next.kind == TokenKind::Symbol {
                    let value = self.stack.pop()?;
                    self.set_var(&next.text, value)?;
                    return Ok(());
                }
                self.token_buffer.push_front(next);
            }
            return Err(WofError::Runtime("支 requires a variable name".into()));
        }

        // ═══════════════════════════════════════════════════════════════
        // CONDITIONALS: 若 (if)
        // ═══════════════════════════════════════════════════════════════
        if name == "若" || name == "if" {
            let condition = self.stack.pop()?;
            let is_true = condition.is_truthy();
            
            if is_true {
                self.push_scope(BlockType::If);
            } else {
                self.skip_depth = 1;
            }
            return Ok(());
        }

        // Check for else: 或
        if name == "或" || name == "else" {
            // If we're here, we executed the then branch - skip the else
            self.skip_depth = 1;
            return Ok(());
        }

        // ═══════════════════════════════════════════════════════════════
        // BLOCK DELIMITERS
        // ═══════════════════════════════════════════════════════════════
        if name == "⺆" {
            self.push_scope(BlockType::Generic);
            return Ok(());
        }

        if name == "⺘" {
            self.pop_scope();
            return Ok(());
        }

        // ═══════════════════════════════════════════════════════════════
        // REGISTERED OPERATIONS
        // ═══════════════════════════════════════════════════════════════
        if let Some(op) = self.registry.get_cloned(name) {
            return op(self);
        }

        // ═══════════════════════════════════════════════════════════════
        // USER-DEFINED FUNCTIONS (call by name)
        // ═══════════════════════════════════════════════════════════════
        if self.has_function(name) {
            return self.call_function(name);
        }

        // ═══════════════════════════════════════════════════════════════
        // VARIABLES (auto-load by name)
        // ═══════════════════════════════════════════════════════════════
        if self.has_var(name) {
            let value = self.get_var(name)?;
            self.stack.push(value);
            return Ok(());
        }

        // Not found: push as symbol
        let _ = span; // Reserved for future error reporting
        self.stack.push(WofValue::symbol(name));
        Ok(())
    }
}

// ═══════════════════════════════════════════════════════════════════════
// InterpreterContext IMPLEMENTATION
// ═══════════════════════════════════════════════════════════════════════

impl InterpreterContext for Interpreter {
    #[inline]
    fn push(&mut self, value: WofValue) {
        self.stack.push(value);
    }

    #[inline]
    fn pop(&mut self) -> Result<WofValue> {
        self.stack.pop()
    }

    #[inline]
    fn peek(&self) -> Result<&WofValue> {
        self.stack.peek()
    }

    #[inline]
    fn has(&self, n: usize) -> bool {
        self.stack.has(n)
    }

    #[inline]
    fn stack(&self) -> &WofStack {
        &self.stack
    }

    #[inline]
    fn stack_mut(&mut self) -> &mut WofStack {
        &mut self.stack
    }

    #[inline]
    fn clear(&mut self) {
        self.stack.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_interp() -> Interpreter {
        let mut interp = Interpreter::new();

        // Register basic ops for testing
        interp.register("+", |ctx| {
            let b = ctx.stack_mut().pop_numeric()?;
            let a = ctx.stack_mut().pop_numeric()?;
            ctx.push(WofValue::double(a + b));
            Ok(())
        });

        interp.register("-", |ctx| {
            let b = ctx.stack_mut().pop_numeric()?;
            let a = ctx.stack_mut().pop_numeric()?;
            ctx.push(WofValue::double(a - b));
            Ok(())
        });

        interp.register("dup", |ctx| ctx.stack_mut().dup());
        interp.register("drop", |ctx| ctx.stack_mut().drop());
        interp.register("swap", |ctx| ctx.stack_mut().swap());

        interp
    }

    #[test]
    fn exec_arithmetic() {
        let mut interp = make_interp();
        interp.exec_line("5 3 +").unwrap();

        let result = interp.stack.pop_numeric().unwrap();
        assert!((result - 8.0).abs() < f64::EPSILON);
    }

    #[test]
    fn exec_stack_ops() {
        let mut interp = make_interp();
        interp.exec_line("42 dup").unwrap();

        assert_eq!(interp.stack.len(), 2);
        assert_eq!(interp.stack.pop_integer().unwrap(), 42);
        assert_eq!(interp.stack.pop_integer().unwrap(), 42);
    }

    #[test]
    fn exec_swap() {
        let mut interp = make_interp();
        interp.exec_line("1 2 swap").unwrap();

        assert_eq!(interp.stack.pop_integer().unwrap(), 1);
        assert_eq!(interp.stack.pop_integer().unwrap(), 2);
    }

    #[test]
    fn unknown_symbol_pushed() {
        let mut interp = make_interp();
        interp.exec_line("undefined_op").unwrap();

        let val = interp.stack.pop().unwrap();
        assert_eq!(val.as_str().unwrap(), "undefined_op");
    }

    #[test]
    fn parse_string_literal() {
        let mut interp = make_interp();
        interp.exec_line(r#""hello world""#).unwrap();

        let val = interp.stack.pop().unwrap();
        assert_eq!(val.as_str().unwrap(), "hello world");
    }

    #[test]
    fn empty_line_noop() {
        let mut interp = make_interp();
        interp.exec_line("").unwrap();
        interp.exec_line("   ").unwrap();

        assert!(interp.stack.is_empty());
    }
}
