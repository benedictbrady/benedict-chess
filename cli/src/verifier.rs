/// Verifier: proves the opening book is a complete forced-mate solution.
///
/// For every position in the book:
/// - If White to move: the book move must either mate immediately or lead to
///   a position that is also in the book (or recursively verified).
/// - If Black to move: EVERY legal move must either flip White's king (Black wins,
///   which should never happen) or lead to a position that is in the book.
///
/// If verification passes, the book is a proven forced mate from 1.e3.
use benedict_engine::board::Board;
use benedict_engine::book;
use benedict_engine::fen;
use benedict_engine::movegen::generate_moves;
use benedict_engine::moves::{Move, MoveList};
use benedict_engine::search::{SharedSearch, ThreadSearcher};
use benedict_engine::eval::EvalParams;
use benedict_engine::types::Color;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Duration;

struct Verifier {
    verified: HashSet<u64>,
    failed: Vec<(u64, String)>,
    positions_checked: u64,
    mates_found: u64,
    max_depth_seen: usize,
    /// New entries discovered by the engine during verification
    fixes: Vec<(u64, String)>,  // (hash, uci_move)
    shared: Arc<SharedSearch>,
    params: EvalParams,
    fix_mode: bool,
}

impl Verifier {
    fn new(fix_mode: bool) -> Self {
        Verifier {
            verified: HashSet::new(),
            failed: Vec::new(),
            positions_checked: 0,
            mates_found: 0,
            max_depth_seen: 0,
            fixes: Vec::new(),
            shared: Arc::new(SharedSearch::new(256)),
            params: EvalParams::default(),
            fix_mode,
        }
    }

    /// Verify that a position is a forced win for White.
    /// Returns true if verified, false if a gap is found.
    fn verify(
        &mut self,
        board: &mut Board,
        depth: usize,
        history: &mut Vec<u64>,
    ) -> bool {
        // Already verified this position
        if self.verified.contains(&board.hash) {
            return true;
        }

        // Depth limit to prevent infinite recursion from bugs
        if depth > 500 {
            self.failed.push((board.hash, format!("depth limit exceeded at depth {}", depth)));
            return false;
        }

        if depth > self.max_depth_seen {
            self.max_depth_seen = depth;
        }

        // Repetition check
        if history[..history.len().saturating_sub(1)].iter().filter(|&&h| h == board.hash).count() >= 1 {
            self.failed.push((board.hash, "repetition detected".to_string()));
            return false;
        }

        self.positions_checked += 1;

        let is_white = board.side_to_move == Color::White;

        if is_white {
            // WHITE TO MOVE: book must have a move, and it must lead to a verified position or mate
            let book_move = book::probe(board.hash).or_else(|| {
                // Check if we already found a fix for this hash
                self.fixes.iter().find(|(h, _)| *h == board.hash)
                    .and_then(|(_, uci)| Move::from_uci(uci))
            });
            if book_move.is_none() {
                if self.fix_mode {
                    // Smart fix: prefer moves with highest instant-mate ratio
                    // This prevents creating deep endgame branches.
                    let mut all_moves = MoveList::new();
                    generate_moves(board, &mut all_moves);

                    // 1) Check for mate-in-1
                    let mut best_move = Move::NULL;
                    for i in 0..all_moves.len() {
                        let m = all_moves.get(i);
                        let undo = board.make_move(m);
                        let them = board.side_to_move;
                        if board.king_flipped(&undo, them) {
                            board.unmake_move(m, &undo);
                            best_move = m;
                            break;
                        }
                        board.unmake_move(m, &undo);
                    }

                    // 2) If no mate-in-1, rank by instant-mate ratio
                    if best_move.is_null() {
                        let mut best_ratio = 0u32;
                        for i in 0..all_moves.len() {
                            let m = all_moves.get(i);
                            let undo = board.make_move(m);
                            let them = board.side_to_move;
                            if board.king_flipped(&undo, them) {
                                board.unmake_move(m, &undo);
                                best_move = m;
                                break;
                            }
                            // Count Black responses that are mate-in-1
                            // SOUNDNESS: skip this move if ANY Black response flips White's king
                            let mut black_moves = MoveList::new();
                            generate_moves(board, &mut black_moves);
                            let mut instant = 0u32;
                            let mut black_wins = false;
                            for j in 0..black_moves.len() {
                                let bm = black_moves.get(j);
                                let bundo = board.make_move(bm);
                                let bthem = board.side_to_move;
                                if board.king_flipped(&bundo, bthem) {
                                    // Black flips White's king — this White move is LOSING
                                    board.unmake_move(bm, &bundo);
                                    black_wins = true;
                                    break;
                                }
                                let mut w2 = MoveList::new();
                                generate_moves(board, &mut w2);
                                let has_mate = (0..w2.len()).any(|k| {
                                    let wm = w2.get(k);
                                    let wundo = board.make_move(wm);
                                    let wthem = board.side_to_move;
                                    let is_m = board.king_flipped(&wundo, wthem);
                                    board.unmake_move(wm, &wundo);
                                    is_m
                                });
                                if has_mate { instant += 1; }
                                board.unmake_move(bm, &bundo);
                            }
                            board.unmake_move(m, &undo);
                            if black_wins { continue; } // Skip — Black wins
                            if instant > best_ratio {
                                best_ratio = instant;
                                best_move = m;
                            }
                        }
                    }

                    // 3) Fallback to engine at depth 16 (higher than default 12)
                    if best_move.is_null() {
                        let mut searcher = ThreadSearcher::with_params(
                            Arc::clone(&self.shared), self.params.clone(),
                        );
                        searcher.set_position_history(history.to_vec());
                        searcher.silent = true;
                        let info = searcher.search(board, 16, Some(Duration::from_secs(5)));
                        if !info.best_move.is_null() {
                            best_move = info.best_move;
                        }
                    }

                    if !best_move.is_null() {
                        let uci = best_move.to_uci();
                        eprintln!("  FIX: 0x{:016x} -> {} (depth {})", board.hash, uci, depth);
                        self.fixes.push((board.hash, uci));
                    } else {
                        self.failed.push((board.hash, "no book entry and all methods failed".to_string()));
                        return false;
                    }
                } else {
                    self.failed.push((board.hash, "no book entry for White position".to_string()));
                    return false;
                }
            }
            let book_move = book_move.unwrap_or_else(|| {
                let uci = &self.fixes.iter().rev().find(|(h, _)| *h == board.hash).unwrap().1;
                Move::from_uci(uci).unwrap()
            });

            // Verify the book move is legal
            let mut moves = MoveList::new();
            generate_moves(board, &mut moves);
            let is_legal = (0..moves.len()).any(|i| {
                let m = moves.get(i);
                m.from_sq() == book_move.from_sq() && m.to_sq() == book_move.to_sq()
                    && (book_move.to_uci().len() <= 4 || m.to_uci() == book_move.to_uci())
            });

            if !is_legal {
                self.failed.push((board.hash, format!("book move {} is illegal", book_move.to_uci())));
                return false;
            }

            // Find the actual legal move (with correct flags)
            let legal_move = (0..moves.len())
                .map(|i| moves.get(i))
                .find(|m| {
                    m.from_sq() == book_move.from_sq() && m.to_sq() == book_move.to_sq()
                        && (book_move.to_uci().len() <= 4 || m.to_uci() == book_move.to_uci())
                })
                .unwrap();

            let undo = board.make_move(legal_move);

            // Check for immediate mate
            let them = board.side_to_move;
            if board.king_flipped(&undo, them) {
                board.unmake_move(legal_move, &undo);
                self.verified.insert(board.hash);
                self.mates_found += 1;
                return true;
            }

            // Recurse
            history.push(board.hash);
            let result = self.verify(board, depth + 1, history);
            history.pop();
            board.unmake_move(legal_move, &undo);

            if result {
                self.verified.insert(board.hash);
            }
            result
        } else {
            // BLACK TO MOVE: EVERY legal move must lead to a verified position
            let mut moves = MoveList::new();
            generate_moves(board, &mut moves);

            if moves.is_empty() {
                self.failed.push((board.hash, "no legal moves for Black".to_string()));
                return false;
            }

            for i in 0..moves.len() {
                let m = moves.get(i);
                let undo = board.make_move(m);

                // Check if Black flipped White's king (Black wins — proof fails!)
                let them = board.side_to_move;
                if board.king_flipped(&undo, them) {
                    board.unmake_move(m, &undo);
                    self.failed.push((
                        board.hash,
                        format!("Black move {} flips White's king!", m.to_uci()),
                    ));
                    return false;
                }

                history.push(board.hash);
                let result = self.verify(board, depth + 1, history);
                history.pop();
                board.unmake_move(m, &undo);

                if !result {
                    self.failed.push((
                        board.hash,
                        format!("Black move {} leads to unverified position", m.to_uci()),
                    ));
                    return false;
                }
            }

            self.verified.insert(board.hash);
            true
        }
    }
}

fn main() {
    // Use a large stack to handle deep recursion (depth 120+)
    let builder = std::thread::Builder::new().stack_size(128 * 1024 * 1024);
    let handler = builder.spawn(|| real_main()).unwrap();
    handler.join().unwrap();
}

fn real_main() {
    benedict_engine::tables::tables();

    let fix_mode = std::env::args().any(|a| a == "--fix");
    let mut verifier = Verifier::new(fix_mode);
    let mut board = Board::startpos();

    // Play 1.e3
    let e3 = Move::from_uci("e2e3").unwrap();
    let mut moves = MoveList::new();
    generate_moves(&board, &mut moves);
    let e3_legal = (0..moves.len())
        .map(|i| moves.get(i))
        .find(|m| m.from_sq() == e3.from_sq() && m.to_sq() == e3.to_sq())
        .expect("e2e3 not legal");
    let undo = board.make_move(e3_legal);

    println!("=== Benedict Chess Solution Verifier ===");
    println!("Verifying that the opening book is a complete forced-mate proof from 1.e3");
    println!("This checks EVERY legal Black move at EVERY position in the tree.\n");

    let mut history = vec![Board::startpos().hash, board.hash];

    let result = verifier.verify(&mut board, 0, &mut history);
    board.unmake_move(e3_legal, &undo);

    println!("=== RESULTS ===");
    println!("Positions checked:  {}", verifier.positions_checked);
    println!("Positions verified: {}", verifier.verified.len());
    println!("Terminal mates:     {}", verifier.mates_found);
    println!("Max tree depth:     {}", verifier.max_depth_seen);
    println!();

    if !verifier.fixes.is_empty() {
        println!("\nNew entries found by engine ({}):", verifier.fixes.len());
        for (hash, uci) in &verifier.fixes {
            println!("    (0x{:016x}, \"{}\"),", hash, uci);
        }
    }

    if result {
        println!("\nPROOF COMPLETE: 1.e3 is a verified forced mate for White.");
        println!("Every legal Black move at every reachable position has been checked.");
        println!("All paths terminate in king flip (checkmate).");
    } else {
        println!("\nPROOF FAILED: {} gaps found", verifier.failed.len());
        println!();
        for (i, (hash, reason)) in verifier.failed.iter().enumerate().take(20) {
            println!("  Gap {}: hash=0x{:016x} — {}", i + 1, hash, reason);
        }
        if verifier.failed.len() > 20 {
            println!("  ... and {} more", verifier.failed.len() - 20);
        }
    }
}
