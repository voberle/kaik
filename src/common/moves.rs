//! Move encoding.
//! <https://www.chessprogramming.org/Encoding_Moves>

use std::fmt::Display;

use itertools::Itertools;

use crate::{common::Piece, common::Square};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move {
    // The minimum infortmation we need to encode a move.
    // Possible optimization: Store it as a u16, since from/to each fit in 6 bits.
    from: Square,
    to: Square,
    promotion: Option<Piece>,
    // Following information helps to avoid board lookups when applying moves.
    piece: Piece, // Piece performing the move
    is_capture: bool,
    // We can add more flags: Castling, double push pawn, en passant.
}

impl Move {
    pub const fn new(
        from: Square,
        to: Square,
        promotion: Option<Piece>,
        piece: Piece,
        is_capture: bool,
    ) -> Self {
        debug_assert!(match promotion {
            None => true,
            Some(p) => !p.is_pawn() && !p.is_king(),
        });
        Self {
            from,
            to,
            promotion,
            piece,
            is_capture,
        }
    }

    pub const fn quiet(from: Square, to: Square, piece: Piece) -> Self {
        Self::new(from, to, None, piece, false)
    }

    pub const fn capture(from: Square, to: Square, piece: Piece) -> Self {
        Self::new(from, to, None, piece, true)
    }

    pub fn get_from(self) -> Square {
        self.from
    }

    pub fn get_to(self) -> Square {
        self.to
    }

    pub fn get_promotion(self) -> Option<Piece> {
        self.promotion
    }

    pub fn get_piece(self) -> Piece {
        self.piece
    }

    pub fn is_capture(self) -> bool {
        self.is_capture
    }

    pub fn is_pawn_double_push(self) -> bool {
        self.piece.is_pawn() && self.from.get_rank().abs_diff(self.to.get_rank()) == 2
    }

    pub fn get_en_passant_target_square(self) -> Option<Square> {
        if self.is_pawn_double_push() {
            debug_assert_eq!(self.from.get_file(), self.to.get_file());
            let rank = (self.from.get_rank() + self.to.get_rank()) / 2;
            Some(Square::new(rank, self.from.get_file()))
        } else {
            None
        }
    }

    pub const KING_TO_KING_SIDE_CASTLING: [Move; 2] = [
        Move::quiet(Square::E1, Square::G1, Piece::WhiteKing),
        Move::quiet(Square::E8, Square::G8, Piece::BlackKing),
    ];

    pub const KING_TO_QUEEN_SIDE_CASTLING: [Move; 2] = [
        Move::quiet(Square::E1, Square::C1, Piece::WhiteKing),
        Move::quiet(Square::E8, Square::C8, Piece::BlackKing),
    ];

    // If this is a castling move, the move itself indicates the king move.
    // This function returns the extra rook move that needs to be done.
    pub fn get_castling_rook_move(self) -> Option<Move> {
        const WHITE_KING_SIDE: Option<Move> =
            Some(Move::quiet(Square::H1, Square::F1, Piece::WhiteRook));
        const WHITE_QUEEN_SIDE: Option<Move> =
            Some(Move::quiet(Square::A1, Square::D1, Piece::WhiteRook));
        const BLACK_KING_SIDE: Option<Move> =
            Some(Move::quiet(Square::H8, Square::F8, Piece::BlackRook));
        const BLACK_QUEEN_SIDE: Option<Move> =
            Some(Move::quiet(Square::A8, Square::D8, Piece::BlackRook));
        if self.piece.is_king() {
            if self.from == Square::E1 {
                // White
                if self.to == Square::G1 {
                    return WHITE_KING_SIDE;
                } else if self.to == Square::C1 {
                    return WHITE_QUEEN_SIDE;
                }
            } else if self.from == Square::E8 {
                // Black
                if self.to == Square::G8 {
                    return BLACK_KING_SIDE;
                } else if self.to == Square::C8 {
                    return BLACK_QUEEN_SIDE;
                }
            }
        }
        None
    }

    fn fmt_as_pure(self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Pure coordinate notation
        // <https://www.chessprogramming.org/Algebraic_Chess_Notation#Pure_coordinate_notation>
        let promotion = match self.get_promotion() {
            Some(Piece::WhiteQueen | Piece::BlackQueen) => "q",
            Some(Piece::WhiteRook | Piece::BlackRook) => "r",
            Some(Piece::WhiteBishop | Piece::BlackBishop) => "b",
            Some(Piece::WhiteKnight | Piece::BlackKnight) => "n",
            None => "",
            _ => panic!("Invalid promotion value"),
        };
        write!(f, "{}{}{}", self.get_from(), self.get_to(), promotion)
    }

    fn fmt_as_lan(self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Long Algebraic Notation
        // <https://www.chessprogramming.org/Algebraic_Chess_Notation#Long_Algebraic_Notation_.28LAN.29>
        let from = self.get_from().to_string().to_uppercase();
        let to = self.get_to().to_string().to_uppercase();
        let separator = if self.is_capture { 'x' } else { '-' };
        if self.piece.is_pawn() {
            let promotion = match self.get_promotion() {
                Some(Piece::WhiteQueen | Piece::BlackQueen) => "Q",
                Some(Piece::WhiteRook | Piece::BlackRook) => "R",
                Some(Piece::WhiteBishop | Piece::BlackBishop) => "B",
                Some(Piece::WhiteKnight | Piece::BlackKnight) => "N",
                None => "",
                _ => panic!("Invalid promotion value"),
            };
            write!(f, "{from}{separator}{to}{promotion}")
        } else {
            write!(f, "{}{from}{separator}{to}", self.get_piece())
        }
    }

    pub fn pure(&self) -> impl std::fmt::Display + '_ {
        struct Pure<'a>(&'a Move);
        impl<'a> std::fmt::Display for Pure<'a> {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                self.0.fmt_as_pure(f)
            }
        }
        Pure(self)
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_as_lan(f)
    }
}

pub fn format_moves_as_pure_string(moves: &[Move]) -> String {
    moves.iter().map(Move::pure).join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{Piece, Square};

    #[test]
    fn test_move_new() {
        let mv = Move::new(Square::E2, Square::E4, None, Piece::WhitePawn, false);
        assert_eq!(mv.get_from(), Square::E2);
        assert_eq!(mv.get_to(), Square::E4);
        assert_eq!(mv.get_promotion(), None);
        assert_eq!(mv.get_piece(), Piece::WhitePawn);
        assert_eq!(mv.is_capture(), false);
    }

    #[test]
    fn test_move_quiet() {
        let mv = Move::quiet(Square::E2, Square::E4, Piece::WhitePawn);
        assert_eq!(mv.get_from(), Square::E2);
        assert_eq!(mv.get_to(), Square::E4);
        assert_eq!(mv.get_promotion(), None);
        assert_eq!(mv.get_piece(), Piece::WhitePawn);
        assert_eq!(mv.is_capture(), false);
    }

    #[test]
    fn test_move_capture() {
        let mv = Move::capture(Square::E2, Square::E4, Piece::WhitePawn);
        assert_eq!(mv.get_from(), Square::E2);
        assert_eq!(mv.get_to(), Square::E4);
        assert_eq!(mv.get_promotion(), None);
        assert_eq!(mv.get_piece(), Piece::WhitePawn);
        assert_eq!(mv.is_capture(), true);
    }

    #[test]
    fn test_is_pawn_double_push() {
        let mv = Move::quiet(Square::E2, Square::E4, Piece::WhitePawn);
        assert!(mv.is_pawn_double_push());
        let mv = Move::quiet(Square::E2, Square::E3, Piece::WhitePawn);
        assert!(!mv.is_pawn_double_push());
    }

    #[test]
    fn test_get_en_passant_target_square() {
        let mv = Move::quiet(Square::E2, Square::E4, Piece::WhitePawn);
        assert_eq!(mv.get_en_passant_target_square(), Some(Square::E3));
        let mv = Move::quiet(Square::E2, Square::E3, Piece::WhitePawn);
        assert_eq!(mv.get_en_passant_target_square(), None);
    }

    #[test]
    fn test_get_castling_rook_move() {
        let mv = Move::quiet(Square::E1, Square::G1, Piece::WhiteKing);
        assert_eq!(
            mv.get_castling_rook_move(),
            Some(Move::quiet(Square::H1, Square::F1, Piece::WhiteRook))
        );
        let mv = Move::quiet(Square::E1, Square::C1, Piece::WhiteKing);
        assert_eq!(
            mv.get_castling_rook_move(),
            Some(Move::quiet(Square::A1, Square::D1, Piece::WhiteRook))
        );
        let mv = Move::quiet(Square::E8, Square::G8, Piece::BlackKing);
        assert_eq!(
            mv.get_castling_rook_move(),
            Some(Move::quiet(Square::H8, Square::F8, Piece::BlackRook))
        );
        let mv = Move::quiet(Square::E8, Square::C8, Piece::BlackKing);
        assert_eq!(
            mv.get_castling_rook_move(),
            Some(Move::quiet(Square::A8, Square::D8, Piece::BlackRook))
        );
    }

    #[test]
    fn test_fmt_as_pure() {
        let mv = Move::quiet(Square::E2, Square::E4, Piece::WhitePawn);
        assert_eq!(format!("{}", mv.pure()), "e2e4");
        let mv = Move::new(
            Square::E7,
            Square::E8,
            Some(Piece::WhiteQueen),
            Piece::WhitePawn,
            false,
        );
        assert_eq!(format!("{}", mv.pure()), "e7e8q");
    }

    #[test]
    fn test_fmt_as_lan() {
        let mv = Move::quiet(Square::E2, Square::E4, Piece::WhitePawn);
        assert_eq!(format!("{}", mv), "E2-E4");
        let mv = Move::capture(Square::E2, Square::E4, Piece::WhitePawn);
        assert_eq!(format!("{}", mv), "E2xE4");
        let mv = Move::new(
            Square::E7,
            Square::E8,
            Some(Piece::WhiteQueen),
            Piece::WhitePawn,
            false,
        );
        assert_eq!(format!("{}", mv), "E7-E8Q");
    }

    #[test]
    fn test_format_moves_as_string() {
        let moves = [
            Move::quiet(Square::E2, Square::E4, Piece::WhitePawn),
            Move::capture(Square::D7, Square::D8, Piece::BlackPawn),
        ];
        assert_eq!(format_moves_as_pure_string(&moves), "e2e4 d7d8");
    }
}
