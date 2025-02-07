#![allow(clippy::unreadable_literal)]

use crate::board::bitboard::BitBoard;

pub const EMPTY: BitBoard = u64::MIN;
pub const UNIVERSAL: BitBoard = u64::MAX;

// All the initial locations.
// Same order as in pieces.rs
pub const INITIAL_BOARD: [BitBoard; 12] = [
    0b0000000000000000000000000000000000000000000000001111111100000000, // White pawns
    0b0000000011111111000000000000000000000000000000000000000000000000, // Black pawns
    0b0000000000000000000000000000000000000000000000000000000001000010, // White knights
    0b0100001000000000000000000000000000000000000000000000000000000000, // Black knights
    0b0000000000000000000000000000000000000000000000000000000000100100, // White bishops
    0b0010010000000000000000000000000000000000000000000000000000000000, // Black bishops
    0b0000000000000000000000000000000000000000000000000000000010000001, // White rooks
    0b1000000100000000000000000000000000000000000000000000000000000000, // Black rooks
    0b0000000000000000000000000000000000000000000000000000000000001000, // White queens
    0b0000100000000000000000000000000000000000000000000000000000000000, // Black queens
    0b0000000000000000000000000000000000000000000000000000000000010000, // White king
    0b0001000000000000000000000000000000000000000000000000000000000000, // Black king
];

// Clipping bit boards. For example the A file is:
//   8  0 1 1 1 1 1 1 1
//   7  0 1 1 1 1 1 1 1
//   6  0 1 1 1 1 1 1 1
//   5  0 1 1 1 1 1 1 1
//   4  0 1 1 1 1 1 1 1
//   3  0 1 1 1 1 1 1 1
//   2  0 1 1 1 1 1 1 1
//   1  0 1 1 1 1 1 1 1
//      a b c d e f g h
pub const NOT_A_FILE: BitBoard = 18374403900871474942;
pub const NOT_H_FILE: BitBoard = 9187201950435737471;
pub const NOT_HG_FILE: BitBoard = 4557430888798830399;
pub const NOT_AB_FILE: BitBoard = 18229723555195321596;
pub const MASK_RANK_3: BitBoard = 16711680;
pub const MASK_RANK_6: BitBoard = 280375465082880;

pub const CASTLING_KING_SIDE_MASKS: [BitBoard; 2] = [
    0b0000000000000000000000000000000000000000000000000000000001100000,
    0b0110000000000000000000000000000000000000000000000000000000000000,
];

pub const CASTLING_QUEEN_SIDE_MASKS: [BitBoard; 2] = [
    0b0000000000000000000000000000000000000000000000000000000000001110,
    0b0000111000000000000000000000000000000000000000000000000000000000,
];

#[cfg(test)]
mod tests {
    use crate::board::bitboard::{self, constants::*};

    #[test]
    fn test_clipping_bitboards() {
        assert_eq!(
            NOT_A_FILE,
            bitboard::from_str(
                r"
              0 1 1 1 1 1 1 1
              0 1 1 1 1 1 1 1
              0 1 1 1 1 1 1 1
              0 1 1 1 1 1 1 1
              0 1 1 1 1 1 1 1
              0 1 1 1 1 1 1 1
              0 1 1 1 1 1 1 1
              0 1 1 1 1 1 1 1"
            )
        );
        assert_eq!(
            NOT_H_FILE,
            bitboard::from_str(
                r"
            1 1 1 1 1 1 1 0
              1 1 1 1 1 1 1 0
              1 1 1 1 1 1 1 0
              1 1 1 1 1 1 1 0
              1 1 1 1 1 1 1 0
              1 1 1 1 1 1 1 0
              1 1 1 1 1 1 1 0
              1 1 1 1 1 1 1 0"
            )
        );
    }

    #[test]
    fn test_masks() {
        assert_eq!(
            MASK_RANK_3,
            bitboard::from_str(
                r"
              0 0 0 0 0 0 0 0
              0 0 0 0 0 0 0 0
              0 0 0 0 0 0 0 0
              0 0 0 0 0 0 0 0
              0 0 0 0 0 0 0 0
              1 1 1 1 1 1 1 1
              0 0 0 0 0 0 0 0
              0 0 0 0 0 0 0 0"
            )
        );
    }
}
