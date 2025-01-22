use itertools::Itertools;

use crate::{
    bitboard::{constants, from_array, movements, BitBoard},
    common::{Color, Piece},
    fen,
    moves::Move,
};

use super::{Board, CastlingAbility};

fn get_all_bitboards(pieces: &[BitBoard]) -> [BitBoard; 2] {
    pieces
        .iter()
        .enumerate()
        .fold([constants::EMPTY, constants::EMPTY], |mut acc, (i, bb)| {
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
            pieces: [constants::EMPTY; 12],
            all: [constants::EMPTY; 2],
            occupied: constants::EMPTY,
            side_to_move: Color::White,
            en_passant_target_square: None,
            castling_ability: CastlingAbility::NONE,
        }
    }

    pub fn initial_board() -> Self {
        let pieces = constants::INITIAL_BOARD;
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
                        if bitboard.is_set(index) {
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

    fn toggle_side(&mut self) {
        self.side_to_move = self.side_to_move.opposite();
    }

    // Updates the bitboards and castling rights only.
    // Update by Move explained at <https://www.chessprogramming.org/General_Setwise_Operations#UpdateByMove>
    fn update_bitboards_by_move(&mut self, mv: Move) {
        let color = mv.get_piece().get_color();
        let from_bb: BitBoard = mv.get_from().into();
        let to_bb: BitBoard = mv.get_to().into();
        let from_to_bb = from_bb ^ to_bb;

        self.pieces[mv.get_piece() as usize] ^= from_to_bb;
        self.all[color as usize] ^= from_to_bb;
        self.occupied ^= from_to_bb;

        if mv.is_capture() {
            // Loop over bitboards opposite color.
            for bb in self
                .pieces
                .iter_mut()
                .skip(color.opposite() as usize)
                .step_by(2)
            {
                if bb.intersects(to_bb) {
                    *bb ^= to_bb;
                    self.all[color.opposite() as usize] ^= to_bb;
                    self.occupied ^= to_bb;
                    break;
                }
            }
        }

        self.castling_ability.clear(mv.get_from());
        self.castling_ability.clear(mv.get_to()); // in case rook gets taken
    }

    // Updates the board with the specified move.
    pub fn update_by_move(&mut self, mv: Move) {
        self.update_bitboards_by_move(mv);

        if let Some(promote_to) = mv.get_promotion() {
            // Pawn was moved. We now need to switch it to the new piece.
            let to_bb: BitBoard = mv.get_to().into();
            self.pieces[mv.get_piece() as usize] &= !to_bb;
            self.pieces[promote_to as usize] |= to_bb;
        }

        self.en_passant_target_square = mv.get_en_passant_target_square();

        if let Some(castling_rook_move) = mv.get_castling() {
            self.update_bitboards_by_move(castling_rook_move);
        }

        self.toggle_side();
    }

    pub fn attacks_king(&self, king_color: Color) -> BitBoard {
        // From <https://www.chessprogramming.org/Checks_and_Pinned_Pieces_(Bitboards)>
        // Could be optimized a bit with things like:
        //   let opposite_pawns = self.pieces[Piece::BlackPawn as usize - king_color as usize];
        let opposite_pawns = self.pieces[Piece::get_pawn_of(king_color.opposite()) as usize];
        let opposite_knights = self.pieces[Piece::get_knight_of(king_color.opposite()) as usize];
        let opposite_rooks_queens = self.pieces
            [Piece::get_queen_of(king_color.opposite()) as usize]
            | self.pieces[Piece::get_rook_of(king_color.opposite()) as usize];
        let opposite_bishops_queens = self.pieces
            [Piece::get_queen_of(king_color.opposite()) as usize]
            | self.pieces[Piece::get_bishop_of(king_color.opposite()) as usize];

        let king_bb = self.pieces[Piece::get_king_of(king_color) as usize];
        let pawn_attacks = if king_color == Color::White {
            movements::get_white_pawn_attacks(king_bb)
        } else {
            movements::get_black_pawn_attacks(king_bb)
        };
        (pawn_attacks & opposite_pawns)
            | (movements::get_knight_attacks(king_bb) & opposite_knights)
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
    use crate::{common::Piece::*, common::Square::*};

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
        assert_eq!(board.pieces, [constants::EMPTY; 12]);
        assert_eq!(board.all, [constants::EMPTY; 2]);
        assert_eq!(board.occupied, constants::EMPTY);
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
    fn test_update_by_move_quiet() {
        let mut board = Board::initial_board();
        let mv = Move::quiet(B2, B3, WhitePawn);
        board.update_by_move(mv);
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
        assert_eq!(board.to_string(), "2k5/8/8/8/8/8/2PK4/8 b - - 0 1");

        let mut board: Board =
            "rnbqkbnr/ppp1pppp/8/3p4/8/2N5/PPPPPPPP/R1BQKBNR w KQkq - 0 1".into();
        let mv = Move::capture(C3, D5, WhiteKnight);
        board.update_by_move(mv);
        assert_eq!(
            board.to_string(),
            "rnbqkbnr/ppp1pppp/8/3N4/8/8/PPPPPPPP/R1BQKBNR b KQkq - 0 1"
        );
    }

    #[test]
    fn test_update_by_move_double_push() {
        let mut board = Board::initial_board();
        let mv = Move::quiet(B2, B4, WhitePawn);
        board.update_by_move(mv);
        assert_eq!(
            board,
            "rnbqkbnr/pppppppp/8/8/1P6/8/P1PPPPPP/RNBQKBNR b KQkq b3 0 1".into()
        );
    }

    #[test]
    fn test_update_by_move_castling() {
        let mut board: Board = "4k3/8/8/8/8/8/PPPPPPPP/R3K1NR w Q - 0 1".into();
        let mv = Move::quiet(E1, C1, WhiteKing); // White queen side castle
        board.update_by_move(mv);
        assert_eq!(board, "4k3/8/8/8/8/8/PPPPPPPP/2KR2NR b - - 0 1".into());
    }

    #[test]
    fn test_update_by_move_castling_clearing() {
        let mut board: Board =
            "rnbqkbnr/ppp1pppp/3p4/8/8/5P2/PPPPP1PP/RNBQKBNR w KQkq - 0 1".into();
        let mv = Move::quiet(E1, F2, WhiteKing);
        board.update_by_move(mv);
        assert_eq!(
            board,
            "rnbqkbnr/ppp1pppp/3p4/8/8/5P2/PPPPPKPP/RNBQ1BNR b kq - 0 1".into()
        );
    }

    #[test]
    fn test_update_by_move_promotion() {
        let mut board: Board = "4k3/1P6/8/8/8/8/8/4K3 w - - 0 1".into();
        let mv = Move::new(B7, B8, Some(WhiteQueen), WhitePawn, false);
        board.print();
        board.update_by_move(mv);
        board.print();
        assert_eq!(board, "1Q2k3/8/8/8/8/8/8/4K3 b - - 0 1".into());
    }
}
