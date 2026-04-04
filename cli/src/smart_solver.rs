/// Smart solver: builds the book preferring moves with highest instant-mate ratio.
/// Instead of just finding any forced mate, it picks the White move that gives
/// the most instant mates among Black's responses, minimizing the sub-tree.
use benedict_engine::board::Board;
use benedict_engine::movegen::generate_moves;
use benedict_engine::moves::{Move, MoveList};
use benedict_engine::search::{SharedSearch, ThreadSearcher, MATE_SCORE};
use benedict_engine::eval::EvalParams;
use benedict_engine::types::Color;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::{Duration, Instant};

struct SmartSolver {
    book: HashMap<u64, (Move, u32)>,
    verified: HashSet<u64>,
    shared: Arc<SharedSearch>,
    params: EvalParams,
    start: Instant,
    stats_mate1: u64,
    stats_smart: u64,
    stats_engine: u64,
}

impl SmartSolver {
    fn new() -> Self {
        SmartSolver {
            book: HashMap::new(),
            verified: HashSet::new(),
            shared: Arc::new(SharedSearch::new(256)),
            params: EvalParams::default(),
            start: Instant::now(),
            stats_mate1: 0,
            stats_smart: 0,
            stats_engine: 0,
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

    /// Score a White move by counting how many Black responses are mate-in-1.
    fn instant_mate_ratio(&self, board: &mut Board, m: Move) -> (u32, u32) {
        let undo = board.make_move(m);
        let them = board.side_to_move;
        if board.king_flipped(&undo, them) {
            board.unmake_move(m, &undo);
            return (1, 1); // The move itself is mate — best possible
        }

        let mut black_moves = MoveList::new();
        generate_moves(board, &mut black_moves);
        let mut instant = 0u32;
        let mut total = 0u32;

        for j in 0..black_moves.len() {
            let bm = black_moves.get(j);
            let bundo = board.make_move(bm);
            let bthem = board.side_to_move;
            if board.king_flipped(&bundo, bthem) {
                board.unmake_move(bm, &bundo);
                continue; // Black wins — skip
            }
            total += 1;
            let mut w2 = MoveList::new();
            generate_moves(board, &mut w2);
            if self.find_mate_in_1(board, &w2).is_some() {
                instant += 1;
            }
            board.unmake_move(bm, &bundo);
        }

        board.unmake_move(m, &undo);
        (instant, total)
    }

    /// Find the White move with the highest instant-mate ratio.
    fn find_best_white_move(&mut self, board: &mut Board, history: &[u64]) -> Option<Move> {
        let mut moves = MoveList::new();
        generate_moves(board, &mut moves);

        // 1) Mate-in-1?
        if let Some(m) = self.find_mate_in_1(board, &moves) {
            self.stats_mate1 += 1;
            return Some(m);
        }

        // 2) Rank by instant-mate ratio
        let mut best: Option<(Move, u32, u32)> = None;
        for i in 0..moves.len() {
            let m = moves.get(i);
            let (instant, total) = self.instant_mate_ratio(board, m);
            if total == 0 { continue; }
            if best.is_none() || instant > best.unwrap().1
                || (instant == best.unwrap().1 && total < best.unwrap().2) {
                best = Some((m, instant, total));
            }
        }

        if let Some((m, inst, tot)) = best {
            if inst > 0 {
                self.stats_smart += 1;
                return Some(m);
            }
        }

        // 3) Fallback to engine
        self.stats_engine += 1;
        let mut searcher = ThreadSearcher::with_params(
            Arc::clone(&self.shared), self.params.clone(),
        );
        searcher.set_position_history(history.to_vec());
        searcher.silent = true;
        let info = searcher.search(board, 14, Some(Duration::from_secs(3)));
        if !info.best_move.is_null() { Some(info.best_move) } else { None }
    }

    fn solve(&mut self, board: &mut Board, max_depth: i32, ply: usize, history: &mut Vec<u64>) -> Option<u32> {
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
            let best = self.find_best_white_move(board, history)?;
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
                if ply <= 4 {
                    eprintln!("  [{:.1}s] ply {} solved: {} (depth {}) [book={} m1={} smart={} eng={}]",
                        self.start.elapsed().as_secs_f64(), ply, best.to_uci(), d+1,
                        self.book.len(), self.stats_mate1, self.stats_smart, self.stats_engine);
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
                        if ply <= 6 {
                            eprintln!("  [{:.1}s] ply {} UNSOLVED: {}", 
                                self.start.elapsed().as_secs_f64(), ply, m.to_uci());
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

fn main() {
    let builder = std::thread::Builder::new().stack_size(64 * 1024 * 1024);
    builder.spawn(|| {
        benedict_engine::tables::tables();
        let mut solver = SmartSolver::new();
        let mut board = Board::startpos();
        let e3 = Move::from_uci("e2e3").unwrap();
        let mut moves = MoveList::new();
        generate_moves(&board, &mut moves);
        let e3_legal = (0..moves.len()).map(|i| moves.get(i))
            .find(|m| m.from_sq() == e3.from_sq() && m.to_sq() == e3.to_sq()).unwrap();
        let undo = board.make_move(e3_legal);

        eprintln!("=== Smart Solver ===");
        eprintln!("Preferring highest instant-mate-ratio moves\n");

        let mut history = vec![Board::startpos().hash, board.hash];
        let result = solver.solve(&mut board, 200, 0, &mut history);
        board.unmake_move(e3_legal, &undo);

        if let Some(d) = result {
            solver.book.insert(board.hash, (e3_legal, d + 1));
        }

        eprintln!("\n=== RESULTS ===");
        eprintln!("Book: {}", solver.book.len());
        eprintln!("Mate-in-1: {}", solver.stats_mate1);
        eprintln!("Smart picks: {}", solver.stats_smart);
        eprintln!("Engine fallback: {}", solver.stats_engine);
        eprintln!("Time: {:.1}s", solver.start.elapsed().as_secs_f64());
        match result {
            Some(d) => eprintln!("SOLVED: mate in {} half-moves", d),
            None => eprintln!("INCOMPLETE"),
        }

        // Output book
        println!("use crate::moves::Move;");
        println!("use std::collections::HashMap;");
        println!("use std::sync::OnceLock;");
        println!();
        println!("/// Smart-solved book: moves chosen by highest instant-mate ratio.");
        println!("/// {} positions.", solver.book.len());
        println!("const BOOK_DATA: &[(u64, &str)] = &[");
        let mut entries: Vec<_> = solver.book.iter().collect();
        entries.sort_by_key(|&(_, &(_, d))| std::cmp::Reverse(d));
        for (&hash, &(mv, d)) in &entries {
            println!("    (0x{:016x}, \"{}\"),   // depth {}", hash, mv.to_uci(), d);
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
    }).unwrap().join().unwrap();
}
