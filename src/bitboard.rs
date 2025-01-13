use std::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, ShlAssign, Shr,
    ShrAssign,
};

use itertools::Itertools;

use crate::common::Square;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BitBoard(u64);

impl BitBoard {
    pub const EMPTY: BitBoard = BitBoard(0);

    pub const fn new(v: u64) -> Self {
        Self(v)
    }

    pub const fn is_set(self, index: u8) -> bool {
        self.0 & (1 << index) != 0
    }

    pub fn set(&mut self, index: u8) {
        self.0 |= 1 << index;
    }

    pub fn print(self) {
        for rank in 0..8 {
            print!("  {} ", 8 - rank); // display starts at 1
            for file in 0..8 {
                let index = (7 - rank) * 8 + file;
                print!(" {}", u8::from(self.is_set(index)));
            }
            println!();
        }
        println!("     a b c d e f g h");
        println!("{:064b}", self.0);
    }
}

impl From<u64> for BitBoard {
    fn from(v: u64) -> Self {
        Self(v)
    }
}

impl From<&[u64]> for BitBoard {
    fn from(value: &[u64]) -> Self {
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
        Self(bb)
    }
}

impl From<&str> for BitBoard {
    // Converts a list of 0 and 1s into a BitBoard. Starts with A8, A7, etc.
    // The string may have line breaks, spaces etc, they are just ignored.
    fn from(value: &str) -> Self {
        let filtered = value
            .chars()
            .filter_map(|c| match c {
                '0' => Some(0u64),
                '1' => Some(1u64),
                _ => None,
            })
            .collect_vec();
        assert_eq!(filtered.len(), 64);
        Into::<BitBoard>::into(&*filtered)
    }
}

impl From<Square> for BitBoard {
    fn from(square: Square) -> Self {
        Self(1 << square as u8)
    }
}

impl Not for BitBoard {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl BitAnd for BitBoard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for BitBoard {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = Self(self.0 & rhs.0);
    }
}

impl BitOr for BitBoard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = Self(self.0 | rhs.0);
    }
}

impl BitXor for BitBoard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for BitBoard {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = Self(self.0 ^ rhs.0);
    }
}

impl Shl<usize> for BitBoard {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self::Output {
        let Self(lhs) = self;
        Self(lhs << rhs)
    }
}

impl ShlAssign<usize> for BitBoard {
    fn shl_assign(&mut self, rhs: usize) {
        self.0 <<= rhs;
    }
}

impl Shr<usize> for BitBoard {
    type Output = Self;

    fn shr(self, rhs: usize) -> Self::Output {
        let Self(lhs) = self;
        Self(lhs >> rhs)
    }
}

impl ShrAssign<usize> for BitBoard {
    fn shr_assign(&mut self, rhs: usize) {
        self.0 >>= rhs;
    }
}

#[cfg(test)]
mod tests {
    use crate::common::Square;

    use super::*;

    #[test]
    fn test_create_set() {
        use Square::*;
        let black_pawns: BitBoard = 71776119061217280.into();

        let mut b = BitBoard::EMPTY;
        for square in [A7, B7, C7, D7, E7, F7, G7, H7] {
            b.set(square as u8);
        }
        // black_pawns.print();
        b.print();

        // assert!(false);
        assert_eq!(b, black_pawns);
    }

    #[test]
    fn test_from_str() {
        let not_a_file = r"0 1 1 1 1 1 1 1
            0 1 1 1 1 1 1 1
            0 1 1 1 1 1 1 1
            0 1 1 1 1 1 1 1
            0 1 1 1 1 1 1 1
            0 1 1 1 1 1 1 1
            0 1 1 1 1 1 1 1
            0 1 1 1 1 1 1 1";
        let bb: BitBoard = not_a_file.into();
        assert_eq!(bb.0, 18374403900871474942);
    }

    #[test]
    fn test_from_square() {
        let bb: BitBoard = Square::C3.into();
        assert_eq!(
            bb.0,
            0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0100_0000_0000_0000_0000
        );
    }
}
