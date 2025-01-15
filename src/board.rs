use std::fmt::Display;

use itertools::Itertools;

use crate::{bitboard::BitBoard, common::Color, common::Piece, fen, moves::Move};

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

    fn from_fen(fen: &str) -> Self {
        let (
            piece_placement,
            _side_to_move,
            _castling_ability,
            _en_passant_target_square,
            _half_move_clock,
            _full_move_counter,
        ) = fen::parse(fen);

        let pieces = Piece::ALL_PIECES
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

    fn as_fen(&self) -> String {
        let piece_placement = (0..8)
            .rev()
            .flat_map(|rank| {
                (0..8).map(move |file| {
                    let index = rank * 8 + file;
                    let mut piece = None;
                    for (piece_index, bitboard) in self.pieces.iter().enumerate() {
                        if bitboard.is_set(index) {
                            piece = Some(Piece::ALL_PIECES[piece_index]);
                            break;
                        }
                    }
                    piece
                })
            })
            .collect_vec();
        let castling_ability = [
            Piece::WhiteKing,
            Piece::WhiteQueen,
            Piece::BlackKing,
            Piece::BlackQueen,
        ];
        fen::create(
            &piece_placement,
            Color::White,
            &castling_ability,
            None,
            0,
            1,
        )
    }

    // Updates the board with the specified move.
    // Update by Move explained at <https://www.chessprogramming.org/General_Setwise_Operations#UpdateByMove>
    pub fn update_by_move(&mut self, mv: Move) {
        // TODO: Support for castling, promotions and en-passant captures.
        if mv.get_promotion().is_some() {
            unimplemented!("Update by move for promotion");
        }
        let from_bb: BitBoard = mv.get_from().into();
        let to_bb: BitBoard = mv.get_to().into();
        let from_to_bb = from_bb ^ to_bb;
        self.pieces[mv.get_piece() as usize] ^= from_to_bb;
        self.all[mv.get_piece().color() as usize] ^= from_to_bb;
        if let Some(captured_piece) = mv.get_captured_piece() {
            self.pieces[captured_piece as usize] ^= to_bb;
            self.all[captured_piece.color() as usize] ^= to_bb;
        }
        self.occupied ^= from_to_bb;
    }
}

// Creates the board from a FEN string.
impl From<&str> for Board {
    fn from(value: &str) -> Self {
        Board::from_fen(value)
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_fen())
    }
}

#[cfg(test)]
mod tests {
    use fen::START_POSITION;

    use crate::{common::Piece, common::Square::*};

    use super::*;

    #[test]
    fn test_initial_board() {
        let board = Board::initial_board();
        assert_eq!(board.pieces.len(), 12);
        assert_eq!(board.all.len(), 2);

        let via_fen: Board = START_POSITION.into();
        assert_eq!(board, via_fen);
    }

    #[test]
    fn test_empty_board() {
        let board = Board::empty();
        assert_eq!(board.pieces, [BitBoard::EMPTY; 12]);
        assert_eq!(board.all, [BitBoard::EMPTY; 2]);
        assert_eq!(board.occupied, BitBoard::EMPTY);
    }

    #[test]
    fn test_update_by_move() {
        let mut board = Board::initial_board();
        let mv = Move::new(B2, B3, None, Piece::WhitePawn, None);
        board.update_by_move(mv);

        // TODO: Would be better to not depend on FEN serialization for this.
        assert_eq!(
            board.to_string(),
            "rnbqkbnr/pppppppp/8/8/8/1P6/P1PPPPPP/RNBQKBNR w KQkq - 0 1"
        );
    }

    #[test]
    fn test_from_fen() {
        let fen = START_POSITION;
        let board: Board = fen.into();
        assert_eq!(board.pieces.len(), 12);
        assert_eq!(board.all.len(), 2);

        let initial_board = Board::initial_board();
        assert_eq!(board, initial_board);
    }

    #[test]
    fn test_print_board() {
        let board = Board::initial_board();
        board.print();
        // Manually verify the printed output
    }
}
