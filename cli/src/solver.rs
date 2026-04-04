/// Exhaustive solver: builds a complete opening book for 1.e3
/// Fast version: checks mate-in-1 and mate-in-2 without engine search,
/// only falls back to engine for deeper positions.
use benedict_engine::board::Board;
use benedict_engine::movegen::generate_moves;
use benedict_engine::moves::{Move, MoveList};
use benedict_engine::search::{SharedSearch, ThreadSearcher, MATE_SCORE};
use benedict_engine::eval::EvalParams;
use benedict_engine::types::Color;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

struct Solver {
    book: HashMap<u64, (Move, i32)>,
    shared: Arc<SharedSearch>,
    params: EvalParams,
    stats_instant: u64,
    stats_searched: u64,
    start: Instant,
}

impl Solver {
    fn new() -> Self {
        Solver {
            book: HashMap::new(),
            shared: Arc::new(SharedSearch::new(256)),
            params: EvalParams::default(),
            stats_instant: 0,
            stats_searched: 0,
            start: Instant::now(),
        }
    }

    /// Check all moves for one that flips the opponent's king.
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

    /// Find a move that leads to mate-in-1 for the next mover.
    /// (i.e., after this move, the opponent has a mate-in-1 in all responses... no,
    /// we need: after this move, WE have mate-in-1 regardless of opponent's response.)
    /// Actually for White: find a move such that after Black's ANY response, White has mate-in-1.
    fn find_mate_in_2_white(&self, board: &mut Board, moves: &MoveList) -> Option<Move> {
        for i in 0..moves.len() {
            let m = moves.get(i);
            let undo = board.make_move(m);
            let them = board.side_to_move;
            if board.king_flipped(&undo, them) {
                board.unmake_move(m, &undo);
                return Some(m); // mate in 1, even better
            }

            // After White's move, check ALL Black responses
            let mut black_moves = MoveList::new();
            generate_moves(board, &mut black_moves);
            let mut all_mate = true;

            if black_moves.is_empty() {
                all_mate = false;
            } else {
                for j in 0..black_moves.len() {
                    let bm = black_moves.get(j);
                    let bundo = board.make_move(bm);
                    let bthem = board.side_to_move;
                    if board.king_flipped(&bundo, bthem) {
                        // Black flipped White's king — skip
                        board.unmake_move(bm, &bundo);
                        all_mate = false;
                        break;
                    }
                    // Now White to move — check for mate-in-1
                    let mut w2_moves = MoveList::new();
                    generate_moves(board, &mut w2_moves);
                    let has_mate = self.find_mate_in_1(board, &w2_moves).is_some();
                    board.unmake_move(bm, &bundo);
                    if !has_mate {
                        all_mate = false;
                        break;
                    }
                }
            }
            board.unmake_move(m, &undo);
            if all_mate {
                return Some(m);
            }
        }
        None
    }

    fn solve(
        &mut self,
        board: &mut Board,
        max_depth: i32,
        ply: usize,
        history: &mut Vec<u64>,
    ) -> Option<i32> {
        if let Some(&(_, d)) = self.book.get(&board.hash) {
            return Some(d);
        }
        if max_depth <= 0 {
            return None;
        }
        // Repetition
        if history[..history.len().saturating_sub(1)].iter().filter(|&&h| h == board.hash).count() >= 1 {
            return None;
        }

        let mut moves = MoveList::new();
        generate_moves(board, &mut moves);
        if moves.is_empty() {
            return None;
        }

        let is_white = board.side_to_move == Color::White;

        if is_white {
            // --- WHITE: find fastest mate ---

            // Mate in 1? (instant)
            if let Some(m) = self.find_mate_in_1(board, &moves) {
                self.stats_instant += 1;
                self.book.insert(board.hash, (m, 1));
                return Some(1);
            }

            // Mate in 2? (fast: ~20*20 = 400 move checks, no engine)
            if let Some(m) = self.find_mate_in_2_white(board, &moves) {
                self.stats_instant += 1;
                self.book.insert(board.hash, (m, 3)); // 3 half-moves: W-B-W#
                // Still need to recurse for the book entries of sub-positions
                let undo = board.make_move(m);
                history.push(board.hash);
                let _ = self.solve(board, max_depth - 1, ply + 1, history);
                history.pop();
                board.unmake_move(m, &undo);
                return Some(3);
            }

            // Engine search
            self.stats_searched += 1;
            let mut searcher = ThreadSearcher::with_params(
                Arc::clone(&self.shared), self.params.clone(),
            );
            searcher.set_position_history(history.to_vec());
            searcher.silent = true;
            let info = searcher.search(board, 12, Some(Duration::from_secs(2)));

            if info.best_move.is_null() {
                return None;
            }
            let best = info.best_move;
            let undo = board.make_move(best);

            // Check immediate mate
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
                return Some(d + 1);
            }
            None
        } else {
            // --- BLACK: ALL moves must lead to mate ---
            let mut longest: Option<i32> = None;
            let mut longest_move = Move::NULL;

            for i in 0..moves.len() {
                let m = moves.get(i);
                let undo = board.make_move(m);
                let them = board.side_to_move;
                if board.king_flipped(&undo, them) {
                    board.unmake_move(m, &undo);
                    return None; // Black wins — not forced for White
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
                        if ply <= 4 {
                            eprintln!(
                                "  [{:.1}s] ply {} UNSOLVED after {}",
                                self.start.elapsed().as_secs_f64(), ply, m.to_uci()
                            );
                        }
                        return None;
                    }
                }
            }

            if let Some(d) = longest {
                let total = d + 1;
                self.book.insert(board.hash, (longest_move, total));
                if ply <= 2 {
                    eprintln!(
                        "  [{:.1}s] ply {} solved: {} (mate in {} half-moves) [{} book, {} instant, {} searched]",
                        self.start.elapsed().as_secs_f64(), ply, longest_move.to_uci(),
                        total, self.book.len(), self.stats_instant, self.stats_searched
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

    let e3 = Move::from_uci("e2e3").unwrap();
    let mut legal = MoveList::new();
    generate_moves(&board, &mut legal);
    let e3_legal = (0..legal.len())
        .map(|i| legal.get(i))
        .find(|m| m.from_sq() == e3.from_sq() && m.to_sq() == e3.to_sq())
        .expect("e2e3 not legal");
    let undo_e3 = board.make_move(e3_legal);

    eprintln!("=== Benedict Chess Solver ===");
    eprintln!("Exhaustively solving all lines from 1.e3\n");

    let mut history = vec![Board::startpos().hash, board.hash];
    let result = solver.solve(&mut board, 40, 0, &mut history);
    board.unmake_move(e3_legal, &undo_e3);

    if let Some(d) = result {
        solver.book.insert(board.hash, (e3_legal, d + 1));
    }

    eprintln!("\n=== RESULTS ===");
    eprintln!("Book entries:      {}", solver.book.len());
    eprintln!("Instant (no search): {}", solver.stats_instant);
    eprintln!("Engine searches:   {}", solver.stats_searched);
    eprintln!("Time: {:.1}s", solver.start.elapsed().as_secs_f64());
    match result {
        Some(d) => eprintln!("SOLVED: 1.e3 forces mate in {} half-moves", d),
        None => eprintln!("INCOMPLETE: some lines unsolved"),
    }

    // Output book
    println!("use crate::moves::Move;");
    println!("use std::collections::HashMap;");
    println!("use std::sync::OnceLock;");
    println!();
    println!("/// Complete solution book for 1.e3 — every possible Black response covered.");
    println!("/// Auto-generated by the solver. {} positions.", solver.book.len());
    println!("const BOOK_DATA: &[(u64, &str)] = &[");
    let mut entries: Vec<_> = solver.book.iter().collect();
    entries.sort_by_key(|&(_, &(_, d))| std::cmp::Reverse(d));
    for (&hash, &(mv, d)) in &entries {
        println!("    (0x{:016x}, \"{}\"),   // mate in {} half-moves", hash, mv.to_uci(), d);
    }
    println!("];");
    println!();
    println!("static BOOK: OnceLock<HashMap<u64, Move>> = OnceLock::new();");
    println!();
    println!("fn init_book() -> HashMap<u64, Move> {{");
    println!("    let mut map = HashMap::with_capacity(BOOK_DATA.len());");
    println!("    for &(hash, uci) in BOOK_DATA {{");
    println!("        if let Some(m) = Move::from_uci(uci) {{");
    println!("            map.insert(hash, m);");
    println!("        }}");
    println!("    }}");
    println!("    map");
    println!("}}");
    println!();
    println!("pub fn probe(hash: u64) -> Option<Move> {{");
    println!("    let book = BOOK.get_or_init(init_book);");
    println!("    book.get(&hash).copied()");
    println!("}}");
}
