//! Neural Chess Module for Woflang.
//!
//! A complete neural network-based chess AI implemented entirely in Rust.
//! Features:
//! - 3-Way GAN Brain Core (CNN + RNN + LSTM)
//! - Universal Ganglion Clock for synchronization
//! - Self-play training capabilities
//! - Interactive game sessions
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────┐
//! │                    NEURAL CHESS BRAIN                           │
//! ├─────────────────────────────────────────────────────────────────┤
//! │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐             │
//! │  │     RNN     │  │     CNN     │  │    LSTM     │             │
//! │  │  (Sequence) │  │  (Pattern)  │  │  (Memory)   │             │
//! │  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘             │
//! │         │                │                │                     │
//! │         └────────────────┼────────────────┘                     │
//! │                          ▼                                      │
//! │              ┌───────────────────────┐                          │
//! │              │   GANGLION CLOCK      │                          │
//! │              │   (Synchronization)   │                          │
//! │              └───────────┬───────────┘                          │
//! │                          ▼                                      │
//! │              ┌───────────────────────┐                          │
//! │              │   GAN OUTPUT HEAD     │                          │
//! │              │   (Generator/Eval)    │                          │
//! │              └───────────────────────┘                          │
//! ├─────────────────────────────────────────────────────────────────┤
//! │                    CHESS ENGINE                                 │
//! │  • Full move generation  • Legal validation  • Self-play       │
//! └─────────────────────────────────────────────────────────────────┘
//! ```

// Sub-modules
pub mod tensor;
pub mod activation;
pub mod layers;
pub mod rnn;
pub mod lstm;
pub mod cnn;
pub mod ganglion;
pub mod brain;
pub mod chess;
pub mod ai;

// Re-exports for convenience
pub use tensor::Tensor;
pub use activation::Activation;
pub use brain::BrainCore;
pub use chess::{Board, Move, Square, Color, GameResult, PieceType};
pub use ai::{NeuralChessAI, GameSession, TrainingConfig};
pub use ganglion::{Ganglion, NeuralClockCoordinator};

use std::error::Error;
use std::sync::Mutex;
use std::sync::OnceLock;
use woflang_runtime::{Interpreter, WofValue};

// ═══════════════════════════════════════════════════════════════════════════
// GLOBAL AI INSTANCE
// ═══════════════════════════════════════════════════════════════════════════

/// Global AI instance for woflang operations.
static GLOBAL_AI: OnceLock<Mutex<NeuralChessAI>> = OnceLock::new();

/// Global game session for interactive play.
static GLOBAL_SESSION: OnceLock<Mutex<Option<GameSession>>> = OnceLock::new();

fn get_ai() -> &'static Mutex<NeuralChessAI> {
    GLOBAL_AI.get_or_init(|| Mutex::new(NeuralChessAI::new()))
}

fn get_session() -> &'static Mutex<Option<GameSession>> {
    GLOBAL_SESSION.get_or_init(|| Mutex::new(None))
}

// ═══════════════════════════════════════════════════════════════════════════
// WOFLANG INTEGRATION
// ═══════════════════════════════════════════════════════════════════════════

/// Register all neural chess operations with the interpreter.
pub fn register(interp: &mut Interpreter) {
    // ─────────────────────────────────────────────────────────────────────
    // AI MANAGEMENT
    // ─────────────────────────────────────────────────────────────────────

    interp.register("chess_ai_new", |interp| {
        let mut ai_lock = get_ai().lock().unwrap();
        *ai_lock = NeuralChessAI::new();
        println!("♟️  Neural Chess AI initialized!");
        println!("{}", ai_lock.status_report());
        Ok(())
    });

    interp.register("chess_ai_status", |interp| {
        let ai = get_ai().lock().unwrap();
        println!("{}", ai.status_report());
        Ok(())
    });

    interp.register("chess_ai_train", |interp| {
        let games = interp.pop()
            .and_then(|v| v.as_int())
            .unwrap_or(10) as usize;
        
        let mut ai = get_ai().lock().unwrap();
        ai.self_play_train(games);
        Ok(())
    });

    interp.register("chess_ai_train_full", |interp| {
        let iterations = interp.pop()
            .and_then(|v| v.as_int())
            .unwrap_or(5) as usize;
        let games = interp.pop()
            .and_then(|v| v.as_int())
            .unwrap_or(50) as usize;
        
        let config = TrainingConfig {
            iterations,
            games_per_iteration: games,
            ..TrainingConfig::default()
        };
        
        let mut ai = get_ai().lock().unwrap();
        ai.train_with_config(&config);
        Ok(())
    });

    // ─────────────────────────────────────────────────────────────────────
    // GAME SESSION
    // ─────────────────────────────────────────────────────────────────────

    interp.register("chess_new_game", |interp| {
        let human_white = interp.pop()
            .and_then(|v| v.as_int())
            .map(|v| v != 0)
            .unwrap_or(true);
        
        let ai = get_ai().lock().unwrap();
        let mut session_lock = get_session().lock().unwrap();
        
        // Create new session with a fresh AI
        let mut new_ai = NeuralChessAI::new();
        new_ai.temperature = ai.temperature;
        new_ai.epsilon = ai.epsilon;
        
        *session_lock = Some(GameSession::with_ai(new_ai, human_white));
        
        let session = session_lock.as_ref().unwrap();
        println!("♟️  New game started!");
        println!("   You play as: {}", if human_white { "White" } else { "Black" });
        println!("\n{}", session.display());
        println!("{}", session.status());
        
        Ok(())
    });

    interp.register("chess_show", |interp| {
        let session_lock = get_session().lock().unwrap();
        if let Some(ref session) = *session_lock {
            println!("{}", session.display());
            println!("{}", session.status());
            
            if !session.is_game_over() && session.is_human_turn() {
                let moves = session.legal_moves();
                if moves.len() <= 20 {
                    println!("Legal moves: {}", moves.join(", "));
                } else {
                    println!("Legal moves: {} moves available", moves.len());
                }
            }
        } else {
            println!("No game in progress. Use 'chess_new_game' to start.");
        }
        Ok(())
    });

    interp.register("chess_move", |interp| {
        let move_str = interp.pop()
            .and_then(|v| v.as_string().map(|s| s.to_string()))
            .ok_or("Expected move string (e.g., 'e2e4')")?;
        
        let mut session_lock = get_session().lock().unwrap();
        if let Some(ref mut session) = *session_lock {
            match session.human_move(&move_str) {
                Ok(()) => {
                    println!("Your move: {}", move_str);
                    
                    // If game not over, let AI respond
                    if !session.is_game_over() && !session.is_human_turn() {
                        if let Some(ai_move) = session.ai_move() {
                            println!("AI plays: {}", ai_move.to_uci());
                        }
                    }
                    
                    println!("\n{}", session.display());
                    println!("{}", session.status());
                },
                Err(e) => println!("❌ {}", e),
            }
        } else {
            println!("No game in progress. Use 'chess_new_game' to start.");
        }
        Ok(())
    });

    interp.register("chess_ai_play", |interp| {
        let mut session_lock = get_session().lock().unwrap();
        if let Some(ref mut session) = *session_lock {
            if session.is_game_over() {
                println!("Game is already over!");
            } else if !session.is_human_turn() {
                if let Some(ai_move) = session.ai_move() {
                    println!("AI plays: {}", ai_move.to_uci());
                    println!("\n{}", session.display());
                    println!("{}", session.status());
                }
            } else {
                println!("It's your turn!");
            }
        } else {
            println!("No game in progress.");
        }
        Ok(())
    });

    interp.register("chess_eval", |interp| {
        let mut session_lock = get_session().lock().unwrap();
        if let Some(ref mut session) = *session_lock {
            let eval = session.get_evaluation();
            let perspective = if session.human_is_white { 
                if eval > 0.0 { "your favor" } else { "AI's favor" }
            } else {
                if eval > 0.0 { "AI's favor" } else { "your favor" }
            };
            
            println!("Position evaluation: {:.3} (in {})", eval, perspective);
            interp.push(WofValue::Float(eval as f64));
        } else {
            println!("No game in progress.");
        }
        Ok(())
    });

    interp.register("chess_legal_moves", |interp| {
        let session_lock = get_session().lock().unwrap();
        if let Some(ref session) = *session_lock {
            let moves = session.legal_moves();
            println!("Legal moves ({}):", moves.len());
            for (i, m) in moves.iter().enumerate() {
                print!("{}", m);
                if i < moves.len() - 1 {
                    print!(", ");
                }
                if (i + 1) % 10 == 0 {
                    println!();
                }
            }
            println!();
            
            interp.push(WofValue::Integer(moves.len() as i64));
        } else {
            println!("No game in progress.");
        }
        Ok(())
    });

    interp.register("chess_undo", |interp| {
        println!("⚠️  Undo not yet implemented (would require game state history)");
        Ok(())
    });

    // ─────────────────────────────────────────────────────────────────────
    // STANDALONE BOARD OPERATIONS
    // ─────────────────────────────────────────────────────────────────────

    interp.register("chess_board_new", |interp| {
        let board = Board::starting_position();
        println!("{}", board);
        Ok(())
    });

    interp.register("chess_perft", |interp| {
        let depth = interp.pop()
            .and_then(|v| v.as_int())
            .unwrap_or(3) as u32;
        
        fn perft(board: &Board, depth: u32) -> u64 {
            if depth == 0 {
                return 1;
            }
            
            let moves = board.generate_legal_moves();
            if depth == 1 {
                return moves.len() as u64;
            }
            
            let mut nodes = 0;
            for m in moves {
                let mut new_board = board.clone();
                new_board.make_move_unchecked(m);
                nodes += perft(&new_board, depth - 1);
            }
            nodes
        }
        
        let board = Board::starting_position();
        let start = std::time::Instant::now();
        let nodes = perft(&board, depth);
        let elapsed = start.elapsed();
        
        println!("Perft({}): {} nodes in {:?}", depth, nodes, elapsed);
        println!("Speed: {:.0} nps", nodes as f64 / elapsed.as_secs_f64());
        
        interp.push(WofValue::Integer(nodes as i64));
        Ok(())
    });

    // ─────────────────────────────────────────────────────────────────────
    // BRAIN DIAGNOSTICS
    // ─────────────────────────────────────────────────────────────────────

    interp.register("chess_brain_info", |interp| {
        let ai = get_ai().lock().unwrap();
        println!("{}", ai.brain.diagnostics());
        Ok(())
    });

    interp.register("chess_ping", |interp| {
        let ai = get_ai().lock().unwrap();
        let avg_ping = ai.ping.average_ping_ms();
        let jitter = ai.ping.jitter() / 1000.0;
        
        println!("🏓 Neural Response Times:");
        println!("   Average: {:.2}ms", avg_ping);
        println!("   Jitter:  {:.2}ms", jitter);
        
        interp.push(WofValue::Float(avg_ping as f64));
        Ok(())
    });

    // ─────────────────────────────────────────────────────────────────────
    // HELP
    // ─────────────────────────────────────────────────────────────────────

    interp.register("chess_help", |interp| {
        println!("╔═══════════════════════════════════════════════════════════════╗");
        println!("║              NEURAL CHESS - WOFLANG OPERATIONS                ║");
        println!("╠═══════════════════════════════════════════════════════════════╣");
        println!("║ AI Management:                                                ║");
        println!("║   chess_ai_new         - Initialize new AI                    ║");
        println!("║   chess_ai_status      - Show AI statistics                   ║");
        println!("║   n chess_ai_train     - Train AI with n self-play games      ║");
        println!("║   g i chess_ai_train_full - Full training (g games, i iter)   ║");
        println!("║                                                               ║");
        println!("║ Game Session:                                                 ║");
        println!("║   1 chess_new_game     - Start new game (1=white, 0=black)    ║");
        println!("║   chess_show           - Display current board                ║");
        println!("║   \"e2e4\" chess_move    - Make a move (UCI notation)          ║");
        println!("║   chess_ai_play        - Force AI to play                     ║");
        println!("║   chess_eval           - Get position evaluation              ║");
        println!("║   chess_legal_moves    - List all legal moves                 ║");
        println!("║                                                               ║");
        println!("║ Utilities:                                                    ║");
        println!("║   chess_board_new      - Display starting position            ║");
        println!("║   n chess_perft        - Performance test (depth n)           ║");
        println!("║   chess_brain_info     - Neural network diagnostics           ║");
        println!("║   chess_ping           - Response time statistics             ║");
        println!("║   chess_help           - This help message                    ║");
        println!("╚═══════════════════════════════════════════════════════════════╝");
        Ok(())
    });

    // ─────────────────────────────────────────────────────────────────────
    // UNICODE ALIASES
    // ─────────────────────────────────────────────────────────────────────

    interp.register("♟", |interp| {
        println!("{}", get_ai().lock().unwrap().status_report());
        Ok(())
    });

    interp.register("♔", |interp| {
        // Show board
        let session = get_session().lock().unwrap();
        if let Some(ref s) = *session {
            println!("{}", s.display());
        } else {
            println!("{}", Board::starting_position());
        }
        Ok(())
    });

    interp.register("♕", |interp| {
        // AI status
        let ai = get_ai().lock().unwrap();
        let ping = ai.stats.avg_move_time_ms;
        println!("♕ AI: {} games, {:.1}% win rate, {:.1}ms avg", 
            ai.games_played, ai.stats.win_rate() * 100.0, ping);
        Ok(())
    });
}

/// Quick test of the neural chess system.
pub fn quick_test() {
    println!("🧠 Neural Chess Quick Test\n");
    
    // Create AI
    let mut ai = NeuralChessAI::new();
    println!("✅ AI created");
    println!("{}", ai.brain.diagnostics());
    
    // Test move selection
    let board = Board::starting_position();
    println!("\n📋 Starting position:");
    println!("{}", board);
    
    if let Some(m) = ai.select_move(&board) {
        println!("🤖 AI suggests: {}", m.to_uci());
    }
    
    // Quick self-play test
    println!("\n🎮 Quick self-play (5 games)...");
    ai.epsilon = 0.5;  // More exploration for variety
    ai.self_play_train(5);
    
    println!("\n{}", ai.status_report());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_integration() {
        let ai = NeuralChessAI::new();
        assert_eq!(ai.games_played, 0);
    }

    #[test]
    fn test_tensor_basics() {
        let t = Tensor::ones(&[3, 3]);
        assert_eq!(t.sum(), 9.0);
    }

    #[test]
    fn test_board_starting() {
        let board = Board::starting_position();
        let moves = board.generate_legal_moves();
        assert_eq!(moves.len(), 20);
    }

    #[test]
    fn test_full_game() {
        let mut ai = NeuralChessAI::new();
        ai.epsilon = 1.0;  // Full random for fast test
        
        let record = ai.self_play_game();
        assert!(record.result != GameResult::Ongoing || record.positions.len() >= 500);
    }
}
