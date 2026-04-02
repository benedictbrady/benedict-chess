use crate::bitboard::Bitboard;
use crate::types::{CastlingRights, PieceKind, Square};
use std::fmt;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Move(u32);

// Bit layout:
// 0-5:   from square
// 6-11:  to square
// 12-14: promotion piece (0=none, 1=Knight, 2=Bishop, 3=Rook, 4=Queen)
// 15-16: flags (0=normal, 1=castling, 2=promotion, 3=double pawn push)

const FROM_MASK: u32 = 0x3F;
const TO_MASK: u32 = 0x3F << 6;
const PROMO_MASK: u32 = 0x7 << 12;
const FLAG_MASK: u32 = 0x3 << 15;

pub const FLAG_NORMAL: u32 = 0;
pub const FLAG_CASTLE: u32 = 1 << 15;
pub const FLAG_PROMOTION: u32 = 2 << 15;
pub const FLAG_DOUBLE_PUSH: u32 = 3 << 15;

impl Move {
    pub const NULL: Move = Move(0);

    #[inline]
    pub fn new(from: Square, to: Square) -> Move {
        Move((from.0 as u32) | ((to.0 as u32) << 6))
    }

    #[inline]
    pub fn new_with_flags(from: Square, to: Square, flags: u32) -> Move {
        Move((from.0 as u32) | ((to.0 as u32) << 6) | flags)
    }

    #[inline]
    pub fn new_promotion(from: Square, to: Square, promo: PieceKind) -> Move {
        let promo_bits = match promo {
            PieceKind::Knight => 1u32,
            PieceKind::Bishop => 2,
            PieceKind::Rook => 3,
            PieceKind::Queen => 4,
            _ => 0,
        };
        Move((from.0 as u32) | ((to.0 as u32) << 6) | (promo_bits << 12) | FLAG_PROMOTION)
    }

    #[inline]
    pub fn from_sq(self) -> Square {
        Square((self.0 & FROM_MASK) as u8)
    }

    #[inline]
    pub fn to_sq(self) -> Square {
        Square(((self.0 & TO_MASK) >> 6) as u8)
    }

    #[inline]
    pub fn promotion(self) -> Option<PieceKind> {
        if self.0 & FLAG_MASK != FLAG_PROMOTION {
            return None;
        }
        match (self.0 & PROMO_MASK) >> 12 {
            1 => Some(PieceKind::Knight),
            2 => Some(PieceKind::Bishop),
            3 => Some(PieceKind::Rook),
            4 => Some(PieceKind::Queen),
            _ => None,
        }
    }

    #[inline]
    pub fn is_castle(self) -> bool {
        self.0 & FLAG_MASK == FLAG_CASTLE
    }

    #[inline]
    pub fn is_promotion(self) -> bool {
        self.0 & FLAG_MASK == FLAG_PROMOTION
    }

    #[inline]
    pub fn is_double_push(self) -> bool {
        self.0 & FLAG_MASK == FLAG_DOUBLE_PUSH
    }

    #[inline]
    pub fn is_null(self) -> bool {
        self.0 == 0
    }

    #[inline]
    pub fn raw(self) -> u32 {
        self.0
    }

    #[inline]
    pub fn from_raw(val: u32) -> Self {
        Move(val)
    }

    pub fn to_uci(self) -> String {
        let mut s = format!(
            "{}{}",
            self.from_sq().to_algebraic(),
            self.to_sq().to_algebraic()
        );
        if let Some(promo) = self.promotion() {
            s.push(promo.to_char());
        }
        s
    }

    pub fn from_uci(s: &str) -> Option<Move> {
        if s.len() < 4 {
            return None;
        }
        let from = Square::from_algebraic(&s[0..2])?;
        let to = Square::from_algebraic(&s[2..4])?;
        if s.len() > 4 {
            let promo = PieceKind::from_char(s.as_bytes()[4] as char)?;
            Some(Move::new_promotion(from, to, promo))
        } else {
            // We don't know flags from UCI alone; caller must resolve castling/double push
            Some(Move::new(from, to))
        }
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_uci())
    }
}

#[derive(Clone, Debug)]
pub struct UndoInfo {
    pub flipped: Bitboard,
    pub castling_rights: CastlingRights,
    pub hash: u64,
    pub halfmove_clock: u16,
}

pub struct MoveList {
    moves: [Move; 256],
    len: usize,
}

impl MoveList {
    pub fn new() -> Self {
        MoveList {
            moves: [Move::NULL; 256],
            len: 0,
        }
    }

    #[inline]
    pub fn push(&mut self, m: Move) {
        self.moves[self.len] = m;
        self.len += 1;
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[inline]
    pub fn get(&self, idx: usize) -> Move {
        self.moves[idx]
    }

    #[inline]
    pub fn swap(&mut self, i: usize, j: usize) {
        self.moves.swap(i, j);
    }

    pub fn as_slice(&self) -> &[Move] {
        &self.moves[..self.len]
    }
}

impl Default for MoveList {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_from_to() {
        let m = Move::new(Square(12), Square(28));
        assert_eq!(m.from_sq(), Square(12));
        assert_eq!(m.to_sq(), Square(28));
        assert!(!m.is_castle());
        assert!(!m.is_promotion());
    }

    #[test]
    fn test_promotion_move() {
        let m = Move::new_promotion(Square(52), Square(60), PieceKind::Queen);
        assert_eq!(m.from_sq(), Square(52));
        assert_eq!(m.to_sq(), Square(60));
        assert!(m.is_promotion());
        assert_eq!(m.promotion(), Some(PieceKind::Queen));
    }

    #[test]
    fn test_uci_roundtrip() {
        let m = Move::new(Square(12), Square(28));
        let uci = m.to_uci();
        assert_eq!(uci, "e2e4");
    }

    #[test]
    fn test_uci_promotion() {
        let m = Move::new_promotion(
            Square::from_file_rank(4, 6), // e7
            Square::from_file_rank(4, 7), // e8
            PieceKind::Queen,
        );
        assert_eq!(m.to_uci(), "e7e8q");
    }
}
