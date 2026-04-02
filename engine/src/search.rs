use crate::board::Board;
use crate::eval::{evaluate_with_params, EvalParams};
use crate::movegen::generate_moves;
use crate::moves::{Move, MoveList};
use crate::tt::{Bound, TranspositionTable};
use crate::types::PieceKind;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
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

/// State shared across all search threads: the transposition table and stop flag.
pub struct SharedSearch {
    pub tt: TranspositionTable,
    pub stop_flag: AtomicBool,
}

impl SharedSearch {
    pub fn new(tt_size_mb: usize) -> Self {
        SharedSearch {
            tt: TranspositionTable::new(tt_size_mb),
            stop_flag: AtomicBool::new(false),
        }
    }
}

/// Per-thread search state. Each thread gets its own ThreadSearcher
/// but shares a single TranspositionTable via Arc<SharedSearch>.
pub struct ThreadSearcher {
    pub shared: Arc<SharedSearch>,
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

impl ThreadSearcher {
    pub fn new(shared: Arc<SharedSearch>) -> Self {
        ThreadSearcher {
            shared,
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

    pub fn with_params(shared: Arc<SharedSearch>, eval_params: EvalParams) -> Self {
        let mut s = Self::new(shared);
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

        let mut best_move = Move::NULL;
        let mut best_score = 0;
        let mut best_pv = Vec::new();

        let mut asp_window = 50;

        for depth in 1..=max_depth {
            self.pv_length[0] = 0;

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
                if score <= alpha {
                    alpha = -MATE_SCORE - 1;
                    asp_window *= 2;
                } else if score >= beta {
                    beta = MATE_SCORE + 1;
                    asp_window *= 2;
                } else {
                    asp_window = 50;
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
            // Check external stop flag
            if self.shared.stop_flag.load(Ordering::Relaxed) {
                self.stopped = true;
                return;
            }
            if let Some(limit) = self.time_limit {
                if self.start_time.elapsed() >= limit {
                    self.stopped = true;
                }
            }
        }
    }

    fn is_repetition(&self, hash: u64) -> bool {
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

        if ply > 0 && self.is_repetition(board.hash) {
            return 0;
        }

        // Probe TT
        let mut tt_move = Move::NULL;
        if let Some(entry) = self.shared.tt.probe(board.hash) {
            tt_move = entry.best_move;
            if entry.depth as i32 >= depth {
                let tt_score = entry.score as i32;
                let can_cutoff = match entry.bound {
                    Bound::Exact => true,
                    Bound::Lower => tt_score >= beta,
                    Bound::Upper => tt_score <= alpha,
                };
                if can_cutoff {
                    if !tt_move.is_null() {
                        self.pv_table[ply][0] = tt_move;
                        self.pv_length[ply] = 1;
                    }
                    return tt_score;
                }
            }
        }

        if depth <= 0 {
            return self.quiescence(board, alpha, beta, ply);
        }

        let mut moves = MoveList::new();
        generate_moves(board, &mut moves);

        if moves.is_empty() {
            return 0;
        }

        self.score_moves(board, &mut moves, tt_move, ply);

        let mut best_move = Move::NULL;
        let mut best_score = -MATE_SCORE - 1;
        let mut bound = Bound::Upper;

        for i in 0..moves.len() {
            self.pick_move(&mut moves, i);
            let m = moves.get(i);

            let undo = board.make_move(m);
            self.nodes += 1;

            let them = board.side_to_move;
            if board.king_flipped(&undo, them) {
                board.unmake_move(m, &undo);
                let score = MATE_SCORE - ply as i32;
                self.shared.tt.store(board.hash, m, score, depth, Bound::Exact);
                self.pv_table[ply][0] = m;
                self.pv_length[ply] = 1;
                return score;
            }

            let mut search_depth = depth - 1;
            if i >= 4 && depth >= 3 && !m.is_promotion() && undo.flipped.is_empty() {
                search_depth -= 1;
            }

            let mut score = -self.alpha_beta(board, search_depth, -beta, -alpha, ply + 1);

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

                    self.pv_table[ply][0] = m;
                    if ply + 1 < MAX_PLY {
                        for j in 0..self.pv_length[ply + 1] {
                            self.pv_table[ply][j + 1] = self.pv_table[ply + 1][j];
                        }
                        self.pv_length[ply] = self.pv_length[ply + 1] + 1;
                    }

                    if score >= beta {
                        bound = Bound::Lower;

                        let piece = board.piece_at(m.from_sq());
                        if piece.is_some() {
                            self.killers[ply][1] = self.killers[ply][0];
                            self.killers[ply][0] = m;

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
            self.shared.tt.store(board.hash, best_move, best_score, depth, bound);
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
            if self.killers[ply][0] == m {
                score += 900_000;
            } else if self.killers[ply][1] == m {
                score += 800_000;
            }

            let t = crate::tables::tables();
            let to = m.to_sq();
            let occ = board.occupied;
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

            score += self.history[piece.color.index()][piece.kind.index()][m.to_sq().index()];
        }

        score
    }
}

/// Run a Lazy SMP search: spawn `num_threads` threads all searching
/// the same position, sharing a transposition table.
/// The main thread (thread 0) prints info lines; helpers are silent.
/// Returns the main thread's result.
pub fn search_smp(
    shared: &Arc<SharedSearch>,
    board: &Board,
    max_depth: i32,
    time_limit: Option<Duration>,
    num_threads: usize,
    eval_params: &EvalParams,
    position_history: Vec<u64>,
    silent: bool,
) -> SearchInfo {
    shared.stop_flag.store(false, Ordering::Relaxed);
    shared.tt.new_generation();

    let num_threads = num_threads.max(1);

    if num_threads == 1 {
        // Single-threaded fast path — no thread spawning overhead
        let mut searcher = ThreadSearcher::with_params(Arc::clone(shared), eval_params.clone());
        searcher.silent = silent;
        searcher.set_position_history(position_history);
        return searcher.search(&mut board.clone(), max_depth, time_limit);
    }

    // Spawn helper threads (1..N-1)
    let mut handles = Vec::with_capacity(num_threads - 1);
    for _thread_id in 1..num_threads {
        let shared = Arc::clone(shared);
        let eval_params = eval_params.clone();
        let mut board = board.clone();
        let position_history = position_history.clone();

        handles.push(std::thread::spawn(move || {
            let mut searcher = ThreadSearcher::with_params(shared, eval_params);
            searcher.silent = true; // only main thread prints
            searcher.set_position_history(position_history);
            searcher.search(&mut board, max_depth, time_limit)
        }));
    }

    // Main thread search
    let mut main_searcher = ThreadSearcher::with_params(Arc::clone(shared), eval_params.clone());
    main_searcher.silent = silent;
    main_searcher.set_position_history(position_history);
    let main_result = main_searcher.search(&mut board.clone(), max_depth, time_limit);

    // Signal helpers to stop and join
    shared.stop_flag.store(true, Ordering::Relaxed);
    let mut total_nodes = main_result.nodes;
    for handle in handles {
        if let Ok(info) = handle.join() {
            total_nodes += info.nodes;
        }
    }

    SearchInfo {
        nodes: total_nodes,
        ..main_result
    }
}

// ---- Backward-compatible Searcher wrapper ----
// Keeps the old API working for match_runner and simple use cases.

pub struct Searcher {
    pub shared: Arc<SharedSearch>,
    pub eval_params: EvalParams,
    pub silent: bool,
    position_history: Vec<u64>,
}

impl Searcher {
    pub fn new(tt_size_mb: usize) -> Self {
        Searcher {
            shared: Arc::new(SharedSearch::new(tt_size_mb)),
            eval_params: EvalParams::default(),
            silent: false,
            position_history: Vec::new(),
        }
    }

    pub fn with_params(tt_size_mb: usize, eval_params: EvalParams) -> Self {
        let mut s = Self::new(tt_size_mb);
        s.eval_params = eval_params;
        s
    }

    pub fn set_position_history(&mut self, hashes: Vec<u64>) {
        self.position_history = hashes;
    }

    /// Convenience accessor so existing code that does `searcher.tt.clear()` still works.
    pub fn tt(&self) -> &TranspositionTable {
        &self.shared.tt
    }

    pub fn search(&mut self, board: &mut Board, max_depth: i32, time_limit: Option<Duration>) -> SearchInfo {
        self.shared.tt.new_generation();
        let mut searcher = ThreadSearcher::with_params(Arc::clone(&self.shared), self.eval_params.clone());
        searcher.silent = self.silent;
        searcher.set_position_history(self.position_history.clone());
        searcher.search(board, max_depth, time_limit)
    }
}
