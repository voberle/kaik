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

    pub fn print(&self) {
        for rank in (0..8).rev() {
            for file in 0..8 {
                let index = rank * 8 + file;
                if file == 0 {
                    print!("  {rank} ");
                }

                let mut piece_char = '.';
                for (piece, bitboard) in [self.white, self.black]
                    .as_flattened()
                    .iter()
                    .enumerate()
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
