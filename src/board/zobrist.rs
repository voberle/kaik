//! Zobrist keys and hashing support.
//! <https://www.chessprogramming.org/Zobrist_Hashing>

use itertools::Itertools;
use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::common::{Color, Piece, Square};

use super::{bitboard, Board, CastlingAbility};

pub struct Keys {
    pieces: [[u64; 12]; 64],
    // We could have only one key for black, but that might be easier to use.
    side: [u64; 2],
    castling: [u64; 16],
    // We need keys only for each file for EP squares.
    // 8 are enough, since it's clear which one it is based on the color.
    en_passant: [u64; 8],
    en_passant_none: u64,
}

impl Keys {
    // Initializes all the keys with random numbers.
    #[allow(clippy::unreadable_literal)]
    fn init() -> Self {
        // We use a fixed seed, for testability.
        let mut rng = StdRng::seed_from_u64(9476900812072076987);

        let pieces = (0..64)
            .map(|_| (0..12).map(|_| rng.gen::<u64>()).collect_array().unwrap())
            .collect_array()
            .unwrap();
        let side = [rng.gen::<u64>(), rng.gen::<u64>()];
        let castling = (0..16).map(|_| rng.gen::<u64>()).collect_array().unwrap();
        let en_passant = (0..8).map(|_| rng.gen::<u64>()).collect_array().unwrap();
        let en_passant_none = rng.gen::<u64>();
        Self {
            pieces,
            side,
            castling,
            en_passant,
            en_passant_none,
        }
    }

    pub fn piece_key(&self, square: Square, piece: Piece) -> u64 {
        self.pieces[square as usize][piece as usize]
    }

    pub fn color_key(&self, color: Color) -> u64 {
        self.side[color as usize]
    }

    pub fn castling_key(&self, castling: CastlingAbility) -> u64 {
        self.castling[castling.0 as usize]
    }

    pub fn en_passant_key(&self, en_passant_square: Option<Square>) -> u64 {
        if let Some(ep_square) = en_passant_square {
            self.en_passant[ep_square.get_file() as usize]
        } else {
            self.en_passant_none
        }
    }
}

use once_cell::sync::Lazy;

pub static ZOBRIST_KEYS: Lazy<Keys> = Lazy::new(Keys::init);

impl Board {
    // Generates a Zobrist key for the board.
    // Use this only for a new board.
    // When only updating the board, update the existing key instead of regenerating a new one.
    pub fn gen_zobrist_key(board: &Board) -> u64 {
        let mut key: u64 = 0;

        for piece in Piece::ALL_PIECES {
            let pieces_bb = board.pieces[piece as usize];
            for from_bb in bitboard::into_iter(pieces_bb) {
                let square = bitboard::get_index(from_bb).into();
                key ^= ZOBRIST_KEYS.piece_key(square, piece);
            }
        }

        key ^= ZOBRIST_KEYS.color_key(board.get_side_to_move());
        key ^= ZOBRIST_KEYS.castling_key(board.castling_ability);
        key ^= ZOBRIST_KEYS.en_passant_key(board.en_passant_target_square);

        key
    }
}
