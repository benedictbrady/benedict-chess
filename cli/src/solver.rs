/// Exhaustive solver: builds a complete opening book for 1.e3
/// by recursively exploring ALL Black responses at every position
/// and finding White's best reply, until mate is reached.
use benedict_engine::board::Board;
use benedict_engine::movegen::generate_moves;
use benedict_engine::moves::{Move, MoveList};
use benedict_engine::search::{SharedSearch, ThreadSearcher, MATE_SCORE};
use benedict_engine::eval::EvalParams;
use benedict_engine::types::Color;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

struct Solver {
    book: HashMap<u64, (Move, i32)>,
    shared: Arc<SharedSearch>,
    params: EvalParams,
    positions_solved: u64,
    positions_searched: u64,
}

impl Solver {
    fn new() -> Self {
        Solver {
            book: HashMap::new(),
            shared: Arc::new(SharedSearch::new(256)),
            params: EvalParams::default(),
            positions_solved: 0,
            positions_searched: 0,
        }
    }

    /// Find a mating move among legal moves (instant check).
    fn find_mate_in_1(&self, board: &mut Board, moves: &MoveList) -> Option<Move> {
        for i in 0..moves.len() {
            let m = moves.get(i);
            let undo = board.make_move(m);
            let them = board.side_to_move;
            let is_mate = board.king_flipped(&undo, them);
            board.unmake_move(m, &undo);
            if is_mate {
                return Some(m);
            }
        }
        None
    }

    /// Use engine search to find best move. Adaptive depth: start shallow, go deeper if needed.
    fn engine_best_move(&mut self, board: &mut Board, history: &[u64]) -> Option<(Move, i32)> {
        self.positions_searched += 1;

        // Try shallow first (fast for obvious positions)
        for &search_depth in &[8, 12, 16] {
            let mut searcher = ThreadSearcher::with_params(
                Arc::clone(&self.shared),
                self.params.clone(),
            );
            searcher.set_position_history(history.to_vec());
            searcher.silent = true;

            let time_limit = match search_depth {
                8 => Duration::from_millis(500),
                12 => Duration::from_secs(2),
                _ => Duration::from_secs(5),
            };

            let info = searcher.search(board, search_depth, Some(time_limit));
            if info.best_move.is_null() {
                continue;
            }

            // If we found mate, no need to search deeper
            if info.score >= MATE_SCORE - 100 {
                return Some((info.best_move, info.score));
            }

            // At max depth, return whatever we found
            if search_depth >= 16 {
                return Some((info.best_move, info.score));
            }
        }
        None
    }

    /// Recursively solve a position.
    /// Returns Some(half_moves_to_mate) or None if unsolvable within budget.
    fn solve(
        &mut self,
        board: &mut Board,
        max_depth: i32,
        ply: usize,
        history: &mut Vec<u64>,
    ) -> Option<i32> {
        // Already solved?
        if let Some(&(_, mate_in)) = self.book.get(&board.hash) {
            return Some(mate_in);
        }

        if max_depth <= 0 {
            return None;
        }

        // Repetition check
        let rep_count = history.iter().filter(|&&h| h == board.hash).count();
        if rep_count >= 2 {
            return None;
        }

        let mut moves = MoveList::new();
        generate_moves(board, &mut moves);
        if moves.is_empty() {
            return None;
        }

        let is_white = board.side_to_move == Color::White;

        if is_white {
            // --- WHITE TO MOVE: find fastest mate ---

            // 1) Check for mate in 1 (instant, no search needed)
            if let Some(mate_move) = self.find_mate_in_1(board, &moves) {
                self.book.insert(board.hash, (mate_move, 1));
                self.positions_solved += 1;
                return Some(1);
            }

            // 2) Use engine to find best move
            let (best, _score) = self.engine_best_move(board, history)?;

            // 3) Recurse on the best move
            let undo = board.make_move(best);
            history.push(board.hash);
            let result = self.solve(board, max_depth - 1, ply + 1, history);
            history.pop();
            board.unmake_move(best, &undo);

            if let Some(mate_in) = result {
                let total = mate_in + 1;
                self.book.insert(board.hash, (best, total));
                self.positions_solved += 1;
                return Some(total);
            }
            None
        } else {
            // --- BLACK TO MOVE: ALL moves must lead to mate ---
            let mut longest_mate: Option<i32> = None;
            let mut longest_move = Move::NULL;
            let total_moves = moves.len();

            for i in 0..total_moves {
                let m = moves.get(i);
                let undo = board.make_move(m);

                // Check if Black flipped White's king (Black wins — shouldn't happen)
                let them = board.side_to_move;
                if board.king_flipped(&undo, them) {
                    board.unmake_move(m, &undo);
                    // Black can win — this position is NOT a forced White win
                    return None;
                }

                history.push(board.hash);
                let result = self.solve(board, max_depth - 1, ply + 1, history);
                history.pop();
                board.unmake_move(m, &undo);

                match result {
                    Some(mate_in) => {
                        if longest_mate.is_none() || mate_in > longest_mate.unwrap() {
                            longest_mate = Some(mate_in);
                            longest_move = m;
                        }
                    }
                    None => {
                        eprintln!(
                            "  [ply {}] UNSOLVED: Black plays {} (hash 0x{:016x})",
                            ply, m.to_uci(), board.hash
                        );
                        return None;
                    }
                }
            }

            if let Some(mate_in) = longest_mate {
                let total = mate_in + 1;
                self.book.insert(board.hash, (longest_move, total));
                self.positions_solved += 1;
                if ply <= 2 {
                    eprintln!(
                        "  [ply {}] Solved: Black best={} mate_in={} half-moves ({}/{} positions)",
                        ply, longest_move.to_uci(), total, self.positions_solved, self.positions_searched
                    );
                }
                return Some(total);
            }
            None
        }
    }
}

fn main() {
    benedict_engine::tables::tables();

    let mut solver = Solver::new();
    let mut board = Board::startpos();

    // Play 1. e3
    let e3 = Move::from_uci("e2e3").unwrap();
    let mut legal = MoveList::new();
    generate_moves(&board, &mut legal);
    let e3_legal = (0..legal.len())
        .map(|i| legal.get(i))
        .find(|m| m.from_sq() == e3.from_sq() && m.to_sq() == e3.to_sq())
        .expect("e2e3 not legal");
    let undo_e3 = board.make_move(e3_legal);

    eprintln!("=== Benedict Chess Solver ===");
    eprintln!("Solving all lines from 1.e3 (hash 0x{:016x})", board.hash);
    eprintln!("This exhaustively checks every possible Black move at every position.\n");

    let mut history = vec![Board::startpos().hash, board.hash];

    let result = solver.solve(&mut board, 40, 0, &mut history);

    board.unmake_move(e3_legal, &undo_e3);

    // Add startpos entry
    if let Some(mate_in) = result {
        solver.book.insert(board.hash, (e3_legal, mate_in + 1));
    }

    eprintln!("\n=== RESULTS ===");
    eprintln!("Positions solved: {}", solver.positions_solved);
    eprintln!("Engine searches:  {}", solver.positions_searched);
    eprintln!("Book entries:     {}", solver.book.len());
    if let Some(mate_in) = result {
        eprintln!("PROVEN: 1.e3 forces mate in {} half-moves", mate_in);
    } else {
        eprintln!("FAILED: Could not prove forced mate for all lines");
    }

    // Output book
    println!("// Auto-generated complete solution book for 1.e3");
    println!("// {} positions — every possible Black response covered", solver.book.len());
    println!("const BOOK_DATA: &[(u64, &str)] = &[");
    let mut entries: Vec<_> = solver.book.iter().collect();
    entries.sort_by_key(|&(_, &(_, d))| std::cmp::Reverse(d));
    for (&hash, &(mv, d)) in &entries {
        println!("    (0x{:016x}, \"{}\"),   // mate in {} half-moves", hash, mv.to_uci(), d);
    }
    println!("];");
}
