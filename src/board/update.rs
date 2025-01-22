//! Board update by move.

use crate::{
    bitboard::{self, BitBoard}, common::Color, moves::Move
};

use super::Board;

impl Board {
    fn toggle_side(&mut self) {
        self.side_to_move = self.side_to_move.opposite();
    }

    // Updates the bitboards and castling rights only.
    // Update by Move explained at <https://www.chessprogramming.org/General_Setwise_Operations#UpdateByMove>
    fn update_bitboards_by_move(&mut self, mv: Move) {
        let color = mv.get_piece().get_color();
        let from_bb: BitBoard = bitboard::from_square(mv.get_from());
        let to_bb: BitBoard = bitboard::from_square(mv.get_to());
        let from_to_bb = from_bb ^ to_bb;

        self.pieces[mv.get_piece() as usize] ^= from_to_bb;
        self.all[color as usize] ^= from_to_bb;
        self.occupied ^= from_to_bb;

        if mv.is_capture() {
            // Loop over bitboards opposite color.
            for bb in self
                .pieces
                .iter_mut()
                .skip(color.opposite() as usize)
                .step_by(2)
            {
                if *bb & to_bb != 0 {
                    *bb ^= to_bb;
                    self.all[color.opposite() as usize] ^= to_bb;
                    self.occupied ^= to_bb;
                    break;
                }
            }
        }

        self.castling_ability.clear(mv.get_from());
        self.castling_ability.clear(mv.get_to()); // in case rook gets taken
    }

    // Updates the board with the specified move.
    pub fn update_by_move(&mut self, mv: Move) {
        self.update_bitboards_by_move(mv);

        if let Some(promote_to) = mv.get_promotion() {
            // Pawn was moved. We now need to switch it to the new piece.
            let to_bb: BitBoard = bitboard::from_square(mv.get_to());
            self.pieces[mv.get_piece() as usize] &= !to_bb;
            self.pieces[promote_to as usize] |= to_bb;
        }

        self.en_passant_target_square = mv.get_en_passant_target_square();

        if let Some(castling_rook_move) = mv.get_castling() {
            self.update_bitboards_by_move(castling_rook_move);
        }

        self.toggle_side();
    }
}

#[cfg(test)]
mod tests {
    use crate::{common::Piece::*, common::Square::*};

    use super::*;

    #[test]
    fn test_update_by_move_quiet() {
        let mut board = Board::initial_board();
        let mv = Move::quiet(B2, B3, WhitePawn);
        board.update_by_move(mv);
        assert_eq!(
            board.to_string(),
            "rnbqkbnr/pppppppp/8/8/8/1P6/P1PPPPPP/RNBQKBNR b KQkq - 0 1"
        );
    }

    #[test]
    fn test_update_by_move_capture() {
        let mut board: Board = "2k5/8/8/8/8/8/2Pp4/2K5 w - - 0 1".into();
        let mv = Move::capture(C1, D2, WhiteKing);
        board.update_by_move(mv);
        assert_eq!(board.to_string(), "2k5/8/8/8/8/8/2PK4/8 b - - 0 1");

        let mut board: Board =
            "rnbqkbnr/ppp1pppp/8/3p4/8/2N5/PPPPPPPP/R1BQKBNR w KQkq - 0 1".into();
        let mv = Move::capture(C3, D5, WhiteKnight);
        board.update_by_move(mv);
        assert_eq!(
            board.to_string(),
            "rnbqkbnr/ppp1pppp/8/3N4/8/8/PPPPPPPP/R1BQKBNR b KQkq - 0 1"
        );
    }

    #[test]
    fn test_update_by_move_double_push() {
        let mut board = Board::initial_board();
        let mv = Move::quiet(B2, B4, WhitePawn);
        board.update_by_move(mv);
        assert_eq!(
            board,
            "rnbqkbnr/pppppppp/8/8/1P6/8/P1PPPPPP/RNBQKBNR b KQkq b3 0 1".into()
        );
    }

    #[test]
    fn test_update_by_move_castling() {
        let mut board: Board = "4k3/8/8/8/8/8/PPPPPPPP/R3K1NR w Q - 0 1".into();
        let mv = Move::quiet(E1, C1, WhiteKing); // White queen side castle
        board.update_by_move(mv);
        assert_eq!(board, "4k3/8/8/8/8/8/PPPPPPPP/2KR2NR b - - 0 1".into());
    }

    #[test]
    fn test_update_by_move_castling_clearing() {
        let mut board: Board =
            "rnbqkbnr/ppp1pppp/3p4/8/8/5P2/PPPPP1PP/RNBQKBNR w KQkq - 0 1".into();
        let mv = Move::quiet(E1, F2, WhiteKing);
        board.update_by_move(mv);
        assert_eq!(
            board,
            "rnbqkbnr/ppp1pppp/3p4/8/8/5P2/PPPPPKPP/RNBQ1BNR b kq - 0 1".into()
        );
    }

    #[test]
    fn test_update_by_move_promotion() {
        let mut board: Board = "4k3/1P6/8/8/8/8/8/4K3 w - - 0 1".into();
        let mv = Move::new(B7, B8, Some(WhiteQueen), WhitePawn, false);
        board.update_by_move(mv);
        assert_eq!(board, "1Q2k3/8/8/8/8/8/8/4K3 b - - 0 1".into());
    }
}
