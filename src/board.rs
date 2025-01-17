use std::fmt::Display;

use itertools::Itertools;

use crate::{
    bitboard::BitBoard,
    common::{Color, Piece, Square},
    fen,
    moves::Move,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Board {
    // Even indexes are white pieces, odd are black pieces.
    pieces: [BitBoard; 12],
    all: [BitBoard; 2],
    occupied: BitBoard,
    // Should we have empty as well?
    side_to_move: Color,
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
    pub fn empty() -> Self {
        Self {
            pieces: [BitBoard::EMPTY; 12],
            all: [BitBoard::EMPTY; 2],
            occupied: BitBoard::EMPTY,
            side_to_move: Color::White,
        }
    }

    #[allow(clippy::wildcard_imports)]
    pub fn initial_board() -> Self {
        let pieces = BitBoard::INITIAL_BOARD;
        let all = get_all_bitboards(&pieces);
        let occupied = get_occupied_bitboard(&all);
        Self {
            pieces,
            all,
            occupied,
            side_to_move: Color::White,
        }
    }

    pub fn print(&self) {
        self.print_with_move(None);
    }

    pub fn print_with_move(&self, mv: Option<Move>) {
        const ASCII_PIECES: &[u8; 12] = b"PpNnBbRrQqKk";
        const UNICODE_PIECES: [char; 12] =
            ['♙', '♟', '♘', '♞', '♗', '♝', '♖', '♜', '♕', '♛', '♔', '♚'];
        const RED: &str = "\x1b[31m";
        const GREEN: &str = "\x1b[32m";
        const RESET: &str = "\x1b[0m";
        const INVERSE: &str = "\x1b[7m";
        for rank in (0..8).rev() {
            print!("  {} ", rank + 1);
            for file in 0..8 {
                let index = rank * 8 + file;
                let square: Square = ((b'a' + file) as char, rank as usize + 1).into();

                let mut piece_char = '.';
                for (piece, bitboard) in self.pieces.iter().enumerate() {
                    if bitboard.is_set(index) {
                        piece_char = UNICODE_PIECES[piece];
                        // piece_char = ASCII_PIECES[piece] as char;
                        break;
                    }
                }
                if let Some(m) = mv {
                    if m.get_from() == square {
                        print!(" {INVERSE}{RED}{piece_char}{RESET}");
                    } else if m.get_to() == square {
                        print!(" {INVERSE}{GREEN}{piece_char}{RESET}");
                    } else {
                        print!(" {piece_char}");
                    }
                } else {
                    print!(" {piece_char}");
                }
            }
            println!();
        }
        println!(
            " {}  a b c d e f g h",
            if self.side_to_move == Color::White {
                "=>"
            } else {
                "  "
            }
        );
    }

    fn from_fen(fen: &str) -> Self {
        let (
            piece_placement,
            side_to_move,
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
            side_to_move,
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
            self.side_to_move,
            &castling_ability,
            None,
            0,
            1,
        )
    }

    pub fn side_to_move(&self) -> Color {
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

    // Generate all possible moves from this board.
    pub fn generate_moves_for(&self, pieces: &[Piece]) -> Vec<Move> {
        // Pseudo-legal or legal ones?

        let mut moves_list = Vec::new();

        for &moving_pieces in pieces
            .iter()
            .filter(|p| self.side_to_move() == p.get_color())
        {
            let own_bb = self.all[self.side_to_move() as usize];
            let opposite_bb = self.all[self.opposite_side() as usize];

            let mut pieces_bb = self.pieces[moving_pieces as usize];
            while !pieces_bb.is_zero() {
                let from_bb = pieces_bb.get_ls1b();
                let from_square = from_bb.get_index().into();
                let mut moves_bb = match moving_pieces {
                    Piece::WhiteKing | Piece::BlackKing => from_bb.get_king_moves(own_bb),
                    Piece::WhiteKnight | Piece::BlackKnight => from_bb.get_knight_moves(own_bb),
                    Piece::WhitePawn => from_bb.get_white_pawn_moves(self.occupied, opposite_bb),
                    Piece::BlackPawn => from_bb.get_black_pawn_moves(self.occupied, opposite_bb),
                    Piece::WhiteBishop | Piece::BlackBishop => {
                        from_bb.get_bishop_moves(self.occupied, own_bb)
                    }
                    Piece::WhiteRook | Piece::BlackRook => {
                        from_bb.get_rook_moves(self.occupied, own_bb)
                    }
                    Piece::WhiteQueen | Piece::BlackQueen => {
                        from_bb.get_queen_moves(self.occupied, own_bb)
                    }
                };

                // Generate moves.
                while !moves_bb.is_zero() {
                    let to_bb = moves_bb.get_ls1b();
                    let to_square: Square = to_bb.get_index().into();
                    let is_capture = opposite_bb.contains(to_bb);

                    let mv = Move::new(from_square, to_square, None, moving_pieces, is_capture);
                    moves_list.push(mv);

                    moves_bb = moves_bb.reset_ls1b();
                }

                pieces_bb = pieces_bb.reset_ls1b();
            }
        }
        moves_list
    }

    pub fn generate_moves(&self) -> Vec<Move> {
        self.generate_moves_for(&Piece::ALL_PIECES)
    }

    // perft function <https://www.chessprogramming.org/Perft>
    pub fn perft(&self, depth: usize) -> usize {
        if depth == 0 {
            return 1;
        }

        let mut nodes = 0;
        let move_list = self.generate_moves();
        for mv in move_list {
            let mut board_copy = *self;
            board_copy.update_by_move(mv);
            nodes += board_copy.perft(depth - 1);
        }
        nodes
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

    use crate::{common::Piece::*, common::Square::*};

    use super::*;

    #[test]
    fn test_initial_board() {
        let board = Board::initial_board();
        assert_eq!(board.pieces.len(), 12);
        assert_eq!(board.all.len(), 2);
        assert_eq!(board, START_POSITION.into());
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
        let board: Board = START_POSITION.into();
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

    #[test]
    fn test_generate_moves_white_king() {
        let board: Board = "2k5/8/8/8/8/8/2Pp4/2K5 w - - 0 1".into();
        let moves = board.generate_moves_for(&[WhiteKing]);
        assert_eq!(
            moves,
            &[
                Move::quiet(C1, B1, WhiteKing),
                Move::quiet(C1, D1, WhiteKing),
                Move::quiet(C1, B2, WhiteKing),
                Move::capture(C1, D2, WhiteKing),
            ]
        );
    }

    #[test]
    fn test_generate_moves_black_king() {
        let board: Board = "2k5/2Pp4/8/8/8/8/8/2K5 b - - 0 1".into();
        let moves = board.generate_moves_for(&[BlackKing]);
        assert_eq!(
            moves,
            &[
                Move::quiet(C8, B7, BlackKing),
                Move::capture(C8, C7, BlackKing),
                Move::quiet(C8, B8, BlackKing),
                Move::quiet(C8, D8, BlackKing),
            ]
        );
    }

    #[test]
    fn test_generate_moves_white_knight() {
        let board: Board = "8/8/6p1/5N2/8/1N6/8/8 w - - 0 1".into();
        let moves = board.generate_moves_for(&[WhiteKnight]);
        assert_eq!(
            moves,
            &[
                Move::quiet(B3, A1, WhiteKnight),
                Move::quiet(B3, C1, WhiteKnight),
                Move::quiet(B3, D2, WhiteKnight),
                Move::quiet(B3, D4, WhiteKnight),
                Move::quiet(B3, A5, WhiteKnight),
                Move::quiet(B3, C5, WhiteKnight),
                Move::quiet(F5, E3, WhiteKnight),
                Move::quiet(F5, G3, WhiteKnight),
                Move::quiet(F5, D4, WhiteKnight),
                Move::quiet(F5, H4, WhiteKnight),
                Move::quiet(F5, D6, WhiteKnight),
                Move::quiet(F5, H6, WhiteKnight),
                Move::quiet(F5, E7, WhiteKnight),
                Move::quiet(F5, G7, WhiteKnight),
            ]
        );
    }

    #[test]
    fn test_generate_moves_white_pawn() {
        let board: Board = "8/8/8/8/4N3/n1pB2P1/PPPPPPPP/8 w - - 0 1".into();
        let moves = board.generate_moves_for(&[WhitePawn]);
        assert_eq!(
            moves,
            &[
                Move::capture(B2, A3, WhitePawn),
                Move::quiet(B2, B3, WhitePawn),
                Move::capture(B2, C3, WhitePawn),
                Move::quiet(B2, B4, WhitePawn),
                Move::capture(D2, C3, WhitePawn),
                Move::quiet(E2, E3, WhitePawn),
                Move::quiet(F2, F3, WhitePawn),
                Move::quiet(F2, F4, WhitePawn),
                Move::quiet(H2, H3, WhitePawn),
                Move::quiet(H2, H4, WhitePawn),
                Move::quiet(G3, G4, WhitePawn),
            ]
        );
    }

    #[test]
    fn test_generate_moves_black_pawn() {
        let board: Board = "8/pppppppp/n1pB2P1/4N3/8/8/8/8 b - - 0 1".into();
        let moves = board.generate_moves_for(&[BlackPawn]);
        assert_eq!(
            moves,
            &[
                Move::quiet(C6, C5, BlackPawn),
                Move::quiet(B7, B5, BlackPawn),
                Move::quiet(B7, B6, BlackPawn),
                Move::capture(C7, D6, BlackPawn),
                Move::capture(E7, D6, BlackPawn),
                Move::quiet(E7, E6, BlackPawn),
                Move::quiet(F7, F5, BlackPawn),
                Move::quiet(F7, F6, BlackPawn),
                Move::capture(F7, G6, BlackPawn),
                Move::quiet(H7, H5, BlackPawn),
                Move::capture(H7, G6, BlackPawn),
                Move::quiet(H7, H6, BlackPawn),
            ]
        );
    }
}
