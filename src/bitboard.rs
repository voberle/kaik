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

pub use debug::from_str;
pub use debug::print;
