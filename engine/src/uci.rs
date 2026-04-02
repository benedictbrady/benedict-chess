use crate::board::Board;
use crate::fen;
use crate::movegen::{generate_moves, perft};
use crate::moves::{Move, MoveList, FLAG_CASTLE, FLAG_DOUBLE_PUSH};
use crate::search::{search_smp, SharedSearch};
use crate::types::{Color, PieceKind};
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Duration;

pub struct Uci {
    board: Board,
    shared: Arc<SharedSearch>,
    num_threads: usize,
    position_history: Vec<u64>,
    search_handle: Option<std::thread::JoinHandle<()>>,
}

impl Uci {
    pub fn new() -> Self {
        Uci {
            board: Board::startpos(),
            shared: Arc::new(SharedSearch::new(64)),
            num_threads: 1,
            position_history: Vec::new(),
            search_handle: None,
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
                    println!("option name Threads type spin default 1 min 1 max 256");
                    println!("uciok");
                }
                "isready" => {
                    self.wait_for_search();
                    println!("readyok");
                }
                "ucinewgame" => {
                    self.wait_for_search();
                    self.shared.tt.clear();
                    self.board = Board::startpos();
                    self.position_history.clear();
                }
                "setoption" => {
                    self.handle_setoption(&parts[1..]);
                }
                "position" => {
                    self.handle_position(&parts[1..]);
                }
                "go" => {
                    self.handle_go(&parts[1..]);
                }
                "stop" => {
                    self.shared.stop_flag.store(true, Ordering::Relaxed);
                    self.wait_for_search();
                }
                "quit" | "exit" => {
                    self.shared.stop_flag.store(true, Ordering::Relaxed);
                    self.wait_for_search();
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

    fn wait_for_search(&mut self) {
        if let Some(handle) = self.search_handle.take() {
            let _ = handle.join();
        }
    }

    fn handle_setoption(&mut self, args: &[&str]) {
        // Format: name <name> value <value>
        if args.len() < 4 || args[0] != "name" {
            return;
        }
        let value_idx = args.iter().position(|&a| a == "value");
        let Some(vi) = value_idx else { return };
        let name = args[1..vi].join(" ").to_lowercase();
        let value = args[vi + 1..].join(" ");

        match name.as_str() {
            "hash" => {
                if let Ok(mb) = value.parse::<usize>() {
                    self.wait_for_search();
                    self.shared = Arc::new(SharedSearch::new(mb));
                }
            }
            "threads" => {
                if let Ok(n) = value.parse::<usize>() {
                    self.num_threads = n.max(1).min(256);
                }
            }
            _ => {}
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
        self.position_history = vec![self.board.hash];
        for move_str in &args[move_start..] {
            if let Some(m) = self.parse_move(move_str) {
                self.board.make_move(m);
                self.position_history.push(self.board.hash);
            } else {
                println!("info string Invalid move: {}", move_str);
                return;
            }
        }
    }

    fn parse_move(&self, s: &str) -> Option<Move> {
        let base = Move::from_uci(s)?;
        let from = base.from_sq();
        let to = base.to_sq();

        if let Some(piece) = self.board.piece_at(from) {
            if piece.kind == PieceKind::King {
                let file_diff = (to.file() as i8 - from.file() as i8).abs();
                if file_diff == 2 {
                    return Some(Move::new_with_flags(from, to, FLAG_CASTLE));
                }
            }

            if piece.kind == PieceKind::Pawn {
                let rank_diff = (to.rank() as i8 - from.rank() as i8).abs();
                if rank_diff == 2 {
                    return Some(Move::new_with_flags(from, to, FLAG_DOUBLE_PUSH));
                }
            }

            if base.is_promotion() {
                return Some(base);
            }
        }

        Some(Move::new(from, to))
    }

    fn handle_go(&mut self, args: &[&str]) {
        self.wait_for_search();

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

        let time_limit = if let Some(mt) = movetime {
            Some(Duration::from_millis(mt))
        } else {
            let our_time = match self.board.side_to_move {
                Color::White => wtime,
                Color::Black => btime,
            };
            our_time.map(|t| Duration::from_millis(t / 30))
        };

        // Check opening book first (disabled by NOBOOK env var for analysis)
        if std::env::var("NOBOOK").is_err() {
            if let Some(book_move) = crate::book::probe(self.board.hash) {
                println!("info string book move");
                println!("bestmove {}", book_move.to_uci());
                return;
            }
        }

        // Spawn the search on a background thread so UCI input stays responsive
        let shared = Arc::clone(&self.shared);
        let board = self.board.clone();
        let num_threads = self.num_threads;
        let eval_params = crate::eval::EvalParams::default();
        let position_history = self.position_history.clone();

        self.search_handle = Some(std::thread::spawn(move || {
            let info = search_smp(
                &shared,
                &board,
                max_depth,
                time_limit,
                num_threads,
                &eval_params,
                position_history,
                false,
            );
            println!("bestmove {}", info.best_move.to_uci());
        }));
    }
}
