use itertools::Itertools;

use crate::{
    bitboard::BitBoard,
    common::{Color, Piece},
    fen,
    moves::Move,
};

use super::Board;

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
    pub fn empty() -> Self {
        Self {
            pieces: [BitBoard::EMPTY; 12],
            all: [BitBoard::EMPTY; 2],
            occupied: BitBoard::EMPTY,
            side_to_move: Color::White,
            en_passant_target_square: None,
        }
    }

    pub fn initial_board() -> Self {
        let pieces = BitBoard::INITIAL_BOARD;
        let all = get_all_bitboards(&pieces);
        let occupied = get_occupied_bitboard(&all);
        Self {
            pieces,
            all,
            occupied,
            side_to_move: Color::White,
            en_passant_target_square: None,
        }
    }

    pub fn from_fen(fen: &str) -> Self {
        let (
            piece_placement,
            side_to_move,
            _castling_ability,
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
            side_to_move,
            en_passant_target_square,
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
            self.side_to_move,
            &castling_ability,
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

    fn toggle_side(&mut self) {
        self.side_to_move = self.side_to_move.opposite();
    }

    // Updates the board with the specified move.
    // Update by Move explained at <https://www.chessprogramming.org/General_Setwise_Operations#UpdateByMove>
    pub fn update_by_move(&mut self, mv: Move) {
        // TODO: Support for castling, promotions and en-passant captures.
        if mv.get_promotion().is_some() {
            unimplemented!("Update by move for promotion");
        }
        let color = mv.get_piece().get_color();
        let from_bb: BitBoard = mv.get_from().into();
        let to_bb: BitBoard = mv.get_to().into();
        let from_to_bb = from_bb ^ to_bb;

        self.pieces[mv.get_piece() as usize] ^= from_to_bb;
        self.all[color as usize] ^= from_to_bb;

        if mv.is_capture() {
            // Loop over bitboards opposite color.
            for bb in &mut self.pieces {
                if *bb == to_bb {
                    *bb ^= to_bb;
                    self.all[color.opposite() as usize] ^= to_bb;
                    // Actually important to avoid setting it back to the other value.
                    // Alternative could be to skip every second bitboard with a step_by(2).
                    break;
                }
            }
        }

        self.occupied ^= from_to_bb;
        self.toggle_side();
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
    use crate::{common::Piece::*, common::Square::*};

    use super::*;

    #[test]
    fn test_initial_board() {
        let board = Board::initial_board();
        assert_eq!(board.pieces.len(), 12);
        assert_eq!(board.all.len(), 2);
        assert_eq!(board, fen::START_POSITION.into());
        assert_eq!(board.side_to_move, Color::White);
    }

    #[test]
    fn test_empty_board() {
        let board = Board::empty();
        assert_eq!(board.pieces, [BitBoard::EMPTY; 12]);
        assert_eq!(board.all, [BitBoard::EMPTY; 2]);
        assert_eq!(board.occupied, BitBoard::EMPTY);
        assert_eq!(board.side_to_move, Color::White);
    }

    #[test]
    fn test_from_fen() {
        let board: Board = fen::START_POSITION.into();
        assert_eq!(board.pieces.len(), 12);
        assert_eq!(board.all.len(), 2);
        assert_eq!(board.side_to_move, Color::White);
        assert_eq!(board, Board::initial_board());
    }

    #[test]
    fn test_update_by_move_quiet() {
        let mut board = Board::initial_board();
        let mv = Move::quiet(B2, B3, WhitePawn);
        board.update_by_move(mv);

        // TODO: Would be better to not depend on FEN serialization for this.
        assert_eq!(
            board.to_string(),
            "rnbqkbnr/pppppppp/8/8/8/1P6/P1PPPPPP/RNBQKBNR b KQkq - 0 1"
        );
    }

    #[test]
    fn test_update_by_move_capture() {
        let mut board: Board = "2k5/8/8/8/8/8/2Pp4/2K5 w - - 0 1".into();
        let mv = Move::capture(C1, D2, WhiteKing);
        board.update_by_move(mv);
        assert_eq!(board.to_string(), "2k5/8/8/8/8/8/2PK4/8 b KQkq - 0 1");
    }
}
