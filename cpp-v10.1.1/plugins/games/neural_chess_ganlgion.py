#!/usr/bin/env python3
"""
Neural Chess "Ganglion Brain" – Claude-style homage.

- Full chess rules via python-chess (castling, en passant, promotions, etc.).
- "Brain" is a synchronized trio of:
    * CNN over board planes
    * GRU over move history
    * LSTM over a Cellular Automaton (CA) grid
  coordinated by a "Ganglion" module.
- GAN-style pair:
    * Generator: GanglionBrain (policy + value)
    * Discriminator: judges (board, move) plausibility

Supports:
  - Human vs AI mode
  - Self-play training for N games
"""

import argparse
import math
import random
from dataclasses import dataclass, field
from typing import List, Tuple, Optional

import numpy as np
import torch
import torch.nn as nn
import torch.optim as optim
import chess


# ===================== Utility / Encoding ============================

NUM_PIECE_PLANES = 12  # 6 white + 6 black
BOARD_SIZE = 8
MAX_HISTORY = 32
POLICY_DIM = BOARD_SIZE * BOARD_SIZE * BOARD_SIZE * BOARD_SIZE  # 64*64


def board_to_planes(board: chess.Board) -> torch.Tensor:
    """
    Encode board to [12, 8, 8] planes:
      0..5: white P,N,B,R,Q,K
      6..11: black p,n,b,r,q,k
    """
    planes = np.zeros((NUM_PIECE_PLANES, BOARD_SIZE, BOARD_SIZE), dtype=np.float32)
    piece_map = board.piece_map()
    for sq, piece in piece_map.items():
        rank = chess.square_rank(sq)
        file = chess.square_file(sq)
        idx = None
        if piece.color == chess.WHITE:
            if piece.piece_type == chess.PAWN:
                idx = 0
            elif piece.piece_type == chess.KNIGHT:
                idx = 1
            elif piece.piece_type == chess.BISHOP:
                idx = 2
            elif piece.piece_type == chess.ROOK:
                idx = 3
            elif piece.piece_type == chess.QUEEN:
                idx = 4
            elif piece.piece_type == chess.KING:
                idx = 5
        else:
            if piece.piece_type == chess.PAWN:
                idx = 6
            elif piece.piece_type == chess.KNIGHT:
                idx = 7
            elif piece.piece_type == chess.BISHOP:
                idx = 8
            elif piece.piece_type == chess.ROOK:
                idx = 9
            elif piece.piece_type == chess.QUEEN:
                idx = 10
            elif piece.piece_type == chess.KING:
                idx = 11
        if idx is not None:
            planes[idx, rank, file] = 1.0
    return torch.from_numpy(planes)


def encode_move_to_index(move: chess.Move) -> int:
    """
    Map move to [0, 4095] via from*64 + to.
    Ignore promotion in index; promotion type is learned implicitly.
    """
    return move.from_square * 64 + move.to_square


def index_to_move(board: chess.Board, idx: int) -> Optional[chess.Move]:
    """
    Map index back to a legal move (if any). If multiple promotions share same
    from/to, prefer queen promotion.
    """
    from_sq = idx // 64
    to_sq = idx % 64
    candidates = [m for m in board.legal_moves if m.from_square == from_sq and m.to_square == to_sq]
    if not candidates:
        return None
    # Prefer queen promotion if available, else first legal
    for m in candidates:
        if m.promotion == chess.QUEEN:
            return m
    return candidates[0]


def encode_history(moves: List[chess.Move], max_len: int = MAX_HISTORY) -> torch.Tensor:
    """
    Encode move history as sequence [T, 4]:
      [from/63, to/63, is_promotion, promotion_type/6]
    """
    seq = []
    last_moves = moves[-max_len:]
    for m in last_moves:
        from_norm = m.from_square / 63.0
        to_norm = m.to_square / 63.0
        is_promo = 1.0 if m.promotion is not None else 0.0
        promo_type = (m.promotion or 0) / 6.0  # piece_type in 1..6
        seq.append([from_norm, to_norm, is_promo, promo_type])
    if not seq:
        # at least one zero vector to keep GRU shapes valid
        seq = [[0.0, 0.0, 0.0, 0.0]]
    arr = np.array(seq, dtype=np.float32)
    return torch.from_numpy(arr)  # [T,4]


# ===================== Cellular Automaton ============================

class CA2D(nn.Module):
    """
    Simple differentiable 2D cellular automaton.

    State: [B, C, 8, 8]
    Input: [B, 12, 8, 8] (board planes)

    Uses a Conv2d over concatenated state+board, and residual update.
    """

    def __init__(self, channels: int = 4):
        super().__init__()
        self.channels = channels
        self.conv = nn.Conv2d(channels + NUM_PIECE_PLANES,
                              channels,
                              kernel_size=3,
                              padding=1)
        nn.init.kaiming_uniform_(self.conv.weight, a=math.sqrt(5.0))

    def forward(self, state: torch.Tensor, board_planes: torch.Tensor) -> torch.Tensor:
        """
        state: [B,C,8,8], board_planes: [B,12,8,8]
        returns: new_state [B,C,8,8]
        """
        x = torch.cat([state, board_planes], dim=1)
        delta = torch.tanh(self.conv(x))
        return state + delta


# ===================== Neural Building Blocks =======================

class BoardCNN(nn.Module):
    def __init__(self, out_dim: int = 128):
        super().__init__()
        self.conv = nn.Sequential(
            nn.Conv2d(NUM_PIECE_PLANES, 32, kernel_size=3, padding=1),
            nn.ReLU(inplace=True),
            nn.Conv2d(32, 64, kernel_size=3, padding=1),
            nn.ReLU(inplace=True),
            nn.Conv2d(64, 64, kernel_size=3, padding=1),
            nn.ReLU(inplace=True),
        )
        self.fc = nn.Linear(64 * BOARD_SIZE * BOARD_SIZE, out_dim)

    def forward(self, x: torch.Tensor) -> torch.Tensor:
        # x: [B,12,8,8]
        h = self.conv(x)
        h = h.view(h.size(0), -1)
        return torch.tanh(self.fc(h))


class HistoryRNN(nn.Module):
    def __init__(self, input_dim: int = 4, hidden_dim: int = 64):
        super().__init__()
        self.gru = nn.GRU(input_dim, hidden_dim, batch_first=True)

    def forward(self, seq: torch.Tensor) -> torch.Tensor:
        # seq: [B,T,4]
        _, h_n = self.gru(seq)
        # h_n: [1,B,H]
        return torch.tanh(h_n.squeeze(0))


class ContextLSTM(nn.Module):
    def __init__(self, input_dim: int, hidden_dim: int = 64):
        super().__init__()
        self.lstm = nn.LSTM(input_dim, hidden_dim, batch_first=True)

    def forward(self, ca_state: torch.Tensor) -> torch.Tensor:
        """
        ca_state: [B,C,8,8]
        Treat spatial positions as sequence of length 64 with C features.
        """
        B, C, H, W = ca_state.shape
        seq = ca_state.view(B, C, H * W).permute(0, 2, 1)  # [B,64,C]
        _, (h_n, _) = self.lstm(seq)
        return torch.tanh(h_n.squeeze(0))


# ===================== Ganglion Brain (Generator) ===================

class GanglionBrain(nn.Module):
    """
    The "brain" coordinating:
      - Board CNN
      - History GRU
      - CA-based context LSTM
    """

    def __init__(self,
                 board_dim: int = 128,
                 hist_dim: int = 64,
                 ctx_dim: int = 64,
                 ca_channels: int = 4,
                 hidden_dim: int = 256):
        super().__init__()
        self.ca = CA2D(channels=ca_channels)
        self.board_cnn = BoardCNN(out_dim=board_dim)
        self.hist_rnn = HistoryRNN(hidden_dim=hist_dim)
        self.ctx_lstm = ContextLSTM(input_dim=ca_channels, hidden_dim=ctx_dim)

        fused_dim = board_dim + hist_dim + ctx_dim
        self.fc_shared = nn.Sequential(
            nn.Linear(fused_dim, hidden_dim),
            nn.ReLU(inplace=True)
        )
        self.policy_head = nn.Linear(hidden_dim, POLICY_DIM)
        self.value_head = nn.Linear(hidden_dim, 1)

    def forward(self,
                board_planes: torch.Tensor,
                history_seq: torch.Tensor,
                ca_state: torch.Tensor) -> Tuple[torch.Tensor, torch.Tensor, torch.Tensor]:
        """
        board_planes: [B,12,8,8]
        history_seq:  [B,T,4]
        ca_state:     [B,C,8,8]

        returns:
          policy_logits: [B,4096]
          value:         [B,1]
          new_ca_state:  [B,C,8,8]
        """
        new_ca = self.ca(ca_state, board_planes)
        b_feat = self.board_cnn(board_planes)
        h_feat = self.hist_rnn(history_seq)
        c_feat = self.ctx_lstm(new_ca)
        fused = torch.cat([b_feat, h_feat, c_feat], dim=1)
        x = self.fc_shared(fused)
        policy_logits = self.policy_head(x)
        value = torch.tanh(self.value_head(x))
        return policy_logits, value, new_ca


# ===================== Discriminator (GAN Pair) =====================

class MoveDiscriminator(nn.Module):
    """
    Simple discriminator: (board_planes, move_index) -> [0,1]
    """

    def __init__(self, board_dim: int = 128, move_embed_dim: int = 32, hidden_dim: int = 128):
        super().__init__()
        self.board_cnn = BoardCNN(out_dim=board_dim)
        self.move_embed = nn.Embedding(POLICY_DIM, move_embed_dim)
        self.fc = nn.Sequential(
            nn.Linear(board_dim + move_embed_dim, hidden_dim),
            nn.ReLU(inplace=True),
            nn.Linear(hidden_dim, 1),
            nn.Sigmoid()
        )

    def forward(self, board_planes: torch.Tensor, move_index: torch.Tensor) -> torch.Tensor:
        """
        board_planes: [B,12,8,8]
        move_index:   [B] (long)
        returns: [B,1] scores in [0,1]
        """
        b_feat = self.board_cnn(board_planes)
        m_emb = self.move_embed(move_index)
        x = torch.cat([b_feat, m_emb], dim=1)
        return self.fc(x)


# ===================== Engine + Training Loop =======================

@dataclass
class BrainState:
    ca_state: torch.Tensor = field(default_factory=lambda: torch.zeros(1, 4, 8, 8))


class NeuralChessEngine:
    """
    Manages:
      - chess.Board
      - GanglionBrain (policy/value)
      - MoveDiscriminator
      - Training (self-play GAN-ish updates)
    """

    def __init__(self, device: Optional[str] = None):
        if device is None:
            device = "cuda" if torch.cuda.is_available() else "cpu"
        self.device = torch.device(device)

        self.brain = GanglionBrain().to(self.device)
        self.disc = MoveDiscriminator().to(self.device)

        self.gen_opt = optim.Adam(self.brain.parameters(), lr=1e-4)
        self.disc_opt = optim.Adam(self.disc.parameters(), lr=1e-4)

        self.board = chess.Board()
        self.history: List[chess.Move] = []
        self.state = BrainState(ca_state=torch.zeros(1, 4, 8, 8, device=self.device))

        self.bce = nn.BCELoss()

    # ---------- Game control ----------

    def reset_game(self) -> None:
        self.board.reset()
        self.history = []
        self.state = BrainState(ca_state=torch.zeros(1, 4, 8, 8, device=self.device))

    # ---------- Neural forward helpers ----------

    def _prepare_inputs(self) -> Tuple[torch.Tensor, torch.Tensor, torch.Tensor]:
        planes = board_to_planes(self.board).unsqueeze(0).to(self.device)  # [1,12,8,8]
        hist_seq = encode_history(self.history).unsqueeze(0).to(self.device)  # [1,T,4]
        ca_state = self.state.ca_state
        return planes, hist_seq, ca_state

    # ---------- Move selection ----------

    def select_move(self, temperature: float = 1.0) -> Optional[chess.Move]:
        if self.board.is_game_over():
            return None

        legal_moves = list(self.board.legal_moves)
        if not legal_moves:
            return None

        planes, hist_seq, ca_state = self._prepare_inputs()
        with torch.no_grad():
            policy_logits, _, new_ca = self.brain(planes, hist_seq, ca_state)
        self.state.ca_state = new_ca.detach()

        logits = policy_logits[0]  # [4096]
        # Mask illegal moves
        mask = torch.full_like(logits, float("-inf"))
        legal_indices = []
        for mv in legal_moves:
            idx = encode_move_to_index(mv)
            legal_indices.append(idx)
        legal_indices_tensor = torch.tensor(legal_indices, device=logits.device, dtype=torch.long)
        mask[legal_indices_tensor] = logits[legal_indices_tensor]

        if temperature <= 0.0:
            # Greedy
            best_idx = torch.argmax(mask).item()
            move = index_to_move(self.board, best_idx)
            return move

        # Softmax with temperature over legal moves
        masked_vals = mask[legal_indices_tensor] / max(temperature, 1e-3)
        probs = torch.softmax(masked_vals, dim=0)
        choice = torch.multinomial(probs, num_samples=1).item()
        chosen_idx = legal_indices[choice]
        return index_to_move(self.board, chosen_idx)

    # ---------- Training step (simple GAN-ish) ----------

    def train_on_position(self,
                          real_move: chess.Move,
                          fake_move: chess.Move) -> None:
        """
        Train discriminator and generator a bit on:
          - real_move: treated as plausible
          - fake_move: treated as implausible
        """
        self.gen_opt.zero_grad()
        self.disc_opt.zero_grad()

        planes = board_to_planes(self.board).unsqueeze(0).to(self.device)  # [1,12,8,8]
        real_idx = torch.tensor([encode_move_to_index(real_move)], device=self.device, dtype=torch.long)
        fake_idx = torch.tensor([encode_move_to_index(fake_move)], device=self.device, dtype=torch.long)

        # --- Train discriminator ---
        real_score = self.disc(planes, real_idx)
        fake_score = self.disc(planes, fake_idx)

        real_target = torch.ones_like(real_score)
        fake_target = torch.zeros_like(fake_score)

        disc_loss = self.bce(real_score, real_target) + self.bce(fake_score, fake_target)
        disc_loss.backward(retain_graph=True)
        self.disc_opt.step()

        # --- Train generator (policy) to like real move ---
        self.gen_opt.zero_grad()

        planes_g, hist_seq_g, ca_state_g = self._prepare_inputs()
        policy_logits, _, _ = self.brain(planes_g, hist_seq_g, ca_state_g)
        logits = policy_logits[0]  # [4096]

        # Negative log-prob of real_move index (softmax over all)
        log_probs = torch.log_softmax(logits, dim=0)
        gen_loss = -log_probs[real_idx[0]]

        gen_loss.backward()
        self.gen_opt.step()

    # ---------- Self-play training ----------

    def self_play_game(self, max_moves: int = 200) -> None:
        self.reset_game()
        for _ply in range(max_moves):
            if self.board.is_game_over():
                break

            legal_moves = list(self.board.legal_moves)
            if not legal_moves:
                break

            # True "real" move: let current policy pick
            real_move = self.select_move(temperature=0.7)
            if real_move is None:
                break

            # Fake move: random legal move distinct from real if possible
            fake_candidates = [m for m in legal_moves if m != real_move]
            if not fake_candidates:
                fake_move = real_move
            else:
                fake_move = random.choice(fake_candidates)

            # Train a little at this position
            self.train_on_position(real_move=real_move, fake_move=fake_move)

            # Play the real move on board and append to history
            self.board.push(real_move)
            self.history.append(real_move)

    def train_self_play(self, games: int = 10, max_moves: int = 200) -> None:
        for g in range(1, games + 1):
            print(f"[train] Self-play game {g}/{games}")
            self.self_play_game(max_moves=max_moves)
            print(f"[train] Game {g} result: {self.board.result(claim_draw=True)}")

    # ---------- Human vs AI ----------

    def print_board(self) -> None:
        print(self.board)
        print(f"Side to move: {'White' if self.board.turn == chess.WHITE else 'Black'}")
        print(f"FEN: {self.board.fen()}")

    def play_vs_human(self, human_plays_white: bool = True) -> None:
        self.reset_game()
        print("Welcome to Neural Chess – Ganglion Brain Edition.")
        print("Enter moves in UCI (e2e4, g1f3, e7e8q, etc.). Type 'quit' to exit.\n")

        while not self.board.is_game_over():
            self.print_board()
            human_turn = (self.board.turn == chess.WHITE and human_plays_white) or \
                         (self.board.turn == chess.BLACK and not human_plays_white)

            if human_turn:
                move_str = input("Your move (UCI): ").strip()
                if move_str.lower() in ("q", "quit", "exit"):
                    print("Exiting game.")
                    return
                try:
                    move = chess.Move.from_uci(move_str)
                except ValueError:
                    print("Invalid move format. Use UCI like 'e2e4' or 'e7e8q'.")
                    continue
                if move not in self.board.legal_moves:
                    print("Illegal move. Try again.")
                    continue
                self.board.push(move)
                self.history.append(move)
            else:
                print("Neural Chess is thinking...")
                ai_move = self.select_move(temperature=0.2)
                if ai_move is None:
                    print("No legal moves for AI.")
                    break
                print(f"AI plays: {ai_move.uci()}")
                self.board.push(ai_move)
                self.history.append(ai_move)

        print("Game over.")
        print(f"Result: {self.board.result(claim_draw=True)}")


# ===================== Main / CLI ===================================

def main() -> None:
    parser = argparse.ArgumentParser(description="Neural Chess Ganglion Brain")
    parser.add_argument("--mode",
                        choices=["human", "self-play"],
                        default="human",
                        help="Run in human-vs-AI or self-play training mode.")
    parser.add_argument("--games",
                        type=int,
                        default=5,
                        help="Number of self-play games for training mode.")
    parser.add_argument("--max-moves",
                        type=int,
                        default=200,
                        help="Max half-moves per self-play game.")
    parser.add_argument("--human-plays-white",
                        action="store_true",
                        help="In human mode, human plays White (default).")
    parser.add_argument("--human-plays-black",
                        action="store_true",
                        help="In human mode, human plays Black.")

    args = parser.parse_args()

    human_white = True
    if args.human_plays_black:
        human_white = False
    if args.human_plays_white:
        human_white = True

    engine = NeuralChessEngine()

    if args.mode == "human":
        engine.play_vs_human(human_plays_white=human_white)
    else:
        engine.train_self_play(games=args.games, max_moves=args.max_moves)


if __name__ == "__main__":
    main()
