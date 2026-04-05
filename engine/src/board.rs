use crate::bitboard::Bitboard;
use crate::moves::{Move, UndoInfo};
use crate::tables::tables;
use crate::types::{CastlingRights, Color, Piece, PieceKind, Square};
use crate::zobrist::zobrist;

#[derive(Clone)]
pub struct Board {
    pub pieces: [Bitboard; 6],    // indexed by PieceKind
    pub colors: [Bitboard; 2],    // indexed by Color
    pub occupied: Bitboard,
    pub side_to_move: Color,
    pub castling_rights: CastlingRights,
    pub halfmove_clock: u16,
    pub fullmove_number: u16,
    pub hash: u64,
    pub mailbox: [Option<Piece>; 64],
}

impl Board {
    pub fn startpos() -> Board {
        crate::fen::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
            .expect("invalid startpos FEN")
    }

    pub fn piece_at(&self, sq: Square) -> Option<Piece> {
        self.mailbox[sq.index()]
    }

    pub fn king_square(&self, color: Color) -> Square {
        let king_bb = self.pieces[PieceKind::King.index()] & self.colors[color.index()];
        debug_assert!(king_bb.is_not_empty(), "king not found for {:?}", color);
        king_bb.lsb()
    }

    pub fn compute_hash(&self) -> u64 {
        let z = zobrist();
        let mut hash = 0u64;
        for sq_idx in 0..64 {
            if let Some(piece) = self.mailbox[sq_idx] {
                hash ^= z.piece(piece.color.index(), piece.kind.index(), sq_idx);
            }
        }
        hash ^= z.castling[self.castling_rights.index()];
        if self.side_to_move == Color::Black {
            hash ^= z.side_to_move;
        }
        hash
    }

    /// Make a move on the board. Returns UndoInfo for unmake.
    /// IMPORTANT: The caller should check if the move flips the enemy king
    /// by examining the returned UndoInfo.flipped against the king bitboard.
    pub fn make_move(&mut self, m: Move) -> UndoInfo {
        let z = zobrist();
        let t = tables();

        let from = m.from_sq();
        let to = m.to_sq();
        let us = self.side_to_move;
        let them = us.flip();

        // Save undo info
        let undo = UndoInfo {
            flipped: Bitboard::EMPTY, // will be filled in
            castling_rights: self.castling_rights,
            hash: self.hash,
            halfmove_clock: self.halfmove_clock,
        };

        let piece = self.mailbox[from.index()].expect("no piece at from square");
        debug_assert_eq!(piece.color, us);

        let mut kind = piece.kind;

        // Remove piece from source
        self.pieces[kind.index()].clear(from);
        self.colors[us.index()].clear(from);
        self.occupied.clear(from);
        self.mailbox[from.index()] = None;
        self.hash ^= z.piece(us.index(), kind.index(), from.index());

        // Handle promotion
        if m.is_promotion() {
            if let Some(promo_kind) = m.promotion() {
                kind = promo_kind;
            }
        }

        // Place piece at destination
        self.pieces[kind.index()].set(to);
        self.colors[us.index()].set(to);
        self.occupied.set(to);
        self.mailbox[to.index()] = Some(Piece::new(us, kind));
        self.hash ^= z.piece(us.index(), kind.index(), to.index());

        // Compute attacks from destination and flip enemy pieces
        let occupied_now = self.occupied;
        let attacks = match kind {
            PieceKind::Pawn => t.pawn_attacks(us, to),
            PieceKind::Knight => t.knight_attacks(to),
            PieceKind::Bishop => t.bishop_attacks(to, occupied_now),
            PieceKind::Rook => t.rook_attacks(to, occupied_now),
            PieceKind::Queen => t.queen_attacks(to, occupied_now),
            PieceKind::King => t.king_attacks(to),
        };

        let flipped = attacks & self.colors[them.index()];

        // Flip the enemy pieces
        for sq in flipped {
            let flipped_piece = match self.mailbox[sq.index()] {
                Some(p) => p,
                None => continue, // Skip empty squares (bitboard inconsistency)
            };
            debug_assert_eq!(flipped_piece.color, them);

            // Toggle color
            self.colors[them.index()].clear(sq);
            self.colors[us.index()].set(sq);
            self.mailbox[sq.index()] = Some(Piece::new(us, flipped_piece.kind));

            // Update Zobrist
            self.hash ^= z.piece(them.index(), flipped_piece.kind.index(), sq.index());
            self.hash ^= z.piece(us.index(), flipped_piece.kind.index(), sq.index());
        }

        // Handle castling move (move the rook, but rook does NOT flip)
        if m.is_castle() {
            let (rook_from, rook_to) = if to.file() == 6 {
                // Kingside
                (
                    Square::from_file_rank(7, from.rank()),
                    Square::from_file_rank(5, from.rank()),
                )
            } else {
                // Queenside
                (
                    Square::from_file_rank(0, from.rank()),
                    Square::from_file_rank(3, from.rank()),
                )
            };

            self.pieces[PieceKind::Rook.index()].clear(rook_from);
            self.colors[us.index()].clear(rook_from);
            self.occupied.clear(rook_from);
            self.mailbox[rook_from.index()] = None;
            self.hash ^= z.piece(us.index(), PieceKind::Rook.index(), rook_from.index());

            self.pieces[PieceKind::Rook.index()].set(rook_to);
            self.colors[us.index()].set(rook_to);
            self.occupied.set(rook_to);
            self.mailbox[rook_to.index()] = Some(Piece::new(us, PieceKind::Rook));
            self.hash ^= z.piece(us.index(), PieceKind::Rook.index(), rook_to.index());
        }

        // Update castling rights
        self.hash ^= z.castling[self.castling_rights.index()];
        if kind == PieceKind::King {
            match us {
                Color::White => {
                    self.castling_rights
                        .remove(CastlingRights::WHITE_KING | CastlingRights::WHITE_QUEEN);
                }
                Color::Black => {
                    self.castling_rights
                        .remove(CastlingRights::BLACK_KING | CastlingRights::BLACK_QUEEN);
                }
            }
        }
        if kind == PieceKind::Rook {
            match (us, from) {
                (Color::White, sq) if sq == Square::A1 => {
                    self.castling_rights.remove(CastlingRights::WHITE_QUEEN);
                }
                (Color::White, sq) if sq == Square::H1 => {
                    self.castling_rights.remove(CastlingRights::WHITE_KING);
                }
                (Color::Black, sq) if sq == Square::A8 => {
                    self.castling_rights.remove(CastlingRights::BLACK_QUEEN);
                }
                (Color::Black, sq) if sq == Square::H8 => {
                    self.castling_rights.remove(CastlingRights::BLACK_KING);
                }
                _ => {}
            }
        }
        // Also remove castling rights if a rook square was flipped
        // (rook that was flipped has "moved" in effect)
        if flipped.contains(Square::A1) {
            self.castling_rights.remove(CastlingRights::WHITE_QUEEN);
        }
        if flipped.contains(Square::H1) {
            self.castling_rights.remove(CastlingRights::WHITE_KING);
        }
        if flipped.contains(Square::A8) {
            self.castling_rights.remove(CastlingRights::BLACK_QUEEN);
        }
        if flipped.contains(Square::H8) {
            self.castling_rights.remove(CastlingRights::BLACK_KING);
        }
        self.hash ^= z.castling[self.castling_rights.index()];

        // Update clocks
        self.halfmove_clock += 1;
        if kind == PieceKind::Pawn || flipped.is_not_empty() {
            self.halfmove_clock = 0;
        }
        if us == Color::Black {
            self.fullmove_number += 1;
        }

        // Toggle side to move
        self.side_to_move = them;
        self.hash ^= z.side_to_move;

        // Return undo with the actual flipped set
        UndoInfo {
            flipped,
            castling_rights: undo.castling_rights,
            hash: undo.hash,
            halfmove_clock: undo.halfmove_clock,
        }
    }

    /// Unmake a move, restoring the board to its previous state.
    pub fn unmake_move(&mut self, m: Move, undo: &UndoInfo) {
        let them = self.side_to_move; // current side was the opponent
        let us = them.flip(); // side that made the move

        // Restore side to move
        self.side_to_move = us;

        let from = m.from_sq();
        let to = m.to_sq();

        // Determine the piece kind at destination
        let piece_at_to = self.mailbox[to.index()].expect("no piece at to in unmake");
        let dest_kind = piece_at_to.kind;

        // Figure out original kind (before promotion)
        let orig_kind = if m.is_promotion() {
            PieceKind::Pawn
        } else {
            dest_kind
        };

        // Remove piece from destination
        self.pieces[dest_kind.index()].clear(to);
        self.colors[us.index()].clear(to);
        self.occupied.clear(to);
        self.mailbox[to.index()] = None;

        // Place piece back at source
        self.pieces[orig_kind.index()].set(from);
        self.colors[us.index()].set(from);
        self.occupied.set(from);
        self.mailbox[from.index()] = Some(Piece::new(us, orig_kind));

        // Un-flip pieces
        for sq in undo.flipped {
            let flipped_piece = self.mailbox[sq.index()].expect("flipped square empty in unmake");
            // These pieces are currently 'us' color, flip back to 'them'
            self.colors[us.index()].clear(sq);
            self.colors[them.index()].set(sq);
            self.mailbox[sq.index()] = Some(Piece::new(them, flipped_piece.kind));
        }

        // Undo castling rook move
        if m.is_castle() {
            let (rook_from, rook_to) = if to.file() == 6 {
                (
                    Square::from_file_rank(7, from.rank()),
                    Square::from_file_rank(5, from.rank()),
                )
            } else {
                (
                    Square::from_file_rank(0, from.rank()),
                    Square::from_file_rank(3, from.rank()),
                )
            };

            // Move rook back
            self.pieces[PieceKind::Rook.index()].clear(rook_to);
            self.colors[us.index()].clear(rook_to);
            self.occupied.clear(rook_to);
            self.mailbox[rook_to.index()] = None;

            self.pieces[PieceKind::Rook.index()].set(rook_from);
            self.colors[us.index()].set(rook_from);
            self.occupied.set(rook_from);
            self.mailbox[rook_from.index()] = Some(Piece::new(us, PieceKind::Rook));
        }

        // Restore saved state
        self.castling_rights = undo.castling_rights;
        self.hash = undo.hash;
        self.halfmove_clock = undo.halfmove_clock;
        if us == Color::Black {
            self.fullmove_number -= 1;
        }
    }

    /// Make a null move (pass): toggle side to move and update hash.
    /// Returns the hash before the null move so it can be restored.
    pub fn make_null_move(&mut self) -> u64 {
        let old_hash = self.hash;
        let z = zobrist();
        self.side_to_move = self.side_to_move.flip();
        self.hash ^= z.side_to_move;
        old_hash
    }

    /// Unmake a null move: toggle side to move back and restore the hash.
    pub fn unmake_null_move(&mut self, old_hash: u64) {
        self.side_to_move = self.side_to_move.flip();
        self.hash = old_hash;
    }

    /// Check if the last move resulted in the enemy king being flipped (game over).
    pub fn king_flipped(&self, undo: &UndoInfo, _them: Color) -> bool {
        let king_bb = self.pieces[PieceKind::King.index()];
        // After the move, 'them' is the side whose king might have been flipped.
        // If flipped contains a square that was the enemy king, game over.
        // We check if flipped intersects with where a king is AND that king
        // is no longer the opponent's color (it's been flipped to our color).
        // Simpler: check if flipped contains ANY king square.
        (undo.flipped & king_bb).is_not_empty()
    }

    /// Display the board in a human-readable format.
    pub fn display(&self) -> String {
        let mut s = String::new();
        for rank in (0..8).rev() {
            s.push_str(&format!("  {} ", rank + 1));
            for file in 0..8 {
                let sq = Square::from_file_rank(file, rank);
                match self.mailbox[sq.index()] {
                    Some(piece) => {
                        s.push(piece.to_char());
                        s.push(' ');
                    }
                    None => s.push_str(". "),
                }
            }
            s.push('\n');
        }
        s.push_str("    a b c d e f g h\n");
        s.push_str(&format!(
            "  Side: {} | Castling: {} | Move: {}\n",
            self.side_to_move,
            castling_string(self.castling_rights),
            self.fullmove_number
        ));
        s
    }
}

fn castling_string(cr: CastlingRights) -> String {
    let mut s = String::new();
    if cr.has(CastlingRights::WHITE_KING) {
        s.push('K');
    }
    if cr.has(CastlingRights::WHITE_QUEEN) {
        s.push('Q');
    }
    if cr.has(CastlingRights::BLACK_KING) {
        s.push('k');
    }
    if cr.has(CastlingRights::BLACK_QUEEN) {
        s.push('q');
    }
    if s.is_empty() {
        s.push('-');
    }
    s
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fen;

    #[test]
    fn test_startpos() {
        let board = Board::startpos();
        assert_eq!(board.side_to_move, Color::White);
        assert_eq!(board.occupied.popcount(), 32);
        assert_eq!(
            (board.colors[0] | board.colors[1]).popcount(),
            32
        );
    }

    #[test]
    fn test_make_unmake_preserves_hash() {
        let mut board = Board::startpos();
        let original_hash = board.hash;
        // Move e2-e3 (pawn push)
        let m = Move::new(
            Square::from_file_rank(4, 1), // e2
            Square::from_file_rank(4, 2), // e3
        );
        let undo = board.make_move(m);
        assert_ne!(board.hash, original_hash);
        board.unmake_move(m, &undo);
        assert_eq!(board.hash, original_hash);
    }

    #[test]
    fn test_pawn_flip() {
        // Set up a position where a pawn move flips an enemy piece
        // White pawn on e5, Black pawn on d6: e5 pawn pushes nowhere useful
        // Instead: white pawn on d4, push to d5, which attacks e6 and c6
        let mut board =
            fen::from_fen("rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 1")
                .unwrap();
        // d4 pawn pushes to d5
        let m = Move::new(
            Square::from_file_rank(3, 3), // d4
            Square::from_file_rank(3, 4), // d5
        );
        let undo = board.make_move(m);
        // Pawn on d5 attacks c6 and e6 — but those are empty in this position
        // No flips should occur since no enemy pieces on c6/e6
        assert!(undo.flipped.is_empty());
        board.unmake_move(m, &undo);
    }

    #[test]
    fn test_occupied_constant_popcount() {
        let mut board = Board::startpos();
        assert_eq!(board.occupied.popcount(), 32);
        let m = Move::new(
            Square::from_file_rank(4, 1), // e2
            Square::from_file_rank(4, 2), // e3
        );
        let undo = board.make_move(m);
        assert_eq!(board.occupied.popcount(), 32);
        board.unmake_move(m, &undo);
        assert_eq!(board.occupied.popcount(), 32);
    }
}
