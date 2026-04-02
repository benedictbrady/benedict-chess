use benedict_engine::board::Board;
use benedict_engine::eval::EvalParams;
use benedict_engine::movegen::generate_moves;
use benedict_engine::moves::MoveList;
use benedict_engine::search::{Searcher, MATE_SCORE, MAX_PLY};
use std::time::Duration;

/// Result of a single game
#[derive(Debug, Clone, Copy)]
enum GameResult {
    WhiteWin,
    BlackWin,
    Draw,
}

/// Play a single game between two searchers.
/// Returns (result, move_count).
fn play_game(
    white_params: &EvalParams,
    black_params: &EvalParams,
    time_per_move: Duration,
    max_moves: u32,
) -> (GameResult, u32) {
    let mut board = Board::startpos();
    let mut white = Searcher::with_params(16, white_params.clone());
    let mut black = Searcher::with_params(16, black_params.clone());
    white.silent = true;
    black.silent = true;

    let mut move_count = 0u32;
    let mut position_hashes = vec![board.hash];

    loop {
        if move_count >= max_moves {
            return (GameResult::Draw, move_count);
        }

        // Check for no legal moves
        let mut moves = MoveList::new();
        generate_moves(&board, &mut moves);
        if moves.is_empty() {
            return (GameResult::Draw, move_count);
        }

        // Select searcher
        let searcher = if board.side_to_move == benedict_engine::types::Color::White {
            &mut white
        } else {
            &mut black
        };
        searcher.set_position_history(position_hashes.clone());
        searcher.tt().new_generation();

        let info = searcher.search(&mut board, 64, Some(time_per_move));
        let best_move = info.best_move;

        if best_move.is_null() {
            return (GameResult::Draw, move_count);
        }

        let undo = board.make_move(best_move);
        move_count += 1;
        position_hashes.push(board.hash);

        // Check if king was flipped
        let them = board.side_to_move;
        if board.king_flipped(&undo, them) {
            // The side that just moved won
            let winner = board.side_to_move.flip();
            return (
                if winner == benedict_engine::types::Color::White {
                    GameResult::WhiteWin
                } else {
                    GameResult::BlackWin
                },
                move_count,
            );
        }

        // Check for the search declaring a mate found
        if info.score.abs() >= MATE_SCORE - MAX_PLY as i32 {
            // Engine thinks it found a forced win — let the game continue naturally
        }
    }
}

/// Compute ELO difference from score percentage.
/// score = wins + draws/2 out of total games.
fn elo_diff(wins: u32, draws: u32, losses: u32) -> f64 {
    let total = (wins + draws + losses) as f64;
    if total == 0.0 {
        return 0.0;
    }
    let score = (wins as f64 + draws as f64 * 0.5) / total;
    if score <= 0.0 || score >= 1.0 {
        if score >= 1.0 { return 999.0; }
        return -999.0;
    }
    -400.0 * (1.0 / score - 1.0).log10()
}

/// Run a match: N games with alternating colors.
fn run_match(
    name_a: &str,
    params_a: &EvalParams,
    name_b: &str,
    params_b: &EvalParams,
    num_games: u32,
    time_per_move: Duration,
) {
    let max_moves = 200; // very generous for Benedict chess
    let mut a_wins = 0u32;
    let mut b_wins = 0u32;
    let mut draws = 0u32;
    let mut total_moves = 0u32;

    println!("=== Match: {} vs {} ===", name_a, name_b);
    println!(
        "Games: {}, Time/move: {}ms",
        num_games,
        time_per_move.as_millis()
    );
    println!();

    for game in 0..num_games {
        // Alternate colors: even games = A is white, odd = B is white
        let (white_params, black_params, a_is_white) = if game % 2 == 0 {
            (params_a, params_b, true)
        } else {
            (params_b, params_a, false)
        };

        let (result, moves) = play_game(white_params, black_params, time_per_move, max_moves);
        total_moves += moves;

        let a_result = match (result, a_is_white) {
            (GameResult::WhiteWin, true) | (GameResult::BlackWin, false) => {
                a_wins += 1;
                "A wins"
            }
            (GameResult::BlackWin, true) | (GameResult::WhiteWin, false) => {
                b_wins += 1;
                "B wins"
            }
            (GameResult::Draw, _) => {
                draws += 1;
                "draw"
            }
        };

        let color_str = if a_is_white { "A=W" } else { "A=B" };
        print!(
            "  Game {:>3}: {} {:>6} in {:>3} moves  |  A {}-{}-{} B",
            game + 1,
            color_str,
            a_result,
            moves,
            a_wins,
            draws,
            b_wins,
        );

        let elo = elo_diff(a_wins, draws, b_wins);
        println!("  [ELO: {:+.0}]", elo);
    }

    let avg_moves = total_moves as f64 / num_games as f64;
    let elo = elo_diff(a_wins, draws, b_wins);

    println!();
    println!("=== Results ===");
    println!("{} (A): {} wins", name_a, a_wins);
    println!("{} (B): {} wins", name_b, b_wins);
    println!("Draws: {}", draws);
    println!("Avg game length: {:.1} moves", avg_moves);
    println!("ELO diff (A - B): {:+.0}", elo);
    println!();
}

fn main() {
    // Initialize attack tables
    benedict_engine::tables::tables();

    let args: Vec<String> = std::env::args().collect();

    let time_ms: u64 = args
        .iter()
        .position(|a| a == "--time")
        .and_then(|i| args.get(i + 1))
        .and_then(|s| s.parse().ok())
        .unwrap_or(1000);

    let num_games: u32 = args
        .iter()
        .position(|a| a == "--games")
        .and_then(|i| args.get(i + 1))
        .and_then(|s| s.parse().ok())
        .unwrap_or(20);

    let time_per_move = Duration::from_millis(time_ms);

    let test_name = args
        .iter()
        .position(|a| a == "--test")
        .and_then(|i| args.get(i + 1))
        .map(|s| s.as_str())
        .unwrap_or("baseline");

    match test_name {
        "baseline" => {
            println!("Running baseline self-play (same engine vs itself)...");
            let params = EvalParams::default();
            run_match("Baseline", &params, "Baseline", &params, num_games, time_per_move);
        }
        "mobility" => {
            let baseline = EvalParams::default();
            let mut candidate = EvalParams::default();
            candidate.mobility_weight = 3;
            run_match("Mobility(3)", &candidate, "Baseline", &baseline, num_games, time_per_move);
        }
        "knight_threats" => {
            let baseline = EvalParams::default();
            let mut candidate = EvalParams::default();
            candidate.knight_threat_bonus = 10;
            run_match("KnightThreat(10)", &candidate, "Baseline", &baseline, num_games, time_per_move);
        }
        "king_shield" => {
            let baseline = EvalParams::default();
            let mut candidate = EvalParams::default();
            candidate.king_shield_bonus = 15;
            run_match("KingShield(15)", &candidate, "Baseline", &baseline, num_games, time_per_move);
        }
        "tempo" => {
            let baseline = EvalParams::default();
            let mut candidate = EvalParams::default();
            candidate.tempo_bonus = 20;
            run_match("Tempo(20)", &candidate, "Baseline", &baseline, num_games, time_per_move);
        }
        "queen_threat_high" => {
            let baseline = EvalParams::default();
            let mut candidate = EvalParams::default();
            candidate.queen_threat_bonus = 30;
            run_match("QueenThreat(30)", &candidate, "Baseline", &baseline, num_games, time_per_move);
        }
        "combined" => {
            let baseline = EvalParams::default();
            let mut candidate = EvalParams::default();
            candidate.mobility_weight = 3;
            candidate.knight_threat_bonus = 10;
            candidate.king_shield_bonus = 15;
            candidate.tempo_bonus = 15;
            candidate.queen_threat_bonus = 25;
            run_match("CombinedAll", &candidate, "Baseline", &baseline, num_games, time_per_move);
        }
        "qt_ks" => {
            // Top 2 cheap features: queen threats + king shield
            let baseline = EvalParams::default();
            let mut candidate = EvalParams::default();
            candidate.queen_threat_bonus = 30;
            candidate.king_shield_bonus = 15;
            run_match("QT30+KS15", &candidate, "Baseline", &baseline, num_games, time_per_move);
        }
        "qt_tempo" => {
            let baseline = EvalParams::default();
            let mut candidate = EvalParams::default();
            candidate.queen_threat_bonus = 30;
            candidate.tempo_bonus = 20;
            run_match("QT30+Tempo20", &candidate, "Baseline", &baseline, num_games, time_per_move);
        }
        "qt_ks_tempo" => {
            let baseline = EvalParams::default();
            let mut candidate = EvalParams::default();
            candidate.queen_threat_bonus = 30;
            candidate.king_shield_bonus = 15;
            candidate.tempo_bonus = 20;
            run_match("QT30+KS15+T20", &candidate, "Baseline", &baseline, num_games, time_per_move);
        }
        "all" => {
            // Run all tests in sequence
            let baseline = EvalParams::default();

            let tests: Vec<(&str, EvalParams)> = vec![
                ("Mobility(3)", {
                    let mut p = baseline.clone();
                    p.mobility_weight = 3;
                    p
                }),
                ("KnightThreat(10)", {
                    let mut p = baseline.clone();
                    p.knight_threat_bonus = 10;
                    p
                }),
                ("KingShield(15)", {
                    let mut p = baseline.clone();
                    p.king_shield_bonus = 15;
                    p
                }),
                ("Tempo(20)", {
                    let mut p = baseline.clone();
                    p.tempo_bonus = 20;
                    p
                }),
                ("QueenThreat(30)", {
                    let mut p = baseline.clone();
                    p.queen_threat_bonus = 30;
                    p
                }),
            ];

            for (name, params) in &tests {
                run_match(name, params, "Baseline", &baseline, num_games, time_per_move);
            }

            println!("\n=== Summary ===");
            println!("See results above for each candidate vs Baseline.");
        }
        "king_danger" => {
            let mut old_eval = EvalParams::default();
            old_eval.king_danger_weight = 0;
            let new_eval = EvalParams::default(); // king_danger_weight=30
            run_match("KingDanger(30)", &new_eval, "NoKingDanger", &old_eval, num_games, time_per_move);
        }
        "pawn_advance" => {
            let baseline = EvalParams::default();
            let mut candidate = baseline.clone();
            candidate.pawn_advance_bonus = 15;
            run_match("PawnAdv(15)", &candidate, "Baseline", &baseline, num_games, time_per_move);
        }
        "flip_balance" => {
            let baseline = EvalParams::default();
            let mut candidate = baseline.clone();
            candidate.flip_balance_weight = 5;
            run_match("FlipBal(5)", &candidate, "Baseline", &baseline, num_games, time_per_move);
        }
        "pawn_advance_10" => {
            let baseline = EvalParams::default();
            let mut candidate = baseline.clone();
            candidate.pawn_advance_bonus = 10;
            run_match("PawnAdv(10)", &candidate, "Baseline", &baseline, num_games, time_per_move);
        }
        "flip_balance_3" => {
            let baseline = EvalParams::default();
            let mut candidate = baseline.clone();
            candidate.flip_balance_weight = 3;
            run_match("FlipBal(3)", &candidate, "Baseline", &baseline, num_games, time_per_move);
        }
        "tempo_15" => {
            let baseline = EvalParams::default();
            let mut candidate = baseline.clone();
            candidate.tempo_bonus = 15;
            run_match("Tempo(15)", &candidate, "Baseline", &baseline, num_games, time_per_move);
        }
        "king_shield_10" => {
            let baseline = EvalParams::default();
            let mut candidate = baseline.clone();
            candidate.king_shield_bonus = 10;
            run_match("KingShield(10)", &candidate, "Baseline", &baseline, num_games, time_per_move);
        }
        "kd_50" => {
            let baseline = EvalParams::default();
            let mut candidate = baseline.clone();
            candidate.king_danger_weight = 50;
            run_match("KD(50)", &candidate, "Baseline(KD30)", &baseline, num_games, time_per_move);
        }
        "mega" => {
            // Tested winners: tempo(15) +89 ELO, king_shield(10) +147 ELO
            let baseline = EvalParams::default();
            let mut candidate = baseline.clone();
            candidate.tempo_bonus = 15;
            candidate.king_shield_bonus = 10;
            run_match("Tempo15+KS10", &candidate, "Baseline", &baseline, num_games, time_per_move);
        }
        other => {
            eprintln!("Unknown test: {}", other);
            eprintln!("Available: baseline, pawn_advance, flip_balance, mega, king_danger, all");
            std::process::exit(1);
        }
    }
}
