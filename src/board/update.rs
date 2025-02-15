//! Board update by move.

use crate::{
    board::bitboard::{self, BitBoard},
    common::{Color, Move, Piece},
};

use super::{zobrist::ZOBRIST_KEYS, Board};

impl Board {
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

        self.zobrist_key ^= ZOBRIST_KEYS.piece_key(mv.get_from(), mv.get_piece());
        self.zobrist_key ^= ZOBRIST_KEYS.piece_key(mv.get_to(), mv.get_piece());

        if mv.is_capture() {
            // If we are trying to move into the en-passant square, we need to correct the square we will clear.
            let to_bb_capture = if mv.get_piece().is_pawn()
                && matches!(self.en_passant_target_square, Some(sq) if sq == mv.get_to())
            {
                if color == Color::White {
                    to_bb >> 8
                } else {
                    to_bb << 8
                }
            } else {
                to_bb
            };

            // Loop over bitboards opposite color.
            for (piece_idx, bb) in self
                .pieces
                .iter_mut()
                .enumerate()
                .skip(color.opposite() as usize)
                .step_by(2)
            {
                if *bb & to_bb_capture != 0 {
                    // Remove the captured piece.
                    *bb ^= to_bb_capture;
                    self.all[color.opposite() as usize] ^= to_bb_capture;
                    self.occupied ^= to_bb_capture;

                    let captured_square = bitboard::get_index(to_bb_capture).into();
                    let piece_captured = Piece::ALL_PIECES[piece_idx];
                    self.zobrist_key ^= ZOBRIST_KEYS.piece_key(captured_square, piece_captured);

                    break;
                }
            }
        }

        self.zobrist_key ^= ZOBRIST_KEYS.castling_key(self.castling_ability);
        self.castling_ability.clear(mv.get_from());
        self.castling_ability.clear(mv.get_to()); // in case rook gets taken
        self.zobrist_key ^= ZOBRIST_KEYS.castling_key(self.castling_ability);
    }

    // Updates the board with the specified move.
    pub fn update_by_move(&mut self, mv: Move) {
        self.update_bitboards_by_move(mv);

        if let Some(promote_to) = mv.get_promotion() {
            // Pawn was moved. We now need to switch it to the new piece.
            let to_bb: BitBoard = bitboard::from_square(mv.get_to());
            self.pieces[mv.get_piece() as usize] &= !to_bb;
            self.pieces[promote_to as usize] |= to_bb;

            self.zobrist_key ^= ZOBRIST_KEYS.piece_key(mv.get_to(), mv.get_piece());
            self.zobrist_key ^= ZOBRIST_KEYS.piece_key(mv.get_to(), promote_to);
        }

        self.zobrist_key ^= ZOBRIST_KEYS.en_passant_key(self.en_passant_target_square);
        self.en_passant_target_square = mv.get_en_passant_target_square();
        self.zobrist_key ^= ZOBRIST_KEYS.en_passant_key(self.en_passant_target_square);

        if let Some(castling_rook_move) = mv.get_castling_rook_move() {
            self.update_bitboards_by_move(castling_rook_move);
        }

        // Update move counters
        if self.side_to_move == Color::Black {
            self.full_move_counter += 1;
        }
        if mv.is_capture() || mv.get_piece().is_pawn() {
            self.half_move_clock = 0;
        } else {
            self.half_move_clock += 1;
        }

        // Toggle side to move.
        self.zobrist_key ^= ZOBRIST_KEYS.color_key(self.get_side_to_move());
        self.side_to_move = self.side_to_move.opposite();
        self.zobrist_key ^= ZOBRIST_KEYS.color_key(self.get_side_to_move());

        // Checking that the Zobrist key was correctly updated (debug builds only).
        debug_assert_eq!(self.zobrist_key, Self::gen_zobrist_key(self));
    }

    // Applies the move to self and returns a new board.
    // Returns None if the move is not legal (king would be left in check).
    pub fn copy_with_move(&self, mv: Move) -> Option<Self> {
        debug_assert_eq!(self.get_side_to_move(), mv.get_piece().get_color());

        let mut board_copy = *self;
        board_copy.update_by_move(mv);

        // Drop the move if the king is left in check
        let king_color = mv.get_piece().get_color(); // Color that just moved.
        if board_copy.attacks_king(king_color) != 0 {
            return None;
        }

        if let Some(rook_mv) = mv.get_castling_rook_move() {
            // We are not allowed to be in check before the castling.
            if self.attacks_king(king_color) != 0 {
                return None;
            }

            // We need to check that the king doesn't pass over an attacked square.
            // That square is where the rook moves.
            if self.attacks_to(rook_mv.get_to()) & self.all[king_color.opposite() as usize] != 0 {
                return None;
            }
        }

        Some(board_copy)
    }
}

#[cfg(test)]
mod tests {
    use crate::common::{
        Piece::{self, *},
        Square::*,
    };

    use super::*;

    #[test]
    fn test_update_by_move() {
        let mut board = Board::initial_board();
        let mv = Move::quiet(B2, B3, WhitePawn);
        board.update_by_move(mv);
        assert_eq!(
            board.to_string(),
            "rnbqkbnr/pppppppp/8/8/8/1P6/P1PPPPPP/RNBQKBNR b KQkq - 0 1"
        );
        board.update_by_move(Move::quiet(G8, F6, BlackKnight));
        board.update_by_move(Move::quiet(G1, F3, WhiteKnight));
        assert_eq!(
            board.to_string(),
            "rnbqkb1r/pppppppp/5n2/8/8/1P3N2/P1PPPPPP/RNBQKB1R b KQkq - 2 2"
        );
        board.update_by_move(Move::quiet(B8, C6, BlackKnight));
        board.update_by_move(Move::quiet(C1, B2, WhiteBishop));
        board.update_by_move(Move::quiet(C6, B4, BlackKnight));
        assert_eq!(
            board.to_string(),
            "r1bqkb1r/pppppppp/5n2/8/1n6/1P3N2/PBPPPPPP/RN1QKB1R w KQkq - 5 4"
        );
        board.update_by_move(Move::capture(B2, F6, WhiteBishop));
        assert_eq!(
            board.to_string(),
            "r1bqkb1r/pppppppp/5B2/8/1n6/1P3N2/P1PPPPPP/RN1QKB1R b KQkq - 0 4"
        );
    }

    #[test]
    fn test_update_by_move_capture() {
        let mut board: Board = "2k5/8/8/8/8/8/2Pp4/2K5 w - - 3 1".into();
        let mv = Move::capture(C1, D2, WhiteKing);
        board.update_by_move(mv);
        assert_eq!(board.to_string(), "2k5/8/8/8/8/8/2PK4/8 b - - 0 1");

        let mut board: Board =
            "rnbqkbnr/ppp1pppp/8/3p4/8/2N5/PPPPPPPP/R1BQKBNR w KQkq - 4 1".into();
        let mv = Move::capture(C3, D5, WhiteKnight);
        board.update_by_move(mv);
        assert_eq!(
            board.to_string(),
            "rnbqkbnr/ppp1pppp/8/3N4/8/8/PPPPPPPP/R1BQKBNR b KQkq - 0 1"
        );
    }

    #[test]
    fn test_update_by_move_capture_2() {
        let mut board: Board = "8/8/8/3k4/2pP4/1B6/6K1/8 b - - 4 1".into();
        let mv = Move::capture(C4, B3, BlackPawn);
        board.update_by_move(mv);
        assert_eq!(board.to_string(), "8/8/8/3k4/3P4/1p6/6K1/8 w - - 0 2");
        assert_eq!(board.pieces[Piece::WhiteBishop as usize], 0);
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
        assert_eq!(board, "4k3/8/8/8/8/8/PPPPPPPP/2KR2NR b - - 1 1".into());
    }

    #[test]
    fn test_update_by_move_castling_clearing() {
        let mut board: Board =
            "rnbqkbnr/ppp1pppp/3p4/8/8/5P2/PPPPP1PP/RNBQKBNR w KQkq - 0 1".into();
        let mv = Move::quiet(E1, F2, WhiteKing);
        board.update_by_move(mv);
        assert_eq!(
            board,
            "rnbqkbnr/ppp1pppp/3p4/8/8/5P2/PPPPPKPP/RNBQ1BNR b kq - 1 1".into()
        );
    }

    #[test]
    fn test_update_by_move_promotion() {
        let mut board: Board = "4k3/1P6/8/8/8/8/8/4K3 w - - 2 1".into();
        let mv = Move::new(B7, B8, Some(WhiteQueen), WhitePawn, false);
        board.update_by_move(mv);
        assert_eq!(board, "1Q2k3/8/8/8/8/8/8/4K3 b - - 0 1".into());
    }

    #[test]
    fn test_update_by_move_en_passant_capture() {
        let mut board: Board = "rnbqkbnr/2pppppp/p7/Pp6/8/8/1PPPPPPP/RNBQKBNR w KQkq b6 0 3".into();
        let mv = Move::capture(A5, B6, WhitePawn);
        board.update_by_move(mv);
        assert_eq!(
            board,
            "rnbqkbnr/2pppppp/pP6/8/8/8/1PPPPPPP/RNBQKBNR b KQkq - 0 3".into()
        );
    }

    #[test]
    fn test_copy_with_move_in_check_castling() {
        let board: Board =
            "r3k2r/p1pp1pb1/bn2Qnp1/2qPN3/1p2P3/2N5/PPPBBPPP/R3K2R b KQkq - 3 2".into();
        let castling_mv = Move::quiet(E8, G8, BlackKing);
        // Not allowed to castle if in check.
        assert_eq!(board.copy_with_move(castling_mv), None);
    }

    #[test]
    fn test_copy_with_move_castling_over_attacked_square() {
        let board: Board = "r3k2r/1b4bq/8/8/8/8/7B/3RK2R b Kkq - 1 1".into();
        let castling_mv = Move::quiet(E8, C8, BlackKing);
        // Not allowed to castle over attacked square
        assert_eq!(board.copy_with_move(castling_mv), None);
    }

    #[test]
    fn test_copy_with_move_castling_rook_attacked() {
        let board: Board = "rnb2k1r/pp1Pbppp/2p5/q7/2B5/8/PPPQNnPP/RNB1K2R w KQ - 3 9".into();
        board.print();
        let castling_mv = Move::quiet(E1, G1, WhiteKing);
        // Rook is attacked, but castling is still allowed then.
        assert!(board.copy_with_move(castling_mv).is_some());
    }

    #[test]
    fn test_copy_with_move_king_moves_next_to_king() {
        let board: Board = "8/2kp4/8/K1P4r/8/8/8/8 w - - 1 2".into();
        let mv = Move::quiet(A5, B6, WhiteKing);
        // Not allowed to move next to opponent king.
        assert_eq!(board.copy_with_move(mv), None);
    }

    #[test]
    fn test_copy_with_move_en_passant() {
        let board: Board = "8/8/8/3k4/2pP4/1B6/6K1/8 b - d3 0 2".into();
        // Push or en passant taking is not allowed, as it leaves the king in check.
        let mv = Move::quiet(C4, C3, BlackPawn);
        assert_eq!(board.copy_with_move(mv), None);
        let mv = Move::capture(C4, D3, BlackPawn);
        assert_eq!(board.copy_with_move(mv), None);

        // But taking the attacker is.
        let mv = Move::capture(C4, B3, BlackPawn);
        assert!(board.copy_with_move(mv).is_some());
    }
}
