use itertools::Itertools;

use crate::{
    board::bitboard::{self, from_array, BitBoard},
    common::Move,
    common::{Color, Piece, Square},
    fen,
};

use super::{Board, CastlingAbility};

fn get_all_bitboards(pieces: &[BitBoard]) -> [BitBoard; 2] {
    pieces.iter().enumerate().fold([0, 0], |mut acc, (i, bb)| {
        acc[i % 2] |= *bb;
        acc
    })
}

fn get_occupied_bitboard(all: &[BitBoard]) -> BitBoard {
    all[0] | all[1]
}

impl Board {
    pub fn empty() -> Self {
        Self {
            pieces: [0; 12],
            all: [0; 2],
            occupied: 0,
            side_to_move: Color::White,
            en_passant_target_square: None,
            castling_ability: CastlingAbility::NONE,
        }
    }

    pub fn initial_board() -> Self {
        let pieces = bitboard::INITIAL_BOARD;
        let all = get_all_bitboards(&pieces);
        let occupied = get_occupied_bitboard(&all);
        Self {
            pieces,
            all,
            occupied,
            side_to_move: Color::White,
            en_passant_target_square: None,
            castling_ability: CastlingAbility::ALL,
        }
    }

    pub fn from_fen(fen: &str) -> Self {
        let (
            piece_placement,
            side_to_move,
            castling_ability,
            en_passant_target_square,
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
                from_array(&filtered)
            })
            .collect_array()
            .unwrap();

        let all = get_all_bitboards(&pieces);
        let occupied = get_occupied_bitboard(&all);
        let castling_ability = CastlingAbility::new(&castling_ability);
        Self {
            pieces,
            all,
            occupied,
            side_to_move,
            en_passant_target_square,
            castling_ability,
        }
    }

    pub fn as_fen(&self) -> String {
        let piece_placement = (0..8)
            .rev()
            .flat_map(|rank| {
                (0..8).map(move |file| {
                    let index = rank * 8 + file;
                    let mut piece = None;
                    for (piece_index, bitboard) in self.pieces.iter().enumerate() {
                        if bitboard::is_set(*bitboard, index) {
                            piece = Some(Piece::ALL_PIECES[piece_index]);
                            break;
                        }
                    }
                    piece
                })
            })
            .collect_vec();
        fen::create(
            &piece_placement,
            self.side_to_move,
            &self.castling_ability.as_pieces_iter().collect_vec(),
            self.en_passant_target_square,
            0,
            1,
        )
    }

    pub fn get_side_to_move(&self) -> Color {
        self.side_to_move
    }

    pub fn opposite_side(&self) -> Color {
        self.side_to_move.opposite()
    }

    pub fn find_piece_on(&self, sq: Square) -> Piece {
        let index = sq as u8;
        *Piece::ALL_PIECES
            .iter()
            .find(|&&p| bitboard::is_set(self.pieces[p as usize], index))
            .unwrap()
    }

    // Creates a valid move based on this board.
    // If there are no pieces on the from position, the code will crash.
    pub fn new_move_from_pure(&self, s: &str) -> Move {
        assert!(s.len() >= 4 && s.len() <= 5);
        let from: Square = s[0..2].try_into().unwrap();
        let to: Square = s[2..4].try_into().unwrap();

        let piece = self.find_piece_on(from);
        let to_bb: BitBoard = bitboard::from_square(to);
        let is_capture = self.occupied & to_bb != 0;
        let promotion = if piece.is_pawn() && to.is_promotion_rank_for(piece.get_color()) {
            let promotion_piece = match &s[4..5] {
                "q" => Piece::get_queen_of(piece.get_color()),
                "r" => Piece::get_rook_of(piece.get_color()),
                "b" => Piece::get_bishop_of(piece.get_color()),
                "n" => Piece::get_knight_of(piece.get_color()),
                _ => panic!("Invalid promotion flag"),
            };
            Some(promotion_piece)
        } else {
            None
        };
        Move::new(from, to, promotion, piece, is_capture)
    }
}

// Creates the board from a FEN string.
impl From<&str> for Board {
    fn from(value: &str) -> Self {
        Board::from_fen(value)
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
        assert_eq!(board, fen::START_POSITION.into());
        assert_eq!(board.side_to_move, Color::White);
        assert_eq!(board.en_passant_target_square, None);
    }

    #[test]
    fn test_empty_board() {
        let board = Board::empty();
        assert_eq!(board.pieces, [0; 12]);
        assert_eq!(board.all, [0; 2]);
        assert_eq!(board.occupied, 0);
        assert_eq!(board.side_to_move, Color::White);
        assert_eq!(board.en_passant_target_square, None);
    }

    #[test]
    fn test_from_fen() {
        let board: Board = fen::START_POSITION.into();
        assert_eq!(board.pieces.len(), 12);
        assert_eq!(board.all.len(), 2);
        assert_eq!(board.side_to_move, Color::White);
        assert_eq!(board, Board::initial_board());
        assert_eq!(board.en_passant_target_square, None);
    }
}
