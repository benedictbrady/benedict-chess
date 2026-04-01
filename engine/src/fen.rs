use crate::bitboard::Bitboard;
use crate::board::Board;
use crate::types::{CastlingRights, Color, Piece, Square};

pub fn from_fen(fen: &str) -> Result<Board, String> {
    let parts: Vec<&str> = fen.split_whitespace().collect();
    if parts.len() < 4 {
        return Err("FEN must have at least 4 fields".into());
    }

    let mut pieces = [Bitboard::EMPTY; 6];
    let mut colors = [Bitboard::EMPTY; 2];
    let mut mailbox: [Option<Piece>; 64] = [None; 64];

    // Parse piece placement
    let mut rank = 7u8;
    let mut file = 0u8;
    for c in parts[0].chars() {
        match c {
            '/' => {
                if rank == 0 {
                    return Err("too many ranks".into());
                }
                rank -= 1;
                file = 0;
            }
            '1'..='8' => {
                file += (c as u8) - b'0';
            }
            _ => {
                let piece = Piece::from_char(c).ok_or_else(|| format!("invalid piece: {}", c))?;
                let sq = Square::from_file_rank(file, rank);
                pieces[piece.kind.index()].set(sq);
                colors[piece.color.index()].set(sq);
                mailbox[sq.index()] = Some(piece);
                file += 1;
            }
        }
    }

    let occupied = colors[0] | colors[1];

    // Side to move
    let side_to_move = match parts[1] {
        "w" => Color::White,
        "b" => Color::Black,
        _ => return Err(format!("invalid side to move: {}", parts[1])),
    };

    // Castling rights
    let mut castling_rights = CastlingRights::NONE;
    if parts[2] != "-" {
        for c in parts[2].chars() {
            match c {
                'K' => castling_rights.0 |= CastlingRights::WHITE_KING,
                'Q' => castling_rights.0 |= CastlingRights::WHITE_QUEEN,
                'k' => castling_rights.0 |= CastlingRights::BLACK_KING,
                'q' => castling_rights.0 |= CastlingRights::BLACK_QUEEN,
                _ => return Err(format!("invalid castling: {}", c)),
            }
        }
    }

    // En passant (always "-" in Benedict chess, but parse for FEN compat)
    // parts[3] is en passant target — ignored

    let halfmove_clock = parts.get(4).and_then(|s| s.parse().ok()).unwrap_or(0);
    let fullmove_number = parts.get(5).and_then(|s| s.parse().ok()).unwrap_or(1);

    let mut board = Board {
        pieces,
        colors,
        occupied,
        side_to_move,
        castling_rights,
        halfmove_clock,
        fullmove_number,
        hash: 0,
        mailbox,
    };

    board.hash = board.compute_hash();
    Ok(board)
}

pub fn to_fen(board: &Board) -> String {
    let mut fen = String::new();

    // Piece placement
    for rank in (0..8).rev() {
        let mut empty = 0;
        for file in 0..8 {
            let sq = Square::from_file_rank(file, rank);
            match board.mailbox[sq.index()] {
                Some(piece) => {
                    if empty > 0 {
                        fen.push_str(&empty.to_string());
                        empty = 0;
                    }
                    fen.push(piece.to_char());
                }
                None => {
                    empty += 1;
                }
            }
        }
        if empty > 0 {
            fen.push_str(&empty.to_string());
        }
        if rank > 0 {
            fen.push('/');
        }
    }

    // Side to move
    fen.push(' ');
    fen.push_str(&board.side_to_move.to_string());

    // Castling
    fen.push(' ');
    let mut castling = String::new();
    if board.castling_rights.has(CastlingRights::WHITE_KING) {
        castling.push('K');
    }
    if board.castling_rights.has(CastlingRights::WHITE_QUEEN) {
        castling.push('Q');
    }
    if board.castling_rights.has(CastlingRights::BLACK_KING) {
        castling.push('k');
    }
    if board.castling_rights.has(CastlingRights::BLACK_QUEEN) {
        castling.push('q');
    }
    if castling.is_empty() {
        fen.push('-');
    } else {
        fen.push_str(&castling);
    }

    // En passant (always "-" for Benedict chess)
    fen.push_str(" -");

    // Halfmove clock and fullmove number
    fen.push(' ');
    fen.push_str(&board.halfmove_clock.to_string());
    fen.push(' ');
    fen.push_str(&board.fullmove_number.to_string());

    fen
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_startpos_roundtrip() {
        let fen_str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let board = from_fen(fen_str).unwrap();
        let result = to_fen(&board);
        assert_eq!(result, fen_str);
    }

    #[test]
    fn test_custom_position() {
        let fen_str = "rnbqkbnr/pppppppp/8/4P3/8/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1";
        let board = from_fen(fen_str).unwrap();
        assert_eq!(board.side_to_move, Color::Black);
        let result = to_fen(&board);
        assert_eq!(result, fen_str);
    }

    #[test]
    fn test_no_castling() {
        let fen_str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - - 0 1";
        let board = from_fen(fen_str).unwrap();
        assert_eq!(board.castling_rights, CastlingRights::NONE);
    }
}
