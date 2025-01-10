use crate::bitboard::BitBoard;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Board {
    pub white: [BitBoard; 6],
    pub black: [BitBoard; 6],

    pub all_white: BitBoard,
    pub all_black: BitBoard,
    pub all: BitBoard,
}

impl Board {
    pub const PAWNS: usize = 0;
    pub const ROOKS: usize = 1;
    pub const KNIGHTS: usize = 2;
    pub const BISHOPS: usize = 3;
    pub const QUEENS: usize = 4;
    pub const KING: usize = 5;

    const ASCII_PIECES: &[u8; 12] = b"PNBRQKpnbrqk";
    const UNICODE_PIECES: [char; 12] = ['♙', '♘', '♗', '♖', '♕', '♔', '♟', '♞', '♝', '♜', '♛', '♚'];

    pub fn empty() -> Self {
        Self {
            white: [BitBoard::EMPTY; 6],
            black: [BitBoard::EMPTY; 6],
            all_white: BitBoard::EMPTY,
            all_black: BitBoard::EMPTY,
            all: BitBoard::EMPTY,
        }
    }

    #[allow(clippy::wildcard_imports)]
    pub fn initial_board() -> Self {
        use crate::constants::*;
        let white = [
            WHITE_PAWNS,
            WHITE_ROOKS,
            WHITE_KNIGHTS,
            WHITE_BISHOPS,
            WHITE_QUEENS,
            WHITE_KING,
        ];
        let black = [
            BLACK_PAWNS,
            BLACK_ROOKS,
            BLACK_KNIGHTS,
            BLACK_BISHOPS,
            BLACK_QUEENS,
            BLACK_KING,
        ];
        let all_white = white.iter().fold(BitBoard::EMPTY, |acc, bb| acc | *bb);
        let all_black = black.iter().fold(BitBoard::EMPTY, |acc, bb| acc | *bb);
        let all = all_white | all_black;
        Self {
            white,
            black,
            all_white,
            all_black,
            all,
        }
    }

    pub fn print(&self) {
        for rank in (0..8).rev() {
            print!("  {rank} ");
            for file in 0..8 {
                let index = rank * 8 + file;

                let mut piece_char = '.';
                for (piece, bitboard) in [self.white, self.black].as_flattened().iter().enumerate()
                {
                    if bitboard.is_set(index) {
                        piece_char = Self::UNICODE_PIECES[piece];
                        // piece_char = Self::ASCII_PIECES[piece] as char;
                        break;
                    }
                }
                print!(" {piece_char}");
            }
            println!();
        }
        println!("     a b c d e f g h");
    }
}
