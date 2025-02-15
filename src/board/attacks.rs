//! Check and attack detection.
//! <https://www.chessprogramming.org/Checks_and_Pinned_Pieces_(Bitboards)>
//! <https://www.chessprogramming.org/Square_Attacked_By#AnyAttackBySide>

use crate::{
    board::bitboard::{self, movements, BitBoard},
    common::{Color, Piece, Square},
};

use super::Board;

impl Board {
    // Is the side to play in check?
    pub fn in_check(&self) -> bool {
        self.attacks_king(self.get_side_to_move()) != 0
    }

    // Returns a bitboard indicating which squares attack the king of the specified color.
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

    // Returns a bitboard indicating which squares attack that square.
    pub fn attacks_to(&self, square: Square) -> BitBoard {
        // From <https://www.chessprogramming.org/Square_Attacked_By#AnyAttackBySide>

        let bb = bitboard::from_square(square);

        let white_pawns = self.pieces[Piece::WhitePawn as usize];
        let black_pawns = self.pieces[Piece::BlackPawn as usize];
        let knights =
            self.pieces[Piece::WhiteKnight as usize] | self.pieces[Piece::BlackKnight as usize];
        let kings = self.pieces[Piece::WhiteKing as usize] | self.pieces[Piece::BlackKing as usize];
        let mut rooks_queens =
            self.pieces[Piece::WhiteQueen as usize] | self.pieces[Piece::BlackQueen as usize];
        let mut bishops_queens = rooks_queens;
        rooks_queens |=
            self.pieces[Piece::WhiteRook as usize] | self.pieces[Piece::BlackRook as usize];
        bishops_queens |=
            self.pieces[Piece::WhiteBishop as usize] | self.pieces[Piece::BlackBishop as usize];

        (movements::get_white_pawn_attacks(bb) & black_pawns)
            | (movements::get_black_pawn_attacks(bb) & white_pawns)
            | (movements::get_knight_attacks(bb) & knights)
            | (movements::get_king_attacks(bb) & kings)
            | (movements::get_bishop_attacks(bb, self.occupied) & bishops_queens)
            | (movements::get_rook_attacks(bb, self.occupied) & rooks_queens)
    }
}

#[cfg(test)]
mod tests {
    use crate::common::Square;

    use super::*;

    #[test]
    fn test_attacks() {
        let board: Board = "4k3/5P2/5N2/1B6/8/8/8/4RK1R b Kkq - 1 1".into();
        let attacks_king_bb = board.attacks_king(Color::Black);
        let attacks_bb = board.attacks_to(Square::E8); // King's square
        assert_eq!(attacks_king_bb, attacks_bb);
    }

    #[test]
    fn test_attacks_king_king_next_to_king() {
        let board: Board = "8/2kp4/1K6/2P4r/8/8/8/8 w - - 1 2".into();
        let bb = board.attacks_king(Color::White);
        // Not allowed to move next to opponent king.
        assert_eq!(
            bb,
            0b0000000000000100000000000000000000000000000000000000000000000000
        );
    }
}
