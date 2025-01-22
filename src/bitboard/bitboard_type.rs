use std::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Mul, MulAssign, Neg, Not, Shl,
    ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

use crate::common::Square;

use super::BitBoard;

impl BitBoard {
    pub const fn new(v: u64) -> Self {
        Self(v)
    }

    pub const fn is_set(self, index: u8) -> bool {
        self.0 & (1 << index) != 0
    }

    pub const fn intersects(self, other: BitBoard) -> bool {
        self.0 & other.0 != 0
    }

    pub fn set(&mut self, index: u8) {
        self.0 |= 1 << index;
    }

    pub fn clear(&mut self, index: u8) {
        self.0 &= !(1 << index);
    }

    pub const fn is_null(self) -> bool {
        self.0 == 0
    }

    // Returns the index of lowest bit in the bitboard.
    #[allow(clippy::cast_possible_truncation)]
    pub const fn get_index(self) -> u8 {
        // Should be one CPU instruction.
        self.0.trailing_zeros() as u8
    }

    // Least Significant One
    // <https://www.chessprogramming.org/General_Setwise_Operations#Least_Significant_One>
    pub fn get_ls1b(self) -> Self {
        self & -self
    }

    pub fn reset_ls1b(self) -> Self {
        const ONE: BitBoard = BitBoard::new(1);
        self & (self - ONE)
    }

    // Creates an iterator that yields each set bit as a separate bitboard.
    pub fn into_iter(self) -> BitBoardIterator {
        BitBoardIterator(self.0)
    }
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

        Some(BitBoard(ls1b))
    }
}

impl From<BitBoard> for u64 {
    fn from(val: BitBoard) -> Self {
        val.0
    }
}

impl From<u64> for BitBoard {
    fn from(v: u64) -> Self {
        Self(v)
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

impl Neg for BitBoard {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(self.0.wrapping_neg())
    }
}

impl Sub for BitBoard {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign for BitBoard {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl Mul for BitBoard {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl MulAssign for BitBoard {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        bitboard::{self, constants},
        common::Square,
    };

    use super::*;

    #[test]
    fn test_create_set() {
        use Square::*;
        let black_pawns: BitBoard = 71776119061217280.into();

        let mut b = constants::EMPTY;
        for square in [A7, B7, C7, D7, E7, F7, G7, H7] {
            b.set(square as u8);
        }
        assert_eq!(b, black_pawns);
    }

    #[test]
    fn test_from_square() {
        let bb: BitBoard = Square::C3.into();
        assert_eq!(
            bb.0,
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
        let x: BitBoard = bitboard::from_str(SAMPLE_BB);
        assert_eq!(x.get_index(), 18);
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

    #[test]
    fn test_ls1b() {
        let x: BitBoard = bitboard::from_str(SAMPLE_BB);
        assert_eq!(
            x.get_ls1b(),
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
}
