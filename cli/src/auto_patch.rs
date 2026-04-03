/// Auto-patcher: iteratively finds and fills gaps in the opening book
/// by running the verifier logic and adding missing entries until
/// the proof is complete.
use benedict_engine::board::Board;
use benedict_engine::book;
use benedict_engine::movegen::generate_moves;
use benedict_engine::moves::{Move, MoveList};
use benedict_engine::search::{SharedSearch, ThreadSearcher, MATE_SCORE};
use benedict_engine::eval::EvalParams;
use benedict_engine::types::Color;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::{Duration, Instant};

struct AutoPatcher {
    /// New entries to add (hash -> uci move string)
    new_entries: HashMap<u64, String>,
    verified: HashSet<u64>,
    shared: Arc<SharedSearch>,
    params: EvalParams,
    start: Instant,
    mate_checks: u64,
    engine_searches: u64,
}

impl AutoPatcher {
    fn new() -> Self {
        AutoPatcher {
            new_entries: HashMap::new(),
            verified: HashSet::new(),
            shared: Arc::new(SharedSearch::new(256)),
            params: EvalParams::default(),
            start: Instant::now(),
            mate_checks: 0,
            engine_searches: 0,
        }
    }

    fn find_mate_in_1(&mut self, board: &mut Board, moves: &MoveList) -> Option<Move> {
        for i in 0..moves.len() {
            let m = moves.get(i);
            let undo = board.make_move(m);
            let them = board.side_to_move;
            let is_mate = board.king_flipped(&undo, them);
            board.unmake_move(m, &undo);
            self.mate_checks += 1;
            if is_mate {
                return Some(m);
            }
        }
        None
    }

    fn engine_best(&mut self, board: &mut Board, history: &[u64]) -> Option<Move> {
        self.engine_searches += 1;
        let mut searcher = ThreadSearcher::with_params(
            Arc::clone(&self.shared), self.params.clone(),
        );
        searcher.set_position_history(history.to_vec());
        searcher.silent = true;
        let info = searcher.search(board, 14, Some(Duration::from_secs(3)));
        if info.best_move.is_null() { None } else { Some(info.best_move) }
    }

    /// Verify and patch. Returns true if position is proven forced mate.
    fn verify_and_patch(
        &mut self,
        board: &mut Board,
        depth: usize,
        history: &mut Vec<u64>,
    ) -> bool {
        if self.verified.contains(&board.hash) {
            return true;
        }
        if depth > 50 {
            return false;
        }
        // Repetition
        if history[..history.len().saturating_sub(1)].iter().any(|&h| h == board.hash) {
            return false;
        }

        let is_white = board.side_to_move == Color::White;
        let mut moves = MoveList::new();
        generate_moves(board, &mut moves);
        if moves.is_empty() {
            return false;
        }

        if is_white {
            // Try book first
            let best = if let Some(bm) = book::probe(board.hash) {
                bm
            } else if let Some(bm_uci) = self.new_entries.get(&board.hash) {
                Move::from_uci(bm_uci).unwrap()
            } else {
                // No book entry — find one via mate-in-1 check or engine
                if let Some(m) = self.find_mate_in_1(board, &moves) {
                    self.new_entries.insert(board.hash, m.to_uci());
                    m
                } else if let Some(m) = self.engine_best(board, history) {
                    self.new_entries.insert(board.hash, m.to_uci());
                    m
                } else {
                    return false;
                }
            };

            // Find the legal version of this move
            let legal = (0..moves.len())
                .map(|i| moves.get(i))
                .find(|m| m.from_sq() == best.from_sq() && m.to_sq() == best.to_sq());

            let legal = match legal {
                Some(m) => m,
                None => return false,
            };

            let undo = board.make_move(legal);
            let them = board.side_to_move;
            if board.king_flipped(&undo, them) {
                board.unmake_move(legal, &undo);
                self.verified.insert(board.hash);
                return true;
            }

            history.push(board.hash);
            let result = self.verify_and_patch(board, depth + 1, history);
            history.pop();
            board.unmake_move(legal, &undo);

            if result {
                self.verified.insert(board.hash);
            }
            result
        } else {
            // Black: ALL moves must lead to forced mate
            // IMPORTANT: don't bail on first failure — continue to fill other gaps
            let mut all_ok = true;
            for i in 0..moves.len() {
                let m = moves.get(i);
                let undo = board.make_move(m);
                let them = board.side_to_move;
                if board.king_flipped(&undo, them) {
                    board.unmake_move(m, &undo);
                    all_ok = false;
                    continue; // Black wins this branch, but keep checking others
                }

                history.push(board.hash);
                let result = self.verify_and_patch(board, depth + 1, history);
                history.pop();
                board.unmake_move(m, &undo);

                if !result {
                    all_ok = false;
                    // Don't return — keep filling gaps in other branches
                }
            }
            if all_ok {
                self.verified.insert(board.hash);
            }
            all_ok
        }
    }
}

fn main() {
    benedict_engine::tables::tables();
    let mut patcher = AutoPatcher::new();
    let mut board = Board::startpos();

    let e3 = Move::from_uci("e2e3").unwrap();
    let mut moves = MoveList::new();
    generate_moves(&board, &mut moves);
    let e3_legal = (0..moves.len())
        .map(|i| moves.get(i))
        .find(|m| m.from_sq() == e3.from_sq() && m.to_sq() == e3.to_sq())
        .unwrap();
    let undo = board.make_move(e3_legal);

    eprintln!("=== Auto-Patcher: filling gaps in the solution book ===\n");

    let mut history = vec![Board::startpos().hash, board.hash];
    let result = patcher.verify_and_patch(&mut board, 0, &mut history);
    board.unmake_move(e3_legal, &undo);

    eprintln!("\n=== RESULTS ===");
    eprintln!("Verified positions: {}", patcher.verified.len());
    eprintln!("New entries added:  {}", patcher.new_entries.len());
    eprintln!("Mate-in-1 checks:  {}", patcher.mate_checks);
    eprintln!("Engine searches:    {}", patcher.engine_searches);
    eprintln!("Time: {:.1}s", patcher.start.elapsed().as_secs_f64());

    // Always output new entries (even if proof incomplete)
    if !patcher.new_entries.is_empty() {
        println!("// Auto-patched entries ({} new positions)", patcher.new_entries.len());
        for (hash, uci) in &patcher.new_entries {
            println!("    (0x{:016x}, \"{}\"),", hash, uci);
        }
    }

    if result {
        eprintln!("\nPROOF COMPLETE!");
    } else {
        eprintln!("\nINCOMPLETE — {} new entries found, some positions unresolvable", patcher.new_entries.len());
    }
}
