use crate::board::Board;
use crate::eval::{evaluate_with_params, EvalParams};
use crate::movegen::generate_moves;
use crate::moves::{Move, MoveList};
use crate::tt::{Bound, TranspositionTable};
use crate::types::PieceKind;
use std::time::{Duration, Instant};

pub const MATE_SCORE: i32 = 100_000;
pub const MAX_PLY: usize = 128;

pub struct SearchInfo {
    pub best_move: Move,
    pub score: i32,
    pub depth: i32,
    pub nodes: u64,
    pub pv: Vec<Move>,
}

pub struct Searcher {
    pub tt: TranspositionTable,
    pub eval_params: EvalParams,
    pub silent: bool,
    nodes: u64,
    start_time: Instant,
    time_limit: Option<Duration>,
    stopped: bool,
    killers: [[Move; 2]; MAX_PLY],
    history: [[[i32; 64]; 6]; 2],
    pv_table: [[Move; MAX_PLY]; MAX_PLY],
    pv_length: [usize; MAX_PLY],
    position_history: Vec<u64>,
    game_history_len: usize,
    move_scores: [i32; 256],
}

impl Searcher {
    pub fn new(tt_size_mb: usize) -> Self {
        Searcher {
            tt: TranspositionTable::new(tt_size_mb),
            eval_params: EvalParams::default(),
            silent: false,
            nodes: 0,
            start_time: Instant::now(),
            time_limit: None,
            stopped: false,
            killers: [[Move::NULL; 2]; MAX_PLY],
            history: [[[0; 64]; 6]; 2],
            pv_table: [[Move::NULL; MAX_PLY]; MAX_PLY],
            pv_length: [0; MAX_PLY],
            position_history: Vec::new(),
            game_history_len: 0,
            move_scores: [0; 256],
        }
    }

    pub fn with_params(tt_size_mb: usize, eval_params: EvalParams) -> Self {
        let mut s = Self::new(tt_size_mb);
        s.eval_params = eval_params;
        s
    }

    pub fn set_position_history(&mut self, hashes: Vec<u64>) {
        self.game_history_len = hashes.len();
        self.position_history = hashes;
    }

    /// Run iterative deepening search.
    pub fn search(&mut self, board: &mut Board, max_depth: i32, time_limit: Option<Duration>) -> SearchInfo {
        self.nodes = 0;
        self.start_time = Instant::now();
        self.time_limit = time_limit;
        self.stopped = false;
        self.tt.new_generation();

        let mut best_move = Move::NULL;
        let mut best_score = 0;
        let mut best_pv = Vec::new();

        let mut asp_window = 50; // aspiration window half-width

        for depth in 1..=max_depth {
            self.pv_length[0] = 0;

            // Aspiration windows: narrow search around previous score
            let (mut alpha, mut beta) = if depth >= 4 {
                (best_score - asp_window, best_score + asp_window)
            } else {
                (-MATE_SCORE - 1, MATE_SCORE + 1)
            };

            let mut score;
            loop {
                score = self.alpha_beta(board, depth, alpha, beta, 0);
                if self.stopped {
                    break;
                }
                // Widen window if score fell outside
                if score <= alpha {
                    alpha = -MATE_SCORE - 1;
                    asp_window *= 2;
                } else if score >= beta {
                    beta = MATE_SCORE + 1;
                    asp_window *= 2;
                } else {
                    asp_window = 50; // reset for next depth
                    break;
                }
            }

            if self.stopped {
                break;
            }

            best_score = score;
            if self.pv_length[0] > 0 {
                best_move = self.pv_table[0][0];
                best_pv = self.pv_table[0][..self.pv_length[0]].to_vec();
            }

            let elapsed = self.start_time.elapsed();
            let nps = if elapsed.as_millis() > 0 {
                (self.nodes as u128 * 1000) / elapsed.as_millis()
            } else {
                0
            };

            // Print UCI info
            let pv_str: String = best_pv.iter().map(|m| m.to_uci()).collect::<Vec<_>>().join(" ");
            let score_str = if score.abs() >= MATE_SCORE - MAX_PLY as i32 {
                let mate_in = if score > 0 {
                    (MATE_SCORE - score + 1) / 2
                } else {
                    -(MATE_SCORE + score + 1) / 2
                };
                format!("mate {}", mate_in)
            } else {
                format!("cp {}", score)
            };

            if !self.silent {
                println!(
                    "info depth {} score {} nodes {} nps {} time {} pv {}",
                    depth,
                    score_str,
                    self.nodes,
                    nps,
                    elapsed.as_millis(),
                    pv_str
                );
            }
        }

        SearchInfo {
            best_move,
            score: best_score,
            depth: max_depth,
            nodes: self.nodes,
            pv: best_pv,
        }
    }

    fn check_time(&mut self) {
        if self.nodes & 4095 == 0 {
            if let Some(limit) = self.time_limit {
                if self.start_time.elapsed() >= limit {
                    self.stopped = true;
                }
            }
        }
    }

    fn is_repetition(&self, hash: u64) -> bool {
        // Check only the game history (not the search stack) for repetitions.
        // A position is a repetition if it appeared at least once before in the game.
        // The game_history_len tracks where the game history ends.
        self.position_history[..self.game_history_len]
            .iter()
            .any(|&h| h == hash)
    }

    fn alpha_beta(
        &mut self,
        board: &mut Board,
        depth: i32,
        mut alpha: i32,
        beta: i32,
        ply: usize,
    ) -> i32 {
        self.check_time();
        if self.stopped {
            return 0;
        }

        self.pv_length[ply] = 0;

        // Repetition detection
        if ply > 0 && self.is_repetition(board.hash) {
            return 0;
        }

        // Probe TT
        let mut tt_move = Move::NULL;
        if let Some(entry) = self.tt.probe(board.hash) {
            tt_move = entry.best_move;
            if entry.depth as i32 >= depth {
                let tt_score = entry.score as i32;
                let can_cutoff = match entry.bound {
                    Bound::Exact => true,
                    Bound::Lower => tt_score >= beta,
                    Bound::Upper => tt_score <= alpha,
                };
                if can_cutoff {
                    // Set PV from TT move so the caller can extract it
                    if !tt_move.is_null() {
                        self.pv_table[ply][0] = tt_move;
                        self.pv_length[ply] = 1;
                    }
                    return tt_score;
                }
            }
        }

        // Leaf node
        if depth <= 0 {
            return self.quiescence(board, alpha, beta, ply);
        }

        let mut moves = MoveList::new();
        generate_moves(board, &mut moves);

        if moves.is_empty() {
            // No legal moves — in Benedict chess this is extremely rare
            return 0; // draw
        }

        // Move ordering
        self.score_moves(board, &mut moves, tt_move, ply);

        let mut best_move = Move::NULL;
        let mut best_score = -MATE_SCORE - 1;
        let mut bound = Bound::Upper;

        for i in 0..moves.len() {
            // Selection sort: pick best move
            self.pick_move(&mut moves, i);
            let m = moves.get(i);

            let undo = board.make_move(m);
            self.nodes += 1;

            // Check if this flipped the enemy king
            let them = board.side_to_move;
            if board.king_flipped(&undo, them) {
                board.unmake_move(m, &undo);
                let score = MATE_SCORE - ply as i32;
                self.tt.store(board.hash, m, score, depth, Bound::Exact);
                self.pv_table[ply][0] = m;
                self.pv_length[ply] = 1;
                return score;
            }

            // Late Move Reductions: search later moves at reduced depth
            let mut search_depth = depth - 1;
            if i >= 4 && depth >= 3 && !m.is_promotion() && undo.flipped.is_empty() {
                // Reduce by 1 for quiet late moves
                search_depth -= 1;
            }

            let mut score = -self.alpha_beta(board, search_depth, -beta, -alpha, ply + 1);

            // Re-search at full depth if reduced search improved alpha
            if search_depth < depth - 1 && score > alpha {
                score = -self.alpha_beta(board, depth - 1, -beta, -alpha, ply + 1);
            }

            board.unmake_move(m, &undo);

            if self.stopped {
                return 0;
            }

            if score > best_score {
                best_score = score;
                best_move = m;

                if score > alpha {
                    alpha = score;
                    bound = Bound::Exact;

                    // Update PV
                    self.pv_table[ply][0] = m;
                    if ply + 1 < MAX_PLY {
                        for j in 0..self.pv_length[ply + 1] {
                            self.pv_table[ply][j + 1] = self.pv_table[ply + 1][j];
                        }
                        self.pv_length[ply] = self.pv_length[ply + 1] + 1;
                    }

                    if score >= beta {
                        bound = Bound::Lower;

                        // Store killer
                        let piece = board.piece_at(m.from_sq());
                        if piece.is_some() {
                            self.killers[ply][1] = self.killers[ply][0];
                            self.killers[ply][0] = m;

                            // History heuristic
                            if let Some(p) = piece {
                                let h = &mut self.history[p.color.index()][p.kind.index()]
                                    [m.to_sq().index()];
                                *h += depth * depth;
                            }
                        }
                        break;
                    }
                }
            }
        }

        if !best_move.is_null() {
            self.tt.store(board.hash, best_move, best_score, depth, bound);
        }

        best_score
    }

    fn quiescence(&mut self, board: &mut Board, alpha: i32, beta: i32, ply: usize) -> i32 {
        self.nodes += 1;
        let _ = (alpha, beta, ply);
        evaluate_with_params(board, &self.eval_params)
    }

    fn score_moves(&mut self, board: &Board, moves: &MoveList, tt_move: Move, ply: usize) {
        for i in 0..moves.len() {
            let m = moves.get(i);
            self.move_scores[i] = self.move_score(board, m, tt_move, ply);
        }
    }

    fn pick_move(&mut self, moves: &mut MoveList, start: usize) {
        let mut best_idx = start;
        let mut best_score = self.move_scores[start];
        for i in (start + 1)..moves.len() {
            if self.move_scores[i] > best_score {
                best_score = self.move_scores[i];
                best_idx = i;
            }
        }
        if best_idx != start {
            moves.swap(start, best_idx);
            self.move_scores.swap(start, best_idx);
        }
    }

    fn move_score(&self, board: &Board, m: Move, tt_move: Move, ply: usize) -> i32 {
        if m == tt_move {
            return 10_000_000;
        }

        let mut score = 0i32;

        if let Some(piece) = board.piece_at(m.from_sq()) {
            // Killer moves
            if self.killers[ply][0] == m {
                score += 900_000;
            } else if self.killers[ply][1] == m {
                score += 800_000;
            }

            // Estimate flip value: what enemy pieces would this move attack?
            let t = crate::tables::tables();
            let to = m.to_sq();
            let occ = board.occupied; // approximate — piece hasn't moved yet
            let enemy = board.colors[piece.color.flip().index()];

            let kind = if m.is_promotion() {
                m.promotion().unwrap_or(piece.kind)
            } else {
                piece.kind
            };

            let attacks = match kind {
                PieceKind::Pawn => t.pawn_attacks(piece.color, to),
                PieceKind::Knight => t.knight_attacks(to),
                PieceKind::Bishop => t.bishop_attacks(to, occ),
                PieceKind::Rook => t.rook_attacks(to, occ),
                PieceKind::Queen => t.queen_attacks(to, occ),
                PieceKind::King => t.king_attacks(to),
            };

            let flipped = attacks & enemy;
            // Score by value of pieces that would be flipped
            for sq in flipped {
                if let Some(target) = board.piece_at(sq) {
                    score += match target.kind {
                        PieceKind::King => 500_000,
                        PieceKind::Queen => 90_000,
                        PieceKind::Rook => 50_000,
                        PieceKind::Bishop => 33_000,
                        PieceKind::Knight => 32_000,
                        PieceKind::Pawn => 10_000,
                    };
                }
            }

            // History heuristic
            score += self.history[piece.color.index()][piece.kind.index()][m.to_sq().index()];
        }

        score
    }
}
