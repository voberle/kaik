//! Bit Board type and manipulation.
#![allow(unused_imports)]

pub mod constants; // TODO make private.
mod debug;
mod sliding_pieces_with_hq;

pub mod movements;

use crate::common::Square;

pub type BitBoard = u64;

pub fn from_array(value: &[u64]) -> BitBoard {
    let bb = value
        .chunks(8)
        .map(|line| {
            line.iter()
                .enumerate()
                .fold(0u64, |acc, (f, b)| acc + (b << f))
        })
        .rev()
        .enumerate()
        .fold(0u64, |acc, (r, b)| acc + (b << (r * 8)));
    bb
}

pub fn from_square(square: Square) -> BitBoard {
    1 << square as u8
}

pub const fn is_set(bitboard: BitBoard, index: u8) -> bool {
    bitboard & (1 << index) != 0
}

pub fn set(bitboard: &mut BitBoard, index: u8) {
    *bitboard |= 1 << index;
}

pub fn clear(bitboard: &mut BitBoard, index: u8) {
    *bitboard &= !(1 << index);
}

pub const fn neg(bitboard: BitBoard) -> BitBoard {
    bitboard.wrapping_neg()
}

// Returns the index of lowest bit in the bitboard.
#[allow(clippy::cast_possible_truncation)]
pub const fn get_index(bitboard: BitBoard) -> u8 {
    // Should be one CPU instruction.
    bitboard.trailing_zeros() as u8
}

// Least Significant One
// <https://www.chessprogramming.org/General_Setwise_Operations#Least_Significant_One>
pub fn get_ls1b(bitboard: BitBoard) -> BitBoard {
    bitboard & neg(bitboard)
}

pub fn reset_ls1b(bitboard: BitBoard) -> BitBoard {
    bitboard & (bitboard - 1)
}

// Creates an iterator that yields each set bit as a separate bitboard.
pub fn into_iter(bitboard: BitBoard) -> BitBoardIterator {
    BitBoardIterator(bitboard)
}

pub struct BitBoardIterator(u64);

impl Iterator for BitBoardIterator {
    type Item = BitBoard;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }

        let ls1b = self.0 & (!self.0 + 1); // Isolate least significant bit
        self.0 &= self.0 - 1; // Reset least significant bit

        Some(ls1b)
    }
}

pub use debug::from_str;
pub use debug::print;

#[cfg(test)]
mod tests {
    use crate::{
        bitboard::{self, constants},
        common::Square,
    };

    use super::*;

    #[test]
    fn test_from_square() {
        let bb: BitBoard = bitboard::from_square(Square::C3);
        assert_eq!(
            bb,
            0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0100_0000_0000_0000_0000
        );
    }

    const SAMPLE_BB: &str = r"
        . . . . . . . .
        . . 1 . 1 . . .
        . 1 . . . 1 . .
        . . . . . . . .
        . 1 . . . 1 . .
        . . 1 . 1 . . .
        . . . . . . . .
        . . . . . . . .";

    #[test]
    fn test_get_index() {
        let bb: BitBoard = bitboard::from_str(SAMPLE_BB);
        assert_eq!(bitboard::get_index(bb), 18);
    }

    #[test]
    fn test_ls1b() {
        let bb: BitBoard = bitboard::from_str(SAMPLE_BB);
        assert_eq!(
            bitboard::get_ls1b(bb),
            bitboard::from_str(
                r"
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . 1 . . . . .
            . . . . . . . .
            . . . . . . . ."
            )
        );
    }

    #[test]
    fn test_neg() {
        let x: BitBoard = bitboard::from_str(SAMPLE_BB);
        assert_eq!(
            neg(x),
            bitboard::from_str(
                r"
                1 1 1 1 1 1 1 1
                1 1 . 1 . 1 1 1
                1 . 1 1 1 . 1 1
                1 1 1 1 1 1 1 1
                1 . 1 1 1 . 1 1
                . . 1 1 . 1 1 1
                . . . . . . . .
                . . . . . . . ."
            )
        );
    }

    #[test]
    fn test_subtraction() {
        let x: BitBoard = bitboard::from_str(SAMPLE_BB);
        assert_eq!(
            x - 1,
            bitboard::from_str(
                r"
            . . . . . . . .
            . . 1 . 1 . . .
            . 1 . . . 1 . .
            . . . . . . . .
            . 1 . . . 1 . .
            1 1 . . 1 . . .
            1 1 1 1 1 1 1 1
            1 1 1 1 1 1 1 1"
            )
        );
    }
}
