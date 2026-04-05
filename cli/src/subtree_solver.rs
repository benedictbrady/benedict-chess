/// Sub-tree solver: builds book entries starting from a specific position.
/// Uses the smart solver's heuristics (instant-mate-ratio + safety + engine).
/// Outputs only the new entries needed for the sub-tree.
use benedict_engine::board::Board;
use benedict_engine::movegen::generate_moves;
use benedict_engine::moves::{Move, MoveList};
use benedict_engine::search::{SharedSearch, ThreadSearcher};
use benedict_engine::eval::EvalParams;
use benedict_engine::types::Color;
use benedict_engine::fen;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

struct SubtreeSolver {
    book: HashMap<u64, (Move, u32)>,
    shared: Arc<SharedSearch>,
    params: EvalParams,
    start: Instant,
    stats_mate1: u64,
    stats_smart: u64,
    stats_engine: u64,
    stats_failed: u64,
}

impl SubtreeSolver {
    fn new() -> Self {
        SubtreeSolver {
            book: HashMap::new(),
            shared: Arc::new(SharedSearch::new(256)),
            params: EvalParams::default(),
            start: Instant::now(),
            stats_mate1: 0,
            stats_smart: 0,
            stats_engine: 0,
            stats_failed: 0,
        }
    }

    fn find_mate_in_1(&self, board: &mut Board, moves: &MoveList) -> Option<Move> {
        for i in 0..moves.len() {
            let m = moves.get(i);
            let undo = board.make_move(m);
            let them = board.side_to_move;
            let is_mate = board.king_flipped(&undo, them);
            board.unmake_move(m, &undo);
            if is_mate { return Some(m); }
        }
        None
    }

    fn is_safe(&self, board: &mut Board, m: Move) -> bool {
        let undo = board.make_move(m);
        let them = board.side_to_move;
        if board.king_flipped(&undo, them) {
            board.unmake_move(m, &undo);
            return true;
        }
        let mut black_moves = MoveList::new();
        generate_moves(board, &mut black_moves);
        for j in 0..black_moves.len() {
            let bm = black_moves.get(j);
            let bundo = board.make_move(bm);
            let bthem = board.side_to_move;
            if board.king_flipped(&bundo, bthem) {
                board.unmake_move(bm, &bundo);
                board.unmake_move(m, &undo);
                return false;
            }
            board.unmake_move(bm, &bundo);
        }
        board.unmake_move(m, &undo);
        true
    }

    fn instant_mate_ratio(&self, board: &mut Board, m: Move) -> u32 {
        let undo = board.make_move(m);
        let them = board.side_to_move;
        if board.king_flipped(&undo, them) {
            board.unmake_move(m, &undo);
            return u32::MAX;
        }
        let mut black_moves = MoveList::new();
        generate_moves(board, &mut black_moves);
        let mut instant = 0u32;
        for j in 0..black_moves.len() {
            let bm = black_moves.get(j);
            let bundo = board.make_move(bm);
            let bthem = board.side_to_move;
            if board.king_flipped(&bundo, bthem) {
                board.unmake_move(bm, &bundo);
                continue;
            }
            let mut w2 = MoveList::new();
            generate_moves(board, &mut w2);
            if self.find_mate_in_1(board, &w2).is_some() {
                instant += 1;
            }
            board.unmake_move(bm, &bundo);
        }
        board.unmake_move(m, &undo);
        instant
    }

    fn find_best_move(&mut self, board: &mut Board, history: &[u64]) -> Option<Move> {
        let mut moves = MoveList::new();
        generate_moves(board, &mut moves);

        if let Some(m) = self.find_mate_in_1(board, &moves) {
            self.stats_mate1 += 1;
            return Some(m);
        }

        let mut best_ratio = 0u32;
        let mut best_move = Move::NULL;
        for i in 0..moves.len() {
            let m = moves.get(i);
            if m.to_uci().ends_with('b') { continue; }
            let undo = board.make_move(m);
            let repeated = history.iter().any(|&h| h == board.hash);
            board.unmake_move(m, &undo);
            if repeated { continue; }
            if !self.is_safe(board, m) { continue; }
            let ratio = self.instant_mate_ratio(board, m);
            if ratio > best_ratio {
                best_ratio = ratio;
                best_move = m;
            }
        }
        if !best_move.is_null() {
            self.stats_smart += 1;
            return Some(best_move);
        }

        let mut searcher = ThreadSearcher::with_params(
            Arc::clone(&self.shared), self.params.clone(),
        );
        searcher.set_position_history(history.to_vec());
        searcher.silent = true;
        let info = searcher.search(board, 20, Some(Duration::from_secs(10)));
        if !info.best_move.is_null() {
            let eng = info.best_move;
            let legal = (0..moves.len())
                .map(|i| moves.get(i))
                .find(|m| m.from_sq() == eng.from_sq() && m.to_sq() == eng.to_sq());
            if let Some(m) = legal {
                // Trust the engine — don't require safety check
                self.stats_engine += 1;
                return Some(m);
            }
        }

        for i in 0..moves.len() {
            let m = moves.get(i);
            if m.to_uci().ends_with('b') { continue; }
            let undo = board.make_move(m);
            let repeated = history.iter().any(|&h| h == board.hash);
            board.unmake_move(m, &undo);
            if repeated { continue; }
            if self.is_safe(board, m) {
                self.stats_engine += 1;
                return Some(m);
            }
        }

        self.stats_failed += 1;
        eprintln!("  FAIL: hash=0x{:016x} FEN={}", board.hash, fen::to_fen(board));
        None
    }

    fn solve(
        &mut self,
        board: &mut Board,
        max_depth: i32,
        ply: usize,
        history: &mut Vec<u64>,
    ) -> Option<u32> {
        if let Some(&(_, d)) = self.book.get(&board.hash) {
            return Some(d);
        }
        if max_depth <= 0 { return None; }
        if history[..history.len().saturating_sub(1)].iter().any(|&h| h == board.hash) {
            return None;
        }

        let is_white = board.side_to_move == Color::White;
        let mut moves = MoveList::new();
        generate_moves(board, &mut moves);
        if moves.is_empty() { return None; }

        if is_white {
            // Also check the existing book
            let book_move = benedict_engine::book::probe(board.hash);
            let best = if let Some(bm) = book_move {
                // Use existing book move
                let legal = (0..moves.len())
                    .map(|i| moves.get(i))
                    .find(|m| m.from_sq() == bm.from_sq() && m.to_sq() == bm.to_sq()
                        && (bm.to_uci().len() <= 4 || m.to_uci() == bm.to_uci()));
                match legal {
                    Some(m) => m,
                    None => self.find_best_move(board, history)?,
                }
            } else {
                self.find_best_move(board, history)?
            };

            let undo = board.make_move(best);
            let them = board.side_to_move;
            if board.king_flipped(&undo, them) {
                board.unmake_move(best, &undo);
                self.book.insert(board.hash, (best, 1));
                return Some(1);
            }
            history.push(board.hash);
            let result = self.solve(board, max_depth - 1, ply + 1, history);
            history.pop();
            board.unmake_move(best, &undo);
            if let Some(d) = result {
                self.book.insert(board.hash, (best, d + 1));
                if ply <= 8 {
                    eprintln!("[{:.1}s] ply {} solved (depth {}) [book={} m1={} smart={} eng={} fail={}]",
                        self.start.elapsed().as_secs_f64(), ply, d+1,
                        self.book.len(), self.stats_mate1, self.stats_smart,
                        self.stats_engine, self.stats_failed);
                }
                return Some(d + 1);
            }
            None
        } else {
            let mut longest: Option<u32> = None;
            let mut longest_move = Move::NULL;
            for i in 0..moves.len() {
                let m = moves.get(i);
                let undo = board.make_move(m);
                let them = board.side_to_move;
                if board.king_flipped(&undo, them) {
                    board.unmake_move(m, &undo);
                    return None;
                }
                history.push(board.hash);
                let result = self.solve(board, max_depth - 1, ply + 1, history);
                history.pop();
                board.unmake_move(m, &undo);
                match result {
                    Some(d) => {
                        if longest.is_none() || d > longest.unwrap() {
                            longest = Some(d);
                            longest_move = m;
                        }
                    }
                    None => {
                        if ply <= 20 {
                            eprintln!("[{:.1}s] ply {} UNSOLVED: {} FEN={}",
                                self.start.elapsed().as_secs_f64(), ply, m.to_uci(),
                                fen::to_fen(board));
                        }
                        return None;
                    }
                }
            }
            if let Some(d) = longest {
                self.book.insert(board.hash, (longest_move, d + 1));
                return Some(d + 1);
            }
            None
        }
    }
}

fn play_uci(board: &mut Board, uci: &str) -> Move {
    let m = Move::from_uci(uci).unwrap();
    let mut moves = MoveList::new();
    generate_moves(board, &mut moves);
    (0..moves.len()).map(|i| moves.get(i))
        .find(|mv| mv.from_sq() == m.from_sq() && mv.to_sq() == m.to_sq())
        .expect(&format!("{} not legal", uci))
}

fn main() {
    let builder = std::thread::Builder::new().stack_size(128 * 1024 * 1024);
    builder.spawn(|| {
        benedict_engine::tables::tables();
        let mut solver = SubtreeSolver::new();
        let mut board = Board::startpos();
        
        // Play to the Qg4 position: 1.e3 a5 2.Nc3 b5 3.Ne4 Bb7 4.Qg4
        let setup_moves = ["e2e3", "a7a5", "b1c3", "b7b5", "c3e4", "c8b7", "d1g4"];
        let mut undos = Vec::new();
        let mut history = vec![board.hash];
        
        for uci in &setup_moves {
            let m = play_uci(&mut board, uci);
            let undo = board.make_move(m);
            history.push(board.hash);
            undos.push((m, undo));
        }

        eprintln!("=== Sub-tree Solver ===");
        eprintln!("Starting from Qg4 position (after 1.e3 a5 2.Nc3 b5 3.Ne4 Bb7 4.Qg4)");
        eprintln!("Hash: 0x{:016x}", board.hash);
        eprintln!("FEN: {}\n", fen::to_fen(&board));

        let result = solver.solve(&mut board, 200, 0, &mut history);

        // Undo setup moves
        for (m, undo) in undos.into_iter().rev() {
            board.unmake_move(m, &undo);
        }

        eprintln!("\n=== RESULTS ===");
        eprintln!("Book: {}", solver.book.len());
        eprintln!("Mate-in-1: {}", solver.stats_mate1);
        eprintln!("Smart: {}", solver.stats_smart);
        eprintln!("Engine: {}", solver.stats_engine);
        eprintln!("Failed: {}", solver.stats_failed);
        eprintln!("Time: {:.1}s", solver.start.elapsed().as_secs_f64());
        match result {
            Some(d) => eprintln!("SOLVED: mate in {} half-moves from Qg4", d),
            None => eprintln!("INCOMPLETE"),
        }

        // Output entries (only new ones not in existing book)
        for (&hash, &(mv, d)) in &solver.book {
            if benedict_engine::book::probe(hash).is_none() {
                println!("    (0x{:016x}, \"{}\"),   // depth {}", hash, mv.to_uci(), d);
            }
        }
    }).unwrap().join().unwrap();
}
