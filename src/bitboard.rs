//! Bit Board type and manipulation.
#![allow(unused_imports)]

mod bitboard_type;
pub mod constants; // TODO make private.
mod debug;
mod sliding_pieces_with_hq;

pub mod movements;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BitBoard(u64);

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
    BitBoard::new(bb)
}

pub const fn is_set(bitboard: BitBoard, index: u8) -> bool {
    bitboard.0 & (1 << index) != 0
}

pub fn set(bitboard: &mut BitBoard, index: u8) {
    bitboard.0 |= 1 << index;
}

pub fn clear(bitboard: &mut BitBoard, index: u8) {
    bitboard.0 &= !(1 << index);
}

// Least Significant One
// <https://www.chessprogramming.org/General_Setwise_Operations#Least_Significant_One>
pub fn get_ls1b(bitboard: BitBoard) -> BitBoard {
    bitboard & -bitboard
}

pub fn reset_ls1b(bitboard: BitBoard) -> BitBoard {
    const ONE: BitBoard = BitBoard::new(1);
    bitboard & (bitboard - ONE)
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
            -x,
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
        let one: BitBoard = BitBoard::new(1);
        assert_eq!(
            x - one,
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
