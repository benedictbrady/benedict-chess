use crate::types::Square;
use std::fmt;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, Shr};

#[derive(Copy, Clone, Eq, PartialEq, Default, Hash)]
pub struct Bitboard(pub u64);

impl Bitboard {
    pub const EMPTY: Bitboard = Bitboard(0);
    pub const ALL: Bitboard = Bitboard(!0u64);

    pub const FILE_A: Bitboard = Bitboard(0x0101010101010101);
    pub const FILE_B: Bitboard = Bitboard(0x0202020202020202);
    pub const FILE_G: Bitboard = Bitboard(0x4040404040404040);
    pub const FILE_H: Bitboard = Bitboard(0x8080808080808080);
    pub const RANK_1: Bitboard = Bitboard(0xFF);
    pub const RANK_2: Bitboard = Bitboard(0xFF00);
    pub const RANK_3: Bitboard = Bitboard(0xFF0000);
    pub const RANK_4: Bitboard = Bitboard(0xFF000000);
    pub const RANK_5: Bitboard = Bitboard(0xFF00000000);
    pub const RANK_6: Bitboard = Bitboard(0xFF0000000000);
    pub const RANK_7: Bitboard = Bitboard(0xFF000000000000);
    pub const RANK_8: Bitboard = Bitboard(0xFF00000000000000);

    pub const NOT_FILE_A: Bitboard = Bitboard(!0x0101010101010101);
    pub const NOT_FILE_H: Bitboard = Bitboard(!0x8080808080808080);
    pub const NOT_FILE_AB: Bitboard = Bitboard(!0x0303030303030303);
    pub const NOT_FILE_GH: Bitboard = Bitboard(!0xC0C0C0C0C0C0C0C0);

    #[inline]
    pub fn from_square(sq: Square) -> Bitboard {
        Bitboard(1u64 << sq.0)
    }

    #[inline]
    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    #[inline]
    pub fn is_not_empty(self) -> bool {
        self.0 != 0
    }

    #[inline]
    pub fn contains(self, sq: Square) -> bool {
        self.0 & (1u64 << sq.0) != 0
    }

    #[inline]
    pub fn set(&mut self, sq: Square) {
        self.0 |= 1u64 << sq.0;
    }

    #[inline]
    pub fn clear(&mut self, sq: Square) {
        self.0 &= !(1u64 << sq.0);
    }

    #[inline]
    pub fn toggle(&mut self, sq: Square) {
        self.0 ^= 1u64 << sq.0;
    }

    #[inline]
    pub fn popcount(self) -> u32 {
        self.0.count_ones()
    }

    #[inline]
    pub fn lsb(self) -> Square {
        debug_assert!(self.is_not_empty());
        Square(self.0.trailing_zeros() as u8)
    }

    #[inline]
    pub fn pop_lsb(&mut self) -> Square {
        let sq = self.lsb();
        self.0 &= self.0 - 1;
        sq
    }

    #[inline]
    pub fn north(self) -> Bitboard {
        Bitboard(self.0 << 8)
    }

    #[inline]
    pub fn south(self) -> Bitboard {
        Bitboard(self.0 >> 8)
    }

    #[inline]
    pub fn east(self) -> Bitboard {
        Bitboard((self.0 << 1) & Self::NOT_FILE_A.0)
    }

    #[inline]
    pub fn west(self) -> Bitboard {
        Bitboard((self.0 >> 1) & Self::NOT_FILE_H.0)
    }
}

impl BitAnd for Bitboard {
    type Output = Bitboard;
    #[inline]
    fn bitand(self, rhs: Bitboard) -> Bitboard {
        Bitboard(self.0 & rhs.0)
    }
}

impl BitAndAssign for Bitboard {
    #[inline]
    fn bitand_assign(&mut self, rhs: Bitboard) {
        self.0 &= rhs.0;
    }
}

impl BitOr for Bitboard {
    type Output = Bitboard;
    #[inline]
    fn bitor(self, rhs: Bitboard) -> Bitboard {
        Bitboard(self.0 | rhs.0)
    }
}

impl BitOrAssign for Bitboard {
    #[inline]
    fn bitor_assign(&mut self, rhs: Bitboard) {
        self.0 |= rhs.0;
    }
}

impl BitXor for Bitboard {
    type Output = Bitboard;
    #[inline]
    fn bitxor(self, rhs: Bitboard) -> Bitboard {
        Bitboard(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for Bitboard {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Bitboard) {
        self.0 ^= rhs.0;
    }
}

impl Not for Bitboard {
    type Output = Bitboard;
    #[inline]
    fn not(self) -> Bitboard {
        Bitboard(!self.0)
    }
}

impl Shl<u8> for Bitboard {
    type Output = Bitboard;
    #[inline]
    fn shl(self, rhs: u8) -> Bitboard {
        Bitboard(self.0 << rhs)
    }
}

impl Shr<u8> for Bitboard {
    type Output = Bitboard;
    #[inline]
    fn shr(self, rhs: u8) -> Bitboard {
        Bitboard(self.0 >> rhs)
    }
}

pub struct BitboardIter(Bitboard);

impl Iterator for BitboardIter {
    type Item = Square;

    #[inline]
    fn next(&mut self) -> Option<Square> {
        if self.0.is_empty() {
            None
        } else {
            Some(self.0.pop_lsb())
        }
    }
}

impl IntoIterator for Bitboard {
    type Item = Square;
    type IntoIter = BitboardIter;

    #[inline]
    fn into_iter(self) -> BitboardIter {
        BitboardIter(self)
    }
}

impl fmt::Debug for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        for rank in (0..8).rev() {
            write!(f, "  {} ", rank + 1)?;
            for file in 0..8 {
                let sq = Square::from_file_rank(file, rank);
                if self.contains(sq) {
                    write!(f, "X ")?;
                } else {
                    write!(f, ". ")?;
                }
            }
            writeln!(f)?;
        }
        writeln!(f, "    a b c d e f g h")?;
        Ok(())
    }
}

impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_square() {
        let bb = Bitboard::from_square(Square(0));
        assert_eq!(bb.0, 1);
        let bb = Bitboard::from_square(Square(63));
        assert_eq!(bb.0, 1u64 << 63);
    }

    #[test]
    fn test_set_clear_contains() {
        let mut bb = Bitboard::EMPTY;
        let sq = Square(28);
        assert!(!bb.contains(sq));
        bb.set(sq);
        assert!(bb.contains(sq));
        bb.clear(sq);
        assert!(!bb.contains(sq));
    }

    #[test]
    fn test_popcount() {
        let bb = Bitboard(0xFF);
        assert_eq!(bb.popcount(), 8);
    }

    #[test]
    fn test_lsb_pop() {
        let mut bb = Bitboard(0b1010);
        assert_eq!(bb.lsb(), Square(1));
        let sq = bb.pop_lsb();
        assert_eq!(sq, Square(1));
        assert_eq!(bb, Bitboard(0b1000));
    }

    #[test]
    fn test_iterator() {
        let bb = Bitboard(0b10101);
        let squares: Vec<Square> = bb.into_iter().collect();
        assert_eq!(squares, vec![Square(0), Square(2), Square(4)]);
    }

    #[test]
    fn test_shifts() {
        let bb = Bitboard::from_square(Square::from_file_rank(3, 3)); // d4
        let north = bb.north();
        assert!(north.contains(Square::from_file_rank(3, 4))); // d5
        let south = bb.south();
        assert!(south.contains(Square::from_file_rank(3, 2))); // d3
    }
}
