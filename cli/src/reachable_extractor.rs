/// Reachable extractor: walks the game tree from 1.e3 and collects
/// every position hash that is actually reachable with the current book.
/// Outputs a minimal book containing only reachable entries.
use benedict_engine::board::Board;
use benedict_engine::book;
use benedict_engine::movegen::generate_moves;
use benedict_engine::moves::{Move, MoveList};
use benedict_engine::types::Color;
use std::collections::HashSet;

struct Extractor {
    reachable_white: HashSet<u64>,  // White-to-move positions with book entries
    total_positions: u64,
    terminal_mates: u64,
    black_positions: u64,
    white_no_book: u64,
}

impl Extractor {
    fn new() -> Self {
        Extractor {
            reachable_white: HashSet::new(),
            total_positions: 0,
            terminal_mates: 0,
            black_positions: 0,
            white_no_book: 0,
        }
    }

    fn walk(
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

        self.total_positions += 1;
        let is_white = board.side_to_move == Color::White;
        let mut moves = MoveList::new();
        generate_moves(board, &mut moves);

        if is_white {
            let book_move = match book::probe(board.hash) {
                Some(m) => m,
                None => {
                    self.white_no_book += 1;
                    return;
                }
            };

            // Record this White position as reachable
            self.reachable_white.insert(board.hash);

            let legal = match (0..moves.len())
                .map(|i| moves.get(i))
                .find(|m| m.from_sq() == book_move.from_sq() && m.to_sq() == book_move.to_sq()
                    && (book_move.to_uci().len() <= 4 || m.to_uci() == book_move.to_uci()))
            {
                Some(m) => m,
                None => return,
            };

            let undo = board.make_move(legal);
            let them = board.side_to_move;
            if board.king_flipped(&undo, them) {
                board.unmake_move(legal, &undo);
                self.terminal_mates += 1;
                return;
            }

            history.push(board.hash);
            self.walk(board, depth - 1, history);
            history.pop();
            board.unmake_move(legal, &undo);
        } else {
            self.black_positions += 1;
            for i in 0..moves.len() {
                let m = moves.get(i);
                let undo = board.make_move(m);
                let them = board.side_to_move;
                if board.king_flipped(&undo, them) {
                    board.unmake_move(m, &undo);
                    continue; // Black wins — skip
                }
                history.push(board.hash);
                self.walk(board, depth - 1, history);
                history.pop();
                board.unmake_move(m, &undo);
            }
        }
    }
}

fn main() {
    benedict_engine::tables::tables();

    let mut ext = Extractor::new();
    let mut board = Board::startpos();

    let e3 = Move::from_uci("e2e3").unwrap();
    let mut moves = MoveList::new();
    generate_moves(&board, &mut moves);
    let e3_legal = (0..moves.len())
        .map(|i| moves.get(i))
        .find(|m| m.from_sq() == e3.from_sq() && m.to_sq() == e3.to_sq())
        .unwrap();

    // Record startpos entry
    ext.reachable_white.insert(board.hash);

    let undo = board.make_move(e3_legal);

    eprintln!("=== Reachable Extractor ===");
    eprintln!("Walking game tree from 1.e3...\n");

    let mut history = vec![Board::startpos().hash, board.hash];
    ext.walk(&mut board, 120, &mut history);
    board.unmake_move(e3_legal, &undo);

    eprintln!("=== STATS ===");
    eprintln!("Total positions visited:   {}", ext.total_positions);
    eprintln!("Reachable White entries:   {}", ext.reachable_white.len());
    eprintln!("Black positions:           {}", ext.black_positions);
    eprintln!("Terminal mates:            {}", ext.terminal_mates);
    eprintln!("White positions w/o book:  {}", ext.white_no_book);

    // Read the current book and filter to reachable entries
    // Output the filtered book to stdout
    let book_src = include_str!("../../engine/src/book.rs");
    let mut kept = 0;
    let mut removed = 0;

    println!("use crate::moves::Move;");
    println!("use std::collections::HashMap;");
    println!("use std::sync::OnceLock;");
    println!();
    println!("/// Minimal solution book for 1.e3 — only reachable positions.");
    println!("/// Cleaned by reachable-extractor from {} entries.", {
        book_src.matches("(0x").count()
    });
    println!("const BOOK_DATA: &[(u64, &str)] = &[");

    for line in book_src.lines() {
        if let Some(hash_start) = line.find("(0x") {
            if let Some(hash_end) = line[hash_start+1..].find(',') {
                let hash_str = &line[hash_start+1..hash_start+1+hash_end];
                if let Ok(hash) = u64::from_str_radix(hash_str.trim_start_matches("0x"), 16) {
                    if ext.reachable_white.contains(&hash) {
                        // Keep this entry
                        println!("{}", line);
                        kept += 1;
                    } else {
                        removed += 1;
                    }
                    continue;
                }
            }
        }
        // Non-entry lines (comments, etc) — skip
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

    eprintln!("\nKept:    {}", kept);
    eprintln!("Removed: {}", removed);
}
