use benedict_engine::board::Board;
use benedict_engine::movegen::generate_moves;
use benedict_engine::moves::{Move, MoveList};

fn main() {
    benedict_engine::tables::tables();

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: hash-dump <move1> <move2> ...");
        eprintln!("Example: hash-dump e2e3 e7e6 d1e2");
        std::process::exit(1);
    }

    let mut board = Board::startpos();
    println!("startpos hash: 0x{:016x}", board.hash);

    for (i, uci_str) in args[1..].iter().enumerate() {
        let mut legal_moves = MoveList::new();
        generate_moves(&board, &mut legal_moves);

        let target = Move::from_uci(uci_str).expect("invalid UCI move");
        let mut found: Option<Move> = None;
        for idx in 0..legal_moves.len() {
            let m = legal_moves.get(idx);
            if m.from_sq() == target.from_sq() && m.to_sq() == target.to_sq()
                && (uci_str.len() <= 4 || m.to_uci() == *uci_str)
            {
                found = Some(m);
                break;
            }
        }

        match found {
            Some(m) => {
                board.make_move(m);
                let move_num = (i / 2) + 1;
                let side = if i % 2 == 0 { "W" } else { "B" };
                println!(
                    "After {}{} {}: hash=0x{:016x}",
                    move_num, side, uci_str, board.hash
                );
            }
            None => {
                eprintln!("Illegal move: {} at position {}", uci_str, i);
                eprintln!("Legal moves:");
                for idx in 0..legal_moves.len() {
                    eprint!("{} ", legal_moves.get(idx).to_uci());
                }
                eprintln!();
                std::process::exit(1);
            }
        }
    }
}
