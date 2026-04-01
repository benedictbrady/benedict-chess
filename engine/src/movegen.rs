use crate::bitboard::Bitboard;
use crate::board::Board;
use crate::moves::{Move, MoveList, FLAG_CASTLE, FLAG_DOUBLE_PUSH};
use crate::tables::tables;
use crate::types::{CastlingRights, Color, PieceKind, Square};

/// Generate all pseudo-legal moves for the current side to move.
/// In Benedict chess, moves are to empty squares only (no captures).
/// After moving, the piece attacks/flips enemy pieces from its destination.
pub fn generate_moves(board: &Board, moves: &mut MoveList) {
    let us = board.side_to_move;
    let empty = !board.occupied;

    generate_pawn_moves(board, us, empty, moves);
    generate_piece_moves(board, us, empty, PieceKind::Knight, moves);
    generate_piece_moves(board, us, empty, PieceKind::Bishop, moves);
    generate_piece_moves(board, us, empty, PieceKind::Rook, moves);
    generate_piece_moves(board, us, empty, PieceKind::Queen, moves);
    generate_king_moves(board, us, empty, moves);
    generate_castling(board, us, moves);
}

fn generate_pawn_moves(board: &Board, us: Color, empty: Bitboard, moves: &mut MoveList) {
    let pawns = board.pieces[PieceKind::Pawn.index()] & board.colors[us.index()];
    let promo_rank = if us == Color::White {
        Bitboard::RANK_8
    } else {
        Bitboard::RANK_1
    };

    for from in pawns {
        let (single_push, start_rank) = match us {
            Color::White => (
                Bitboard::from_square(Square::new(from.0 + 8)),
                from.rank() == 1,
            ),
            Color::Black => (
                Bitboard::from_square(Square::new(from.0 - 8)),
                from.rank() == 6,
            ),
        };

        let single = single_push & empty;
        if single.is_not_empty() {
            let to = single.lsb();
            if (Bitboard::from_square(to) & promo_rank).is_not_empty() {
                // Promotion
                for promo in PieceKind::PROMOTION {
                    moves.push(Move::new_promotion(from, to, promo));
                }
            } else {
                if start_rank {
                    // Can also try double push
                    let double_sq = match us {
                        Color::White => Square::new(from.0 + 16),
                        Color::Black => Square::new(from.0 - 16),
                    };
                    let double = Bitboard::from_square(double_sq) & empty;
                    if double.is_not_empty() {
                        moves.push(Move::new_with_flags(from, double_sq, FLAG_DOUBLE_PUSH));
                    }
                }
                moves.push(Move::new(from, to));
            }
        }
    }
}

fn generate_piece_moves(
    board: &Board,
    us: Color,
    empty: Bitboard,
    kind: PieceKind,
    moves: &mut MoveList,
) {
    let t = tables();
    let our_pieces = board.pieces[kind.index()] & board.colors[us.index()];

    for from in our_pieces {
        let destinations = match kind {
            PieceKind::Knight => t.knight_attacks(from) & empty,
            PieceKind::Bishop => t.bishop_attacks(from, board.occupied) & empty,
            PieceKind::Rook => t.rook_attacks(from, board.occupied) & empty,
            PieceKind::Queen => t.queen_attacks(from, board.occupied) & empty,
            _ => unreachable!(),
        };

        for to in destinations {
            moves.push(Move::new(from, to));
        }
    }
}

fn generate_king_moves(board: &Board, us: Color, empty: Bitboard, moves: &mut MoveList) {
    let t = tables();
    let king_sq = board.king_square(us);
    let destinations = t.king_attacks(king_sq) & empty;

    for to in destinations {
        moves.push(Move::new(king_sq, to));
    }
}

fn generate_castling(board: &Board, us: Color, moves: &mut MoveList) {
    match us {
        Color::White => {
            // Kingside: e1-g1, need f1 and g1 empty
            if board.castling_rights.has(CastlingRights::WHITE_KING) {
                let f1 = Square::from_file_rank(5, 0);
                let g1 = Square::from_file_rank(6, 0);
                if !board.occupied.contains(f1) && !board.occupied.contains(g1) {
                    moves.push(Move::new_with_flags(Square::E1, g1, FLAG_CASTLE));
                }
            }
            // Queenside: e1-c1, need b1, c1, d1 empty
            if board.castling_rights.has(CastlingRights::WHITE_QUEEN) {
                let b1 = Square::from_file_rank(1, 0);
                let c1 = Square::from_file_rank(2, 0);
                let d1 = Square::from_file_rank(3, 0);
                if !board.occupied.contains(b1)
                    && !board.occupied.contains(c1)
                    && !board.occupied.contains(d1)
                {
                    moves.push(Move::new_with_flags(Square::E1, c1, FLAG_CASTLE));
                }
            }
        }
        Color::Black => {
            // Kingside: e8-g8
            if board.castling_rights.has(CastlingRights::BLACK_KING) {
                let f8 = Square::from_file_rank(5, 7);
                let g8 = Square::from_file_rank(6, 7);
                if !board.occupied.contains(f8) && !board.occupied.contains(g8) {
                    moves.push(Move::new_with_flags(Square::E8, g8, FLAG_CASTLE));
                }
            }
            // Queenside: e8-c8
            if board.castling_rights.has(CastlingRights::BLACK_QUEEN) {
                let b8 = Square::from_file_rank(1, 7);
                let c8 = Square::from_file_rank(2, 7);
                let d8 = Square::from_file_rank(3, 7);
                if !board.occupied.contains(b8)
                    && !board.occupied.contains(c8)
                    && !board.occupied.contains(d8)
                {
                    moves.push(Move::new_with_flags(Square::E8, c8, FLAG_CASTLE));
                }
            }
        }
    }
}

/// Perft: count leaf nodes at a given depth. Used for move generation validation.
pub fn perft(board: &mut Board, depth: u32) -> u64 {
    if depth == 0 {
        return 1;
    }

    let mut moves = MoveList::new();
    generate_moves(board, &mut moves);

    let mut nodes = 0u64;
    for i in 0..moves.len() {
        let m = moves.get(i);
        let undo = board.make_move(m);

        // Check if this move flipped the enemy king — game over
        let them = board.side_to_move; // after make_move, side_to_move is the opponent
        if board.king_flipped(&undo, them) {
            // This is a terminal node — the game is won
            nodes += 1;
        } else {
            nodes += perft(board, depth - 1);
        }

        board.unmake_move(m, &undo);
    }

    nodes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_startpos_move_count() {
        let board = Board::startpos();
        let mut moves = MoveList::new();
        generate_moves(&board, &mut moves);
        // Same as standard chess: 16 pawn moves + 4 knight moves = 20
        assert_eq!(moves.len(), 20);
    }

    #[test]
    fn test_perft_1() {
        let mut board = Board::startpos();
        assert_eq!(perft(&mut board, 1), 20);
    }

    #[test]
    fn test_no_diagonal_pawn_moves() {
        // Verify pawns can only push forward, not move diagonally
        // Kings on a1/a8 so they don't interfere with pawn tests
        let board =
            crate::fen::from_fen("k7/8/8/3p4/4P3/8/8/K7 w - - 0 1").unwrap();
        let mut moves = MoveList::new();
        generate_moves(&board, &mut moves);
        // White pawn on e4: push to e5 (1 move) + king moves
        let pawn_moves: Vec<_> = moves.as_slice().iter()
            .filter(|m| m.from_sq() == Square::from_file_rank(4, 3))
            .collect();
        assert_eq!(pawn_moves.len(), 1);
        assert_eq!(pawn_moves[0].to_sq(), Square::from_file_rank(4, 4)); // e5
    }

    #[test]
    fn test_pawn_blocked() {
        // Pawn blocked by piece directly ahead
        let board =
            crate::fen::from_fen("k7/8/8/4p3/4P3/8/8/K7 w - - 0 1").unwrap();
        let mut moves = MoveList::new();
        generate_moves(&board, &mut moves);
        // e4 pawn is blocked by e5 pawn — only king moves
        let pawn_moves: Vec<_> = moves.as_slice().iter()
            .filter(|m| m.from_sq() == Square::from_file_rank(4, 3))
            .collect();
        assert_eq!(pawn_moves.len(), 0);
    }

    #[test]
    fn test_promotion() {
        let board =
            crate::fen::from_fen("k7/4P3/8/8/8/8/8/K7 w - - 0 1").unwrap();
        let mut moves = MoveList::new();
        generate_moves(&board, &mut moves);
        // Pawn on e7 can push to e8, promoting to N/B/R/Q = 4 promo moves + king moves
        let promo_moves: Vec<_> = moves.as_slice().iter()
            .filter(|m| m.is_promotion())
            .collect();
        assert_eq!(promo_moves.len(), 4);
    }
}
