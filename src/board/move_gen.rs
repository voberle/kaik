//! Move generation.

use super::Board;

use crate::{
    board::bitboard::{self, movements},
    common::Move,
    common::{Piece, Square},
};

impl Board {
    fn can_castle_king_side(&self) -> bool {
        let side_to_move = self.get_side_to_move();
        self.castling_ability.can_castle_king_side(side_to_move)
            && movements::can_castle_king_side(self.occupied, side_to_move)
    }

    fn can_castle_queen_side(&self) -> bool {
        let side_to_move = self.get_side_to_move();
        self.castling_ability.can_castle_queen_side(side_to_move)
            && movements::can_castle_queen_side(self.occupied, side_to_move)
    }

    // Generate all possible moves from this board.
    pub fn generate_moves_for(&self, pieces: &[Piece]) -> Vec<Move> {
        // Pseudo-legal or legal ones?

        let mut moves_list = Vec::new();

        for &piece in pieces
            .iter()
            .filter(|p| self.get_side_to_move() == p.get_color())
        {
            let own_bb = self.all[self.get_side_to_move() as usize];
            let opposite_bb = self.all[self.opposite_side() as usize];

            let pieces_bb = self.pieces[piece as usize];
            for from_bb in bitboard::into_iter(pieces_bb) {
                let from_square = bitboard::get_index(from_bb).into();

                let moves_bb = match piece {
                    Piece::WhiteKing | Piece::BlackKing => {
                        movements::get_king_moves(from_bb, own_bb)
                    }
                    Piece::WhiteKnight | Piece::BlackKnight => {
                        movements::get_knight_moves(from_bb, own_bb)
                    }
                    Piece::WhitePawn => {
                        movements::get_white_pawn_moves(from_bb, self.occupied, opposite_bb)
                    }
                    Piece::BlackPawn => {
                        movements::get_black_pawn_moves(from_bb, self.occupied, opposite_bb)
                    }
                    Piece::WhiteBishop | Piece::BlackBishop => {
                        movements::get_bishop_moves(from_bb, self.occupied, own_bb)
                    }
                    Piece::WhiteRook | Piece::BlackRook => {
                        movements::get_rook_moves(from_bb, self.occupied, own_bb)
                    }
                    Piece::WhiteQueen | Piece::BlackQueen => {
                        movements::get_queen_moves(from_bb, self.occupied, own_bb)
                    }
                };

                // Generate moves.
                for to_bb in bitboard::into_iter(moves_bb) {
                    let to_square: Square = bitboard::get_index(to_bb).into();
                    let is_capture = opposite_bb & to_bb != 0;

                    // Promotions
                    if piece.is_pawn() && to_square.is_promotion_rank_for(self.get_side_to_move()) {
                        moves_list.extend(
                            Piece::PROMOTION_PIECES[self.get_side_to_move() as usize]
                                .iter()
                                .map(|&promotion_piece| {
                                    Move::new(
                                        from_square,
                                        to_square,
                                        Some(promotion_piece),
                                        piece,
                                        is_capture,
                                    )
                                }),
                        );
                    } else {
                        moves_list.push(Move::new(from_square, to_square, None, piece, is_capture));
                    }
                }

                // En passant.
                if let Some(en_passant) = self.en_passant_target_square {
                    let target_bb = bitboard::from_square(en_passant);
                    let ep_attacks_bb = match piece {
                        Piece::WhitePawn => {
                            movements::get_valid_white_pawn_attacks(from_bb, target_bb)
                        }
                        Piece::BlackPawn => {
                            movements::get_valid_black_pawn_attacks(from_bb, target_bb)
                        }
                        _ => 0,
                    };

                    moves_list.extend(bitboard::into_iter(ep_attacks_bb).map(|to_bb| {
                        Move::capture(from_square, bitboard::get_index(to_bb).into(), piece)
                    }));
                }
            }
        }

        // Castling
        if self.can_castle_king_side() {
            moves_list.push(Move::KING_TO_KING_SIDE_CASTLING[self.get_side_to_move() as usize]);
        }
        if self.can_castle_queen_side() {
            moves_list.push(Move::KING_TO_QUEEN_SIDE_CASTLING[self.get_side_to_move() as usize]);
        }

        moves_list
    }

    pub fn generate_moves(&self) -> Vec<Move> {
        self.generate_moves_for(&Piece::ALL_PIECES)
    }
}

#[cfg(test)]
mod tests {
    use crate::{common::Piece::*, common::Square::*};

    use super::*;
    #[test]
    fn test_white_king_moves() {
        let board: Board = "2k5/8/8/8/8/8/2Pp4/2K5 w - - 0 1".into();
        let moves = board.generate_moves_for(&[WhiteKing]);
        assert_eq!(
            moves,
            &[
                Move::quiet(C1, B1, WhiteKing),
                Move::quiet(C1, D1, WhiteKing),
                Move::quiet(C1, B2, WhiteKing),
                Move::capture(C1, D2, WhiteKing),
            ]
        );
    }

    #[test]
    fn test_black_king_moves() {
        let board: Board = "2k5/2Pp4/8/8/8/8/8/2K5 b - - 0 1".into();
        let moves = board.generate_moves_for(&[BlackKing]);
        assert_eq!(
            moves,
            &[
                Move::quiet(C8, B7, BlackKing),
                Move::capture(C8, C7, BlackKing),
                Move::quiet(C8, B8, BlackKing),
                Move::quiet(C8, D8, BlackKing),
            ]
        );
    }

    #[test]
    fn test_white_knight_moves() {
        let board: Board = "8/8/6p1/5N2/8/1N6/8/8 w - - 0 1".into();
        let moves = board.generate_moves_for(&[WhiteKnight]);
        assert_eq!(
            moves,
            &[
                Move::quiet(B3, A1, WhiteKnight),
                Move::quiet(B3, C1, WhiteKnight),
                Move::quiet(B3, D2, WhiteKnight),
                Move::quiet(B3, D4, WhiteKnight),
                Move::quiet(B3, A5, WhiteKnight),
                Move::quiet(B3, C5, WhiteKnight),
                Move::quiet(F5, E3, WhiteKnight),
                Move::quiet(F5, G3, WhiteKnight),
                Move::quiet(F5, D4, WhiteKnight),
                Move::quiet(F5, H4, WhiteKnight),
                Move::quiet(F5, D6, WhiteKnight),
                Move::quiet(F5, H6, WhiteKnight),
                Move::quiet(F5, E7, WhiteKnight),
                Move::quiet(F5, G7, WhiteKnight),
            ]
        );
    }

    #[test]
    fn test_white_pawn_moves() {
        let board: Board = "8/8/8/8/4N3/n1pB2P1/PPPPPPPP/8 w - - 0 1".into();
        let moves = board.generate_moves_for(&[WhitePawn]);
        assert_eq!(
            moves,
            &[
                Move::capture(B2, A3, WhitePawn),
                Move::quiet(B2, B3, WhitePawn),
                Move::capture(B2, C3, WhitePawn),
                Move::quiet(B2, B4, WhitePawn),
                Move::capture(D2, C3, WhitePawn),
                Move::quiet(E2, E3, WhitePawn),
                Move::quiet(F2, F3, WhitePawn),
                Move::quiet(F2, F4, WhitePawn),
                Move::quiet(H2, H3, WhitePawn),
                Move::quiet(H2, H4, WhitePawn),
                Move::quiet(G3, G4, WhitePawn),
            ]
        );
    }

    #[test]
    fn test_black_pawn_moves() {
        let board: Board = "8/pppppppp/n1pB2P1/4N3/8/8/8/8 b - - 0 1".into();
        let moves = board.generate_moves_for(&[BlackPawn]);
        assert_eq!(
            moves,
            &[
                Move::quiet(C6, C5, BlackPawn),
                Move::quiet(B7, B5, BlackPawn),
                Move::quiet(B7, B6, BlackPawn),
                Move::capture(C7, D6, BlackPawn),
                Move::capture(E7, D6, BlackPawn),
                Move::quiet(E7, E6, BlackPawn),
                Move::quiet(F7, F5, BlackPawn),
                Move::quiet(F7, F6, BlackPawn),
                Move::capture(F7, G6, BlackPawn),
                Move::quiet(H7, H5, BlackPawn),
                Move::capture(H7, G6, BlackPawn),
                Move::quiet(H7, H6, BlackPawn),
            ]
        );
    }

    #[test]
    fn test_en_passant_attacks_1() {
        // Two black pawns can take the same en passant white pawn.
        // Example from <https://www.chessprogramming.org/En_passant#En_passant_bugs>
        let board: Board = "2r3k1/1q1nbppp/r3p3/3pP3/pPpP4/P1Q2N2/2RN1PPP/2R4K b - b3 0 23".into();
        let moves = board.generate_moves_for(&[BlackPawn]);
        assert_eq!(
            moves,
            &[
                Move::capture(A4, B3, BlackPawn),
                Move::capture(C4, B3, BlackPawn),
                Move::quiet(F7, F5, BlackPawn),
                Move::quiet(F7, F6, BlackPawn),
                Move::quiet(G7, G5, BlackPawn),
                Move::quiet(G7, G6, BlackPawn),
                Move::quiet(H7, H5, BlackPawn),
                Move::quiet(H7, H6, BlackPawn),
            ]
        );
    }

    #[test]
    fn test_en_passant_attacks_2() {
        let board: Board = "8/8/8/3k4/2pP4/1B6/6K1/8 b - d3 0 2".into();
        let moves = board.generate_moves_for(&[BlackPawn]);
        assert_eq!(
            moves,
            &[
                Move::capture(C4, B3, BlackPawn),
                Move::quiet(C4, C3, BlackPawn), // Push, leaves the king in check.
                Move::capture(C4, D3, BlackPawn), // En passant, leaves the king in check.
            ]
        );
    }

    #[test]
    fn test_generate_castling() {
        let board: Board = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8".into();
        let moves = board.generate_moves_for(&[WhiteKing]);
        assert_eq!(
            moves,
            &[
                Move::quiet(E1, F1, WhiteKing),
                Move::quiet(E1, D2, WhiteKing),
                Move::capture(E1, F2, WhiteKing),
                Move::quiet(E1, G1, WhiteKing),
            ]
        );
    }
}
