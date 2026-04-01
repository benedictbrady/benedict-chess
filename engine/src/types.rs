use std::fmt;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
#[repr(u8)]
pub enum Color {
    White = 0,
    Black = 1,
}

impl Color {
    #[inline]
    pub fn flip(self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    #[inline]
    pub fn index(self) -> usize {
        self as usize
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::White => write!(f, "w"),
            Color::Black => write!(f, "b"),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
#[repr(u8)]
pub enum PieceKind {
    Pawn = 0,
    Knight = 1,
    Bishop = 2,
    Rook = 3,
    Queen = 4,
    King = 5,
}

impl PieceKind {
    pub const ALL: [PieceKind; 6] = [
        PieceKind::Pawn,
        PieceKind::Knight,
        PieceKind::Bishop,
        PieceKind::Rook,
        PieceKind::Queen,
        PieceKind::King,
    ];

    pub const PROMOTION: [PieceKind; 4] = [
        PieceKind::Knight,
        PieceKind::Bishop,
        PieceKind::Rook,
        PieceKind::Queen,
    ];

    #[inline]
    pub fn index(self) -> usize {
        self as usize
    }

    pub fn from_char(c: char) -> Option<PieceKind> {
        match c.to_ascii_lowercase() {
            'p' => Some(PieceKind::Pawn),
            'n' => Some(PieceKind::Knight),
            'b' => Some(PieceKind::Bishop),
            'r' => Some(PieceKind::Rook),
            'q' => Some(PieceKind::Queen),
            'k' => Some(PieceKind::King),
            _ => None,
        }
    }

    pub fn to_char(self) -> char {
        match self {
            PieceKind::Pawn => 'p',
            PieceKind::Knight => 'n',
            PieceKind::Bishop => 'b',
            PieceKind::Rook => 'r',
            PieceKind::Queen => 'q',
            PieceKind::King => 'k',
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Piece {
    pub color: Color,
    pub kind: PieceKind,
}

impl Piece {
    #[inline]
    pub fn new(color: Color, kind: PieceKind) -> Self {
        Self { color, kind }
    }

    pub fn from_char(c: char) -> Option<Piece> {
        let kind = PieceKind::from_char(c)?;
        let color = if c.is_ascii_uppercase() {
            Color::White
        } else {
            Color::Black
        };
        Some(Piece { color, kind })
    }

    pub fn to_char(self) -> char {
        let c = self.kind.to_char();
        if self.color == Color::White {
            c.to_ascii_uppercase()
        } else {
            c
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, PartialOrd, Ord)]
pub struct Square(pub u8);

impl Square {
    pub const A1: Square = Square(0);
    pub const B1: Square = Square(1);
    pub const C1: Square = Square(2);
    pub const D1: Square = Square(3);
    pub const E1: Square = Square(4);
    pub const F1: Square = Square(5);
    pub const G1: Square = Square(6);
    pub const H1: Square = Square(7);
    pub const A8: Square = Square(56);
    pub const B8: Square = Square(57);
    pub const C8: Square = Square(58);
    pub const D8: Square = Square(59);
    pub const E8: Square = Square(60);
    pub const F8: Square = Square(61);
    pub const G8: Square = Square(62);
    pub const H8: Square = Square(63);

    #[inline]
    pub fn new(index: u8) -> Self {
        debug_assert!(index < 64);
        Square(index)
    }

    #[inline]
    pub fn from_file_rank(file: u8, rank: u8) -> Self {
        debug_assert!(file < 8 && rank < 8);
        Square(rank * 8 + file)
    }

    #[inline]
    pub fn file(self) -> u8 {
        self.0 & 7
    }

    #[inline]
    pub fn rank(self) -> u8 {
        self.0 >> 3
    }

    #[inline]
    pub fn index(self) -> usize {
        self.0 as usize
    }

    pub fn from_algebraic(s: &str) -> Option<Square> {
        let bytes = s.as_bytes();
        if bytes.len() != 2 {
            return None;
        }
        let file = bytes[0].wrapping_sub(b'a');
        let rank = bytes[1].wrapping_sub(b'1');
        if file < 8 && rank < 8 {
            Some(Square::from_file_rank(file, rank))
        } else {
            None
        }
    }

    pub fn to_algebraic(self) -> String {
        let file = (b'a' + self.file()) as char;
        let rank = (b'1' + self.rank()) as char;
        format!("{}{}", file, rank)
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_algebraic())
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct CastlingRights(pub u8);

impl CastlingRights {
    pub const NONE: CastlingRights = CastlingRights(0);
    pub const WHITE_KING: u8 = 1;
    pub const WHITE_QUEEN: u8 = 2;
    pub const BLACK_KING: u8 = 4;
    pub const BLACK_QUEEN: u8 = 8;
    pub const ALL: CastlingRights = CastlingRights(15);

    #[inline]
    pub fn has(self, flag: u8) -> bool {
        self.0 & flag != 0
    }

    #[inline]
    pub fn remove(&mut self, flag: u8) {
        self.0 &= !flag;
    }

    #[inline]
    pub fn index(self) -> usize {
        self.0 as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_flip() {
        assert_eq!(Color::White.flip(), Color::Black);
        assert_eq!(Color::Black.flip(), Color::White);
    }

    #[test]
    fn test_square_file_rank() {
        let sq = Square::from_file_rank(4, 3); // e4
        assert_eq!(sq.file(), 4);
        assert_eq!(sq.rank(), 3);
        assert_eq!(sq.0, 28);
    }

    #[test]
    fn test_square_algebraic() {
        assert_eq!(Square::from_algebraic("e4"), Some(Square(28)));
        assert_eq!(Square::from_algebraic("a1"), Some(Square(0)));
        assert_eq!(Square::from_algebraic("h8"), Some(Square(63)));
        assert_eq!(Square(28).to_algebraic(), "e4");
    }

    #[test]
    fn test_piece_from_char() {
        assert_eq!(Piece::from_char('K'), Some(Piece::new(Color::White, PieceKind::King)));
        assert_eq!(Piece::from_char('p'), Some(Piece::new(Color::Black, PieceKind::Pawn)));
    }
}
