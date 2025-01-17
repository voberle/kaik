use crate::{bitboard::BitBoard, common::Color};

mod board_type;
mod display;
mod move_gen;
mod perft;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Board {
    // Even indexes are white pieces, odd are black pieces.
    pieces: [BitBoard; 12],
    all: [BitBoard; 2],
    occupied: BitBoard,
    // Should we have empty as well?
    side_to_move: Color,
    // En passant square
    // en_passant: Square,
    // Castle
    // castle: TBD,
}
