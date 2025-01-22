//! Functions to help debug Bit Boards.

use super::BitBoard;

pub fn print(bitboard: BitBoard) {
    for rank in 0..8 {
        print!("  {} ", 8 - rank); // display starts at 1
        for file in 0..8 {
            let index = (7 - rank) * 8 + file;
            // print!(" {}", u8::from(self.is_set(index)));
            print!(" {}", if bitboard.is_set(index) { '1' } else { '.' });
        }
        println!();
    }
    println!("     a b c d e f g h");
    println!("{} = {:064b}", bitboard.0, bitboard.0);
}
