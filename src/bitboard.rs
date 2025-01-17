//! Bit Board type and manipulation.

mod bitboard_type;
mod constants;
mod sliding_pieces_with_hq;

pub mod movements;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BitBoard(u64);
