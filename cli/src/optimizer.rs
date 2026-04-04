/// Tree optimizer: finds shorter White moves to replace deep branches.
///
/// Walks the book tree, measures the depth-to-mate for each White position,
/// and when it finds deep branches (40+ plies), tries ALL legal White moves
/// to find one that leads to a shallower mate. Outputs replacement entries.
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

struct Optimizer {
    /// Cache: hash -> depth to mate (half-moves)
    depth_cache: HashMap<u64, Option<u32>>,
    shared: Arc<SharedSearch>,
    params: EvalParams,
    start: Instant,
    improvements: Vec<(u64, String, u32, String, u32)>, // hash, old_move, old_depth, new_move, new_depth
}

impl Optimizer {
    fn new() -> Self {
        Optimizer {
            depth_cache: HashMap::new(),
            shared: Arc::new(SharedSearch::new(256)),
            params: EvalParams::default(),
            start: Instant::now(),
            improvements: Vec::new(),
        }
    }

    /// Measure the depth-to-mate from a position using the book.
    /// Returns None if the book doesn't cover this position or hits a cycle.
    fn measure_depth(
        &mut self,
        board: &mut Board,
        max_depth: u32,
        history: &mut Vec<u64>,
    ) -> Option<u32> {
        if let Some(&cached) = self.depth_cache.get(&board.hash) {
            return cached;
        }

        if max_depth == 0 {
            return None;
        }

        // Cycle detection
        if history[..history.len().saturating_sub(1)].iter().any(|&h| h == board.hash) {
            return None;
        }

        let is_white = board.side_to_move == Color::White;
        let mut moves = MoveList::new();
        generate_moves(board, &mut moves);

        if moves.is_empty() {
            return None;
        }

        if is_white {
            // Use book move
            let book_move = book::probe(board.hash)?;
            let legal = (0..moves.len())
                .map(|i| moves.get(i))
                .find(|m| m.from_sq() == book_move.from_sq() && m.to_sq() == book_move.to_sq())?;

            let undo = board.make_move(legal);
            let them = board.side_to_move;
            if board.king_flipped(&undo, them) {
                board.unmake_move(legal, &undo);
                self.depth_cache.insert(board.hash, Some(1));
                return Some(1);
            }

            history.push(board.hash);
            let sub = self.measure_depth(board, max_depth - 1, history);
            history.pop();
            board.unmake_move(legal, &undo);

            let result = sub.map(|d| d + 1);
            self.depth_cache.insert(board.hash, result);
            result
        } else {
            // Black: find the LONGEST (worst case) sub-tree
            let mut worst: Option<u32> = None;
            for i in 0..moves.len() {
                let m = moves.get(i);
                let undo = board.make_move(m);
                let them = board.side_to_move;
                if board.king_flipped(&undo, them) {
                    board.unmake_move(m, &undo);
                    return None; // Black wins — shouldn't happen
                }

                history.push(board.hash);
                let sub = self.measure_depth(board, max_depth - 1, history);
                history.pop();
                board.unmake_move(m, &undo);

                match sub {
                    Some(d) => {
                        worst = Some(worst.map_or(d, |w: u32| w.max(d)));
                    }
                    None => {
                        // Can't measure this branch — return None
                        self.depth_cache.insert(board.hash, None);
                        return None;
                    }
                }
            }

            let result = worst.map(|d| d + 1);
            self.depth_cache.insert(board.hash, result);
            result
        }
    }

    /// For a White-to-move position, try ALL legal moves and measure which
    /// leads to the shortest forced mate.
    fn find_shortest_mate(
        &mut self,
        board: &mut Board,
        history: &mut Vec<u64>,
    ) -> Option<(Move, u32)> {
        let mut moves = MoveList::new();
        generate_moves(board, &mut moves);

        let mut best: Option<(Move, u32)> = None;

        for i in 0..moves.len() {
            let m = moves.get(i);
            let undo = board.make_move(m);
            let them = board.side_to_move;

            if board.king_flipped(&undo, them) {
                board.unmake_move(m, &undo);
                return Some((m, 1)); // Mate in 1 — can't do better
            }

            // Try to measure the sub-tree depth
            // Use a limited depth to avoid exploring huge trees
            history.push(board.hash);
            // Clear cache for this exploration since we're trying different moves
            let sub = self.measure_depth(board, 30, history);
            history.pop();
            board.unmake_move(m, &undo);

            if let Some(d) = sub {
                let total = d + 1;
                if best.is_none() || total < best.unwrap().1 {
                    best = Some((m, total));
                }
            }
        }

        // If no move leads to a measurable mate, try engine search
        if best.is_none() {
            let mut searcher = ThreadSearcher::with_params(
                Arc::clone(&self.shared), self.params.clone(),
            );
            searcher.set_position_history(history.to_vec());
            searcher.silent = true;
            let info = searcher.search(board, 14, Some(Duration::from_secs(3)));
            if !info.best_move.is_null() && info.score >= MATE_SCORE - 200 {
                // Engine found mate but we can't measure exact depth
                // Use engine's mate distance estimate
                let mate_dist = (MATE_SCORE - info.score) as u32;
                best = Some((info.best_move, mate_dist.max(2)));
            }
        }

        best
    }

    /// Walk the tree and find positions where shorter mates exist.
    fn optimize(
        &mut self,
        board: &mut Board,
        depth: u32,
        history: &mut Vec<u64>,
    ) {
        if depth == 0 {
            return;
        }
        if history[..history.len().saturating_sub(1)].iter().any(|&h| h == board.hash) {
            return;
        }

        let is_white = board.side_to_move == Color::White;
        let mut moves = MoveList::new();
        generate_moves(board, &mut moves);

        if is_white {
            let book_move = match book::probe(board.hash) {
                Some(m) => m,
                None => return,
            };

            let legal = match (0..moves.len())
                .map(|i| moves.get(i))
                .find(|m| m.from_sq() == book_move.from_sq() && m.to_sq() == book_move.to_sq())
            {
                Some(m) => m,
                None => return,
            };

            // Measure current depth
            let current_depth = self.measure_depth(board, 80, history);

            if let Some(cd) = current_depth {
                if cd >= 30 {
                    // This position has a deep sub-tree. Try to find a shorter mate.
                    eprintln!(
                        "  [{:.1}s] Optimizing: hash=0x{:016x} current_depth={} move={}",
                        self.start.elapsed().as_secs_f64(),
                        board.hash, cd, book_move.to_uci()
                    );

                    // Clear depth cache before exploring alternatives
                    // (the cache has depths assuming the CURRENT book moves)
                    let saved_cache = self.depth_cache.clone();

                    if let Some((better_move, better_depth)) = self.find_shortest_mate(board, history) {
                        if better_depth < cd {
                            eprintln!(
                                "    IMPROVEMENT: {} (depth {}) -> {} (depth {})",
                                book_move.to_uci(), cd,
                                better_move.to_uci(), better_depth
                            );
                            self.improvements.push((
                                board.hash,
                                book_move.to_uci(),
                                cd,
                                better_move.to_uci(),
                                better_depth,
                            ));
                        }
                    }

                    // Restore cache
                    self.depth_cache = saved_cache;
                }
            }

            // Recurse into the current book line
            let undo = board.make_move(legal);
            let them = board.side_to_move;
            if !board.king_flipped(&undo, them) {
                history.push(board.hash);
                self.optimize(board, depth - 1, history);
                history.pop();
            }
            board.unmake_move(legal, &undo);
        } else {
            // Black: recurse into all moves
            for i in 0..moves.len() {
                let m = moves.get(i);
                let undo = board.make_move(m);
                let them = board.side_to_move;
                if !board.king_flipped(&undo, them) {
                    history.push(board.hash);
                    self.optimize(board, depth - 1, history);
                    history.pop();
                }
                board.unmake_move(m, &undo);
            }
        }
    }
}

fn main() {
    benedict_engine::tables::tables();

    let mut opt = Optimizer::new();
    let mut board = Board::startpos();

    let e3 = Move::from_uci("e2e3").unwrap();
    let mut moves = MoveList::new();
    generate_moves(&board, &mut moves);
    let e3_legal = (0..moves.len())
        .map(|i| moves.get(i))
        .find(|m| m.from_sq() == e3.from_sq() && m.to_sq() == e3.to_sq())
        .unwrap();
    let undo = board.make_move(e3_legal);

    eprintln!("=== Benedict Chess Tree Optimizer ===");
    eprintln!("Finding shorter mates to replace deep branches\n");

    let mut history = vec![Board::startpos().hash, board.hash];
    opt.optimize(&mut board, 40, &mut history);
    board.unmake_move(e3_legal, &undo);

    eprintln!("\n=== RESULTS ===");
    eprintln!("Improvements found: {}", opt.improvements.len());
    eprintln!("Time: {:.1}s", opt.start.elapsed().as_secs_f64());

    if opt.improvements.is_empty() {
        println!("No improvements found — all book moves are already optimal within depth 30.");
    } else {
        println!("// Tree optimizations: shorter mates replacing deep branches");
        println!("// Format: (hash, new_move) — replaces the existing book entry");
        for (hash, old_move, old_depth, new_move, new_depth) in &opt.improvements {
            println!(
                "    (0x{:016x}, \"{}\"),   // was {} (depth {}) -> {} (depth {})",
                hash, new_move, old_move, old_depth, new_move, new_depth
            );
        }
    }
}
