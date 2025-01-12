use crate::bitboard::BitBoard;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Board {
    // Even indexes are white pieces, odd are black pieces.
    pieces: [BitBoard; 12],
    all: [BitBoard; 2],
    occupied: BitBoard,
    // Should we have empty as well?

    // Side to move
    // side: usize,
    // En passant square
    // en_passant: Square,
    // Castle
    // castle: TBD,
}

impl Board {
    const WHITE_PAWNS: usize = 0;
    const BLACK_PAWNS: usize = 1;
    const WHITE_KNIGHTS: usize = 2;
    const BLACK_KNIGHTS: usize = 3;
    const WHITE_BISHOPS: usize = 4;
    const BLACK_BISHOPS: usize = 5;
    const WHITE_ROOKS: usize = 6;
    const BLACK_ROOKS: usize = 7;
    const WHITE_QUEENS: usize = 8;
    const BLACK_QUEENS: usize = 9;
    const WHITE_KING: usize = 10;
    const BLACK_KING: usize = 11;

    const ASCII_PIECES: &[u8; 12] = b"PpNnBbRrQqKk";
    const UNICODE_PIECES: [char; 12] = ['♙', '♟', '♘', '♞', '♗', '♝', '♖', '♜', '♕', '♛', '♔', '♚'];

    pub fn empty() -> Self {
        Self {
            pieces: [BitBoard::EMPTY; 12],
            all: [BitBoard::EMPTY; 2],
            occupied: BitBoard::EMPTY,
        }
    }

    #[allow(clippy::wildcard_imports)]
    pub fn initial_board() -> Self {
        use crate::constants::*;
        let pieces = [
            WHITE_PAWNS,
            BLACK_PAWNS,
            WHITE_KNIGHTS,
            BLACK_KNIGHTS,
            WHITE_BISHOPS,
            BLACK_BISHOPS,
            WHITE_ROOKS,
            BLACK_ROOKS,
            WHITE_QUEENS,
            BLACK_QUEENS,
            WHITE_KING,
            BLACK_KING,
        ];
        let all = pieces.iter().enumerate().fold(
            [BitBoard::EMPTY, BitBoard::EMPTY],
            |mut acc, (i, bb)| {
                acc[i % 2] |= *bb;
                acc
            },
        );
        let occupied = all[0] | all[1];
        Self {
            pieces,
            all,
            occupied,
        }
    }

    pub fn print(&self) {
        for rank in (0..8).rev() {
            print!("  {rank} ");
            for file in 0..8 {
                let index = rank * 8 + file;

                let mut piece_char = '.';
                for (piece, bitboard) in self.pieces.iter().enumerate() {
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
