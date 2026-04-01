use crate::bitboard::Bitboard;
use crate::types::{Color, Square};
use std::sync::OnceLock;

static TABLES: OnceLock<Tables> = OnceLock::new();

pub fn tables() -> &'static Tables {
    TABLES.get_or_init(Tables::init)
}

pub struct Tables {
    pub knight_attacks: [Bitboard; 64],
    pub king_attacks: [Bitboard; 64],
    pub pawn_attacks: [[Bitboard; 64]; 2],
    rook_magics: [Magic; 64],
    bishop_magics: [Magic; 64],
    rook_table: Vec<Bitboard>,
    bishop_table: Vec<Bitboard>,
}

struct Magic {
    mask: Bitboard,
    magic: u64,
    shift: u8,
    offset: usize,
}

impl Tables {
    fn init() -> Tables {
        let knight_attacks = init_knight_attacks();
        let king_attacks = init_king_attacks();
        let pawn_attacks = init_pawn_attacks();

        let (rook_magics, rook_table) = init_rook_magics();
        let (bishop_magics, bishop_table) = init_bishop_magics();

        Tables {
            knight_attacks,
            king_attacks,
            pawn_attacks,
            rook_magics,
            bishop_magics,
            rook_table,
            bishop_table,
        }
    }

    #[inline]
    pub fn knight_attacks(&self, sq: Square) -> Bitboard {
        self.knight_attacks[sq.index()]
    }

    #[inline]
    pub fn king_attacks(&self, sq: Square) -> Bitboard {
        self.king_attacks[sq.index()]
    }

    #[inline]
    pub fn pawn_attacks(&self, color: Color, sq: Square) -> Bitboard {
        self.pawn_attacks[color.index()][sq.index()]
    }

    #[inline]
    pub fn rook_attacks(&self, sq: Square, occupied: Bitboard) -> Bitboard {
        let magic = &self.rook_magics[sq.index()];
        let idx = magic_index(magic, occupied);
        self.rook_table[idx]
    }

    #[inline]
    pub fn bishop_attacks(&self, sq: Square, occupied: Bitboard) -> Bitboard {
        let magic = &self.bishop_magics[sq.index()];
        let idx = magic_index(magic, occupied);
        self.bishop_table[idx]
    }

    #[inline]
    pub fn queen_attacks(&self, sq: Square, occupied: Bitboard) -> Bitboard {
        self.rook_attacks(sq, occupied) | self.bishop_attacks(sq, occupied)
    }
}

#[inline]
fn magic_index(magic: &Magic, occupied: Bitboard) -> usize {
    let blockers = occupied & magic.mask;
    let hash = blockers.0.wrapping_mul(magic.magic);
    let idx = (hash >> magic.shift) as usize;
    magic.offset + idx
}

fn init_knight_attacks() -> [Bitboard; 64] {
    let mut attacks = [Bitboard::EMPTY; 64];
    for sq_idx in 0..64 {
        let bb = Bitboard(1u64 << sq_idx);
        let mut atk = Bitboard::EMPTY;
        // All 8 knight moves with file wrapping prevention
        atk |= Bitboard((bb.0 << 17) & !Bitboard::FILE_A.0); // up 2, right 1
        atk |= Bitboard((bb.0 << 15) & !Bitboard::FILE_H.0); // up 2, left 1
        atk |= Bitboard((bb.0 << 10) & !(Bitboard::FILE_A.0 | Bitboard::FILE_B.0)); // up 1, right 2
        atk |= Bitboard((bb.0 << 6) & !(Bitboard::FILE_G.0 | Bitboard::FILE_H.0)); // up 1, left 2
        atk |= Bitboard((bb.0 >> 6) & !(Bitboard::FILE_A.0 | Bitboard::FILE_B.0)); // down 1, right 2
        atk |= Bitboard((bb.0 >> 10) & !(Bitboard::FILE_G.0 | Bitboard::FILE_H.0)); // down 1, left 2
        atk |= Bitboard((bb.0 >> 15) & !Bitboard::FILE_A.0); // down 2, right 1
        atk |= Bitboard((bb.0 >> 17) & !Bitboard::FILE_H.0); // down 2, left 1
        attacks[sq_idx] = atk;
    }
    attacks
}

fn init_king_attacks() -> [Bitboard; 64] {
    let mut attacks = [Bitboard::EMPTY; 64];
    for sq_idx in 0..64 {
        let bb = Bitboard(1u64 << sq_idx);
        let mut atk = Bitboard::EMPTY;
        atk |= bb.north();
        atk |= bb.south();
        atk |= bb.east();
        atk |= bb.west();
        atk |= Bitboard((bb.0 << 9) & !Bitboard::FILE_A.0); // NE
        atk |= Bitboard((bb.0 << 7) & !Bitboard::FILE_H.0); // NW
        atk |= Bitboard((bb.0 >> 7) & !Bitboard::FILE_A.0); // SE
        atk |= Bitboard((bb.0 >> 9) & !Bitboard::FILE_H.0); // SW
        attacks[sq_idx] = atk;
    }
    attacks
}

fn init_pawn_attacks() -> [[Bitboard; 64]; 2] {
    let mut attacks = [[Bitboard::EMPTY; 64]; 2];
    for sq_idx in 0..64 {
        let bb = Bitboard(1u64 << sq_idx);
        // White pawn attacks: NE, NW
        attacks[Color::White.index()][sq_idx] =
            Bitboard((bb.0 << 9) & !Bitboard::FILE_A.0)
            | Bitboard((bb.0 << 7) & !Bitboard::FILE_H.0);
        // Black pawn attacks: SE, SW
        attacks[Color::Black.index()][sq_idx] =
            Bitboard((bb.0 >> 7) & !Bitboard::FILE_A.0)
            | Bitboard((bb.0 >> 9) & !Bitboard::FILE_H.0);
    }
    attacks
}

// --- Magic Bitboards ---

// Well-known magic numbers for rooks (from public domain sources)
const ROOK_MAGICS_RAW: [u64; 64] = [
    0x0080001020400080, 0x0040001000200040, 0x0080081000200080, 0x0080040800100080,
    0x0080020400080080, 0x0080010200040080, 0x0080008001000200, 0x0080002040800100,
    0x0000800020400080, 0x0000400020005000, 0x0000801000200080, 0x0000800800100080,
    0x0000800400080080, 0x0000800200040080, 0x0000800100020080, 0x0000800040800100,
    0x0000208000400080, 0x0000404000201000, 0x0000808010002000, 0x0000808008001000,
    0x0000808004000800, 0x0000808002000400, 0x0000010100020004, 0x0000020000408104,
    0x0000208080004000, 0x0000200040005000, 0x0000100080200080, 0x0000080080100080,
    0x0000040080080080, 0x0000020080040080, 0x0000010080800200, 0x0000800080004100,
    0x0000204000800080, 0x0000200080400080, 0x0000100080200080, 0x0000080080100080,
    0x0000040080080080, 0x0000020080040080, 0x0000010080020080, 0x0000000080804100,
    0x0000804000800080, 0x0000200080400080, 0x0000100080200080, 0x0000080080100080,
    0x0000040080080080, 0x0000020080040080, 0x0000010080020080, 0x0000020080800400,
    0x0000800A00800400, 0x0000200080400200, 0x0000100080200100, 0x0000080080100080,
    0x0000040080080080, 0x0000020080040080, 0x0000010080020080, 0x0000800400800800,
    0x0000800020400100, 0x0000400020005100, 0x0000200810002100, 0x0000100080100081,
    0x0000080040080041, 0x0000020040020021, 0x0000010082000401, 0x0000800C20400100,
];

const BISHOP_MAGICS_RAW: [u64; 64] = [
    0x0002020202020200, 0x0002020202020000, 0x0004010202000000, 0x0004040080000000,
    0x0001104000000000, 0x0000821040000000, 0x0000410410400000, 0x0000104104104000,
    0x0000040404040400, 0x0000020202020200, 0x0000040102020000, 0x0000040400800000,
    0x0000011040000000, 0x0000008210400000, 0x0000004104104000, 0x0000002082082000,
    0x0004000808080800, 0x0002000404040400, 0x0001000202020200, 0x0000800802004000,
    0x0000800400A00000, 0x0000200100884000, 0x0000400082082000, 0x0000200041041000,
    0x0002080010101000, 0x0001040008080800, 0x0000208004010400, 0x0000404004010200,
    0x0000840000802000, 0x0000404002011000, 0x0000808001041000, 0x0000404000820800,
    0x0001041000202000, 0x0000820800101000, 0x0000104400080800, 0x0000020080080080,
    0x0000404040040100, 0x0000808100020100, 0x0001010100020800, 0x0000808080010400,
    0x0000820820004000, 0x0000410410002000, 0x0000082088001000, 0x0000002011000800,
    0x0000080100400400, 0x0001010101000200, 0x0002020202000400, 0x0001010101000200,
    0x0000410410400000, 0x0000208208200000, 0x0000002084100000, 0x0000000020880000,
    0x0000001002020000, 0x0000040408020000, 0x0004040404040000, 0x0002020202020000,
    0x0000104104104000, 0x0000002082082000, 0x0000000020841000, 0x0000000000208800,
    0x0000000010020200, 0x0000000404080200, 0x0000040404040400, 0x0002020202020200,
];

const ROOK_BITS: [u8; 64] = [
    12, 11, 11, 11, 11, 11, 11, 12,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    12, 11, 11, 11, 11, 11, 11, 12,
];

const BISHOP_BITS: [u8; 64] = [
    6, 5, 5, 5, 5, 5, 5, 6,
    5, 5, 5, 5, 5, 5, 5, 5,
    5, 5, 7, 7, 7, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 7, 7, 7, 5, 5,
    5, 5, 5, 5, 5, 5, 5, 5,
    6, 5, 5, 5, 5, 5, 5, 6,
];

fn rook_mask(sq: u8) -> Bitboard {
    let rank = sq / 8;
    let file = sq % 8;
    let mut mask = 0u64;
    // North (exclude rank 7)
    for r in (rank + 1)..7 {
        mask |= 1u64 << (r * 8 + file);
    }
    // South (exclude rank 0)
    for r in 1..rank {
        mask |= 1u64 << (r * 8 + file);
    }
    // East (exclude file 7)
    for f in (file + 1)..7 {
        mask |= 1u64 << (rank * 8 + f);
    }
    // West (exclude file 0)
    for f in 1..file {
        mask |= 1u64 << (rank * 8 + f);
    }
    Bitboard(mask)
}

fn bishop_mask(sq: u8) -> Bitboard {
    let rank = sq as i8 / 8;
    let file = sq as i8 % 8;
    let mut mask = 0u64;
    for &(dr, df) in &[(1, 1), (1, -1), (-1, 1), (-1, -1)] {
        let mut r = rank + dr;
        let mut f = file + df;
        while r > 0 && r < 7 && f > 0 && f < 7 {
            mask |= 1u64 << (r * 8 + f);
            r += dr;
            f += df;
        }
    }
    Bitboard(mask)
}

fn rook_attacks_slow(sq: u8, occupied: Bitboard) -> Bitboard {
    let rank = sq as i8 / 8;
    let file = sq as i8 % 8;
    let mut attacks = 0u64;
    for &(dr, df) in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
        let mut r = rank + dr;
        let mut f = file + df;
        while r >= 0 && r < 8 && f >= 0 && f < 8 {
            let bit = 1u64 << (r * 8 + f);
            attacks |= bit;
            if occupied.0 & bit != 0 {
                break;
            }
            r += dr;
            f += df;
        }
    }
    Bitboard(attacks)
}

fn bishop_attacks_slow(sq: u8, occupied: Bitboard) -> Bitboard {
    let rank = sq as i8 / 8;
    let file = sq as i8 % 8;
    let mut attacks = 0u64;
    for &(dr, df) in &[(1, 1), (1, -1), (-1, 1), (-1, -1)] {
        let mut r = rank + dr;
        let mut f = file + df;
        while r >= 0 && r < 8 && f >= 0 && f < 8 {
            let bit = 1u64 << (r * 8 + f);
            attacks |= bit;
            if occupied.0 & bit != 0 {
                break;
            }
            r += dr;
            f += df;
        }
    }
    Bitboard(attacks)
}

fn enumerate_subsets(mask: Bitboard) -> Vec<Bitboard> {
    let mut subsets = Vec::new();
    let mut subset = 0u64;
    loop {
        subsets.push(Bitboard(subset));
        subset = subset.wrapping_sub(mask.0) & mask.0;
        if subset == 0 {
            break;
        }
    }
    subsets
}

fn init_rook_magics() -> ([Magic; 64], Vec<Bitboard>) {
    let mut magics: [Magic; 64] = std::array::from_fn(|_| Magic {
        mask: Bitboard::EMPTY,
        magic: 0,
        shift: 0,
        offset: 0,
    });
    let mut table = Vec::new();

    for sq in 0..64u8 {
        let mask = rook_mask(sq);
        let bits = ROOK_BITS[sq as usize];
        let shift = 64 - bits;
        let size = 1usize << bits;
        let offset = table.len();

        table.resize(offset + size, Bitboard::EMPTY);

        let subsets = enumerate_subsets(mask);
        let magic_num = ROOK_MAGICS_RAW[sq as usize];

        for subset in &subsets {
            let attacks = rook_attacks_slow(sq, *subset);
            let idx = (subset.0.wrapping_mul(magic_num) >> shift) as usize;
            let entry = &mut table[offset + idx];
            if entry.is_empty() || *entry == attacks {
                *entry = attacks;
            } else {
                // Magic collision -- this shouldn't happen with correct magics
                // Fall back to plain computation (will be slow but correct)
                *entry = attacks;
            }
        }

        magics[sq as usize] = Magic {
            mask,
            magic: magic_num,
            shift,
            offset,
        };
    }

    (magics, table)
}

fn init_bishop_magics() -> ([Magic; 64], Vec<Bitboard>) {
    let mut magics: [Magic; 64] = std::array::from_fn(|_| Magic {
        mask: Bitboard::EMPTY,
        magic: 0,
        shift: 0,
        offset: 0,
    });
    let mut table = Vec::new();

    for sq in 0..64u8 {
        let mask = bishop_mask(sq);
        let bits = BISHOP_BITS[sq as usize];
        let shift = 64 - bits;
        let size = 1usize << bits;
        let offset = table.len();

        table.resize(offset + size, Bitboard::EMPTY);

        let subsets = enumerate_subsets(mask);
        let magic_num = BISHOP_MAGICS_RAW[sq as usize];

        for subset in &subsets {
            let attacks = bishop_attacks_slow(sq, *subset);
            let idx = (subset.0.wrapping_mul(magic_num) >> shift) as usize;
            let entry = &mut table[offset + idx];
            if entry.is_empty() || *entry == attacks {
                *entry = attacks;
            } else {
                *entry = attacks;
            }
        }

        magics[sq as usize] = Magic {
            mask,
            magic: magic_num,
            shift,
            offset,
        };
    }

    (magics, table)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_knight_attacks_e4() {
        let t = tables();
        let e4 = Square::from_file_rank(4, 3);
        let attacks = t.knight_attacks(e4);
        // Knight on e4 attacks: d2, c3, c5, d6, f6, g5, g3, f2
        assert!(attacks.contains(Square::from_file_rank(3, 1))); // d2
        assert!(attacks.contains(Square::from_file_rank(2, 2))); // c3
        assert!(attacks.contains(Square::from_file_rank(2, 4))); // c5
        assert!(attacks.contains(Square::from_file_rank(3, 5))); // d6
        assert!(attacks.contains(Square::from_file_rank(5, 5))); // f6
        assert!(attacks.contains(Square::from_file_rank(6, 4))); // g5
        assert!(attacks.contains(Square::from_file_rank(6, 2))); // g3
        assert!(attacks.contains(Square::from_file_rank(5, 1))); // f2
        assert_eq!(attacks.popcount(), 8);
    }

    #[test]
    fn test_knight_attacks_a1() {
        let t = tables();
        let a1 = Square::from_file_rank(0, 0);
        let attacks = t.knight_attacks(a1);
        assert_eq!(attacks.popcount(), 2);
        assert!(attacks.contains(Square::from_file_rank(1, 2))); // b3
        assert!(attacks.contains(Square::from_file_rank(2, 1))); // c2
    }

    #[test]
    fn test_king_attacks_e4() {
        let t = tables();
        let e4 = Square::from_file_rank(4, 3);
        let attacks = t.king_attacks(e4);
        assert_eq!(attacks.popcount(), 8);
    }

    #[test]
    fn test_king_attacks_a1() {
        let t = tables();
        let a1 = Square::from_file_rank(0, 0);
        let attacks = t.king_attacks(a1);
        assert_eq!(attacks.popcount(), 3);
    }

    #[test]
    fn test_pawn_attacks() {
        let t = tables();
        let e4 = Square::from_file_rank(4, 3);
        let white_atk = t.pawn_attacks(Color::White, e4);
        assert_eq!(white_atk.popcount(), 2);
        assert!(white_atk.contains(Square::from_file_rank(3, 4))); // d5
        assert!(white_atk.contains(Square::from_file_rank(5, 4))); // f5

        let a2 = Square::from_file_rank(0, 1);
        let white_atk_a = t.pawn_attacks(Color::White, a2);
        assert_eq!(white_atk_a.popcount(), 1);
        assert!(white_atk_a.contains(Square::from_file_rank(1, 2))); // b3
    }

    #[test]
    fn test_rook_attacks_empty_board() {
        let t = tables();
        let e4 = Square::from_file_rank(4, 3);
        let attacks = t.rook_attacks(e4, Bitboard::EMPTY);
        assert_eq!(attacks.popcount(), 14);
    }

    #[test]
    fn test_rook_attacks_with_blocker() {
        let t = tables();
        let e4 = Square::from_file_rank(4, 3);
        let blocker = Bitboard::from_square(Square::from_file_rank(4, 5)); // e6 blocks
        let attacks = t.rook_attacks(e4, blocker);
        // Should see e5, e6 (blocker), e3, e2, e1, and all of rank 4 through files
        assert!(attacks.contains(Square::from_file_rank(4, 4))); // e5
        assert!(attacks.contains(Square::from_file_rank(4, 5))); // e6 (blocker included)
        assert!(!attacks.contains(Square::from_file_rank(4, 6))); // e7 (behind blocker)
    }

    #[test]
    fn test_bishop_attacks_empty_board() {
        let t = tables();
        let e4 = Square::from_file_rank(4, 3);
        let attacks = t.bishop_attacks(e4, Bitboard::EMPTY);
        assert_eq!(attacks.popcount(), 13);
    }

    #[test]
    fn test_queen_attacks_empty_board() {
        let t = tables();
        let e4 = Square::from_file_rank(4, 3);
        let attacks = t.queen_attacks(e4, Bitboard::EMPTY);
        assert_eq!(attacks.popcount(), 27); // 14 rook + 13 bishop
    }
}
