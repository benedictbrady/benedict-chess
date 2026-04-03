/// Targeted solver: patches the a6 gap in the solution book.
/// Runs the same solver logic but starts from 1.e3 a6 with higher limits.
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
            shared: Arc::new(SharedSearch::new(512)),
            params: EvalParams::default(),
            stats_instant: 0,
            stats_searched: 0,
            start: Instant::now(),
        }
    }

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

    fn find_mate_in_2_white(&self, board: &mut Board, moves: &MoveList) -> Option<Move> {
        for i in 0..moves.len() {
            let m = moves.get(i);
            let undo = board.make_move(m);
            let them = board.side_to_move;
            if board.king_flipped(&undo, them) {
                board.unmake_move(m, &undo);
                return Some(m);
            }
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
                        board.unmake_move(bm, &bundo);
                        all_mate = false;
                        break;
                    }
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
        if history[..history.len().saturating_sub(1)].iter().any(|&h| h == board.hash) {
            return None;
        }

        let mut moves = MoveList::new();
        generate_moves(board, &mut moves);
        if moves.is_empty() {
            return None;
        }

        let is_white = board.side_to_move == Color::White;

        if is_white {
            if let Some(m) = self.find_mate_in_1(board, &moves) {
                self.stats_instant += 1;
                self.book.insert(board.hash, (m, 1));
                return Some(1);
            }
            if let Some(m) = self.find_mate_in_2_white(board, &moves) {
                self.stats_instant += 1;
                self.book.insert(board.hash, (m, 3));
                let undo = board.make_move(m);
                history.push(board.hash);
                let _ = self.solve(board, max_depth - 1, ply + 1, history);
                history.pop();
                board.unmake_move(m, &undo);
                return Some(3);
            }

            // Engine search — use higher depth (16) and longer time (5s) for this patch
            self.stats_searched += 1;
            if self.stats_searched % 10 == 0 {
                eprintln!(
                    "  [{:.1}s] searched={} book={} ply={}",
                    self.start.elapsed().as_secs_f64(),
                    self.stats_searched, self.book.len(), ply
                );
            }
            let mut searcher = ThreadSearcher::with_params(
                Arc::clone(&self.shared), self.params.clone(),
            );
            searcher.set_position_history(history.to_vec());
            searcher.silent = true;
            let info = searcher.search(board, 16, Some(Duration::from_secs(5)));

            if info.best_move.is_null() {
                return None;
            }
            let best = info.best_move;
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
                return Some(d + 1);
            }
            None
        } else {
            let mut longest: Option<i32> = None;
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
                        eprintln!(
                            "  [{:.1}s] ply {} UNSOLVED after {} (hash 0x{:016x})",
                            self.start.elapsed().as_secs_f64(), ply, m.to_uci(), board.hash
                        );
                        return None;
                    }
                }
            }

            if let Some(d) = longest {
                let total = d + 1;
                self.book.insert(board.hash, (longest_move, total));
                if ply <= 4 {
                    eprintln!(
                        "  [{:.1}s] ply {} solved: {} (mate in {} half-moves) [book={}, instant={}, searched={}]",
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

    // Play 1.e3 a6
    let mut legal = MoveList::new();
    generate_moves(&board, &mut legal);

    let find_move = |board: &mut Board, uci: &str| -> Move {
        let target = Move::from_uci(uci).unwrap();
        let mut moves = MoveList::new();
        generate_moves(board, &mut moves);
        (0..moves.len())
            .map(|i| moves.get(i))
            .find(|m| m.from_sq() == target.from_sq() && m.to_sq() == target.to_sq())
            .expect(&format!("{} not legal", uci))
    };

    let e3 = find_move(&mut board, "e2e3");
    let undo_e3 = board.make_move(e3);
    let a6 = find_move(&mut board, "a7a6");
    let undo_a6 = board.make_move(a6);

    eprintln!("=== Patch Solver: solving 1.e3 a6 branch ===");
    eprintln!("Position hash: 0x{:016x}\n", board.hash);

    let mut history = vec![Board::startpos().hash];
    // Add the intermediate hashes
    board.unmake_move(a6, &undo_a6);
    history.push(board.hash); // after e3
    board.make_move(a6);
    history.push(board.hash); // after e3 a6

    let result = solver.solve(&mut board, 50, 0, &mut history);

    eprintln!("\n=== RESULTS ===");
    eprintln!("Book entries: {}", solver.book.len());
    eprintln!("Instant: {}", solver.stats_instant);
    eprintln!("Searched: {}", solver.stats_searched);
    eprintln!("Time: {:.1}s", solver.start.elapsed().as_secs_f64());
    match result {
        Some(d) => eprintln!("SOLVED: 1.e3 a6 forces mate in {} half-moves", d),
        None => eprintln!("FAILED"),
    }

    // Output just the new entries as patch data
    println!("// Patch: 1.e3 a6 sub-tree ({} positions)", solver.book.len());
    for (&hash, &(mv, d)) in &solver.book {
        println!("    (0x{:016x}, \"{}\"),   // mate in {} half-moves", hash, mv.to_uci(), d);
    }
}
