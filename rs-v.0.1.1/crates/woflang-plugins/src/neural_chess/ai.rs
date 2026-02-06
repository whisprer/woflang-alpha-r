//! Neural Chess AI Player with Self-Play Training.
//!
//! Combines the Brain Core with the Chess Engine to create
//! a complete AI opponent that can:
//! - Play chess games
//! - Learn from games
//! - Self-train by playing against itself

use super::tensor::Tensor;
use super::brain::BrainCore;
use super::chess::{Board, Move, Square, Color, GameResult, PieceType};
use super::cnn::board_to_planes;
use super::ganglion::PingMeasurer;

use std::collections::HashMap;

// ═══════════════════════════════════════════════════════════════════════════
// TRAINING EXAMPLE
// ═══════════════════════════════════════════════════════════════════════════

/// A single training example from a game.
#[derive(Clone)]
pub struct TrainingExample {
    /// Board planes (12 × 8 × 8)
    pub planes: Vec<Tensor>,
    /// Policy target (4096-dimensional)
    pub policy: Tensor,
    /// Value target (-1 to 1)
    pub value: f32,
}

// ═══════════════════════════════════════════════════════════════════════════
// GAME RECORD
// ═══════════════════════════════════════════════════════════════════════════

/// Record of a complete game.
pub struct GameRecord {
    /// Positions encountered
    pub positions: Vec<Vec<Tensor>>,
    /// Moves played (as policy targets)
    pub moves_played: Vec<Tensor>,
    /// Final result
    pub result: GameResult,
    /// Side to move at each position
    pub sides: Vec<Color>,
}

impl GameRecord {
    pub fn new() -> Self {
        GameRecord {
            positions: Vec::new(),
            moves_played: Vec::new(),
            result: GameResult::Ongoing,
            sides: Vec::new(),
        }
    }

    /// Convert to training examples.
    pub fn to_training_examples(&self) -> Vec<TrainingExample> {
        let final_value = match self.result {
            GameResult::WhiteWins => 1.0,
            GameResult::BlackWins => -1.0,
            GameResult::Draw => 0.0,
            GameResult::Ongoing => 0.0,
        };

        self.positions
            .iter()
            .zip(self.moves_played.iter())
            .zip(self.sides.iter())
            .map(|((planes, policy), &side)| {
                // Value from perspective of side to move
                let value = if side == Color::White {
                    final_value
                } else {
                    -final_value
                };
                
                TrainingExample {
                    planes: planes.clone(),
                    policy: policy.clone(),
                    value,
                }
            })
            .collect()
    }
}

impl Default for GameRecord {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// NEURAL CHESS AI
// ═══════════════════════════════════════════════════════════════════════════

/// Complete Neural Chess AI with learning capabilities.
pub struct NeuralChessAI {
    /// The neural brain
    pub brain: BrainCore,
    /// Temperature for move selection (higher = more random)
    pub temperature: f32,
    /// Games played
    pub games_played: u64,
    /// Training examples buffer
    training_buffer: Vec<TrainingExample>,
    /// Maximum buffer size
    pub buffer_size: usize,
    /// Batch size for training
    pub batch_size: usize,
    /// Learning rate
    pub learning_rate: f32,
    /// Exploration rate (epsilon for epsilon-greedy)
    pub epsilon: f32,
    /// Latency tracker
    pub ping: PingMeasurer,
    /// Win/loss/draw statistics
    pub stats: AIStats,
}

/// AI performance statistics.
#[derive(Clone, Default)]
pub struct AIStats {
    pub wins_as_white: u64,
    pub wins_as_black: u64,
    pub losses_as_white: u64,
    pub losses_as_black: u64,
    pub draws: u64,
    pub total_moves: u64,
    pub avg_move_time_ms: f32,
}

impl AIStats {
    pub fn win_rate(&self) -> f32 {
        let total = self.total_games();
        if total == 0 {
            return 0.5;
        }
        (self.wins_as_white + self.wins_as_black) as f32 / total as f32
    }

    pub fn total_games(&self) -> u64 {
        self.wins_as_white + self.wins_as_black + 
        self.losses_as_white + self.losses_as_black + 
        self.draws
    }
}

impl NeuralChessAI {
    /// Create a new AI.
    pub fn new() -> Self {
        NeuralChessAI {
            brain: BrainCore::new(),
            temperature: 1.0,
            games_played: 0,
            training_buffer: Vec::new(),
            buffer_size: 10000,
            batch_size: 32,
            learning_rate: 0.001,
            epsilon: 0.1,
            ping: PingMeasurer::new(100),
            stats: AIStats::default(),
        }
    }

    /// Convert board to neural network input.
    fn board_to_input(board: &Board) -> Vec<Tensor> {
        board_to_planes(&board.squares)
    }

    /// Convert move to policy index.
    fn move_to_policy_index(m: &Move) -> usize {
        (m.from.0 as usize) * 64 + (m.to.0 as usize)
    }

    /// Create policy target from a move.
    fn move_to_policy_target(m: &Move) -> Tensor {
        let mut data = vec![0.0; 4096];
        data[Self::move_to_policy_index(m)] = 1.0;
        Tensor::vector(data)
    }

    /// Select best legal move.
    pub fn select_move(&mut self, board: &Board) -> Option<Move> {
        let ping_start = self.ping.ping();
        
        let legal_moves = board.generate_legal_moves();
        if legal_moves.is_empty() {
            return None;
        }

        // Get neural network evaluation
        let planes = Self::board_to_input(board);
        let (value, policy) = self.brain.forward(&planes);

        // Record move for RNN/LSTM context
        if let Some(last_move) = legal_moves.first() {
            let encoded = BrainCore::encode_move(last_move.from.0 as usize, last_move.to.0 as usize);
            self.brain.record_move(encoded);
        }

        // Epsilon-greedy exploration
        let mut seed: u64 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        let r = ((seed >> 33) as f32) / (u32::MAX as f32);

        let selected_move = if r < self.epsilon {
            // Random move
            let idx = (r * legal_moves.len() as f32) as usize;
            legal_moves.get(idx.min(legal_moves.len() - 1)).cloned()
        } else {
            // Best move according to network
            let mut best_move = None;
            let mut best_score = f32::NEG_INFINITY;

            for m in &legal_moves {
                let idx = Self::move_to_policy_index(m);
                let score = policy.data[idx];
                
                if score > best_score {
                    best_score = score;
                    best_move = Some(m.clone());
                }
            }

            best_move
        };

        let ping_time = self.ping.pong(ping_start);
        self.stats.total_moves += 1;
        self.stats.avg_move_time_ms = 
            (self.stats.avg_move_time_ms * (self.stats.total_moves - 1) as f32 + 
             ping_time as f32 / 1000.0) / self.stats.total_moves as f32;

        selected_move
    }

    /// Evaluate a position.
    pub fn evaluate(&mut self, board: &Board) -> f32 {
        let planes = Self::board_to_input(board);
        let (value, _) = self.brain.forward(&planes);
        value
    }

    /// Add training example to buffer.
    pub fn add_training_example(&mut self, example: TrainingExample) {
        self.training_buffer.push(example);
        
        // Remove old examples if buffer is full
        while self.training_buffer.len() > self.buffer_size {
            self.training_buffer.remove(0);
        }
    }

    /// Add examples from a game.
    pub fn add_game_examples(&mut self, record: &GameRecord) {
        for example in record.to_training_examples() {
            self.add_training_example(example);
        }
    }

    /// Train on a batch of examples.
    pub fn train_batch(&mut self) {
        if self.training_buffer.len() < self.batch_size {
            return;
        }

        // Sample random batch
        let mut seed: u64 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;

        for _ in 0..self.batch_size {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            let idx = (seed as usize) % self.training_buffer.len();
            
            let example = &self.training_buffer[idx];
            self.brain.train_step(&example.planes, example.value, &example.policy);
        }
    }

    /// Play a complete game against another AI (or self).
    pub fn play_game(&mut self, mut opponent: Option<&mut NeuralChessAI>) -> GameRecord {
        let mut board = Board::starting_position();
        let mut record = GameRecord::new();
        
        self.brain.reset_history();
        if let Some(opp) = opponent.as_ref() {
            // opponent.brain.reset_history();  // Can't mutate here
        }

        let max_moves = 500;  // Prevent infinite games
        let mut move_count = 0;

        while !board.is_game_over() && move_count < max_moves {
            let planes = Self::board_to_input(&board);
            record.positions.push(planes);
            record.sides.push(board.side_to_move);

            let selected_move = if board.side_to_move == Color::White {
                self.select_move(&board)
            } else if let Some(ref mut opp) = opponent {
                opp.select_move(&board)
            } else {
                self.select_move(&board)
            };

            if let Some(m) = selected_move {
                record.moves_played.push(Self::move_to_policy_target(&m));
                board.make_move_unchecked(m);
            } else {
                break;
            }

            move_count += 1;
        }

        record.result = board.game_result();
        
        // Update statistics
        self.games_played += 1;
        match record.result {
            GameResult::WhiteWins => self.stats.wins_as_white += 1,
            GameResult::BlackWins => self.stats.losses_as_white += 1,
            GameResult::Draw => self.stats.draws += 1,
            GameResult::Ongoing => {},
        }

        record
    }

    /// Self-play training: play games against itself and learn.
    pub fn self_play_train(&mut self, num_games: usize) {
        println!("🧠 Starting self-play training: {} games", num_games);
        
        for game_num in 0..num_games {
            // Play a game against self
            let record = self.self_play_game();
            
            // Add examples to buffer
            self.add_game_examples(&record);
            
            // Train on batch
            self.train_batch();
            
            // Progress report
            if (game_num + 1) % 10 == 0 {
                println!(
                    "  Game {}/{}: Win rate: {:.1}%, Avg move time: {:.2}ms, Buffer: {}",
                    game_num + 1,
                    num_games,
                    self.stats.win_rate() * 100.0,
                    self.stats.avg_move_time_ms,
                    self.training_buffer.len(),
                );
            }
        }

        println!("✅ Self-play training complete!");
        println!("{}", self.status_report());
    }

    /// Play a single game against itself.
    fn self_play_game(&mut self) -> GameRecord {
        let mut board = Board::starting_position();
        let mut record = GameRecord::new();
        
        self.brain.reset_history();

        let max_moves = 500;
        let mut move_count = 0;

        while !board.is_game_over() && move_count < max_moves {
            let planes = Self::board_to_input(&board);
            record.positions.push(planes);
            record.sides.push(board.side_to_move);

            if let Some(m) = self.select_move(&board) {
                record.moves_played.push(Self::move_to_policy_target(&m));
                board.make_move_unchecked(m);
            } else {
                break;
            }

            move_count += 1;
        }

        record.result = board.game_result();
        
        // Update stats based on perspective
        self.games_played += 1;
        match record.result {
            GameResult::WhiteWins | GameResult::BlackWins => {
                // In self-play, count as one win and one loss
                self.stats.wins_as_white += 1;
                self.stats.losses_as_black += 1;
            },
            GameResult::Draw => self.stats.draws += 1,
            GameResult::Ongoing => {},
        }

        record
    }

    /// Get status report.
    pub fn status_report(&self) -> String {
        format!(
            "╔═══════════════════════════════════════════╗\n\
             ║        NEURAL CHESS AI STATUS             ║\n\
             ╠═══════════════════════════════════════════╣\n\
             ║ Games Played:     {:>20}   ║\n\
             ║ Win Rate:         {:>19.1}%  ║\n\
             ║ Avg Move Time:    {:>17.2}ms  ║\n\
             ║ Training Buffer:  {:>20}   ║\n\
             ║ Temperature:      {:>20.2}   ║\n\
             ║ Epsilon:          {:>20.2}   ║\n\
             ╠═══════════════════════════════════════════╣\n\
             ║ Wins as White:    {:>20}   ║\n\
             ║ Wins as Black:    {:>20}   ║\n\
             ║ Losses as White:  {:>20}   ║\n\
             ║ Losses as Black:  {:>20}   ║\n\
             ║ Draws:            {:>20}   ║\n\
             ╚═══════════════════════════════════════════╝",
            self.games_played,
            self.stats.win_rate() * 100.0,
            self.stats.avg_move_time_ms,
            self.training_buffer.len(),
            self.temperature,
            self.epsilon,
            self.stats.wins_as_white,
            self.stats.wins_as_black,
            self.stats.losses_as_white,
            self.stats.losses_as_black,
            self.stats.draws,
        )
    }

    /// Decrease temperature over time (for training annealing).
    pub fn anneal_temperature(&mut self, factor: f32) {
        self.temperature *= factor;
        self.temperature = self.temperature.max(0.1);
    }

    /// Decrease epsilon over time.
    pub fn anneal_epsilon(&mut self, factor: f32) {
        self.epsilon *= factor;
        self.epsilon = self.epsilon.max(0.01);
    }
}

impl Default for NeuralChessAI {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// INTERACTIVE GAME SESSION
// ═══════════════════════════════════════════════════════════════════════════

/// Interactive game session for playing against the AI.
pub struct GameSession {
    /// The AI opponent
    pub ai: NeuralChessAI,
    /// Current board state
    pub board: Board,
    /// Human plays as (true = white)
    pub human_is_white: bool,
    /// Move history
    pub move_history: Vec<Move>,
}

impl GameSession {
    /// Create new game session.
    pub fn new(human_is_white: bool) -> Self {
        GameSession {
            ai: NeuralChessAI::new(),
            board: Board::starting_position(),
            human_is_white,
            move_history: Vec::new(),
        }
    }

    /// Create with existing AI.
    pub fn with_ai(ai: NeuralChessAI, human_is_white: bool) -> Self {
        GameSession {
            ai,
            board: Board::starting_position(),
            human_is_white,
            move_history: Vec::new(),
        }
    }

    /// Check if it's the human's turn.
    pub fn is_human_turn(&self) -> bool {
        (self.board.side_to_move == Color::White) == self.human_is_white
    }

    /// Make a human move (from algebraic notation).
    pub fn human_move(&mut self, move_str: &str) -> Result<(), String> {
        if !self.is_human_turn() {
            return Err("Not your turn!".to_string());
        }

        let m = Move::from_uci(move_str)
            .ok_or_else(|| "Invalid move format. Use UCI notation (e.g., e2e4)".to_string())?;

        let legal_moves = self.board.generate_legal_moves();
        let legal_move = legal_moves.iter()
            .find(|lm| lm.from == m.from && lm.to == m.to)
            .cloned()
            .ok_or_else(|| "Illegal move!".to_string())?;

        self.board.make_move_unchecked(legal_move);
        self.move_history.push(legal_move);

        Ok(())
    }

    /// Get AI's move.
    pub fn ai_move(&mut self) -> Option<Move> {
        if self.is_human_turn() {
            return None;
        }

        let m = self.ai.select_move(&self.board)?;
        self.board.make_move_unchecked(m);
        self.move_history.push(m);
        
        Some(m)
    }

    /// Get current position display.
    pub fn display(&self) -> String {
        format!("{}", self.board)
    }

    /// Get legal moves list.
    pub fn legal_moves(&self) -> Vec<String> {
        self.board.generate_legal_moves()
            .iter()
            .map(|m| m.to_uci())
            .collect()
    }

    /// Get game status.
    pub fn status(&self) -> String {
        match self.board.game_result() {
            GameResult::Ongoing => {
                let turn = if self.is_human_turn() { "Your" } else { "AI's" };
                format!("{} turn to move", turn)
            },
            GameResult::WhiteWins => {
                if self.human_is_white { "You win! 🎉".to_string() }
                else { "AI wins!".to_string() }
            },
            GameResult::BlackWins => {
                if !self.human_is_white { "You win! 🎉".to_string() }
                else { "AI wins!".to_string() }
            },
            GameResult::Draw => "Game drawn!".to_string(),
        }
    }

    /// Check if game is over.
    pub fn is_game_over(&self) -> bool {
        self.board.is_game_over()
    }

    /// Get AI evaluation of current position.
    pub fn get_evaluation(&mut self) -> f32 {
        self.ai.evaluate(&self.board)
    }

    /// Reset for new game.
    pub fn new_game(&mut self) {
        self.board = Board::starting_position();
        self.move_history.clear();
        self.ai.brain.reset_history();
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TRAINING CONFIGURATION
// ═══════════════════════════════════════════════════════════════════════════

/// Configuration for training runs.
pub struct TrainingConfig {
    /// Number of self-play games per iteration
    pub games_per_iteration: usize,
    /// Number of training iterations
    pub iterations: usize,
    /// Learning rate
    pub learning_rate: f32,
    /// Initial temperature
    pub initial_temperature: f32,
    /// Temperature decay per iteration
    pub temperature_decay: f32,
    /// Initial epsilon
    pub initial_epsilon: f32,
    /// Epsilon decay per iteration
    pub epsilon_decay: f32,
    /// Buffer size
    pub buffer_size: usize,
    /// Batch size
    pub batch_size: usize,
}

impl Default for TrainingConfig {
    fn default() -> Self {
        TrainingConfig {
            games_per_iteration: 100,
            iterations: 10,
            learning_rate: 0.001,
            initial_temperature: 1.0,
            temperature_decay: 0.95,
            initial_epsilon: 0.3,
            epsilon_decay: 0.9,
            buffer_size: 10000,
            batch_size: 32,
        }
    }
}

impl NeuralChessAI {
    /// Run full training with configuration.
    pub fn train_with_config(&mut self, config: &TrainingConfig) {
        self.temperature = config.initial_temperature;
        self.epsilon = config.initial_epsilon;
        self.buffer_size = config.buffer_size;
        self.batch_size = config.batch_size;
        self.learning_rate = config.learning_rate;

        println!("🚀 Starting Neural Chess Training");
        println!("   Iterations: {}", config.iterations);
        println!("   Games/iteration: {}", config.games_per_iteration);

        for iteration in 0..config.iterations {
            println!("\n📊 Iteration {}/{}", iteration + 1, config.iterations);
            
            self.self_play_train(config.games_per_iteration);
            
            // Anneal hyperparameters
            self.anneal_temperature(config.temperature_decay);
            self.anneal_epsilon(config.epsilon_decay);
            
            println!(
                "   Temperature: {:.3}, Epsilon: {:.3}",
                self.temperature,
                self.epsilon,
            );
        }

        println!("\n🏆 Training Complete!");
        println!("{}", self.status_report());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_creation() {
        let ai = NeuralChessAI::new();
        assert_eq!(ai.games_played, 0);
    }

    #[test]
    fn test_select_move() {
        let mut ai = NeuralChessAI::new();
        let board = Board::starting_position();
        
        let m = ai.select_move(&board);
        assert!(m.is_some());
    }

    #[test]
    fn test_self_play_game() {
        let mut ai = NeuralChessAI::new();
        ai.epsilon = 1.0;  // Full random for fast test
        
        let record = ai.self_play_game();
        assert!(!record.positions.is_empty());
    }

    #[test]
    fn test_game_session() {
        let mut session = GameSession::new(true);
        assert!(session.is_human_turn());
        
        let result = session.human_move("e2e4");
        assert!(result.is_ok());
        assert!(!session.is_human_turn());
    }
}
