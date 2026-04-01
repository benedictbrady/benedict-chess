use crate::board::Board;
use crate::tables::tables;
use crate::types::{Color, PieceKind};

/// Tunable evaluation parameters.
#[derive(Clone, Debug)]
pub struct EvalParams {
    /// Piece values [Pawn, Knight, Bishop, Rook, Queen, King]
    pub piece_values: [i32; 6],
    /// Weight for PST scores (0 = disable PST)
    pub pst_weight: i32,
    /// Per-enemy-piece bonus for queen threats
    pub queen_threat_bonus: i32,
    /// Per-enemy-piece bonus for knight threats
    pub knight_threat_bonus: i32,
    /// Per-square mobility bonus for our pieces
    pub mobility_weight: i32,
    /// Bonus per friendly piece adjacent to our king
    pub king_shield_bonus: i32,
    /// Bonus for having more total pieces (piece count advantage)
    pub piece_count_weight: i32,
    /// Tempo bonus for side to move
    pub tempo_bonus: i32,
}

impl Default for EvalParams {
    fn default() -> Self {
        EvalParams {
            piece_values: [100, 320, 330, 500, 900, 20000],
            pst_weight: 100, // percent — 100 = full PST, 0 = none
            queen_threat_bonus: 15,
            knight_threat_bonus: 0,
            mobility_weight: 0,
            king_shield_bonus: 0,
            piece_count_weight: 0,
            tempo_bonus: 0,
        }
    }
}

impl EvalParams {
    pub fn name(&self) -> String {
        format!(
            "pv={:?} pst={} qt={} kt={} mob={} ks={} pc={} tempo={}",
            &self.piece_values[..5],
            self.pst_weight,
            self.queen_threat_bonus,
            self.knight_threat_bonus,
            self.mobility_weight,
            self.king_shield_bonus,
            self.piece_count_weight,
            self.tempo_bonus,
        )
    }
}

/// Piece-square tables from White's perspective.
const PAWN_PST: [i32; 64] = [
     0,  0,  0,  0,  0,  0,  0,  0,
     5, 10, 10,-20,-20, 10, 10,  5,
     5, -5,-10,  0,  0,-10, -5,  5,
     0,  0,  0, 20, 20,  0,  0,  0,
     5,  5, 10, 25, 25, 10,  5,  5,
    10, 10, 20, 30, 30, 20, 10, 10,
    50, 50, 50, 50, 50, 50, 50, 50,
     0,  0,  0,  0,  0,  0,  0,  0,
];

const KNIGHT_PST: [i32; 64] = [
    -50,-40,-30,-30,-30,-30,-40,-50,
    -40,-20,  0,  5,  5,  0,-20,-40,
    -30,  5, 10, 15, 15, 10,  5,-30,
    -30,  0, 15, 20, 20, 15,  0,-30,
    -30,  5, 15, 20, 20, 15,  5,-30,
    -30,  0, 10, 15, 15, 10,  0,-30,
    -40,-20,  0,  0,  0,  0,-20,-40,
    -50,-40,-30,-30,-30,-30,-40,-50,
];

const BISHOP_PST: [i32; 64] = [
    -20,-10,-10,-10,-10,-10,-10,-20,
    -10,  5,  0,  0,  0,  0,  5,-10,
    -10, 10, 10, 10, 10, 10, 10,-10,
    -10,  0, 10, 10, 10, 10,  0,-10,
    -10,  5,  5, 10, 10,  5,  5,-10,
    -10,  0,  5, 10, 10,  5,  0,-10,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -20,-10,-10,-10,-10,-10,-10,-20,
];

const ROOK_PST: [i32; 64] = [
     0,  0,  0,  5,  5,  0,  0,  0,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
     5, 10, 10, 10, 10, 10, 10,  5,
     0,  0,  0,  0,  0,  0,  0,  0,
];

const QUEEN_PST: [i32; 64] = [
    -20,-10,-10, -5, -5,-10,-10,-20,
    -10,  0,  5,  0,  0,  0,  0,-10,
    -10,  5,  5,  5,  5,  5,  0,-10,
      0,  0,  5,  5,  5,  5,  0, -5,
     -5,  0,  5,  5,  5,  5,  0, -5,
    -10,  0,  5,  5,  5,  5,  0,-10,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -20,-10,-10, -5, -5,-10,-10,-20,
];

const KING_PST: [i32; 64] = [
     20, 30, 10,  0,  0, 10, 30, 20,
     20, 20,  0,  0,  0,  0, 20, 20,
    -10,-20,-20,-20,-20,-20,-20,-10,
    -20,-30,-30,-40,-40,-30,-30,-20,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
];

const PST: [[i32; 64]; 6] = [
    PAWN_PST, KNIGHT_PST, BISHOP_PST, ROOK_PST, QUEEN_PST, KING_PST,
];

/// Evaluate with default params (backwards compat).
pub fn evaluate(board: &Board) -> i32 {
    evaluate_with_params(board, &EvalParams::default())
}

/// Evaluate the board with tunable parameters.
/// Returns score from the perspective of the side to move.
#[inline]
pub fn evaluate_with_params(board: &Board, params: &EvalParams) -> i32 {
    let t = tables();
    let mut score = 0i32;

    let white = board.colors[0];
    let black = board.colors[1];

    // Material + PST (hot loop — keep tight)
    for kind in PieceKind::ALL {
        let ki = kind.index();
        let value = params.piece_values[ki];
        let piece_bb = board.pieces[ki];

        // Fast material via popcount
        let w_count = (piece_bb & white).popcount() as i32;
        let b_count = (piece_bb & black).popcount() as i32;
        score += value * (w_count - b_count);

        // PST only when enabled
        if params.pst_weight > 0 {
            let pst = &PST[ki];
            for sq in piece_bb & white {
                score += pst[sq.index()] * params.pst_weight / 100;
            }
            for sq in piece_bb & black {
                score -= pst[sq.index() ^ 56] * params.pst_weight / 100;
            }
        }
    }

    // Queen threat bonus
    if params.queen_threat_bonus != 0 {
        let queen_bb = board.pieces[PieceKind::Queen.index()];
        let wq = queen_bb & white;
        if wq.is_not_empty() {
            let attacks = t.queen_attacks(wq.lsb(), board.occupied);
            score += (attacks & black).popcount() as i32 * params.queen_threat_bonus;
        }
        let bq = queen_bb & black;
        if bq.is_not_empty() {
            let attacks = t.queen_attacks(bq.lsb(), board.occupied);
            score -= (attacks & white).popcount() as i32 * params.queen_threat_bonus;
        }
    }

    // Knight threat bonus
    if params.knight_threat_bonus != 0 {
        let knight_bb = board.pieces[PieceKind::Knight.index()];
        for sq in knight_bb & white {
            score += (t.knight_attacks(sq) & black).popcount() as i32 * params.knight_threat_bonus;
        }
        for sq in knight_bb & black {
            score -= (t.knight_attacks(sq) & white).popcount() as i32 * params.knight_threat_bonus;
        }
    }

    // Mobility (expensive — only when enabled)
    if params.mobility_weight != 0 {
        let empty = !board.occupied;
        let mut w_mob = 0i32;
        let mut b_mob = 0i32;

        let occ = board.occupied;
        for sq in board.pieces[PieceKind::Knight.index()] & white {
            w_mob += (t.knight_attacks(sq) & empty).popcount() as i32;
        }
        for sq in board.pieces[PieceKind::Knight.index()] & black {
            b_mob += (t.knight_attacks(sq) & empty).popcount() as i32;
        }
        for sq in board.pieces[PieceKind::Bishop.index()] & white {
            w_mob += (t.bishop_attacks(sq, occ) & empty).popcount() as i32;
        }
        for sq in board.pieces[PieceKind::Bishop.index()] & black {
            b_mob += (t.bishop_attacks(sq, occ) & empty).popcount() as i32;
        }
        for sq in board.pieces[PieceKind::Rook.index()] & white {
            w_mob += (t.rook_attacks(sq, occ) & empty).popcount() as i32;
        }
        for sq in board.pieces[PieceKind::Rook.index()] & black {
            b_mob += (t.rook_attacks(sq, occ) & empty).popcount() as i32;
        }
        for sq in board.pieces[PieceKind::Queen.index()] & white {
            w_mob += (t.queen_attacks(sq, occ) & empty).popcount() as i32;
        }
        for sq in board.pieces[PieceKind::Queen.index()] & black {
            b_mob += (t.queen_attacks(sq, occ) & empty).popcount() as i32;
        }

        score += (w_mob - b_mob) * params.mobility_weight;
    }

    // King shield
    if params.king_shield_bonus != 0 {
        let wk = board.king_square(Color::White);
        let bk = board.king_square(Color::Black);
        let w_shield = (t.king_attacks(wk) & white).popcount() as i32;
        let b_shield = (t.king_attacks(bk) & black).popcount() as i32;
        score += (w_shield - b_shield) * params.king_shield_bonus;
    }

    // Piece count
    if params.piece_count_weight != 0 {
        score += (white.popcount() as i32 - black.popcount() as i32) * params.piece_count_weight;
    }

    // Tempo
    if params.tempo_bonus != 0 {
        score += if board.side_to_move == Color::White {
            params.tempo_bonus
        } else {
            -params.tempo_bonus
        };
    }

    // Return from side-to-move perspective
    if board.side_to_move == Color::Black { -score } else { score }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Board;

    #[test]
    fn test_startpos_roughly_zero() {
        let board = Board::startpos();
        let score = evaluate(&board);
        assert!(score.abs() < 100, "startpos eval: {}", score);
    }

    #[test]
    fn test_params_default_matches_evaluate() {
        let board = Board::startpos();
        let a = evaluate(&board);
        let b = evaluate_with_params(&board, &EvalParams::default());
        assert_eq!(a, b);
    }
}
