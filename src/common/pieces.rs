use std::fmt;

use crate::common::Color;

// The order of the enum is important because it is used to index arrays.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    WhitePawn,
    BlackPawn,
    WhiteKnight,
    BlackKnight,
    WhiteBishop,
    BlackBishop,
    WhiteRook,
    BlackRook,
    WhiteQueen,
    BlackQueen,
    WhiteKing,
    BlackKing,
}

pub type PieceListBoard = Vec<Option<Piece>>;

impl TryFrom<char> for Piece {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'P' => Ok(Piece::WhitePawn),
            'p' => Ok(Piece::BlackPawn),
            'N' => Ok(Piece::WhiteKnight),
            'n' => Ok(Piece::BlackKnight),
            'B' => Ok(Piece::WhiteBishop),
            'b' => Ok(Piece::BlackBishop),
            'R' => Ok(Piece::WhiteRook),
            'r' => Ok(Piece::BlackRook),
            'Q' => Ok(Piece::WhiteQueen),
            'q' => Ok(Piece::BlackQueen),
            'K' => Ok(Piece::WhiteKing),
            'k' => Ok(Piece::BlackKing),
            _ => Err("Invalid char piece"),
        }
    }
}

impl From<Piece> for char {
    fn from(val: Piece) -> Self {
        match val {
            Piece::WhitePawn => 'P',
            Piece::BlackPawn => 'p',
            Piece::WhiteKnight => 'N',
            Piece::BlackKnight => 'n',
            Piece::WhiteBishop => 'B',
            Piece::BlackBishop => 'b',
            Piece::WhiteRook => 'R',
            Piece::BlackRook => 'r',
            Piece::WhiteQueen => 'Q',
            Piece::BlackQueen => 'q',
            Piece::WhiteKing => 'K',
            Piece::BlackKing => 'k',
        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // write!(f, "{}", self.as_unicode())
        write!(f, "{}", char::from(*self))
    }
}

impl Piece {
    pub const ALL_PIECES: [Piece; 12] = [
        Piece::WhitePawn,
        Piece::BlackPawn,
        Piece::WhiteKnight,
        Piece::BlackKnight,
        Piece::WhiteBishop,
        Piece::BlackBishop,
        Piece::WhiteRook,
        Piece::BlackRook,
        Piece::WhiteQueen,
        Piece::BlackQueen,
        Piece::WhiteKing,
        Piece::BlackKing,
    ];

    pub const PROMOTION_PIECES: [[Piece; 4]; 2] = [
        [
            Piece::WhiteQueen,
            Piece::WhiteKnight,
            Piece::WhiteRook,
            Piece::WhiteBishop,
        ],
        [
            Piece::BlackQueen,
            Piece::BlackKnight,
            Piece::BlackRook,
            Piece::BlackBishop,
        ],
    ];

    pub const fn is_pawn(self) -> bool {
        matches!(self, Piece::WhitePawn | Piece::BlackPawn)
    }

    pub const fn is_knight(self) -> bool {
        matches!(self, Piece::WhiteKnight | Piece::BlackKnight)
    }

    pub const fn is_bishop(self) -> bool {
        matches!(self, Piece::WhiteBishop | Piece::BlackBishop)
    }

    pub const fn is_rook(self) -> bool {
        matches!(self, Piece::WhiteRook | Piece::BlackRook)
    }

    pub const fn is_queen(self) -> bool {
        matches!(self, Piece::WhiteQueen | Piece::BlackQueen)
    }

    pub const fn is_king(self) -> bool {
        matches!(self, Piece::WhiteKing | Piece::BlackKing)
    }

    pub const fn get_color(self) -> Color {
        Color::new(self as usize % 2)
    }

    pub const fn get_pawn_of(color: Color) -> Self {
        if matches!(color, Color::White) {
            Piece::WhitePawn
        } else {
            Piece::BlackPawn
        }
    }

    pub const fn get_knight_of(color: Color) -> Self {
        if matches!(color, Color::White) {
            Piece::WhiteKnight
        } else {
            Piece::BlackKnight
        }
    }

    pub const fn get_bishop_of(color: Color) -> Self {
        if matches!(color, Color::White) {
            Piece::WhiteBishop
        } else {
            Piece::BlackBishop
        }
    }

    pub const fn get_rook_of(color: Color) -> Self {
        if matches!(color, Color::White) {
            Piece::WhiteRook
        } else {
            Piece::BlackRook
        }
    }

    pub const fn get_queen_of(color: Color) -> Self {
        if matches!(color, Color::White) {
            Piece::WhiteQueen
        } else {
            Piece::BlackQueen
        }
    }

    pub const fn get_king_of(color: Color) -> Self {
        if matches!(color, Color::White) {
            Piece::WhiteKing
        } else {
            Piece::BlackKing
        }
    }

    pub fn as_unicode(self) -> char {
        match self {
            Piece::WhitePawn => '♙',
            Piece::BlackPawn => '♟',
            Piece::WhiteKnight => '♘',
            Piece::BlackKnight => '♞',
            Piece::WhiteBishop => '♗',
            Piece::BlackBishop => '♝',
            Piece::WhiteRook => '♖',
            Piece::BlackRook => '♜',
            Piece::WhiteQueen => '♕',
            Piece::BlackQueen => '♛',
            Piece::WhiteKing => '♔',
            Piece::BlackKing => '♚',
        }
    }

    // Converts a string with pieces into vector of Piece. Starts with pieces on A8, A7, etc.
    // Empty squares are represented with dots.
    // The string may have line breaks, spaces etc, they are just ignored.
    pub fn build_list_board(value: &str) -> PieceListBoard {
        value
            .chars()
            .filter_map(|c| match c.try_into() {
                Ok(p) => Some(Some(p)),
                Err(_) => {
                    if c == '.' {
                        Some(None)
                    } else {
                        None
                    }
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order() {
        assert_eq!(Piece::WhitePawn as usize, 0);
        assert_eq!(Piece::BlackPawn as usize, 1);
        assert_eq!(Piece::WhiteKnight as usize, 2);
        assert_eq!(Piece::BlackKnight as usize, 3);
        assert_eq!(Piece::WhiteBishop as usize, 4);
        assert_eq!(Piece::BlackBishop as usize, 5);
        assert_eq!(Piece::WhiteRook as usize, 6);
        assert_eq!(Piece::BlackRook as usize, 7);
        assert_eq!(Piece::WhiteQueen as usize, 8);
        assert_eq!(Piece::BlackQueen as usize, 9);
        assert_eq!(Piece::WhiteKing as usize, 10);
        assert_eq!(Piece::BlackKing as usize, 11);
    }
}
