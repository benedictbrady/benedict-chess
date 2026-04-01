use std::sync::OnceLock;

static ZOBRIST: OnceLock<ZobristKeys> = OnceLock::new();

pub fn zobrist() -> &'static ZobristKeys {
    ZOBRIST.get_or_init(ZobristKeys::init)
}

pub struct ZobristKeys {
    pub piece_square: [[[u64; 64]; 6]; 2], // [color][piece_kind][square]
    pub castling: [u64; 16],
    pub side_to_move: u64,
}

impl ZobristKeys {
    fn init() -> ZobristKeys {
        // Simple xorshift64 PRNG with fixed seed for deterministic keys
        let mut state: u64 = 0x12345678DEADBEEF;

        let mut next = || -> u64 {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state
        };

        let mut piece_square = [[[0u64; 64]; 6]; 2];
        for color in 0..2 {
            for kind in 0..6 {
                for sq in 0..64 {
                    piece_square[color][kind][sq] = next();
                }
            }
        }

        let mut castling = [0u64; 16];
        for c in &mut castling {
            *c = next();
        }

        let side_to_move = next();

        ZobristKeys {
            piece_square,
            castling,
            side_to_move,
        }
    }

    #[inline]
    pub fn piece(&self, color: usize, kind: usize, sq: usize) -> u64 {
        self.piece_square[color][kind][sq]
    }
}
