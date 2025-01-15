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
    piece: Piece, // Piece performing the move
    is_capture: bool,
    // We can add more flags: Castling, double push pawn, en passant.
}

impl Move {
    pub fn new(
        from: Square,
        to: Square,
        promotion: Option<Piece>,
        piece: Piece,
        is_capture: bool,
    ) -> Self {
        assert!(promotion.is_none_or(|p| !p.is_pawn() && !p.is_king()));
        Self {
            from,
            to,
            promotion,
            piece,
            is_capture,
        }
    }

    pub fn quiet(from: Square, to: Square, piece: Piece) -> Self {
        Self::new(from, to, None, piece, false)
    }

    pub fn capture(from: Square, to: Square, piece: Piece) -> Self {
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

    pub fn print_list(moves: &[Move]) {
        for mv in moves {
            println!("{mv}");
        }
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
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_as_lan(f)
    }
}
