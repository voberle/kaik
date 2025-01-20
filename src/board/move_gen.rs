use super::Board;

use crate::{
    bitboard::BitBoard,
    common::{Piece, Square},
    moves::Move,
};

impl Board {
    fn can_castle_king_side(&self) -> bool {
        let side_to_move = self.get_side_to_move();
        let castling_mask = BitBoard::CASTLING_KING_SIDE_MASKS[side_to_move as usize];
        self.castling_ability.can_castle_king_side(side_to_move)
            && (self.occupied & castling_mask).is_empty()
    }

    fn can_castle_queen_side(&self) -> bool {
        let side_to_move = self.get_side_to_move();
        let castling_mask = BitBoard::CASTLING_QUEEN_SIDE_MASKS[side_to_move as usize];
        self.castling_ability.can_castle_queen_side(side_to_move)
            && (self.occupied & castling_mask).is_empty()
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
            for from_bb in pieces_bb.into_iter() {
                let from_square = from_bb.get_index().into();

                let moves_bb = match piece {
                    Piece::WhiteKing | Piece::BlackKing => from_bb.get_king_moves(own_bb),
                    Piece::WhiteKnight | Piece::BlackKnight => from_bb.get_knight_moves(own_bb),
                    Piece::WhitePawn => from_bb.get_white_pawn_moves(self.occupied, opposite_bb),
                    Piece::BlackPawn => from_bb.get_black_pawn_moves(self.occupied, opposite_bb),
                    Piece::WhiteBishop | Piece::BlackBishop => {
                        from_bb.get_bishop_moves(self.occupied, own_bb)
                    }
                    Piece::WhiteRook | Piece::BlackRook => {
                        from_bb.get_rook_moves(self.occupied, own_bb)
                    }
                    Piece::WhiteQueen | Piece::BlackQueen => {
                        from_bb.get_queen_moves(self.occupied, own_bb)
                    }
                };

                // Generate moves.
                for to_bb in moves_bb.into_iter() {
                    let to_square: Square = to_bb.get_index().into();
                    let is_capture = opposite_bb.intersects(to_bb);

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
                    let target_bb = en_passant.into();
                    let ep_attacks_bb = match piece {
                        Piece::WhitePawn => from_bb.get_valid_white_pawn_attacks(target_bb),
                        Piece::BlackPawn => from_bb.get_valid_black_pawn_attacks(target_bb),
                        _ => BitBoard::EMPTY,
                    };

                    moves_list.extend(
                        ep_attacks_bb.into_iter().map(|to_bb| {
                            Move::capture(from_square, to_bb.get_index().into(), piece)
                        }),
                    );
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
    fn test_generate_moves_white_king() {
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
    fn test_generate_moves_black_king() {
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
    fn test_generate_moves_white_knight() {
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
    fn test_generate_moves_white_pawn() {
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
    fn test_generate_moves_black_pawn() {
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
    fn test_generate_en_passant_attacks() {
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
}
