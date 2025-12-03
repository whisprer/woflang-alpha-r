//! Simple chess engine for Woflang.
//!
//! A minimal but complete chess engine:
//! - Full board representation
//! - Legal move generation (no castling/en passant for simplicity)
//! - 3-ply alpha-beta search for engine reply
//! - Auto-promotion to queen
//!
//! ## Operations
//!
//! - `chess_new` - Start a new game
//! - `chess_show` - Display the board
//! - `chess_move` - Make a move (e.g., "e2e4")

use std::sync::{Mutex, OnceLock};
use woflang_core::WofValue;
use woflang_runtime::Interpreter;

// ═══════════════════════════════════════════════════════════════════════════
// BOARD REPRESENTATION
// ═══════════════════════════════════════════════════════════════════════════

/// Chess position.
#[derive(Clone)]
struct ChessPosition {
    /// Board squares: 'P','N','B','R','Q','K' for white (uppercase),
    /// 'p','n','b','r','q','k' for black (lowercase), '.' for empty.
    board: [char; 64],
    /// True if white to move.
    white_to_move: bool,
}

impl Default for ChessPosition {
    fn default() -> Self {
        let mut pos = ChessPosition {
            board: ['.'; 64],
            white_to_move: true,
        };
        pos.init_start();
        pos
    }
}

impl ChessPosition {
    /// Initialize to standard starting position.
    fn init_start(&mut self) {
        self.board.fill('.');
        
        // Back rank pieces
        let back = ['R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R'];
        
        for (f, &piece) in back.iter().enumerate() {
            // White back rank
            self.board[square_index(f as i32, 0)] = piece;
            // White pawns
            self.board[square_index(f as i32, 1)] = 'P';
            // Black pawns
            self.board[square_index(f as i32, 6)] = 'p';
            // Black back rank
            self.board[square_index(f as i32, 7)] = piece.to_ascii_lowercase();
        }
        
        self.white_to_move = true;
    }

    /// Get piece at square.
    fn at(&self, sq: i32) -> char {
        if sq >= 0 && sq < 64 {
            self.board[sq as usize]
        } else {
            '.'
        }
    }

    /// Set piece at square.
    fn set(&mut self, sq: i32, piece: char) {
        if sq >= 0 && sq < 64 {
            self.board[sq as usize] = piece;
        }
    }
}

/// Global game state.
fn game_state() -> &'static Mutex<ChessPosition> {
    static STATE: OnceLock<Mutex<ChessPosition>> = OnceLock::new();
    STATE.get_or_init(|| Mutex::new(ChessPosition::default()))
}

// ═══════════════════════════════════════════════════════════════════════════
// BOARD INDEXING
// ═══════════════════════════════════════════════════════════════════════════

/// Convert file (0-7) and rank (0-7) to square index (0-63).
fn square_index(file: i32, rank: i32) -> usize {
    (rank * 8 + file) as usize
}

/// Parse algebraic square notation (e.g., "e2" -> index).
fn parse_square(s: &str) -> Option<i32> {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() < 2 {
        return None;
    }
    
    let file = chars[0] as i32 - 'a' as i32;
    let rank = chars[1] as i32 - '1' as i32;
    
    if file < 0 || file > 7 || rank < 0 || rank > 7 {
        return None;
    }
    
    Some(rank * 8 + file)
}

/// Check if square is inside the board.
fn inside(file: i32, rank: i32) -> bool {
    file >= 0 && file < 8 && rank >= 0 && rank < 8
}

// ═══════════════════════════════════════════════════════════════════════════
// PIECE HELPERS
// ═══════════════════════════════════════════════════════════════════════════

fn is_white_piece(p: char) -> bool {
    matches!(p, 'P' | 'N' | 'B' | 'R' | 'Q' | 'K')
}

fn is_black_piece(p: char) -> bool {
    matches!(p, 'p' | 'n' | 'b' | 'r' | 'q' | 'k')
}

fn is_empty(p: char) -> bool {
    p == '.'
}

fn piece_value(p: char) -> i32 {
    match p {
        'P' => 100,  'N' => 320,  'B' => 330,
        'R' => 500,  'Q' => 900,  'K' => 10000,
        'p' => -100, 'n' => -320, 'b' => -330,
        'r' => -500, 'q' => -900, 'k' => -10000,
        _ => 0,
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// MOVE REPRESENTATION
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Clone, Copy, Default)]
struct Move {
    from: i32,
    to: i32,
    promo: char,  // 'Q'/'q' or '\0' for no promotion
}

impl Move {
    fn to_string(&self) -> String {
        if self.from < 0 || self.to < 0 {
            return String::new();
        }
        
        let ff = self.from % 8;
        let fr = self.from / 8;
        let tf = self.to % 8;
        let tr = self.to / 8;
        
        let mut s = String::new();
        s.push((b'a' + ff as u8) as char);
        s.push((b'1' + fr as u8) as char);
        s.push((b'a' + tf as u8) as char);
        s.push((b'1' + tr as u8) as char);
        
        if self.promo != '\0' {
            s.push(self.promo);
        }
        
        s
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// ATTACK DETECTION
// ═══════════════════════════════════════════════════════════════════════════

/// Find king square for given side.
fn king_square(pos: &ChessPosition, white: bool) -> i32 {
    let k = if white { 'K' } else { 'k' };
    for i in 0..64 {
        if pos.board[i] == k {
            return i as i32;
        }
    }
    -1
}

/// Check if a square is attacked by the given side.
fn is_square_attacked(pos: &ChessPosition, sq: i32, by_white: bool) -> bool {
    let file = sq % 8;
    let rank = sq / 8;
    
    // Pawn attacks
    let pawn_dir = if by_white { 1 } else { -1 };
    let pawn_rank = rank - pawn_dir;
    for df in [-1, 1] {
        let pf = file + df;
        if inside(pf, pawn_rank) {
            let p = pos.at(pawn_rank * 8 + pf);
            if (by_white && p == 'P') || (!by_white && p == 'p') {
                return true;
            }
        }
    }
    
    // Knight attacks
    const KNIGHT_MOVES: [(i32, i32); 8] = [
        (1, 2), (2, 1), (2, -1), (1, -2),
        (-1, -2), (-2, -1), (-2, 1), (-1, 2)
    ];
    for (df, dr) in KNIGHT_MOVES {
        let nf = file + df;
        let nr = rank + dr;
        if inside(nf, nr) {
            let p = pos.at(nr * 8 + nf);
            if (by_white && p == 'N') || (!by_white && p == 'n') {
                return true;
            }
        }
    }
    
    // Bishop/Queen (diagonals)
    const DIAG_DIRS: [(i32, i32); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
    for (df, dr) in DIAG_DIRS {
        let mut nf = file + df;
        let mut nr = rank + dr;
        while inside(nf, nr) {
            let p = pos.at(nr * 8 + nf);
            if !is_empty(p) {
                if (by_white && (p == 'B' || p == 'Q')) ||
                   (!by_white && (p == 'b' || p == 'q')) {
                    return true;
                }
                break;
            }
            nf += df;
            nr += dr;
        }
    }
    
    // Rook/Queen (orthogonals)
    const ORTHO_DIRS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    for (df, dr) in ORTHO_DIRS {
        let mut nf = file + df;
        let mut nr = rank + dr;
        while inside(nf, nr) {
            let p = pos.at(nr * 8 + nf);
            if !is_empty(p) {
                if (by_white && (p == 'R' || p == 'Q')) ||
                   (!by_white && (p == 'r' || p == 'q')) {
                    return true;
                }
                break;
            }
            nf += df;
            nr += dr;
        }
    }
    
    // King attacks
    for df in -1..=1 {
        for dr in -1..=1 {
            if df == 0 && dr == 0 { continue; }
            let nf = file + df;
            let nr = rank + dr;
            if inside(nf, nr) {
                let p = pos.at(nr * 8 + nf);
                if (by_white && p == 'K') || (!by_white && p == 'k') {
                    return true;
                }
            }
        }
    }
    
    false
}

/// Check if the given side is in check.
fn in_check(pos: &ChessPosition, white: bool) -> bool {
    let ksq = king_square(pos, white);
    if ksq < 0 { return false; }
    is_square_attacked(pos, ksq, !white)
}

// ═══════════════════════════════════════════════════════════════════════════
// MOVE GENERATION
// ═══════════════════════════════════════════════════════════════════════════

/// Add a move if it's legal (doesn't leave own king in check).
fn add_move_if_legal(pos: &ChessPosition, from: i32, to: i32, moves: &mut Vec<Move>) {
    if from < 0 || from >= 64 || to < 0 || to >= 64 {
        return;
    }
    
    let p = pos.at(from);
    let t = pos.at(to);
    
    if is_empty(p) { return; }
    
    let white = pos.white_to_move;
    if (white && !is_white_piece(p)) || (!white && !is_black_piece(p)) {
        return;
    }
    
    // Can't capture own piece
    if (white && is_white_piece(t)) || (!white && is_black_piece(t)) {
        return;
    }
    
    // Make move on copy and check legality
    let mut tmp = pos.clone();
    tmp.set(to, p);
    tmp.set(from, '.');
    
    // Handle promotion
    let mut promo = '\0';
    let to_rank = to / 8;
    if p == 'P' && to_rank == 7 {
        tmp.set(to, 'Q');
        promo = 'Q';
    } else if p == 'p' && to_rank == 0 {
        tmp.set(to, 'q');
        promo = 'q';
    }
    
    tmp.white_to_move = !pos.white_to_move;
    
    // Can't leave own king in check
    if in_check(&tmp, pos.white_to_move) {
        return;
    }
    
    moves.push(Move { from, to, promo });
}

/// Generate all legal moves for the current position.
fn generate_moves(pos: &ChessPosition) -> Vec<Move> {
    let mut moves = Vec::with_capacity(64);
    let white = pos.white_to_move;
    
    for sq in 0..64 {
        let p = pos.board[sq];
        if is_empty(p) { continue; }
        if (white && !is_white_piece(p)) || (!white && !is_black_piece(p)) {
            continue;
        }
        
        let f = (sq % 8) as i32;
        let r = (sq / 8) as i32;
        let sq = sq as i32;
        
        match p.to_ascii_uppercase() {
            'P' => {
                let dir = if p == 'P' { 1 } else { -1 };
                let start_rank = if p == 'P' { 1 } else { 6 };
                
                // Single push
                let nr = r + dir;
                if inside(f, nr) {
                    let to = nr * 8 + f;
                    if is_empty(pos.at(to)) {
                        add_move_if_legal(pos, sq, to, &mut moves);
                        
                        // Double push
                        if r == start_rank {
                            let nr2 = r + 2 * dir;
                            if inside(f, nr2) {
                                let to2 = nr2 * 8 + f;
                                if is_empty(pos.at(to2)) {
                                    add_move_if_legal(pos, sq, to2, &mut moves);
                                }
                            }
                        }
                    }
                }
                
                // Captures
                for df in [-1, 1] {
                    let nf = f + df;
                    let nr = r + dir;
                    if inside(nf, nr) {
                        let to = nr * 8 + nf;
                        let t = pos.at(to);
                        if (p == 'P' && is_black_piece(t)) || (p == 'p' && is_white_piece(t)) {
                            add_move_if_legal(pos, sq, to, &mut moves);
                        }
                    }
                }
            }
            'N' => {
                const KNIGHT_MOVES: [(i32, i32); 8] = [
                    (1, 2), (2, 1), (2, -1), (1, -2),
                    (-1, -2), (-2, -1), (-2, 1), (-1, 2)
                ];
                for (df, dr) in KNIGHT_MOVES {
                    let nf = f + df;
                    let nr = r + dr;
                    if inside(nf, nr) {
                        add_move_if_legal(pos, sq, nr * 8 + nf, &mut moves);
                    }
                }
            }
            'B' | 'R' | 'Q' => {
                let is_bishop_like = matches!(p.to_ascii_uppercase(), 'B' | 'Q');
                let is_rook_like = matches!(p.to_ascii_uppercase(), 'R' | 'Q');
                
                if is_bishop_like {
                    const DIAG_DIRS: [(i32, i32); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
                    for (df, dr) in DIAG_DIRS {
                        let mut nf = f + df;
                        let mut nr = r + dr;
                        while inside(nf, nr) {
                            let to = nr * 8 + nf;
                            let t = pos.at(to);
                            if (white && is_white_piece(t)) || (!white && is_black_piece(t)) {
                                break;
                            }
                            add_move_if_legal(pos, sq, to, &mut moves);
                            if !is_empty(t) { break; }
                            nf += df;
                            nr += dr;
                        }
                    }
                }
                
                if is_rook_like {
                    const ORTHO_DIRS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
                    for (df, dr) in ORTHO_DIRS {
                        let mut nf = f + df;
                        let mut nr = r + dr;
                        while inside(nf, nr) {
                            let to = nr * 8 + nf;
                            let t = pos.at(to);
                            if (white && is_white_piece(t)) || (!white && is_black_piece(t)) {
                                break;
                            }
                            add_move_if_legal(pos, sq, to, &mut moves);
                            if !is_empty(t) { break; }
                            nf += df;
                            nr += dr;
                        }
                    }
                }
            }
            'K' => {
                for df in -1..=1 {
                    for dr in -1..=1 {
                        if df == 0 && dr == 0 { continue; }
                        let nf = f + df;
                        let nr = r + dr;
                        if inside(nf, nr) {
                            add_move_if_legal(pos, sq, nr * 8 + nf, &mut moves);
                        }
                    }
                }
            }
            _ => {}
        }
    }
    
    moves
}

// ═══════════════════════════════════════════════════════════════════════════
// EVALUATION AND SEARCH
// ═══════════════════════════════════════════════════════════════════════════

/// Simple material evaluation.
fn evaluate(pos: &ChessPosition) -> i32 {
    let mut score = 0;
    for &p in &pos.board {
        score += piece_value(p);
    }
    // Slight tempo bonus
    if pos.white_to_move { score += 10; } else { score -= 10; }
    score
}

/// Make a move and return the new position.
fn make_move(pos: &ChessPosition, m: &Move) -> ChessPosition {
    let mut next = pos.clone();
    let p = next.at(m.from);
    next.set(m.from, '.');
    next.set(m.to, if m.promo != '\0' { m.promo } else { p });
    next.white_to_move = !pos.white_to_move;
    next
}

/// Alpha-beta search.
fn search(pos: &ChessPosition, depth: i32, mut alpha: i32, mut beta: i32) -> i32 {
    if depth == 0 {
        return evaluate(pos);
    }
    
    let moves = generate_moves(pos);
    
    if moves.is_empty() {
        // No moves: checkmate or stalemate
        if in_check(pos, pos.white_to_move) {
            return if pos.white_to_move { -1_000_000 } else { 1_000_000 };
        } else {
            return 0; // Stalemate
        }
    }
    
    if pos.white_to_move {
        let mut best = i32::MIN;
        for m in &moves {
            let next = make_move(pos, m);
            let score = search(&next, depth - 1, alpha, beta);
            best = best.max(score);
            alpha = alpha.max(score);
            if alpha >= beta { break; }
        }
        best
    } else {
        let mut best = i32::MAX;
        for m in &moves {
            let next = make_move(pos, m);
            let score = search(&next, depth - 1, alpha, beta);
            best = best.min(score);
            beta = beta.min(score);
            if alpha >= beta { break; }
        }
        best
    }
}

/// Find the best move for the current position.
fn find_best_move(pos: &ChessPosition, depth: i32) -> Option<Move> {
    let moves = generate_moves(pos);
    if moves.is_empty() { return None; }
    
    let mut best_move = moves[0];
    let mut alpha = i32::MIN;
    let beta = i32::MAX;
    
    if pos.white_to_move {
        let mut best_score = i32::MIN;
        for m in &moves {
            let next = make_move(pos, m);
            let score = search(&next, depth - 1, alpha, beta);
            if score > best_score {
                best_score = score;
                best_move = *m;
            }
            alpha = alpha.max(score);
        }
    } else {
        let mut best_score = i32::MAX;
        for m in &moves {
            let next = make_move(pos, m);
            let score = search(&next, depth - 1, alpha, beta);
            if score < best_score {
                best_score = score;
                best_move = *m;
            }
        }
    }
    
    Some(best_move)
}

// ═══════════════════════════════════════════════════════════════════════════
// DISPLAY
// ═══════════════════════════════════════════════════════════════════════════

fn print_board(pos: &ChessPosition) {
    println!("   +------------------------+");
    for r in (0..8).rev() {
        print!(" {} |", r + 1);
        for f in 0..8 {
            let c = pos.at(r * 8 + f);
            print!(" {}", if c == '.' { '.' } else { c });
        }
        println!(" |");
    }
    println!("   +------------------------+");
    println!("     a b c d e f g h");
    println!("Side to move: {}", if pos.white_to_move { "White" } else { "Black" });
}

// ═══════════════════════════════════════════════════════════════════════════
// REGISTRATION
// ═══════════════════════════════════════════════════════════════════════════

/// Register all chess operations.
pub fn register(interp: &mut Interpreter) {
    // Start a new game
    interp.register("chess_new", |_interp| {
        if let Ok(mut pos) = game_state().lock() {
            pos.init_start();
            println!("[simple_chess] New game started.");
            print_board(&pos);
        }
        Ok(())
    });

    // Show current board
    interp.register("chess_show", |_interp| {
        if let Ok(pos) = game_state().lock() {
            print_board(&pos);
        }
        Ok(())
    });

    // Make a move (e.g., "e2e4")
    // Stack: "e2e4" → engine_reply_move
    interp.register("chess_move", |interp| {
        let move_str = interp.stack_mut().pop()?.as_string()?;
        
        if move_str.len() < 4 {
            println!("[simple_chess] Invalid move string: {}", move_str);
            return Ok(());
        }
        
        let from = match parse_square(&move_str[0..2]) {
            Some(sq) => sq,
            None => {
                println!("[simple_chess] Invalid from square: {}", &move_str[0..2]);
                return Ok(());
            }
        };
        
        let to = match parse_square(&move_str[2..4]) {
            Some(sq) => sq,
            None => {
                println!("[simple_chess] Invalid to square: {}", &move_str[2..4]);
                return Ok(());
            }
        };
        
        let mut pos = game_state().lock().unwrap();
        let legal = generate_moves(&pos);
        
        // Find matching legal move
        let user_move = legal.iter().find(|m| m.from == from && m.to == to);
        
        let user_move = match user_move {
            Some(m) => *m,
            None => {
                println!("[simple_chess] Illegal move: {}", move_str);
                return Ok(());
            }
        };
        
        // Apply user move
        *pos = make_move(&pos, &user_move);
        println!("[simple_chess] You played: {}", user_move.to_string());
        print_board(&pos);
        
        // Engine reply
        if let Some(engine_move) = find_best_move(&pos, 3) {
            *pos = make_move(&pos, &engine_move);
            let eng_str = engine_move.to_string();
            println!("[simple_chess] Engine plays: {}", eng_str);
            print_board(&pos);
            interp.stack_mut().push(WofValue::string(eng_str));
        } else {
            // Game over
            if in_check(&pos, pos.white_to_move) {
                println!("[simple_chess] Checkmate. {} is checkmated.",
                    if pos.white_to_move { "White" } else { "Black" });
            } else {
                println!("[simple_chess] Stalemate.");
            }
            interp.stack_mut().push(WofValue::string(String::new()));
        }
        
        Ok(())
    });

    // Get legal moves (for debugging)
    interp.register("chess_moves", |_interp| {
        if let Ok(pos) = game_state().lock() {
            let moves = generate_moves(&pos);
            println!("[simple_chess] Legal moves ({}):", moves.len());
            for m in &moves {
                print!("{} ", m.to_string());
            }
            println!();
        }
        Ok(())
    });

    // Help
    interp.register("chess_help", |_interp| {
        println!("Simple Chess Operations:");
        println!();
        println!("  chess_new           - Start a new game");
        println!("  chess_show          - Display the board");
        println!("  \"e2e4\" chess_move   - Make a move, engine replies");
        println!("  chess_moves         - List all legal moves");
        println!();
        println!("Move format: from-square + to-square (e.g., \"e2e4\", \"g1f3\")");
        println!();
        println!("Notes:");
        println!("  - Simplified rules: no castling, no en passant");
        println!("  - Pawns auto-promote to queen");
        println!("  - Engine uses 3-ply alpha-beta search");
        Ok(())
    });
}
