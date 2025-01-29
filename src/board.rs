use crate::{
    bitboard::BitBoard,
    common::{Color, Square},
};

mod attacks;
mod board_type;
mod castling;
mod display;
mod eval;
mod move_gen;
mod update;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct CastlingAbility(u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Board {
    // Even indexes are white pieces, odd are black pieces.
    pieces: [BitBoard; 12],
    all: [BitBoard; 2],
    occupied: BitBoard,
    side_to_move: Color,
    en_passant_target_square: Option<Square>,
    castling_ability: CastlingAbility,
}
