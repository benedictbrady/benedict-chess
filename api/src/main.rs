use axum::{
    extract::Json,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use benedict_engine::board::Board;
use benedict_engine::eval::EvalParams;
use benedict_engine::fen;
use benedict_engine::movegen::generate_moves;
use benedict_engine::moves::{Move, MoveList, FLAG_CASTLE, FLAG_DOUBLE_PUSH};
use benedict_engine::search::Searcher;
use benedict_engine::types::{Color, PieceKind};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tower_http::cors::{Any, CorsLayer};

#[derive(Deserialize)]
struct AnalyzeRequest {
    /// FEN string, or "startpos"
    position: String,
    /// Moves to apply after the position (UCI notation)
    #[serde(default)]
    moves: Vec<String>,
    /// Max search time in milliseconds (default: 1000)
    #[serde(default = "default_time")]
    time_ms: u64,
    /// Max search depth (default: 64)
    #[serde(default = "default_depth")]
    max_depth: i32,
}

fn default_time() -> u64 { 1000 }
fn default_depth() -> i32 { 64 }

#[derive(Serialize)]
struct AnalyzeResponse {
    best_move: String,
    score: i32,
    depth: i32,
    nodes: u64,
    pv: Vec<String>,
    fen: String,
    game_over: bool,
    winner: Option<String>,
}

#[derive(Deserialize)]
struct PositionRequest {
    position: String,
    #[serde(default)]
    moves: Vec<String>,
}

#[derive(Serialize)]
struct LegalMovesResponse {
    moves: Vec<MoveInfo>,
    fen: String,
    side_to_move: String,
}

#[derive(Serialize)]
struct MoveInfo {
    uci: String,
    from: String,
    to: String,
    promotion: Option<String>,
}

#[derive(Serialize)]
struct MakeMoveResponse {
    fen: String,
    flipped: Vec<String>,
    game_over: bool,
    winner: Option<String>,
    board: Vec<Vec<Option<PieceInfo>>>,
}

#[derive(Serialize)]
struct PieceInfo {
    kind: String,
    color: String,
}

#[derive(Deserialize)]
struct MakeMoveRequest {
    position: String,
    #[serde(default)]
    moves: Vec<String>,
    #[serde(rename = "move")]
    new_move: String,
}

fn setup_position(position: &str, moves: &[String]) -> Result<Board, String> {
    let mut board = if position == "startpos" {
        Board::startpos()
    } else {
        fen::from_fen(position)?
    };

    for move_str in moves {
        let m = parse_move(&board, move_str)?;
        board.make_move(m);
    }

    Ok(board)
}

fn parse_move(board: &Board, s: &str) -> Result<Move, String> {
    let base = Move::from_uci(s).ok_or_else(|| format!("invalid move: {}", s))?;
    let from = base.from_sq();
    let to = base.to_sq();

    if let Some(piece) = board.piece_at(from) {
        if piece.kind == PieceKind::King {
            let file_diff = (to.file() as i8 - from.file() as i8).abs();
            if file_diff == 2 {
                return Ok(Move::new_with_flags(from, to, FLAG_CASTLE));
            }
        }
        if piece.kind == PieceKind::Pawn {
            let rank_diff = (to.rank() as i8 - from.rank() as i8).abs();
            if rank_diff == 2 {
                return Ok(Move::new_with_flags(from, to, FLAG_DOUBLE_PUSH));
            }
        }
        if base.is_promotion() {
            return Ok(base);
        }
    }

    Ok(Move::new(from, to))
}

fn board_to_array(board: &Board) -> Vec<Vec<Option<PieceInfo>>> {
    let mut rows = Vec::new();
    for rank in (0..8).rev() {
        let mut row = Vec::new();
        for file in 0..8 {
            let sq = benedict_engine::types::Square::from_file_rank(file, rank);
            row.push(board.piece_at(sq).map(|p| PieceInfo {
                kind: format!("{}", p.kind.to_char()),
                color: if p.color == Color::White { "white".into() } else { "black".into() },
            }));
        }
        rows.push(row);
    }
    rows
}

async fn health() -> impl IntoResponse {
    (StatusCode::OK, "ok")
}

async fn analyze(Json(req): Json<AnalyzeRequest>) -> Result<Json<AnalyzeResponse>, (StatusCode, String)> {
    let mut board = setup_position(&req.position, &req.moves)
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    // Check opening book first
    if let Some(book_move) = benedict_engine::book::probe(board.hash) {
        let fen_str = fen::to_fen(&board);
        return Ok(Json(AnalyzeResponse {
            best_move: book_move.to_uci(),
            score: 0,
            depth: 0,
            nodes: 0,
            pv: vec![book_move.to_uci()],
            fen: fen_str,
            game_over: false,
            winner: None,
        }));
    }

    let time_limit = Duration::from_millis(req.time_ms);
    let mut searcher = Searcher::with_params(32, EvalParams::default());
    searcher.silent = true;
    searcher.set_position_history(vec![board.hash]);

    let info = searcher.search(&mut board, req.max_depth, Some(time_limit));

    let fen_str = fen::to_fen(&board);
    let pv: Vec<String> = info.pv.iter().map(|m| m.to_uci()).collect();

    Ok(Json(AnalyzeResponse {
        best_move: info.best_move.to_uci(),
        score: info.score,
        depth: info.depth,
        nodes: info.nodes,
        pv,
        fen: fen_str,
        game_over: false,
        winner: None,
    }))
}

async fn legal_moves(Json(req): Json<PositionRequest>) -> Result<Json<LegalMovesResponse>, (StatusCode, String)> {
    let board = setup_position(&req.position, &req.moves)
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    let mut moves = MoveList::new();
    generate_moves(&board, &mut moves);

    let move_infos: Vec<MoveInfo> = (0..moves.len())
        .map(|i| {
            let m = moves.get(i);
            MoveInfo {
                uci: m.to_uci(),
                from: m.from_sq().to_algebraic(),
                to: m.to_sq().to_algebraic(),
                promotion: m.promotion().map(|k| k.to_char().to_string()),
            }
        })
        .collect();

    Ok(Json(LegalMovesResponse {
        moves: move_infos,
        fen: fen::to_fen(&board),
        side_to_move: if board.side_to_move == Color::White {
            "white".into()
        } else {
            "black".into()
        },
    }))
}

async fn make_move(Json(req): Json<MakeMoveRequest>) -> Result<Json<MakeMoveResponse>, (StatusCode, String)> {
    let mut board = setup_position(&req.position, &req.moves)
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    let m = parse_move(&board, &req.new_move)
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    let undo = board.make_move(m);
    let them = board.side_to_move;
    let game_over = board.king_flipped(&undo, them);
    let winner = if game_over {
        Some(if board.side_to_move.flip() == Color::White {
            "white".into()
        } else {
            "black".into()
        })
    } else {
        None
    };

    let flipped_squares: Vec<String> = undo.flipped.into_iter()
        .map(|sq| sq.to_algebraic())
        .collect();

    Ok(Json(MakeMoveResponse {
        fen: fen::to_fen(&board),
        flipped: flipped_squares,
        game_over,
        winner,
        board: board_to_array(&board),
    }))
}

#[tokio::main]
async fn main() {
    // Initialize attack tables
    benedict_engine::tables::tables();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(health))
        .route("/api/analyze", post(analyze))
        .route("/api/legal-moves", post(legal_moves))
        .route("/api/make-move", post(make_move))
        .layer(cors);

    let port = std::env::var("PORT").unwrap_or_else(|_| "3001".to_string());
    let addr = format!("0.0.0.0:{}", port);
    println!("Benedict Chess API listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
