use itertools::Itertools;

use crate::{bitboard::BitBoard, fen, pieces::ALL_PIECES};

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

fn get_all_bitboards(pieces: &[BitBoard]) -> [BitBoard; 2] {
    pieces
        .iter()
        .enumerate()
        .fold([BitBoard::EMPTY, BitBoard::EMPTY], |mut acc, (i, bb)| {
            acc[i % 2] |= *bb;
            acc
        })
}

fn get_occupied_bitboard(all: &[BitBoard]) -> BitBoard {
    all[0] | all[1]
}

impl Board {
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
        // Same order as in pieces.rs
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
        let all = get_all_bitboards(&pieces);
        let occupied = get_occupied_bitboard(&all);
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

// Creates the board from a FEN string.
impl From<&str> for Board {
    fn from(value: &str) -> Self {
        let (
            piece_placement,
            _side_to_move,
            _castling_ability,
            _en_passant_target_square,
            _half_move_clock,
            _full_move_counter,
        ) = fen::parse(value);

        let pieces = ALL_PIECES
            .iter()
            .map(|piece| {
                // Get a vector of 0/1 where 1 is set if there is the same piece as 'piece' at this position.
                let filtered = piece_placement
                    .iter()
                    .map(|c| match c {
                        Some(p) if p == piece => 1u64,
                        _ => 0u64,
                    })
                    .collect_vec();
                assert_eq!(filtered.len(), 64);
                Into::<BitBoard>::into(&*filtered)
            })
            .collect_array()
            .unwrap();

        let all = get_all_bitboards(&pieces);
        let occupied = get_occupied_bitboard(&all);
        Self {
            pieces,
            all,
            occupied,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_board() {
        let board = Board::initial_board();
        assert_eq!(board.pieces.len(), 12);
        assert_eq!(board.all.len(), 2);

        let via_fen: Board = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".into();
        assert_eq!(board, via_fen);
    }
}
