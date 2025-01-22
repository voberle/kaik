//! Check and attack detection.
//! <https://www.chessprogramming.org/Checks_and_Pinned_Pieces_(Bitboards)>
//! <https://www.chessprogramming.org/Square_Attacked_By#AnyAttackBySide>

use crate::{
    bitboard::{movements, BitBoard},
    common::{Color, Piece},
};

use super::Board;

impl Board {
    pub fn attacks_king(&self, king_color: Color) -> BitBoard {
        // From <https://www.chessprogramming.org/Checks_and_Pinned_Pieces_(Bitboards)>
        // Note that the example there doesn't check king creating checks.

        let king_bb = self.pieces[Piece::get_king_of(king_color) as usize];
        let opp_king_color = king_color.opposite();

        // Could be optimized a bit with things like:
        //   let opposite_pawns = self.pieces[Piece::BlackPawn as usize - king_color as usize];
        let opposite_pawns = self.pieces[Piece::get_pawn_of(opp_king_color) as usize];
        let opposite_knights = self.pieces[Piece::get_knight_of(opp_king_color) as usize];
        let opposite_king = self.pieces[Piece::get_king_of(opp_king_color) as usize];

        let opposite_rooks_queens = self.pieces[Piece::get_queen_of(opp_king_color) as usize]
            | self.pieces[Piece::get_rook_of(opp_king_color) as usize];
        let opposite_bishops_queens = self.pieces[Piece::get_queen_of(opp_king_color) as usize]
            | self.pieces[Piece::get_bishop_of(opp_king_color) as usize];

        let pawn_attacks = if king_color == Color::White {
            movements::get_white_pawn_attacks(king_bb)
        } else {
            movements::get_black_pawn_attacks(king_bb)
        };

        (pawn_attacks & opposite_pawns)
            | (movements::get_knight_attacks(king_bb) & opposite_knights)
            | (movements::get_king_attacks(king_bb) & opposite_king)
            | (movements::get_bishop_attacks(king_bb, self.occupied) & opposite_bishops_queens)
            | (movements::get_rook_attacks(king_bb, self.occupied) & opposite_rooks_queens)
    }
}

#[cfg(test)]
mod tests {
    use crate::bitboard;

    use super::*;

    #[test]
    fn test_attacks_king_king_next_to_king() {
        let board: Board = "8/2kp4/1K6/2P4r/8/8/8/8 w - - 1 2".into();
        board.print();
        let bb = board.attacks_king(Color::White);
        bitboard::print(bb);
        // Not allowed to move next to opponent king.
        assert_eq!(
            bb,
            0b0000000000000100000000000000000000000000000000000000000000000000
        );
    }
}
