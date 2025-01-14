//! Move encoding.
//! <https://www.chessprogramming.org/Encoding_Moves>

use std::fmt::Display;

use crate::{common::Piece, common::Square};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move {
    // The minimum infortmation we need to encode a move.
    // Possible optimization: Store it as a u16, since from/to each fit in 6 bits.
    from: Square,
    to: Square,
    promotion: Option<Piece>,
    // Following information helps to avoid board lookups when applying moves.
    piece: Piece,                  // Piece performing the move
    captured_piece: Option<Piece>, // Piece being captured.
}

impl Move {
    pub fn new(
        from: Square,
        to: Square,
        promotion: Option<Piece>,
        piece: Piece,
        captured_piece: Option<Piece>,
    ) -> Self {
        assert!(promotion.is_none_or(|p| !p.is_pawn() && !p.is_king()));
        Self {
            from,
            to,
            promotion,
            piece,
            captured_piece,
        }
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

    pub fn get_captured_piece(self) -> Option<Piece> {
        self.captured_piece
    }

    pub fn print_list(moves: &[Move]) {
        for mv in moves {
            println!("{mv}");
        }
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Pure coordinate notation
        // <https://www.chessprogramming.org/Algebraic_Chess_Notation#Pure_coordinate_notation>
        let promotion = match self.promotion {
            Some(Piece::WhiteQueen | Piece::BlackQueen) => "q",
            Some(Piece::WhiteRook | Piece::BlackRook) => "r",
            Some(Piece::WhiteBishop | Piece::BlackBishop) => "b",
            Some(Piece::WhiteKnight | Piece::BlackKnight) => "n",
            None => "",
            _ => panic!("Invalid promotion value"),
        };
        write!(f, "{}{}{}", self.from, self.to, promotion)
    }
}
