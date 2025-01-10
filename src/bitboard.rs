use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BitBoard(pub u64);

impl BitBoard {
    pub const EMPTY: BitBoard = BitBoard(0);

    pub fn is_set(self, index: u8) -> bool {
        self.0 & (1 << index) != 0
    }

    pub fn set(&mut self, index: u8) {
        self.0 |= 1 << index;
    }

    pub fn print(self) {
        for rank in (0..8).rev() {
            print!("  {rank} ");
            for file in 0..8 {
                let index = rank * 8 + file;
                print!(" {}", u8::from(self.is_set(index)));
            }
            println!();
        }
        println!("     a b c d e f g h");
    }
}

impl From<u64> for BitBoard {
    fn from(v: u64) -> Self {
        Self(v)
    }
}

impl From<&str> for BitBoard {
    // Converts a list of 0 and 1s into a BitBoard. Starts with A8, A7, etc.
    // The string may have line breaks, spaces etc, they are just ignored.
    fn from(value: &str) -> Self {
        let filtered = value
            .chars()
            .filter_map(|c| match c {
                '0' => Some(0),
                '1' => Some(1),
                _ => None,
            })
            .collect_vec();
        assert_eq!(filtered.len(), 64);
        let bb = filtered
            .chunks(8)
            .map(|line| {
                line.iter()
                    .enumerate()
                    .fold(0, |acc, (f, b)| acc + (b << f))
            })
            .rev()
            .enumerate()
            .fold(0, |acc, (r, b)| acc + (b << (r * 8)));
        Self(bb)
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

#[cfg(test)]
mod tests {
    use crate::squares::Square;

    use super::*;

    #[test]
    fn test_create_set() {
        use Square::*;
        let white_pawns: BitBoard = 71776119061217280.into();

        let mut b = BitBoard::EMPTY;
        for square in [A2, B2, C2, D2, E2, F2, G2, H2] {
            b.set(square.into());
        }
        assert_eq!(b, white_pawns);
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
}
