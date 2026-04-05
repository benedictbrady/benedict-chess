/// Tree analyzer: measures tree size and depth at each branching point.
/// Identifies positions where a different White move could shrink the tree.
///
/// For each White-to-move position in the book, it:
/// 1. Measures the sub-tree size (positions) and max depth of the current book move
/// 2. Tries all legal alternatives with a quick engine search
/// 3. Reports positions where alternatives could lead to smaller trees
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

struct TreeAnalyzer {
    /// Cache of sub-tree stats: hash -> (tree_size, max_depth)
    cache: HashMap<u64, (u32, u32)>,
    visited: HashSet<u64>,
    shared: Arc<SharedSearch>,
    params: EvalParams,
    start: Instant,
}

impl TreeAnalyzer {
    fn new() -> Self {
        TreeAnalyzer {
            cache: HashMap::new(),
            visited: HashSet::new(),
            shared: Arc::new(SharedSearch::new(256)),
            params: EvalParams::default(),
            start: Instant::now(),
        }
    }

    /// Measure the sub-tree from this position following the book.
    /// Returns (tree_size, max_depth) or None if the book doesn't cover it.
    fn measure_tree(
        &mut self,
        board: &mut Board,
        depth_limit: u32,
        history: &mut Vec<u64>,
    ) -> Option<(u32, u32)> {
        if let Some(&cached) = self.cache.get(&board.hash) {
            return Some(cached);
        }

        if depth_limit == 0 {
            return None;
        }

        // Cycle detection
        if history[..history.len().saturating_sub(1)].iter().any(|&h| h == board.hash) {
            return Some((0, 0));
        }

        let is_white = board.side_to_move == Color::White;
        let mut moves = MoveList::new();
        generate_moves(board, &mut moves);

        if moves.is_empty() {
            return None;
        }

        if is_white {
            let book_move = book::probe(board.hash)?;
            let legal = (0..moves.len())
                .map(|i| moves.get(i))
                .find(|m| m.from_sq() == book_move.from_sq() && m.to_sq() == book_move.to_sq())?;

            let undo = board.make_move(legal);
            let them = board.side_to_move;
            if board.king_flipped(&undo, them) {
                board.unmake_move(legal, &undo);
                let result = (1, 1);
                self.cache.insert(board.hash, result);
                return Some(result);
            }

            history.push(board.hash);
            let sub = self.measure_tree(board, depth_limit - 1, history);
            history.pop();
            board.unmake_move(legal, &undo);

            let result = sub.map(|(size, depth)| (size + 1, depth + 1));
            if let Some(r) = result {
                self.cache.insert(board.hash, r);
            }
            result
        } else {
            // Black: sum all branches, take max depth
            let mut total_size = 0u32;
            let mut max_depth = 0u32;

            for i in 0..moves.len() {
                let m = moves.get(i);
                let undo = board.make_move(m);
                let them = board.side_to_move;
                if board.king_flipped(&undo, them) {
                    board.unmake_move(m, &undo);
                    return None; // Black wins — shouldn't happen
                }

                history.push(board.hash);
                let sub = self.measure_tree(board, depth_limit - 1, history);
                history.pop();
                board.unmake_move(m, &undo);

                match sub {
                    Some((size, depth)) => {
                        total_size = total_size.saturating_add(size);
                        max_depth = max_depth.max(depth);
                    }
                    None => {
                        // Unmeasurable branch — return what we have but mark incomplete
                        let result = (total_size + 1000, max_depth.max(99));
                        self.cache.insert(board.hash, result);
                        return Some(result);
                    }
                }
            }

            let result = (total_size + 1, max_depth + 1);
            self.cache.insert(board.hash, result);
            Some(result)
        }
    }

    /// Check if an alternative White move leads to a mate-in-1.
    fn is_mate_in_1(&self, board: &mut Board, m: Move) -> bool {
        let undo = board.make_move(m);
        let them = board.side_to_move;
        let result = board.king_flipped(&undo, them);
        board.unmake_move(m, &undo);
        result
    }

    /// For a White-to-move position, estimate the "quality" of each legal move.
    /// Returns moves sorted by estimated tree size (smallest first).
    fn rank_moves(
        &mut self,
        board: &mut Board,
        history: &[u64],
    ) -> Vec<(Move, String)> {
        let mut moves = MoveList::new();
        generate_moves(board, &mut moves);

        let mut ranked: Vec<(Move, String)> = Vec::new();

        for i in 0..moves.len() {
            let m = moves.get(i);

            // Check mate in 1
            if self.is_mate_in_1(board, m) {
                ranked.insert(0, (m, "MATE-1".to_string()));
                continue;
            }

            // Check how many Black responses after this move are mate-in-1 for White
            let undo = board.make_move(m);
            let mut black_moves = MoveList::new();
            generate_moves(board, &mut black_moves);

            let mut instant_mates = 0;
            let mut total_black = 0;
            for j in 0..black_moves.len() {
                let bm = black_moves.get(j);
                let bundo = board.make_move(bm);
                let bthem = board.side_to_move;
                if board.king_flipped(&bundo, bthem) {
                    board.unmake_move(bm, &bundo);
                    continue; // Black wins this — bad
                }
                total_black += 1;

                // Check if White has mate-in-1 from here
                let mut w2 = MoveList::new();
                generate_moves(board, &mut w2);
                let has_mate1 = (0..w2.len()).any(|k| {
                    let wm = w2.get(k);
                    let wundo = board.make_move(wm);
                    let wthem = board.side_to_move;
                    let is_m = board.king_flipped(&wundo, wthem);
                    board.unmake_move(wm, &wundo);
                    is_m
                });
                if has_mate1 {
                    instant_mates += 1;
                }
                board.unmake_move(bm, &bundo);
            }
            board.unmake_move(m, &undo);

            // Score: higher instant_mates ratio = better (smaller tree)
            let desc = format!("{}/{} responses mate-in-1", instant_mates, total_black);
            ranked.push((m, desc));
        }

        // Sort: most instant mates first
        ranked.sort_by(|a, b| {
            let a_mates: u32 = a.1.split('/').next().unwrap_or("0").parse().unwrap_or(0);
            let b_mates: u32 = b.1.split('/').next().unwrap_or("0").parse().unwrap_or(0);
            b_mates.cmp(&a_mates)
        });

        ranked
    }

    /// Walk the tree and report on each White branching point.
    fn analyze(
        &mut self,
        board: &mut Board,
        depth: u32,
        ply: usize,
        history: &mut Vec<u64>,
        path: &mut Vec<String>,
    ) {
        if depth == 0 || history[..history.len().saturating_sub(1)].iter().any(|&h| h == board.hash) {
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

            // Measure current tree
            let current_stats = self.measure_tree(board, 40, history);

            if let Some((size, max_d)) = current_stats {
                if size >= 20 || max_d >= 10 {
                    // This is a non-trivial branching point. Rank alternatives.
                    let ranked = self.rank_moves(board, history);

                    // Find where the current book move ranks
                    let book_rank = ranked.iter().position(|(m, _)| {
                        m.from_sq() == book_move.from_sq() && m.to_sq() == book_move.to_sq()
                    });

                    let path_str = path.join(" ");
                    let top = &ranked[..ranked.len().min(5)];
                    let book_desc = ranked.iter()
                        .find(|(m, _)| m.from_sq() == book_move.from_sq() && m.to_sq() == book_move.to_sq())
                        .map(|(_, d)| d.as_str())
                        .unwrap_or("?");

                    // Is there a better move?
                    let top_move = &top[0];
                    let top_mates: u32 = top_move.1.split('/').next().unwrap_or("0").parse().unwrap_or(0);
                    let book_mates: u32 = book_desc.split('/').next().unwrap_or("0").parse().unwrap_or(0);

                    if top_mates > book_mates || (top_move.1.contains("MATE-1") && !book_desc.contains("MATE-1")) {
                        println!(
                            "IMPROVABLE [ply {}] {} | tree=({}, depth={}) | book: {} ({}) | best: {} ({})",
                            ply, path_str, size, max_d,
                            book_move.to_uci(), book_desc,
                            top_move.0.to_uci(), top_move.1
                        );
                    } else if size >= 100 {
                        println!(
                            "LARGE [ply {}] {} | tree=({}, depth={}) | book: {} ({})",
                            ply, path_str, size, max_d, book_move.to_uci(), book_desc
                        );
                    }
                }
            }

            // Recurse
            path.push(format!("{}.", legal.to_uci()));
            let undo = board.make_move(legal);
            let them = board.side_to_move;
            if !board.king_flipped(&undo, them) {
                history.push(board.hash);
                self.analyze(board, depth - 1, ply + 1, history, path);
                history.pop();
            }
            board.unmake_move(legal, &undo);
            path.pop();
        } else {
            // Black: recurse into all branches
            for i in 0..moves.len() {
                let m = moves.get(i);
                let undo = board.make_move(m);
                let them = board.side_to_move;
                if !board.king_flipped(&undo, them) {
                    path.push(format!("..{}", m.to_uci()));
                    history.push(board.hash);
                    self.analyze(board, depth - 1, ply + 1, history, path);
                    history.pop();
                    path.pop();
                }
                board.unmake_move(m, &undo);
            }
        }
    }
}

fn main() {
    benedict_engine::tables::tables();

    let mut analyzer = TreeAnalyzer::new();
    let mut board = Board::startpos();

    let e3 = Move::from_uci("e2e3").unwrap();
    let mut moves = MoveList::new();
    generate_moves(&board, &mut moves);
    let e3_legal = (0..moves.len())
        .map(|i| moves.get(i))
        .find(|m| m.from_sq() == e3.from_sq() && m.to_sq() == e3.to_sq())
        .unwrap();
    let undo = board.make_move(e3_legal);

    println!("=== Benedict Chess Tree Analyzer ===");
    println!("Analyzing tree size and move quality at each branching point\n");

    let mut history = vec![Board::startpos().hash, board.hash];
    let mut path = vec!["e3".to_string()];

    analyzer.analyze(&mut board, 20, 0, &mut history, &mut path);
    board.unmake_move(e3_legal, &undo);

    println!("\n=== DONE ===");
}
