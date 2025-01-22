//! Functions to help debug Bit Boards.

use itertools::Itertools;

use crate::bitboard;

use super::BitBoard;

pub fn print(bitboard: BitBoard) {
    for rank in 0..8 {
        print!("  {} ", 8 - rank); // display starts at 1
        for file in 0..8 {
            let index = (7 - rank) * 8 + file;
            // print!(" {}", u8::from(self.is_set(index)));
            print!(
                " {}",
                if bitboard::is_set(bitboard, index) {
                    '1'
                } else {
                    '.'
                }
            );
        }
        println!();
    }
    println!("     a b c d e f g h");
    println!("{bitboard} = {bitboard:064b}");
}

// Converts a list of 0 and 1s into a BitBoard. Starts with A8, A7, etc.
// Dot ('.') is synonymn with 0.
// The string may have line breaks, spaces etc, they are just ignored.
pub fn from_str(value: &str) -> BitBoard {
    let filtered = value
        .chars()
        .filter_map(|c| match c {
            '0' | '.' => Some(0u64),
            '1' => Some(1u64),
            _ => None,
        })
        .collect_vec();
    assert_eq!(filtered.len(), 64);
    bitboard::from_array(&filtered)
}

#[cfg(test)]
mod tests {
    use crate::{bitboard::constants, common::Square};

    use super::*;

    #[test]
    fn test_from_str() {
        let not_a_file = bitboard::from_str(
            r"0 1 1 1 1 1 1 1
            0 1 1 1 1 1 1 1
            0 1 1 1 1 1 1 1
            0 1 1 1 1 1 1 1
            0 1 1 1 1 1 1 1
            0 1 1 1 1 1 1 1
            0 1 1 1 1 1 1 1
            0 1 1 1 1 1 1 1",
        );
        assert_eq!(not_a_file, 18374403900871474942);
    }
}
