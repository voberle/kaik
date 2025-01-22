//! Visualization of a Board

use std::fmt::Display;

use crate::{
    bitboard::{self, BitBoard},
    common::{Color, Piece, Square},
    moves::Move,
};

use super::Board;

impl Board {
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
                    if bitboard::is_set(*bitboard, index) {
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
            if self.get_side_to_move() == Color::White {
                "=>"
            } else {
                "  "
            }
        );
    }

    pub fn print_bitboards(&self) {
        for piece in Piece::ALL_PIECES {
            println!("Bitboard for {piece}");
            bitboard::print(self.pieces[piece as usize]);
        }
        println!("Bitboard for occupied white");
        bitboard::print(self.all[Color::White as usize]);
        println!("Bitboard for occupied black");
        bitboard::print(self.all[Color::Black as usize]);
        println!("Bitboard for occupied");
        bitboard::print(self.occupied);
    }

    fn find_piece_on(&self, sq: Square) -> Piece {
        let index = sq as u8;
        *Piece::ALL_PIECES
            .iter()
            .find(|&&p| bitboard::is_set(self.pieces[p as usize], index))
            .unwrap()
    }

    // Creates a valid move based on this board.
    // In case of promotion, we promote to a queen.
    // If there are no pieces on the from position, the code wull crash.
    pub fn new_move(&self, from: Square, to: Square) -> Move {
        let piece = self.find_piece_on(from);
        let to_bb: BitBoard = to.into();
        let is_capture = self.occupied.intersects(to_bb);
        let promotion = if to.is_promotion_rank_for(piece.get_color()) {
            Some(Piece::get_queen_of(piece.get_color()))
        } else {
            None
        };
        Move::new(from, to, promotion, piece, is_capture)
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_fen())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_move() {
        let board = Board::initial_board();
        let from = Square::E2;
        let to = Square::E4;
        let mv = board.new_move(from, to);
        assert_eq!(mv.get_from(), from);
        assert_eq!(mv.get_to(), to);
        assert_eq!(mv.get_piece(), Piece::WhitePawn);
        assert!(!mv.is_capture());
        assert!(mv.get_promotion().is_none());
    }

    #[test]
    fn test_new_move_capture() {
        let board: Board = "rnbqkbnr/pppp1ppp/8/8/4p3/2N2P2/PPPPP1PP/R1BQKBNR w KQkq - 0 3".into();
        let from = Square::E2;
        let to = Square::E4;
        let mv = board.new_move(from, to);
        assert_eq!(mv.get_from(), from);
        assert_eq!(mv.get_to(), to);
        assert_eq!(mv.get_piece(), Piece::WhitePawn);
        assert!(mv.is_capture());
        assert!(mv.get_promotion().is_none());
    }

    #[test]
    fn test_new_move_promotion() {
        let board: Board = "6k1/4P3/8/8/8/8/8/4K3 w - - 0 1".into();
        let from = Square::E7;
        let to = Square::E8;
        let mv = board.new_move(from, to);
        assert_eq!(mv.get_from(), from);
        assert_eq!(mv.get_to(), to);
        assert_eq!(mv.get_piece(), Piece::WhitePawn);
        assert!(!mv.is_capture());
        assert_eq!(mv.get_promotion(), Some(Piece::WhiteQueen));
    }
}
