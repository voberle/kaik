#![allow(clippy::unreadable_literal)]

use crate::bitboard::BitBoard;

// All the initial locations.
pub const WHITE_PAWNS: BitBoard =
    BitBoard::new(0b0000000000000000000000000000000000000000000000001111111100000000);
pub const WHITE_ROOKS: BitBoard =
    BitBoard::new(0b0000000000000000000000000000000000000000000000000000000010000001);
pub const WHITE_KNIGHTS: BitBoard =
    BitBoard::new(0b0000000000000000000000000000000000000000000000000000000001000010);
pub const WHITE_BISHOPS: BitBoard =
    BitBoard::new(0b0000000000000000000000000000000000000000000000000000000000100100);
pub const WHITE_QUEENS: BitBoard =
    BitBoard::new(0b0000000000000000000000000000000000000000000000000000000000001000);
pub const WHITE_KING: BitBoard =
    BitBoard::new(0b0000000000000000000000000000000000000000000000000000000000010000);
pub const BLACK_PAWNS: BitBoard =
    BitBoard::new(0b0000000011111111000000000000000000000000000000000000000000000000);
pub const BLACK_ROOKS: BitBoard =
    BitBoard::new(0b1000000100000000000000000000000000000000000000000000000000000000);
pub const BLACK_KNIGHTS: BitBoard =
    BitBoard::new(0b0100001000000000000000000000000000000000000000000000000000000000);
pub const BLACK_BISHOPS: BitBoard =
    BitBoard::new(0b0010010000000000000000000000000000000000000000000000000000000000);
pub const BLACK_QUEENS: BitBoard =
    BitBoard::new(0b0000100000000000000000000000000000000000000000000000000000000000);
pub const BLACK_KING: BitBoard =
    BitBoard::new(0b0001000000000000000000000000000000000000000000000000000000000000);

//   8  0 1 1 1 1 1 1 1
//   7  0 1 1 1 1 1 1 1
//   6  0 1 1 1 1 1 1 1
//   5  0 1 1 1 1 1 1 1
//   4  0 1 1 1 1 1 1 1
//   3  0 1 1 1 1 1 1 1
//   2  0 1 1 1 1 1 1 1
//   1  0 1 1 1 1 1 1 1
//      a b c d e f g h
pub const NOT_A_FILE: BitBoard = BitBoard::new(18374403900871474942);

pub const NOT_H_FILE: BitBoard = BitBoard::new(9187201950435737471);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clear_file() {
        assert_eq!(
            NOT_A_FILE,
            r"0 1 1 1 1 1 1 1
            0 1 1 1 1 1 1 1
            0 1 1 1 1 1 1 1
            0 1 1 1 1 1 1 1
            0 1 1 1 1 1 1 1
            0 1 1 1 1 1 1 1
            0 1 1 1 1 1 1 1
            0 1 1 1 1 1 1 1"
                .into()
        );
        assert_eq!(
            NOT_H_FILE,
            r"1 1 1 1 1 1 1 0
            1 1 1 1 1 1 1 0
            1 1 1 1 1 1 1 0
            1 1 1 1 1 1 1 0
            1 1 1 1 1 1 1 0
            1 1 1 1 1 1 1 0
            1 1 1 1 1 1 1 0
            1 1 1 1 1 1 1 0"
                .into()
        );
    }
}
