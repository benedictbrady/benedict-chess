use crate::board::Board;
use crate::fen;
use crate::movegen::{generate_moves, perft};
use crate::moves::{Move, MoveList, FLAG_CASTLE, FLAG_DOUBLE_PUSH};
use crate::search::Searcher;
use crate::types::{Color, PieceKind};
use std::time::Duration;

pub struct Uci {
    board: Board,
    searcher: Searcher,
}

impl Uci {
    pub fn new() -> Self {
        Uci {
            board: Board::startpos(),
            searcher: Searcher::new(64), // 64 MB TT
        }
    }

    pub fn run(&mut self) {
        let stdin = std::io::stdin();
        let mut line = String::new();

        loop {
            line.clear();
            if stdin.read_line(&mut line).is_err() {
                break;
            }
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let parts: Vec<&str> = line.split_whitespace().collect();
            match parts[0] {
                "uci" => {
                    println!("id name Benedict Engine 0.1");
                    println!("id author Benedict Chess Project");
                    println!("option name Hash type spin default 64 min 1 max 1024");
                    println!("uciok");
                }
                "isready" => {
                    println!("readyok");
                }
                "ucinewgame" => {
                    self.searcher.tt.clear();
                    self.board = Board::startpos();
                }
                "position" => {
                    self.handle_position(&parts[1..]);
                }
                "go" => {
                    self.handle_go(&parts[1..]);
                }
                "stop" => {
                    // Handled by search timeout
                }
                "quit" | "exit" => {
                    break;
                }
                "d" | "display" => {
                    println!("{}", self.board);
                    println!("FEN: {}", fen::to_fen(&self.board));
                    println!("Hash: {:016x}", self.board.hash);
                }
                "perft" => {
                    if let Some(depth_str) = parts.get(1) {
                        if let Ok(depth) = depth_str.parse::<u32>() {
                            let start = std::time::Instant::now();
                            let nodes = perft(&mut self.board, depth);
                            let elapsed = start.elapsed();
                            println!(
                                "perft({}) = {} ({:.2}s, {:.0} nps)",
                                depth,
                                nodes,
                                elapsed.as_secs_f64(),
                                nodes as f64 / elapsed.as_secs_f64()
                            );
                        }
                    }
                }
                "moves" => {
                    let mut moves = MoveList::new();
                    generate_moves(&self.board, &mut moves);
                    println!("Legal moves ({}):", moves.len());
                    for i in 0..moves.len() {
                        print!("{} ", moves.get(i).to_uci());
                    }
                    println!();
                }
                _ => {
                    println!("Unknown command: {}", parts[0]);
                }
            }
        }
    }

    fn handle_position(&mut self, args: &[&str]) {
        if args.is_empty() {
            return;
        }

        let mut move_start = 0;

        if args[0] == "startpos" {
            self.board = Board::startpos();
            move_start = 1;
            if args.get(1) == Some(&"moves") {
                move_start = 2;
            }
        } else if args[0] == "fen" {
            // Collect FEN string (up to 6 parts)
            let mut fen_end = 1;
            for i in 1..args.len() {
                if args[i] == "moves" {
                    break;
                }
                fen_end = i + 1;
            }
            let fen_str = args[1..fen_end].join(" ");
            match fen::from_fen(&fen_str) {
                Ok(board) => self.board = board,
                Err(e) => {
                    println!("info string Invalid FEN: {}", e);
                    return;
                }
            }
            move_start = fen_end;
            if args.get(fen_end) == Some(&"moves") {
                move_start = fen_end + 1;
            }
        }

        // Apply moves
        let mut position_hashes = vec![self.board.hash];
        for move_str in &args[move_start..] {
            if let Some(m) = self.parse_move(move_str) {
                self.board.make_move(m);
                position_hashes.push(self.board.hash);
            } else {
                println!("info string Invalid move: {}", move_str);
                return;
            }
        }
        self.searcher.set_position_history(position_hashes);
    }

    fn parse_move(&self, s: &str) -> Option<Move> {
        let base = Move::from_uci(s)?;
        let from = base.from_sq();
        let to = base.to_sq();

        // Check if this is a castling move
        if let Some(piece) = self.board.piece_at(from) {
            if piece.kind == PieceKind::King {
                let file_diff = (to.file() as i8 - from.file() as i8).abs();
                if file_diff == 2 {
                    return Some(Move::new_with_flags(from, to, FLAG_CASTLE));
                }
            }

            // Check for double pawn push
            if piece.kind == PieceKind::Pawn {
                let rank_diff = (to.rank() as i8 - from.rank() as i8).abs();
                if rank_diff == 2 {
                    return Some(Move::new_with_flags(from, to, FLAG_DOUBLE_PUSH));
                }
            }

            // Check for promotion
            if base.is_promotion() {
                return Some(base);
            }
        }

        Some(Move::new(from, to))
    }

    fn handle_go(&mut self, args: &[&str]) {
        let mut max_depth = 64i32;
        let mut movetime: Option<u64> = None;
        let mut wtime: Option<u64> = None;
        let mut btime: Option<u64> = None;

        let mut i = 0;
        while i < args.len() {
            match args[i] {
                "depth" => {
                    if let Some(d) = args.get(i + 1).and_then(|s| s.parse().ok()) {
                        max_depth = d;
                    }
                    i += 2;
                }
                "movetime" => {
                    movetime = args.get(i + 1).and_then(|s| s.parse().ok());
                    i += 2;
                }
                "wtime" => {
                    wtime = args.get(i + 1).and_then(|s| s.parse().ok());
                    i += 2;
                }
                "btime" => {
                    btime = args.get(i + 1).and_then(|s| s.parse().ok());
                    i += 2;
                }
                "infinite" => {
                    max_depth = 64;
                    i += 1;
                }
                _ => {
                    i += 1;
                }
            }
        }

        // Determine time limit
        let time_limit = if let Some(mt) = movetime {
            Some(Duration::from_millis(mt))
        } else {
            // Simple time management: use 1/30 of remaining time
            let our_time = match self.board.side_to_move {
                Color::White => wtime,
                Color::Black => btime,
            };
            our_time.map(|t| Duration::from_millis(t / 30))
        };

        let info = self.searcher.search(&mut self.board, max_depth, time_limit);
        println!("bestmove {}", info.best_move.to_uci());
    }
}
