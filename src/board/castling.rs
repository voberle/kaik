use std::fmt::Display;

use crate::common::{Color, Piece, Square};

use super::CastlingAbility;

impl CastlingAbility {
    pub const ALL: CastlingAbility = CastlingAbility(0b1111);
    pub const NONE: CastlingAbility = CastlingAbility(0b0000);

    fn get_mask_for_piece(piece: Piece) -> u8 {
        match piece {
            Piece::WhiteKing => 0b0001,
            Piece::WhiteQueen => 0b0010,
            Piece::BlackKing => 0b0100,
            Piece::BlackQueen => 0b1000,
            _ => panic!("Piece not supported for castling"),
        }
    }

    pub fn new(pieces: &[Piece]) -> Self {
        Self(
            pieces
                .iter()
                .fold(0, |acc, p| acc | Self::get_mask_for_piece(*p)),
        )
    }

    pub fn any(self) -> bool {
        self.0 != 0
    }

    pub fn white_can_castle_king_side(self) -> bool {
        self.0 & 0b0001 != 0
    }

    pub fn white_can_castle_queen_side(self) -> bool {
        self.0 & 0b0010 != 0
    }

    pub fn black_can_castle_king_side(self) -> bool {
        self.0 & 0b0100 != 0
    }

    pub fn black_can_castle_queen_side(self) -> bool {
        self.0 & 0b1000 != 0
    }

    pub fn can_castle_king_side(self, color: Color) -> bool {
        self.0 & (0b0001 << ((color as u8) * 2)) != 0
    }

    pub fn can_castle_queen_side(self, color: Color) -> bool {
        self.0 & (0b0010 << ((color as u8) * 2)) != 0
    }

    pub fn as_pieces_iter(self) -> impl Iterator<Item = Piece> {
        [
            (self.white_can_castle_king_side(), Piece::WhiteKing),
            (self.white_can_castle_queen_side(), Piece::WhiteQueen),
            (self.black_can_castle_king_side(), Piece::BlackKing),
            (self.black_can_castle_queen_side(), Piece::BlackQueen),
        ]
        .into_iter()
        .filter_map(
            |(condition, piece)| {
                if condition {
                    Some(piece)
                } else {
                    None
                }
            },
        )
    }

    fn as_fen(self) -> String {
        let mut s = String::new();
        if self.white_can_castle_king_side() {
            s.push('K');
        }
        if self.white_can_castle_queen_side() {
            s.push('Q');
        }
        if self.black_can_castle_king_side() {
            s.push('k');
        }
        if self.black_can_castle_queen_side() {
            s.push('q');
        }
        if s.is_empty() {
            s.push('-');
        }
        s
    }

    // An array used to clear the castling ability if a move touches one of the original rook/king squares.
    // These bit values are used to update the castling rights based on the movement of the king and rooks.
    // - `0b1111`: Kings and rooks didn't move.
    // - `0b1100`: White king moved.
    // - `0b1110`: White rook king side moved.
    // - `0b1101`: White rook queen side moved.
    // - `0b0011`: Black king moved.
    // - `0b1011`: Black rook king side moved.
    // - `0b0111`: Black rook queen side moved.
    //
    // NB: White is up
    #[rustfmt::skip]
    const UPDATE_ARRAY: [u8; 64] = [
        0b1101, 0b1111, 0b1111, 0b1111, 0b1100, 0b1111, 0b1111, 0b1110,
        0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111,
        0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111,
        0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111,
        0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111,
        0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111,
        0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111,
        0b0111, 0b1111, 0b1111, 0b1111, 0b0011, 0b1111, 0b1111, 0b1011,
    ];

    // Clears the castling ability if we are touching one of the 6 original rook/king squares.
    pub fn clear(&mut self, sq: Square) {
        self.0 &= Self::UPDATE_ARRAY[sq as usize];
    }
}

impl Display for CastlingAbility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_fen())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_castling_ability() {
        assert!(CastlingAbility::ALL.white_can_castle_king_side());
        assert!(CastlingAbility::ALL.white_can_castle_queen_side());
        assert!(CastlingAbility::ALL.black_can_castle_king_side());
        assert!(CastlingAbility::ALL.black_can_castle_queen_side());

        assert!(!CastlingAbility::NONE.white_can_castle_king_side());
        assert!(!CastlingAbility::NONE.white_can_castle_queen_side());
        assert!(!CastlingAbility::NONE.black_can_castle_king_side());
        assert!(!CastlingAbility::NONE.black_can_castle_queen_side());
    }

    #[test]
    fn test_clear_white_king() {
        let mut castling_ability = CastlingAbility::ALL;
        castling_ability.clear(Square::E1);
        assert!(!castling_ability.white_can_castle_king_side());
        assert!(!castling_ability.white_can_castle_queen_side());
        assert!(castling_ability.black_can_castle_king_side());
        assert!(castling_ability.black_can_castle_queen_side());
    }

    #[test]
    fn test_clear_white_rooks() {
        let mut castling_ability = CastlingAbility::ALL;
        castling_ability.clear(Square::A1); // queen side
        assert!(castling_ability.white_can_castle_king_side());
        assert!(!castling_ability.white_can_castle_queen_side());
        castling_ability.clear(Square::H1); // king side
        assert!(!castling_ability.white_can_castle_king_side());
        assert!(!castling_ability.white_can_castle_queen_side());
        assert!(castling_ability.black_can_castle_king_side());
        assert!(castling_ability.black_can_castle_queen_side());
    }

    #[test]
    fn test_clear_black_king() {
        let mut castling_ability = CastlingAbility::ALL;
        castling_ability.clear(Square::E8);
        assert!(castling_ability.white_can_castle_king_side());
        assert!(castling_ability.white_can_castle_queen_side());
        assert!(!castling_ability.black_can_castle_king_side());
        assert!(!castling_ability.black_can_castle_queen_side());
    }

    #[test]
    fn test_clear_black_queen_side() {
        let mut castling_ability = CastlingAbility::ALL;
        castling_ability.clear(Square::A8);
        assert!(castling_ability.white_can_castle_king_side());
        assert!(castling_ability.white_can_castle_queen_side());
        assert!(castling_ability.black_can_castle_king_side());
        assert!(!castling_ability.black_can_castle_queen_side());
    }
}
