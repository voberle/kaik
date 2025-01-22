//! Bit Board type and manipulation.

mod bitboard_type;
pub mod constants; // TODO make private.
mod debug;
mod sliding_pieces_with_hq;

pub mod movements;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BitBoard(u64);

pub use debug::print;
