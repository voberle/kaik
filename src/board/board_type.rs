use itertools::Itertools;

use crate::{
    bitboard::{self, from_array, movements, BitBoard},
    common::{Color, Piece},
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

    pub fn attacks_king(&self, king_color: Color) -> BitBoard {
        // From <https://www.chessprogramming.org/Checks_and_Pinned_Pieces_(Bitboards)>
        // Note that the example there doesn't check king creating checks.

        let opp_king_color = king_color.opposite();

        // Could be optimized a bit with things like:
        //   let opposite_pawns = self.pieces[Piece::BlackPawn as usize - king_color as usize];
        let opposite_pawns = self.pieces[Piece::get_pawn_of(opp_king_color) as usize];
        let opposite_knights = self.pieces[Piece::get_knight_of(opp_king_color) as usize];
        let opposite_king = self.pieces[Piece::get_king_of(opp_king_color) as usize];

        let opposite_rooks_queens = self.pieces[Piece::get_queen_of(opp_king_color) as usize]
            | self.pieces[Piece::get_rook_of(opp_king_color) as usize];
        let opposite_bishops_queens = self.pieces[Piece::get_queen_of(opp_king_color) as usize]
            | self.pieces[Piece::get_bishop_of(opp_king_color) as usize];

        let king_bb = self.pieces[Piece::get_king_of(king_color) as usize];
        let pawn_attacks = if king_color == Color::White {
            movements::get_white_pawn_attacks(king_bb)
        } else {
            movements::get_black_pawn_attacks(king_bb)
        };

        (pawn_attacks & opposite_pawns)
            | (movements::get_knight_attacks(king_bb) & opposite_knights)
            | (movements::get_king_attacks(king_bb) & opposite_king)
            | (movements::get_bishop_attacks(king_bb, self.occupied) & opposite_bishops_queens)
            | (movements::get_rook_attacks(king_bb, self.occupied) & opposite_rooks_queens)
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

    #[test]
    fn test_attacks_king_king_next_to_king() {
        let board: Board = "8/2kp4/1K6/2P4r/8/8/8/8 w - - 1 2".into();
        board.print();
        let bb = board.attacks_king(Color::White);
        bitboard::print(bb);
        // Not allowed to move next to opponent king.
        assert_eq!(
            bb,
            0b0000000000000100000000000000000000000000000000000000000000000000
        );
    }
}
