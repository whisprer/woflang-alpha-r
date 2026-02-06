//! Opcode definitions for Woflang.
//!
//! This module defines the complete set of operations supported by the
//! Woflang interpreter, including their Unicode glyph representations
//! and semantic categories.

use std::fmt;

/// Woflang operation codes.
///
/// Each variant corresponds to a primitive operation in the language.
/// Many operations have Unicode glyph aliases for concise notation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum Opcode {
    // ═══════════════════════════════════════════════════════════════════
    // CONTROL FLOW (0-19)
    // ═══════════════════════════════════════════════════════════════════
    /// No operation (無)
    Nop = 0,
    /// Define function (⊕)
    DefineFunc = 1,
    /// If condition (若)
    If = 2,
    /// Then branch (則)
    Then = 3,
    /// Else branch (或)
    Else = 4,
    /// Return from function (至)
    Return = 5,
    /// Block open (⺆)
    BlockOpen = 6,
    /// Block close (⺘)
    BlockClose = 7,
    /// Else-if branch (另)
    ElseIf = 8,
    /// Logical join (而)
    Join = 9,

    // ═══════════════════════════════════════════════════════════════════
    // ARITHMETIC (10-19)
    // ═══════════════════════════════════════════════════════════════════
    /// Addition (+)
    Add = 10,
    /// Subtraction (-)
    Sub = 11,
    /// Multiplication (*)
    Mul = 12,
    /// Division (/)
    Div = 13,
    /// Modulo (%)
    Mod = 14,
    /// Power (^, pow)
    Pow = 15,
    /// Negation (neg)
    Neg = 16,
    /// Absolute value (abs)
    Abs = 17,
    /// Increment (inc)
    Inc = 18,
    /// Decrement (dec)
    Dec = 19,

    // ═══════════════════════════════════════════════════════════════════
    // STACK MANIPULATION (20-39)
    // ═══════════════════════════════════════════════════════════════════
    /// Duplicate top (dup)
    Dup = 20,
    /// Swap top two (swap)
    Swap = 21,
    /// Drop top (drop)
    Drop = 22,
    /// Copy second to top (over)
    Over = 23,
    /// Rotate top three (rot)
    Rot = 24,
    /// Remove second (nip)
    Nip = 25,
    /// Copy top below second (tuck)
    Tuck = 26,
    /// Push stack depth (depth)
    Depth = 27,
    /// Clear stack (clear)
    Clear = 28,
    /// Pick nth item (pick)
    Pick = 29,
    /// Roll n items (roll)
    Roll = 30,
    /// Stack push marker (卜)
    StackPush = 31,
    /// Stack pop marker (卩)
    StackPop = 32,

    // ═══════════════════════════════════════════════════════════════════
    // COMPARISON (40-59)
    // ═══════════════════════════════════════════════════════════════════
    /// Equal (=, ==, 齊)
    Eq = 40,
    /// Not equal (!=, ≠)
    Ne = 41,
    /// Less than (<, 小)
    Lt = 42,
    /// Greater than (>, 大)
    Gt = 43,
    /// Less or equal (<=, ≤)
    Le = 44,
    /// Greater or equal (>=, ≥)
    Ge = 45,
    /// Compare (比)
    Cmp = 46,

    // ═══════════════════════════════════════════════════════════════════
    // LOGIC (60-79)
    // ═══════════════════════════════════════════════════════════════════
    /// Logical AND (∧, and)
    And = 60,
    /// Logical OR (∨, or)
    Or = 61,
    /// Logical NOT (¬, not, 非)
    Not = 62,
    /// Logical XOR (⊕, xor)
    Xor = 63,
    /// Implication (→, implies)
    Implies = 64,
    /// Biconditional (↔, iff)
    Iff = 65,
    /// NAND (⊼)
    Nand = 66,
    /// NOR (⊽)
    Nor = 67,

    // ═══════════════════════════════════════════════════════════════════
    // I/O (80-99)
    // ═══════════════════════════════════════════════════════════════════
    /// Emit/print (響, emit)
    Emit = 88,
    /// Show stack (.)
    ShowStack = 89,
    /// Print newline (cr)
    Cr = 90,
    /// Read input (read)
    Read = 91,

    // ═══════════════════════════════════════════════════════════════════
    // FUNCTIONS & CALLS (99-109)
    // ═══════════════════════════════════════════════════════════════════
    /// Function call (巡, call)
    Call = 99,
    /// Jump to label (@label)
    Jump = 100,
    /// Define label (:label)
    Label = 101,
    /// Recursion (自)
    Recur = 102,
    /// Alternate/fork (又)
    Alt = 103,

    // ═══════════════════════════════════════════════════════════════════
    // VARIABLES (110-119)
    // ═══════════════════════════════════════════════════════════════════
    /// Define symbol (字)
    DefineVar = 110,
    /// Read symbol (読)
    ReadVar = 111,
    /// Assign value (支, set)
    Set = 112,
    /// Variable declaration (谷)
    Var = 113,
    /// Self reference (己)
    SelfRef = 114,
    /// Bind/alias (押)
    Bind = 115,

    // ═══════════════════════════════════════════════════════════════════
    // MODULES (120-139)
    // ═══════════════════════════════════════════════════════════════════
    /// Module open (門)
    Module = 120,
    /// Module scope (⻔)
    ModScope = 121,
    /// Import (⺢)
    Import = 122,
    /// Macro (⻤)
    Macro = 123,

    // ═══════════════════════════════════════════════════════════════════
    // DEBUG & CONTROL (200-219)
    // ═══════════════════════════════════════════════════════════════════
    /// Assert (assert)
    Assert = 200,
    /// Loop (⟳)
    Loop = 201,
    /// Repeat N times (⨯)
    Repeat = 202,
    /// For loop (⺀)
    For = 203,
    /// Break loop (🛑, 出)
    Break = 204,
    /// Continue loop (↻)
    Continue = 205,
    /// Halt execution (止)
    Halt = 206,
    /// End program (終)
    End = 207,
    /// Await/delay (待)
    Await = 208,
    /// Sleep (眠)
    Sleep = 209,

    // ═══════════════════════════════════════════════════════════════════
    // META & SPECIAL (220-255)
    // ═══════════════════════════════════════════════════════════════════
    /// Error (⺣)
    Error = 220,
    /// Wildcard/random (⺨)
    Wild = 221,
    /// Failure (舛)
    Fail = 222,
    /// Metadata (⺙)
    Meta = 223,
    /// Flag (⻙)
    Flag = 224,
    /// Legacy fallback (老)
    Legacy = 225,
    /// Context marker (⽰)
    CtxMark = 226,
    /// Generic operation (工)
    Op = 227,
    /// Separator (丶)
    Sep = 228,
    /// Arrow (丿)
    Arrow = 229,

    // ═══════════════════════════════════════════════════════════════════
    // PUSH LITERAL (special, not a real opcode)
    // ═══════════════════════════════════════════════════════════════════
    /// Push a literal value (internal use)
    PushLiteral = 1000,
    /// Push a symbol name (internal use)
    PushSymbol = 1001,
}

impl Opcode {
    /// Try to parse an opcode from a glyph string.
    #[must_use]
    pub fn from_glyph(glyph: &str) -> Option<Self> {
        Some(match glyph {
            // Control flow
            "無" | "nop" => Self::Nop,
            "⊕" | "def" => Self::DefineFunc,
            "若" | "if" => Self::If,
            "則" | "then" => Self::Then,
            "或" | "else" => Self::Else,
            "至" | "ret" | "return" => Self::Return,
            "⺆" => Self::BlockOpen,
            "⺘" => Self::BlockClose,
            "另" | "elif" => Self::ElseIf,
            "而" | "join" => Self::Join,

            // Arithmetic
            "+" | "add" => Self::Add,
            "-" | "sub" => Self::Sub,
            "*" | "×" | "mul" => Self::Mul,
            "/" | "÷" | "div" => Self::Div,
            "%" | "mod" => Self::Mod,
            "^" | "pow" => Self::Pow,
            "neg" => Self::Neg,
            "abs" => Self::Abs,
            "inc" => Self::Inc,
            "dec" => Self::Dec,

            // Stack
            "dup" => Self::Dup,
            "swap" => Self::Swap,
            "drop" => Self::Drop,
            "over" => Self::Over,
            "rot" => Self::Rot,
            "nip" => Self::Nip,
            "tuck" => Self::Tuck,
            "depth" => Self::Depth,
            "clear" | "stack_slayer" => Self::Clear,
            "pick" => Self::Pick,
            "roll" => Self::Roll,
            "卜" => Self::StackPush,
            "卩" => Self::StackPop,

            // Comparison
            "=" | "==" | "齊" | "eq" => Self::Eq,
            "!=" | "≠" | "ne" => Self::Ne,
            "<" | "小" | "lt" => Self::Lt,
            ">" | "大" | "gt" => Self::Gt,
            "<=" | "≤" | "le" => Self::Le,
            ">=" | "≥" | "ge" => Self::Ge,
            "比" | "cmp" => Self::Cmp,

            // Logic
            "∧" | "and" => Self::And,
            "∨" | "or" => Self::Or,
            "¬" | "not" | "非" => Self::Not,
            "⊻" | "xor" => Self::Xor,
            "→" | "implies" => Self::Implies,
            "↔" | "iff" => Self::Iff,
            "⊼" | "nand" => Self::Nand,
            "⊽" | "nor" => Self::Nor,

            // I/O
            "響" | "emit" | "print" => Self::Emit,
            "." | ".s" | "show" => Self::ShowStack,
            "cr" => Self::Cr,
            "read" => Self::Read,

            // Functions
            "巡" | "call" => Self::Call,
            "自" | "recur" => Self::Recur,
            "又" | "alt" => Self::Alt,

            // Variables
            "字" | "define" => Self::DefineVar,
            "読" | "load" | "get" => Self::ReadVar,
            "支" | "set" | "store" => Self::Set,
            "谷" | "var" => Self::Var,
            "己" | "self" => Self::SelfRef,
            "押" | "bind" => Self::Bind,

            // Modules
            "門" | "module" => Self::Module,
            "⻔" | "mod_scope" => Self::ModScope,
            "⺢" | "import" => Self::Import,
            "⻤" | "macro" => Self::Macro,

            // Debug & control
            "assert" => Self::Assert,
            "⟳" | "loop" => Self::Loop,
            "⨯" | "repeat" => Self::Repeat,
            "⺀" | "for" => Self::For,
            "🛑" | "出" | "break" => Self::Break,
            "↻" | "continue" => Self::Continue,
            "止" | "halt" => Self::Halt,
            "終" | "end" => Self::End,
            "待" | "await" => Self::Await,
            "眠" | "sleep" => Self::Sleep,

            // Meta
            "⺣" | "err" => Self::Error,
            "⺨" | "wild" => Self::Wild,
            "舛" | "fail" => Self::Fail,
            "⺙" | "meta" => Self::Meta,
            "⻙" | "flag" => Self::Flag,
            "老" | "legacy" => Self::Legacy,
            "⽰" | "ctx" => Self::CtxMark,
            "工" | "op" => Self::Op,
            "丶" | "sep" => Self::Sep,
            "丿" | "arrow" => Self::Arrow,

            _ => return None,
        })
    }

    /// Get the primary glyph for this opcode.
    #[must_use]
    pub const fn glyph(&self) -> &'static str {
        match self {
            Self::Nop => "無",
            Self::DefineFunc => "⊕",
            Self::If => "若",
            Self::Then => "則",
            Self::Else => "或",
            Self::Return => "至",
            Self::BlockOpen => "⺆",
            Self::BlockClose => "⺘",
            Self::ElseIf => "另",
            Self::Join => "而",
            Self::Add => "+",
            Self::Sub => "-",
            Self::Mul => "*",
            Self::Div => "/",
            Self::Mod => "%",
            Self::Pow => "^",
            Self::Neg => "neg",
            Self::Abs => "abs",
            Self::Inc => "inc",
            Self::Dec => "dec",
            Self::Dup => "dup",
            Self::Swap => "swap",
            Self::Drop => "drop",
            Self::Over => "over",
            Self::Rot => "rot",
            Self::Nip => "nip",
            Self::Tuck => "tuck",
            Self::Depth => "depth",
            Self::Clear => "clear",
            Self::Pick => "pick",
            Self::Roll => "roll",
            Self::StackPush => "卜",
            Self::StackPop => "卩",
            Self::Eq => "=",
            Self::Ne => "≠",
            Self::Lt => "<",
            Self::Gt => ">",
            Self::Le => "≤",
            Self::Ge => "≥",
            Self::Cmp => "比",
            Self::And => "∧",
            Self::Or => "∨",
            Self::Not => "¬",
            Self::Xor => "⊻",
            Self::Implies => "→",
            Self::Iff => "↔",
            Self::Nand => "⊼",
            Self::Nor => "⊽",
            Self::Emit => "響",
            Self::ShowStack => ".",
            Self::Cr => "cr",
            Self::Read => "read",
            Self::Call => "巡",
            Self::Jump => "@",
            Self::Label => ":",
            Self::Recur => "自",
            Self::Alt => "又",
            Self::DefineVar => "字",
            Self::ReadVar => "読",
            Self::Set => "支",
            Self::Var => "谷",
            Self::SelfRef => "己",
            Self::Bind => "押",
            Self::Module => "門",
            Self::ModScope => "⻔",
            Self::Import => "⺢",
            Self::Macro => "⻤",
            Self::Assert => "assert",
            Self::Loop => "⟳",
            Self::Repeat => "⨯",
            Self::For => "⺀",
            Self::Break => "🛑",
            Self::Continue => "↻",
            Self::Halt => "止",
            Self::End => "終",
            Self::Await => "待",
            Self::Sleep => "眠",
            Self::Error => "⺣",
            Self::Wild => "⺨",
            Self::Fail => "舛",
            Self::Meta => "⺙",
            Self::Flag => "⻙",
            Self::Legacy => "老",
            Self::CtxMark => "⽰",
            Self::Op => "工",
            Self::Sep => "丶",
            Self::Arrow => "丿",
            Self::PushLiteral => "<literal>",
            Self::PushSymbol => "<symbol>",
        }
    }

    /// Get the semantic category of this opcode.
    #[must_use]
    pub const fn category(&self) -> OpcodeCategory {
        match self {
            Self::Nop | Self::DefineFunc | Self::If | Self::Then | Self::Else
            | Self::Return | Self::BlockOpen | Self::BlockClose | Self::ElseIf
            | Self::Join => OpcodeCategory::Control,

            Self::Add | Self::Sub | Self::Mul | Self::Div | Self::Mod
            | Self::Pow | Self::Neg | Self::Abs | Self::Inc | Self::Dec => OpcodeCategory::Arithmetic,

            Self::Dup | Self::Swap | Self::Drop | Self::Over | Self::Rot
            | Self::Nip | Self::Tuck | Self::Depth | Self::Clear | Self::Pick
            | Self::Roll | Self::StackPush | Self::StackPop => OpcodeCategory::Stack,

            Self::Eq | Self::Ne | Self::Lt | Self::Gt | Self::Le | Self::Ge
            | Self::Cmp => OpcodeCategory::Comparison,

            Self::And | Self::Or | Self::Not | Self::Xor | Self::Implies
            | Self::Iff | Self::Nand | Self::Nor => OpcodeCategory::Logic,

            Self::Emit | Self::ShowStack | Self::Cr | Self::Read => OpcodeCategory::Io,

            Self::Call | Self::Jump | Self::Label | Self::Recur | Self::Alt => OpcodeCategory::Function,

            Self::DefineVar | Self::ReadVar | Self::Set | Self::Var
            | Self::SelfRef | Self::Bind => OpcodeCategory::Variable,

            Self::Module | Self::ModScope | Self::Import | Self::Macro => OpcodeCategory::Module,

            Self::Assert | Self::Loop | Self::Repeat | Self::For | Self::Break
            | Self::Continue | Self::Halt | Self::End | Self::Await | Self::Sleep => OpcodeCategory::Debug,

            Self::Error | Self::Wild | Self::Fail | Self::Meta | Self::Flag
            | Self::Legacy | Self::CtxMark | Self::Op | Self::Sep | Self::Arrow
            | Self::PushLiteral | Self::PushSymbol => OpcodeCategory::Meta,
        }
    }

    /// Check if this opcode affects control flow.
    #[must_use]
    pub const fn is_control_flow(&self) -> bool {
        matches!(
            self,
            Self::If | Self::Then | Self::Else | Self::ElseIf | Self::Return
            | Self::Jump | Self::Call | Self::Loop | Self::Repeat | Self::For
            | Self::Break | Self::Continue | Self::Halt | Self::End
        )
    }

    /// Check if this opcode opens a new block.
    #[must_use]
    pub const fn opens_block(&self) -> bool {
        matches!(
            self,
            Self::BlockOpen | Self::DefineFunc | Self::If | Self::Then
            | Self::Else | Self::ElseIf | Self::Loop | Self::For
        )
    }

    /// Check if this opcode closes a block.
    #[must_use]
    pub const fn closes_block(&self) -> bool {
        matches!(self, Self::BlockClose | Self::End)
    }
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.glyph())
    }
}

/// Categories of opcodes for semantic grouping.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OpcodeCategory {
    /// Control flow operations
    Control,
    /// Arithmetic operations
    Arithmetic,
    /// Stack manipulation
    Stack,
    /// Comparison operations
    Comparison,
    /// Logical operations
    Logic,
    /// Input/output operations
    Io,
    /// Function-related operations
    Function,
    /// Variable operations
    Variable,
    /// Module operations
    Module,
    /// Debug and flow control
    Debug,
    /// Meta and special operations
    Meta,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn glyph_parsing() {
        assert_eq!(Opcode::from_glyph("若"), Some(Opcode::If));
        assert_eq!(Opcode::from_glyph("if"), Some(Opcode::If));
        assert_eq!(Opcode::from_glyph("⺆"), Some(Opcode::BlockOpen));
        assert_eq!(Opcode::from_glyph("+"), Some(Opcode::Add));
    }

    #[test]
    fn opcode_categories() {
        assert_eq!(Opcode::If.category(), OpcodeCategory::Control);
        assert_eq!(Opcode::Add.category(), OpcodeCategory::Arithmetic);
        assert_eq!(Opcode::Dup.category(), OpcodeCategory::Stack);
    }

    #[test]
    fn block_detection() {
        assert!(Opcode::BlockOpen.opens_block());
        assert!(Opcode::If.opens_block());
        assert!(Opcode::BlockClose.closes_block());
        assert!(!Opcode::Add.opens_block());
    }
}
