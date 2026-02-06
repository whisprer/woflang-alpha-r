//! Chess Engine - Complete game logic in pure Rust.
//!
//! Implements:
//! - Board representation (bitboard-inspired but simple)
//! - Legal move generation
//! - Game state management
//! - Position hashing (Zobrist)

use std::fmt;

// ═══════════════════════════════════════════════════════════════════════════
// PIECE AND COLOR DEFINITIONS
// ═══════════════════════════════════════════════════════════════════════════

/// Chess piece types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PieceType {
    Pawn = 1,
    Knight = 2,
    Bishop = 3,
    Rook = 4,
    Queen = 5,
    King = 6,
}

impl PieceType {
    pub fn from_i8(val: i8) -> Option<Self> {
        match val.abs() {
            1 => Some(PieceType::Pawn),
            2 => Some(PieceType::Knight),
            3 => Some(PieceType::Bishop),
            4 => Some(PieceType::Rook),
            5 => Some(PieceType::Queen),
            6 => Some(PieceType::King),
            _ => None,
        }
    }

    pub fn symbol(&self, white: bool) -> char {
        let c = match self {
            PieceType::Pawn => 'P',
            PieceType::Knight => 'N',
            PieceType::Bishop => 'B',
            PieceType::Rook => 'R',
            PieceType::Queen => 'Q',
            PieceType::King => 'K',
        };
        if white { c } else { c.to_ascii_lowercase() }
    }

    /// Material value in centipawns.
    pub fn value(&self) -> i32 {
        match self {
            PieceType::Pawn => 100,
            PieceType::Knight => 320,
            PieceType::Bishop => 330,
            PieceType::Rook => 500,
            PieceType::Queen => 900,
            PieceType::King => 20000,
        }
    }
}

/// Color (side to move).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn opposite(&self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    pub fn sign(&self) -> i8 {
        match self {
            Color::White => 1,
            Color::Black => -1,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// SQUARE AND MOVE
// ═══════════════════════════════════════════════════════════════════════════

/// A chess square (0-63).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Square(pub u8);

impl Square {
    pub fn new(rank: u8, file: u8) -> Self {
        Square(rank * 8 + file)
    }

    pub fn from_algebraic(s: &str) -> Option<Self> {
        let chars: Vec<char> = s.chars().collect();
        if chars.len() != 2 {
            return None;
        }
        
        let file = (chars[0] as u8).checked_sub(b'a')?;
        let rank = (chars[1] as u8).checked_sub(b'1')?;
        
        if file < 8 && rank < 8 {
            Some(Square::new(rank, file))
        } else {
            None
        }
    }

    pub fn rank(&self) -> u8 {
        self.0 / 8
    }

    pub fn file(&self) -> u8 {
        self.0 % 8
    }

    pub fn to_algebraic(&self) -> String {
        let file = (b'a' + self.file()) as char;
        let rank = (b'1' + self.rank()) as char;
        format!("{}{}", file, rank)
    }

    pub fn is_valid(&self) -> bool {
        self.0 < 64
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_algebraic())
    }
}

/// A chess move.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub promotion: Option<PieceType>,
    pub is_castling: bool,
    pub is_en_passant: bool,
}

impl Move {
    pub fn new(from: Square, to: Square) -> Self {
        Move {
            from,
            to,
            promotion: None,
            is_castling: false,
            is_en_passant: false,
        }
    }

    pub fn with_promotion(from: Square, to: Square, piece: PieceType) -> Self {
        Move {
            from,
            to,
            promotion: Some(piece),
            is_castling: false,
            is_en_passant: false,
        }
    }

    pub fn castling(from: Square, to: Square) -> Self {
        Move {
            from,
            to,
            promotion: None,
            is_castling: true,
            is_en_passant: false,
        }
    }

    pub fn en_passant(from: Square, to: Square) -> Self {
        Move {
            from,
            to,
            promotion: None,
            is_castling: false,
            is_en_passant: true,
        }
    }

    pub fn to_uci(&self) -> String {
        let mut s = format!("{}{}", self.from, self.to);
        if let Some(promo) = self.promotion {
            s.push(promo.symbol(false));
        }
        s
    }

    pub fn from_uci(s: &str) -> Option<Self> {
        if s.len() < 4 {
            return None;
        }
        
        let from = Square::from_algebraic(&s[0..2])?;
        let to = Square::from_algebraic(&s[2..4])?;
        
        let promotion = if s.len() > 4 {
            match s.chars().nth(4)? {
                'q' | 'Q' => Some(PieceType::Queen),
                'r' | 'R' => Some(PieceType::Rook),
                'b' | 'B' => Some(PieceType::Bishop),
                'n' | 'N' => Some(PieceType::Knight),
                _ => None,
            }
        } else {
            None
        };
        
        Some(Move {
            from,
            to,
            promotion,
            is_castling: false,
            is_en_passant: false,
        })
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_uci())
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// CASTLING RIGHTS
// ═══════════════════════════════════════════════════════════════════════════

/// Castling rights.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CastlingRights {
    pub white_kingside: bool,
    pub white_queenside: bool,
    pub black_kingside: bool,
    pub black_queenside: bool,
}

impl CastlingRights {
    pub fn all() -> Self {
        CastlingRights {
            white_kingside: true,
            white_queenside: true,
            black_kingside: true,
            black_queenside: true,
        }
    }

    pub fn none() -> Self {
        CastlingRights {
            white_kingside: false,
            white_queenside: false,
            black_kingside: false,
            black_queenside: false,
        }
    }

    pub fn to_u8(&self) -> u8 {
        let mut val = 0u8;
        if self.white_kingside { val |= 1; }
        if self.white_queenside { val |= 2; }
        if self.black_kingside { val |= 4; }
        if self.black_queenside { val |= 8; }
        val
    }
}

impl Default for CastlingRights {
    fn default() -> Self {
        Self::all()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// CHESS BOARD
// ═══════════════════════════════════════════════════════════════════════════

/// Complete chess board state.
#[derive(Clone)]
pub struct Board {
    /// Piece placement: positive = white, negative = black, 0 = empty
    /// Index 0 = a1, Index 7 = h1, Index 56 = a8, Index 63 = h8
    pub squares: [i8; 64],
    /// Side to move
    pub side_to_move: Color,
    /// Castling rights
    pub castling: CastlingRights,
    /// En passant target square (if any)
    pub en_passant: Option<Square>,
    /// Halfmove clock (for 50-move rule)
    pub halfmove_clock: u32,
    /// Fullmove number
    pub fullmove_number: u32,
    /// Zobrist hash
    pub hash: u64,
}

impl Board {
    /// Create starting position.
    pub fn starting_position() -> Self {
        let mut board = Board {
            squares: [0; 64],
            side_to_move: Color::White,
            castling: CastlingRights::all(),
            en_passant: None,
            halfmove_clock: 0,
            fullmove_number: 1,
            hash: 0,
        };

        // White pieces (rank 1)
        board.squares[0] = 4;   // Ra1
        board.squares[1] = 2;   // Nb1
        board.squares[2] = 3;   // Bc1
        board.squares[3] = 5;   // Qd1
        board.squares[4] = 6;   // Ke1
        board.squares[5] = 3;   // Bf1
        board.squares[6] = 2;   // Ng1
        board.squares[7] = 4;   // Rh1

        // White pawns (rank 2)
        for i in 8..16 {
            board.squares[i] = 1;
        }

        // Black pieces (rank 8)
        board.squares[56] = -4;  // Ra8
        board.squares[57] = -2;  // Nb8
        board.squares[58] = -3;  // Bc8
        board.squares[59] = -5;  // Qd8
        board.squares[60] = -6;  // Ke8
        board.squares[61] = -3;  // Bf8
        board.squares[62] = -2;  // Ng8
        board.squares[63] = -4;  // Rh8

        // Black pawns (rank 7)
        for i in 48..56 {
            board.squares[i] = -1;
        }

        board.update_hash();
        board
    }

    /// Create empty board.
    pub fn empty() -> Self {
        Board {
            squares: [0; 64],
            side_to_move: Color::White,
            castling: CastlingRights::none(),
            en_passant: None,
            halfmove_clock: 0,
            fullmove_number: 1,
            hash: 0,
        }
    }

    /// Get piece at square.
    pub fn piece_at(&self, sq: Square) -> Option<(PieceType, Color)> {
        let val = self.squares[sq.0 as usize];
        if val == 0 {
            return None;
        }
        
        let color = if val > 0 { Color::White } else { Color::Black };
        let piece = PieceType::from_i8(val)?;
        Some((piece, color))
    }

    /// Set piece at square.
    pub fn set_piece(&mut self, sq: Square, piece: PieceType, color: Color) {
        self.squares[sq.0 as usize] = (piece as i8) * color.sign();
    }

    /// Remove piece from square.
    pub fn remove_piece(&mut self, sq: Square) {
        self.squares[sq.0 as usize] = 0;
    }

    /// Check if square is empty.
    pub fn is_empty(&self, sq: Square) -> bool {
        self.squares[sq.0 as usize] == 0
    }

    /// Check if square has piece of given color.
    pub fn has_color(&self, sq: Square, color: Color) -> bool {
        let val = self.squares[sq.0 as usize];
        match color {
            Color::White => val > 0,
            Color::Black => val < 0,
        }
    }

    /// Find king of given color.
    pub fn find_king(&self, color: Color) -> Option<Square> {
        let king_val = 6 * color.sign();
        for i in 0..64 {
            if self.squares[i] == king_val {
                return Some(Square(i as u8));
            }
        }
        None
    }

    /// Check if square is attacked by given color.
    pub fn is_attacked(&self, sq: Square, by_color: Color) -> bool {
        let sign = by_color.sign();
        let rank = sq.rank() as i8;
        let file = sq.file() as i8;

        // Check knight attacks
        let knight_offsets: [(i8, i8); 8] = [
            (-2, -1), (-2, 1), (-1, -2), (-1, 2),
            (1, -2), (1, 2), (2, -1), (2, 1),
        ];
        for (dr, df) in knight_offsets {
            let nr = rank + dr;
            let nf = file + df;
            if nr >= 0 && nr < 8 && nf >= 0 && nf < 8 {
                let nsq = Square::new(nr as u8, nf as u8);
                if self.squares[nsq.0 as usize] == 2 * sign {
                    return true;
                }
            }
        }

        // Check pawn attacks
        let pawn_dr = if by_color == Color::White { -1 } else { 1 };
        for df in [-1, 1] {
            let nr = rank + pawn_dr;
            let nf = file + df;
            if nr >= 0 && nr < 8 && nf >= 0 && nf < 8 {
                let nsq = Square::new(nr as u8, nf as u8);
                if self.squares[nsq.0 as usize] == 1 * sign {
                    return true;
                }
            }
        }

        // Check king attacks
        for dr in -1..=1 {
            for df in -1..=1 {
                if dr == 0 && df == 0 { continue; }
                let nr = rank + dr;
                let nf = file + df;
                if nr >= 0 && nr < 8 && nf >= 0 && nf < 8 {
                    let nsq = Square::new(nr as u8, nf as u8);
                    if self.squares[nsq.0 as usize] == 6 * sign {
                        return true;
                    }
                }
            }
        }

        // Check sliding pieces (rook/queen on ranks/files)
        let rook_dirs: [(i8, i8); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        for (dr, df) in rook_dirs {
            let mut nr = rank + dr;
            let mut nf = file + df;
            while nr >= 0 && nr < 8 && nf >= 0 && nf < 8 {
                let nsq = Square::new(nr as u8, nf as u8);
                let piece = self.squares[nsq.0 as usize];
                if piece != 0 {
                    if piece == 4 * sign || piece == 5 * sign {
                        return true;
                    }
                    break;
                }
                nr += dr;
                nf += df;
            }
        }

        // Check sliding pieces (bishop/queen on diagonals)
        let bishop_dirs: [(i8, i8); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
        for (dr, df) in bishop_dirs {
            let mut nr = rank + dr;
            let mut nf = file + df;
            while nr >= 0 && nr < 8 && nf >= 0 && nf < 8 {
                let nsq = Square::new(nr as u8, nf as u8);
                let piece = self.squares[nsq.0 as usize];
                if piece != 0 {
                    if piece == 3 * sign || piece == 5 * sign {
                        return true;
                    }
                    break;
                }
                nr += dr;
                nf += df;
            }
        }

        false
    }

    /// Check if current side is in check.
    pub fn is_in_check(&self) -> bool {
        if let Some(king_sq) = self.find_king(self.side_to_move) {
            self.is_attacked(king_sq, self.side_to_move.opposite())
        } else {
            false
        }
    }

    /// Update Zobrist hash (simplified).
    pub fn update_hash(&mut self) {
        // Simple hash based on board state
        let mut hash = 0u64;
        for (i, &piece) in self.squares.iter().enumerate() {
            if piece != 0 {
                hash ^= (piece as u64).wrapping_mul(0x517cc1b727220a95);
                hash ^= (i as u64).wrapping_mul(0x9e3779b97f4a7c15);
            }
        }
        if self.side_to_move == Color::Black {
            hash ^= 0xc3a5c85c97cb3127;
        }
        hash ^= (self.castling.to_u8() as u64).wrapping_mul(0x2545f4914f6cdd1d);
        if let Some(ep) = self.en_passant {
            hash ^= (ep.0 as u64).wrapping_mul(0x27d4eb2f165667c5);
        }
        self.hash = hash;
    }

    /// Material evaluation (positive = white advantage).
    pub fn material_balance(&self) -> i32 {
        let mut balance = 0;
        for &piece in &self.squares {
            if piece != 0 {
                if let Some(pt) = PieceType::from_i8(piece) {
                    let value = pt.value();
                    if piece > 0 {
                        balance += value;
                    } else {
                        balance -= value;
                    }
                }
            }
        }
        balance
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "  ┌───┬───┬───┬───┬───┬───┬───┬───┐")?;
        
        for rank in (0..8).rev() {
            write!(f, "{} │", rank + 1)?;
            for file in 0..8 {
                let sq = Square::new(rank, file);
                let c = if let Some((piece, color)) = self.piece_at(sq) {
                    piece.symbol(color == Color::White)
                } else {
                    ' '
                };
                write!(f, " {} │", c)?;
            }
            writeln!(f)?;
            
            if rank > 0 {
                writeln!(f, "  ├───┼───┼───┼───┼───┼───┼───┼───┤")?;
            }
        }
        
        writeln!(f, "  └───┴───┴───┴───┴───┴───┴───┴───┘")?;
        writeln!(f, "    a   b   c   d   e   f   g   h")?;
        writeln!(f)?;
        writeln!(f, "Side to move: {:?}", self.side_to_move)?;
        writeln!(f, "Castling: K={} Q={} k={} q={}",
            self.castling.white_kingside,
            self.castling.white_queenside,
            self.castling.black_kingside,
            self.castling.black_queenside,
        )?;
        if let Some(ep) = self.en_passant {
            writeln!(f, "En passant: {}", ep)?;
        }
        
        Ok(())
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::starting_position()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// MOVE GENERATION
// ═══════════════════════════════════════════════════════════════════════════

impl Board {
    /// Generate all pseudo-legal moves (may leave king in check).
    pub fn generate_pseudo_legal_moves(&self) -> Vec<Move> {
        let mut moves = Vec::with_capacity(64);
        let sign = self.side_to_move.sign();

        for from_idx in 0..64 {
            let piece = self.squares[from_idx];
            if piece == 0 || piece.signum() != sign {
                continue;
            }

            let from = Square(from_idx as u8);
            let piece_type = PieceType::from_i8(piece).unwrap();

            match piece_type {
                PieceType::Pawn => self.generate_pawn_moves(from, &mut moves),
                PieceType::Knight => self.generate_knight_moves(from, &mut moves),
                PieceType::Bishop => self.generate_bishop_moves(from, &mut moves),
                PieceType::Rook => self.generate_rook_moves(from, &mut moves),
                PieceType::Queen => {
                    self.generate_bishop_moves(from, &mut moves);
                    self.generate_rook_moves(from, &mut moves);
                }
                PieceType::King => self.generate_king_moves(from, &mut moves),
            }
        }

        moves
    }

    fn generate_pawn_moves(&self, from: Square, moves: &mut Vec<Move>) {
        let rank = from.rank();
        let file = from.file();
        let is_white = self.side_to_move == Color::White;
        let dir: i8 = if is_white { 1 } else { -1 };
        let start_rank = if is_white { 1 } else { 6 };
        let promo_rank = if is_white { 7 } else { 0 };

        // Single push
        let to_rank = (rank as i8 + dir) as u8;
        if to_rank < 8 {
            let to = Square::new(to_rank, file);
            if self.is_empty(to) {
                if to_rank == promo_rank {
                    for promo in [PieceType::Queen, PieceType::Rook, PieceType::Bishop, PieceType::Knight] {
                        moves.push(Move::with_promotion(from, to, promo));
                    }
                } else {
                    moves.push(Move::new(from, to));
                    
                    // Double push
                    if rank == start_rank {
                        let to2 = Square::new((to_rank as i8 + dir) as u8, file);
                        if self.is_empty(to2) {
                            moves.push(Move::new(from, to2));
                        }
                    }
                }
            }
        }

        // Captures
        for df in [-1i8, 1] {
            let nf = file as i8 + df;
            if nf >= 0 && nf < 8 && to_rank < 8 {
                let to = Square::new(to_rank, nf as u8);
                let can_capture = self.has_color(to, self.side_to_move.opposite());
                let is_ep = self.en_passant == Some(to);
                
                if can_capture || is_ep {
                    if to_rank == promo_rank {
                        for promo in [PieceType::Queen, PieceType::Rook, PieceType::Bishop, PieceType::Knight] {
                            moves.push(Move::with_promotion(from, to, promo));
                        }
                    } else if is_ep {
                        moves.push(Move::en_passant(from, to));
                    } else {
                        moves.push(Move::new(from, to));
                    }
                }
            }
        }
    }

    fn generate_knight_moves(&self, from: Square, moves: &mut Vec<Move>) {
        let rank = from.rank() as i8;
        let file = from.file() as i8;
        
        let offsets: [(i8, i8); 8] = [
            (-2, -1), (-2, 1), (-1, -2), (-1, 2),
            (1, -2), (1, 2), (2, -1), (2, 1),
        ];

        for (dr, df) in offsets {
            let nr = rank + dr;
            let nf = file + df;
            if nr >= 0 && nr < 8 && nf >= 0 && nf < 8 {
                let to = Square::new(nr as u8, nf as u8);
                if !self.has_color(to, self.side_to_move) {
                    moves.push(Move::new(from, to));
                }
            }
        }
    }

    fn generate_sliding_moves(&self, from: Square, directions: &[(i8, i8)], moves: &mut Vec<Move>) {
        let rank = from.rank() as i8;
        let file = from.file() as i8;

        for &(dr, df) in directions {
            let mut nr = rank + dr;
            let mut nf = file + df;
            
            while nr >= 0 && nr < 8 && nf >= 0 && nf < 8 {
                let to = Square::new(nr as u8, nf as u8);
                
                if self.has_color(to, self.side_to_move) {
                    break;
                }
                
                moves.push(Move::new(from, to));
                
                if self.has_color(to, self.side_to_move.opposite()) {
                    break;
                }
                
                nr += dr;
                nf += df;
            }
        }
    }

    fn generate_bishop_moves(&self, from: Square, moves: &mut Vec<Move>) {
        let directions: [(i8, i8); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
        self.generate_sliding_moves(from, &directions, moves);
    }

    fn generate_rook_moves(&self, from: Square, moves: &mut Vec<Move>) {
        let directions: [(i8, i8); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        self.generate_sliding_moves(from, &directions, moves);
    }

    fn generate_king_moves(&self, from: Square, moves: &mut Vec<Move>) {
        let rank = from.rank() as i8;
        let file = from.file() as i8;

        // Normal king moves
        for dr in -1..=1 {
            for df in -1..=1 {
                if dr == 0 && df == 0 { continue; }
                let nr = rank + dr;
                let nf = file + df;
                if nr >= 0 && nr < 8 && nf >= 0 && nf < 8 {
                    let to = Square::new(nr as u8, nf as u8);
                    if !self.has_color(to, self.side_to_move) {
                        moves.push(Move::new(from, to));
                    }
                }
            }
        }

        // Castling
        let (kingside, queenside) = match self.side_to_move {
            Color::White => (self.castling.white_kingside, self.castling.white_queenside),
            Color::Black => (self.castling.black_kingside, self.castling.black_queenside),
        };

        let back_rank = if self.side_to_move == Color::White { 0 } else { 7 };
        let enemy = self.side_to_move.opposite();

        // Kingside castling
        if kingside && from.rank() == back_rank && from.file() == 4 {
            let f1 = Square::new(back_rank, 5);
            let g1 = Square::new(back_rank, 6);
            if self.is_empty(f1) && self.is_empty(g1) {
                if !self.is_attacked(from, enemy) && 
                   !self.is_attacked(f1, enemy) && 
                   !self.is_attacked(g1, enemy) {
                    moves.push(Move::castling(from, g1));
                }
            }
        }

        // Queenside castling
        if queenside && from.rank() == back_rank && from.file() == 4 {
            let d1 = Square::new(back_rank, 3);
            let c1 = Square::new(back_rank, 2);
            let b1 = Square::new(back_rank, 1);
            if self.is_empty(d1) && self.is_empty(c1) && self.is_empty(b1) {
                if !self.is_attacked(from, enemy) && 
                   !self.is_attacked(d1, enemy) && 
                   !self.is_attacked(c1, enemy) {
                    moves.push(Move::castling(from, c1));
                }
            }
        }
    }

    /// Generate all legal moves.
    pub fn generate_legal_moves(&self) -> Vec<Move> {
        self.generate_pseudo_legal_moves()
            .into_iter()
            .filter(|m| {
                let mut board = self.clone();
                board.make_move_unchecked(*m);
                !board.is_in_check_for(self.side_to_move)
            })
            .collect()
    }

    /// Check if specific color's king is in check.
    fn is_in_check_for(&self, color: Color) -> bool {
        if let Some(king_sq) = self.find_king(color) {
            self.is_attacked(king_sq, color.opposite())
        } else {
            false
        }
    }

    /// Make a move (unchecked - doesn't verify legality).
    pub fn make_move_unchecked(&mut self, m: Move) {
        let piece = self.squares[m.from.0 as usize];
        let captured = self.squares[m.to.0 as usize];
        
        // Clear from square
        self.squares[m.from.0 as usize] = 0;
        
        // Handle en passant capture
        if m.is_en_passant {
            let ep_capture_sq = if self.side_to_move == Color::White {
                m.to.0 - 8
            } else {
                m.to.0 + 8
            };
            self.squares[ep_capture_sq as usize] = 0;
        }
        
        // Handle promotion
        let final_piece = if let Some(promo) = m.promotion {
            (promo as i8) * self.side_to_move.sign()
        } else {
            piece
        };
        
        // Place piece
        self.squares[m.to.0 as usize] = final_piece;
        
        // Handle castling rook movement
        if m.is_castling {
            let back_rank = m.from.rank();
            if m.to.file() == 6 {  // Kingside
                self.squares[Square::new(back_rank, 7).0 as usize] = 0;
                self.squares[Square::new(back_rank, 5).0 as usize] = 4 * self.side_to_move.sign();
            } else {  // Queenside
                self.squares[Square::new(back_rank, 0).0 as usize] = 0;
                self.squares[Square::new(back_rank, 3).0 as usize] = 4 * self.side_to_move.sign();
            }
        }
        
        // Update en passant square
        self.en_passant = None;
        if piece.abs() == 1 {  // Pawn
            let rank_diff = (m.to.rank() as i8 - m.from.rank() as i8).abs();
            if rank_diff == 2 {
                let ep_rank = (m.from.rank() + m.to.rank()) / 2;
                self.en_passant = Some(Square::new(ep_rank, m.from.file()));
            }
        }
        
        // Update castling rights
        // King moved
        if piece.abs() == 6 {
            if self.side_to_move == Color::White {
                self.castling.white_kingside = false;
                self.castling.white_queenside = false;
            } else {
                self.castling.black_kingside = false;
                self.castling.black_queenside = false;
            }
        }
        
        // Rook moved or captured
        if m.from == Square(0) || m.to == Square(0) { self.castling.white_queenside = false; }
        if m.from == Square(7) || m.to == Square(7) { self.castling.white_kingside = false; }
        if m.from == Square(56) || m.to == Square(56) { self.castling.black_queenside = false; }
        if m.from == Square(63) || m.to == Square(63) { self.castling.black_kingside = false; }
        
        // Update halfmove clock
        if piece.abs() == 1 || captured != 0 {
            self.halfmove_clock = 0;
        } else {
            self.halfmove_clock += 1;
        }
        
        // Update fullmove number
        if self.side_to_move == Color::Black {
            self.fullmove_number += 1;
        }
        
        // Switch side
        self.side_to_move = self.side_to_move.opposite();
        
        // Update hash
        self.update_hash();
    }

    /// Make a move (checks legality).
    pub fn make_move(&mut self, m: Move) -> bool {
        let legal_moves = self.generate_legal_moves();
        if legal_moves.iter().any(|lm| lm.from == m.from && lm.to == m.to) {
            self.make_move_unchecked(m);
            true
        } else {
            false
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// GAME RESULT
// ═══════════════════════════════════════════════════════════════════════════

/// Game result.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameResult {
    Ongoing,
    WhiteWins,
    BlackWins,
    Draw,
}

impl Board {
    /// Check game result.
    pub fn game_result(&self) -> GameResult {
        let legal_moves = self.generate_legal_moves();
        
        if legal_moves.is_empty() {
            if self.is_in_check() {
                // Checkmate
                match self.side_to_move {
                    Color::White => GameResult::BlackWins,
                    Color::Black => GameResult::WhiteWins,
                }
            } else {
                // Stalemate
                GameResult::Draw
            }
        } else if self.halfmove_clock >= 100 {
            // 50-move rule
            GameResult::Draw
        } else {
            GameResult::Ongoing
        }
    }

    /// Check if game is over.
    pub fn is_game_over(&self) -> bool {
        self.game_result() != GameResult::Ongoing
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_starting_position() {
        let board = Board::starting_position();
        assert_eq!(board.piece_at(Square(0)), Some((PieceType::Rook, Color::White)));
        assert_eq!(board.piece_at(Square(4)), Some((PieceType::King, Color::White)));
        assert_eq!(board.piece_at(Square(60)), Some((PieceType::King, Color::Black)));
    }

    #[test]
    fn test_legal_moves_starting() {
        let board = Board::starting_position();
        let moves = board.generate_legal_moves();
        assert_eq!(moves.len(), 20);  // 16 pawn moves + 4 knight moves
    }

    #[test]
    fn test_make_move() {
        let mut board = Board::starting_position();
        let e2e4 = Move::new(Square::from_algebraic("e2").unwrap(), Square::from_algebraic("e4").unwrap());
        assert!(board.make_move(e2e4));
        assert_eq!(board.side_to_move, Color::Black);
        assert_eq!(board.en_passant, Some(Square::from_algebraic("e3").unwrap()));
    }

    #[test]
    fn test_square_algebraic() {
        assert_eq!(Square::from_algebraic("e4").unwrap().to_algebraic(), "e4");
        assert_eq!(Square::from_algebraic("a1").unwrap().0, 0);
        assert_eq!(Square::from_algebraic("h8").unwrap().0, 63);
    }
}
