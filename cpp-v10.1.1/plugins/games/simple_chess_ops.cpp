// plugins/games/simple_chess_ops.cpp
//
// Minimal chess engine plugin for Woflang v10.
// - No castling, no en passant, promotion = queen only.
// - Legal move generation (no moving off board, no capturing own,
//   and no leaving your king in check).
// - Simple 3-ply alpha-beta search for engine reply.

#include "woflang.hpp"

#include <array>
#include <cctype>
#include <iostream>
#include <limits>
#include <string>
#include <utility>
#include <vector>

using namespace woflang;

#ifndef WOFLANG_PLUGIN_EXPORT
# ifdef _WIN32
#  define WOFLANG_PLUGIN_EXPORT extern "C" __declspec(dllexport)
# else
#  define WOFLANG_PLUGIN_EXPORT extern "C"
# endif
#endif

// ---------------------- Board representation ------------------------

struct ChessPosition {
    std::array<char, 64> board{}; // 'P','N','B','R','Q','K' for white,
                                  // 'p','n','b','r','q','k' for black,
                                  // '.' for empty
    bool whiteToMove = true;
};

// Single global game state for this plugin
static ChessPosition g_pos;

// Helpers: board indexing
static int square_index(int file, int rank) {
    // file: 0..7 for a..h, rank: 0..7 for 1..8
    return rank * 8 + file;
}

static int square_index_from_alg(const std::string &mv, int offset) {
    // mv like "e2e4"; offset=0 => "e2"; offset=2 => "e4"
    char f = mv[offset + 0];
    char r = mv[offset + 1];
    if (f < 'a' || f > 'h' || r < '1' || r > '8') return -1;
    int file = f - 'a';
    int rank = r - '1';
    return square_index(file, rank);
}

static void init_start_position(ChessPosition &pos) {
    pos.board.fill('.');
    const std::string back = "rnbqkbnr";
    for (int f = 0; f < 8; ++f) {
        pos.board[square_index(f, 0)] = std::toupper(back[f]); // white
        pos.board[square_index(f, 1)] = 'P';
        pos.board[square_index(f, 6)] = 'p';
        pos.board[square_index(f, 7)] = back[f];
    }
    pos.whiteToMove = true;
}

static void print_board(const ChessPosition &pos) {
    std::cout << "   +------------------------+\n";
    for (int r = 7; r >= 0; --r) {
        std::cout << " " << (r + 1) << " |";
        for (int f = 0; f < 8; ++f) {
            char c = pos.board[square_index(f, r)];
            if (c == '.') c = '.';
            std::cout << " " << c;
        }
        std::cout << " |\n";
    }
    std::cout << "   +------------------------+\n";
    std::cout << "     a b c d e f g h\n";
    std::cout << "Side to move: " << (pos.whiteToMove ? "White" : "Black") << "\n";
}

// ---------------------- Move representation -------------------------

struct Move {
    int from = -1;
    int to = -1;
    char promo = 0; // 'Q'/'q' or 0 for non-promotion
};

static std::string move_to_string(const Move &m) {
    if (m.from < 0 || m.to < 0) return "";
    int ff = m.from % 8;
    int fr = m.from / 8;
    int tf = m.to % 8;
    int tr = m.to / 8;
    std::string s;
    s.push_back(static_cast<char>('a' + ff));
    s.push_back(static_cast<char>('1' + fr));
    s.push_back(static_cast<char>('a' + tf));
    s.push_back(static_cast<char>('1' + tr));
    if (m.promo != 0) {
        s.push_back(m.promo);
    }
    return s;
}

// ---------------------- Piece helpers -------------------------------

static bool is_white_piece(char p) {
    return p == 'P' || p == 'N' || p == 'B' ||
           p == 'R' || p == 'Q' || p == 'K';
}

static bool is_black_piece(char p) {
    return p == 'p' || p == 'n' || p == 'b' ||
           p == 'r' || p == 'q' || p == 'k';
}

static bool is_empty(char p) {
    return p == '.';
}

// ---------------------- Attack / legality ---------------------------

static bool is_square_attacked(const ChessPosition &pos, int sq, bool byWhite);

static int king_square(const ChessPosition &pos, bool white) {
    char k = white ? 'K' : 'k';
    for (int i = 0; i < 64; ++i) {
        if (pos.board[i] == k) return i;
    }
    return -1;
}

static bool in_check(const ChessPosition &pos, bool white) {
    int ksq = king_square(pos, white);
    if (ksq < 0) return false;
    return is_square_attacked(pos, ksq, !white);
}

static bool is_square_attacked(const ChessPosition &pos, int sq, bool byWhite) {
    int file = sq % 8;
    int rank = sq / 8;

    auto inside = [](int f, int r) {
        return f >= 0 && f < 8 && r >= 0 && r < 8;
    };

    // Pawn attacks
    int pawn_dir = byWhite ? 1 : -1;
    int pawn_rank = rank - pawn_dir; // square from which pawn would attack sq
    for (int df : {-1, 1}) {
        int pf = file + df;
        if (!inside(pf, pawn_rank)) continue;
        char p = pos.board[square_index(pf, pawn_rank)];
        if (byWhite && p == 'P') return true;
        if (!byWhite && p == 'p') return true;
    }

    // Knight attacks
    const int knight_moves[8][2] = {
        {1, 2}, {2, 1}, {2, -1}, {1, -2},
        {-1, -2}, {-2, -1}, {-2, 1}, {-1, 2}
    };
    for (auto &nm : knight_moves) {
        int nf = file + nm[0];
        int nr = rank + nm[1];
        if (!inside(nf, nr)) continue;
        char p = pos.board[square_index(nf, nr)];
        if (byWhite && p == 'N') return true;
        if (!byWhite && p == 'n') return true;
    }

    // Bishop / Queen (diagonals)
    const int diag_dirs[4][2] = {{1,1},{1,-1},{-1,1},{-1,-1}};
    for (auto &d : diag_dirs) {
        int nf = file + d[0];
        int nr = rank + d[1];
        while (inside(nf, nr)) {
            char p = pos.board[square_index(nf, nr)];
            if (!is_empty(p)) {
                if (byWhite && (p == 'B' || p == 'Q')) return true;
                if (!byWhite && (p == 'b' || p == 'q')) return true;
                break;
            }
            nf += d[0];
            nr += d[1];
        }
    }

    // Rook / Queen (orthogonals)
    const int ortho_dirs[4][2] = {{1,0},{-1,0},{0,1},{0,-1}};
    for (auto &d : ortho_dirs) {
        int nf = file + d[0];
        int nr = rank + d[1];
        while (inside(nf, nr)) {
            char p = pos.board[square_index(nf, nr)];
            if (!is_empty(p)) {
                if (byWhite && (p == 'R' || p == 'Q')) return true;
                if (!byWhite && (p == 'r' || p == 'q')) return true;
                break;
            }
            nf += d[0];
            nr += d[1];
        }
    }

    // King attacks (adjacent squares)
    for (int df = -1; df <= 1; ++df) {
        for (int dr = -1; dr <= 1; ++dr) {
            if (df == 0 && dr == 0) continue;
            int nf = file + df;
            int nr = rank + dr;
            if (!inside(nf, nr)) continue;
            char p = pos.board[square_index(nf, nr)];
            if (byWhite && p == 'K') return true;
            if (!byWhite && p == 'k') return true;
        }
    }

    return false;
}

// ---------------------- Move generation -----------------------------

static void add_move_if_legal(const ChessPosition &pos,
                              int from, int to,
                              std::vector<Move> &moves) {
    if (from < 0 || from >= 64 || to < 0 || to >= 64) return;

    char p = pos.board[from];
    char t = pos.board[to];
    if (p == '.' || p == 0) return;

    bool white = pos.whiteToMove;
    if (white && !is_white_piece(p)) return;
    if (!white && !is_black_piece(p)) return;

    if ((white && is_white_piece(t)) || (!white && is_black_piece(t))) {
        // can't capture own piece
        return;
    }

    // Make a copy and test king safety
    ChessPosition tmp = pos;
    tmp.board[to] = p;
    tmp.board[from] = '.';

    // Handle promotion
    char promo = 0;
    int toRank = to / 8;
    if (p == 'P' && toRank == 7) {
        tmp.board[to] = 'Q';
        promo = 'Q';
    } else if (p == 'p' && toRank == 0) {
        tmp.board[to] = 'q';
        promo = 'q';
    }

    tmp.whiteToMove = !pos.whiteToMove;

    if (in_check(tmp, pos.whiteToMove)) {
        // Can't leave our own king in check
        return;
    }

    Move m;
    m.from = from;
    m.to = to;
    m.promo = promo;
    moves.push_back(m);
}

static void generate_moves(const ChessPosition &pos, std::vector<Move> &moves) {
    moves.clear();
    moves.reserve(64);

    auto inside = [](int f, int r) {
        return f >= 0 && f < 8 && r >= 0 && r < 8;
    };

    bool white = pos.whiteToMove;

    for (int sq = 0; sq < 64; ++sq) {
        char p = pos.board[sq];
        if (p == '.') continue;
        if (white && !is_white_piece(p)) continue;
        if (!white && !is_black_piece(p)) continue;

        int f = sq % 8;
        int r = sq / 8;

        if (p == 'P' || p == 'p') {
            int dir = (p == 'P') ? 1 : -1;
            int startRank = (p == 'P') ? 1 : 6;

            // single push
            int nr = r + dir;
            if (inside(f, nr)) {
                int to = square_index(f, nr);
                if (is_empty(pos.board[to])) {
                    add_move_if_legal(pos, sq, to, moves);

                    // double push
                    if (r == startRank) {
                        nr = r + 2 * dir;
                        if (inside(f, nr)) {
                            int to2 = square_index(f, nr);
                            if (is_empty(pos.board[to2])) {
                                add_move_if_legal(pos, sq, to2, moves);
                            }
                        }
                    }
                }
            }

            // captures
            for (int df : {-1, 1}) {
                int nf = f + df;
                int nr2 = r + dir;
                if (!inside(nf, nr2)) continue;
                int to = square_index(nf, nr2);
                char t = pos.board[to];
                if (p == 'P' && is_black_piece(t)) {
                    add_move_if_legal(pos, sq, to, moves);
                } else if (p == 'p' && is_white_piece(t)) {
                    add_move_if_legal(pos, sq, to, moves);
                }
            }
        } else if (p == 'N' || p == 'n') {
            const int knight_moves[8][2] = {
                {1, 2}, {2, 1}, {2, -1}, {1, -2},
                {-1, -2}, {-2, -1}, {-2, 1}, {-1, 2}
            };
            for (auto &m : knight_moves) {
                int nf = f + m[0];
                int nr = r + m[1];
                if (!inside(nf, nr)) continue;
                int to = square_index(nf, nr);
                char t = pos.board[to];
                if (white && is_white_piece(t)) continue;
                if (!white && is_black_piece(t)) continue;
                add_move_if_legal(pos, sq, to, moves);
            }
        } else if (p == 'B' || p == 'b' || p == 'R' || p == 'r' ||
                   p == 'Q' || p == 'q') {
            bool isBishopLike = (p == 'B' || p == 'b' || p == 'Q' || p == 'q');
            bool isRookLike   = (p == 'R' || p == 'r' || p == 'Q' || p == 'q');

            if (isBishopLike) {
                const int diag_dirs[4][2] = {{1,1},{1,-1},{-1,1},{-1,-1}};
                for (auto &d : diag_dirs) {
                    int nf = f + d[0];
                    int nr = r + d[1];
                    while (inside(nf, nr)) {
                        int to = square_index(nf, nr);
                        char t = pos.board[to];
                        if (white && is_white_piece(t)) break;
                        if (!white && is_black_piece(t)) break;
                        add_move_if_legal(pos, sq, to, moves);
                        if (!is_empty(t)) break;
                        nf += d[0];
                        nr += d[1];
                    }
                }
            }

            if (isRookLike) {
                const int ortho_dirs[4][2] = {{1,0},{-1,0},{0,1},{0,-1}};
                for (auto &d : ortho_dirs) {
                    int nf = f + d[0];
                    int nr = r + d[1];
                    while (inside(nf, nr)) {
                        int to = square_index(nf, nr);
                        char t = pos.board[to];
                        if (white && is_white_piece(t)) break;
                        if (!white && is_black_piece(t)) break;
                        add_move_if_legal(pos, sq, to, moves);
                        if (!is_empty(t)) break;
                        nf += d[0];
                        nr += d[1];
                    }
                }
            }
        } else if (p == 'K' || p == 'k') {
            for (int df = -1; df <= 1; ++df) {
                for (int dr = -1; dr <= 1; ++dr) {
                    if (df == 0 && dr == 0) continue;
                    int nf = f + df;
                    int nr = r + dr;
                    if (!inside(nf, nr)) continue;
                    int to = square_index(nf, nr);
                    char t = pos.board[to];
                    if (white && is_white_piece(t)) continue;
                    if (!white && is_black_piece(t)) continue;
                    add_move_if_legal(pos, sq, to, moves);
                }
            }
        }
    }
}

// ---------------------- Evaluation & search -------------------------

static int piece_value(char p) {
    switch (p) {
        case 'P': return 100;
        case 'N': return 320;
        case 'B': return 330;
        case 'R': return 500;
        case 'Q': return 900;
        case 'K': return 10000;
        case 'p': return -100;
        case 'n': return -320;
        case 'b': return -330;
        case 'r': return -500;
        case 'q': return -900;
        case 'k': return -10000;
        default:  return 0;
    }
}

static int evaluate(const ChessPosition &pos) {
    int score = 0;
    for (char p : pos.board) {
        score += piece_value(p);
    }
    // Slight preference for side to move
    if (pos.whiteToMove) score += 10;
    else score -= 10;
    return score;
}

static ChessPosition make_move(const ChessPosition &pos, const Move &m) {
    ChessPosition next = pos;
    char p = next.board[m.from];
    next.board[m.from] = '.';
    next.board[m.to] = p;
    if (m.promo != 0) {
        next.board[m.to] = m.promo;
    }
    next.whiteToMove = !pos.whiteToMove;
    return next;
}

// depth: remaining plies; alpha/beta standard meanings
static int search(const ChessPosition &pos, int depth, int alpha, int beta) {
    if (depth == 0) {
        return evaluate(pos);
    }

    std::vector<Move> moves;
    generate_moves(pos, moves);

    if (moves.empty()) {
        // No moves: checkmate or stalemate
        if (in_check(pos, pos.whiteToMove)) {
            // Mate in this position
            return pos.whiteToMove ? -1000000 : 1000000;
        } else {
            // Stalemate
            return 0;
        }
    }

    if (pos.whiteToMove) {
        int best = std::numeric_limits<int>::min();
        for (const auto &m : moves) {
            ChessPosition nxt = make_move(pos, m);
            int score = search(nxt, depth - 1, alpha, beta);
            if (score > best) best = score;
            if (score > alpha) alpha = score;
            if (alpha >= beta) break;
        }
        return best;
    } else {
        int best = std::numeric_limits<int>::max();
        for (const auto &m : moves) {
            ChessPosition nxt = make_move(pos, m);
            int score = search(nxt, depth - 1, alpha, beta);
            if (score < best) best = score;
            if (score < beta) beta = score;
            if (alpha >= beta) break;
        }
        return best;
    }
}

static bool find_best_move(const ChessPosition &pos, int depth, Move &bestMove) {
    std::vector<Move> moves;
    generate_moves(pos, moves);
    if (moves.empty()) return false;

    int alpha = std::numeric_limits<int>::min();
    int beta  = std::numeric_limits<int>::max();

    if (pos.whiteToMove) {
        int bestScore = std::numeric_limits<int>::min();
        for (const auto &m : moves) {
            ChessPosition nxt = make_move(pos, m);
            int score = search(nxt, depth - 1, alpha, beta);
            if (score > bestScore) {
                bestScore = score;
                bestMove = m;
            }
            if (score > alpha) alpha = score;
        }
    } else {
        int bestScore = std::numeric_limits<int>::max();
        for (const auto &m : moves) {
            ChessPosition nxt = make_move(pos, m);
            int score = search(nxt, depth - 1, alpha, beta);
            if (score < bestScore) {
                bestScore = score;
                bestMove = m;
            }
            if (score < beta) beta = score;
        }
    }

    return true;
}

// ---------------------- Woflang ops ---------------------------------

// chess_new: reset to start position
static void op_chess_new(WoflangInterpreter &interp) {
    (void)interp;
    init_start_position(g_pos);
    std::cout << "[simple_chess] New game started.\n";
    print_board(g_pos);
}

// chess_show: print current board
static void op_chess_show(WoflangInterpreter &interp) {
    (void)interp;
    print_board(g_pos);
}

// chess_move: pop a move string like "e2e4", play it if legal,
// then let engine respond and push engine move string ("" if none).
static void op_chess_move(WoflangInterpreter &interp) {
    auto &st = interp.stack;
    if (st.empty()) {
        std::cout << "[simple_chess] chess_move: stack empty (need \"e2e4\" etc).\n";
        return;
    }

    WofValue v = st.back();
    st.pop_back();

    if (v.type != WofType::String) {
        std::cout << "[simple_chess] chess_move: expected string move like \"e2e4\" on stack.\n";
        return;
    }

    std::string moveStr = std::get<std::string>(v.value);
    if (moveStr.size() < 4) {
        std::cout << "[simple_chess] Invalid move string: " << moveStr << "\n";
        return;
    }

    int from = square_index_from_alg(moveStr, 0);
    int to   = square_index_from_alg(moveStr, 2);
    if (from < 0 || to < 0) {
        std::cout << "[simple_chess] Invalid move coordinates: " << moveStr << "\n";
        return;
    }

    std::vector<Move> legal;
    generate_moves(g_pos, legal);

    Move userMove;
    bool found = false;
    for (const auto &m : legal) {
        if (m.from == from && m.to == to) {
            userMove = m;
            found = true;
            break;
        }
    }

    if (!found) {
        std::cout << "[simple_chess] Illegal move: " << moveStr << "\n";
        return;
    }

    // Apply user move
    g_pos = make_move(g_pos, userMove);
    std::cout << "[simple_chess] You played: " << move_to_string(userMove) << "\n";
    print_board(g_pos);

    // Engine reply
    Move engineMove;
    if (!find_best_move(g_pos, 3, engineMove)) {
        // No moves: game over
        bool stmWhite = g_pos.whiteToMove;
        if (in_check(g_pos, stmWhite)) {
            std::cout << "[simple_chess] Checkmate. "
                      << (stmWhite ? "White" : "Black")
                      << " is checkmated.\n";
        } else {
            std::cout << "[simple_chess] Stalemate.\n";
        }
        WofValue empty;
        empty.type = WofType::String;
        empty.value = std::string("");
        interp.stack.push_back(empty);
        return;
    }

    g_pos = make_move(g_pos, engineMove);
    std::string engStr = move_to_string(engineMove);
    std::cout << "[simple_chess] Engine plays: " << engStr << "\n";
    print_board(g_pos);

    // Push engine move string onto stack
    WofValue engVal;
    engVal.type = WofType::String;
    engVal.value = engStr;
    interp.stack.push_back(engVal);
}

// ---------------------- Plugin entry --------------------------------

WOFLANG_PLUGIN_EXPORT void register_plugin(WoflangInterpreter &interp) {
    interp.register_op("chess_new",  [](WoflangInterpreter &ip) { op_chess_new(ip); });
    interp.register_op("chess_show", [](WoflangInterpreter &ip) { op_chess_show(ip); });
    interp.register_op("chess_move", [](WoflangInterpreter &ip) { op_chess_move(ip); });
    std::cout << "[simple_chess] Plugin loaded.\n";
}
