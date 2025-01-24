//! Visualization of a Board

use std::{fmt::Display, io::Write};

use crate::{
    bitboard::{self, BitBoard},
    common::{Color, Piece, Square},
    moves::Move,
};

use super::Board;

impl Board {
    const ASCII_PIECES: [char; 12] = ['P', 'p', 'N', 'n', 'B', 'b', 'R', 'r', 'Q', 'q', 'K', 'k'];
    const UNICODE_PIECES: [char; 12] = ['♙', '♟', '♘', '♞', '♗', '♝', '♖', '♜', '♕', '♛', '♔', '♚'];

    pub fn print(&self) {
        self.print_with_move(None);
    }

    pub fn print_with_move(&self, mv: Option<Move>) {
        // We don't use write() here because we want the print functions to be captured
        // in tests, and stdout doesn't capture in tests <https://github.com/rust-lang/rust/issues/90785>
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
                        piece_char = Self::UNICODE_PIECES[piece];
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
        // println!();
        // println!("FEN: {}", self.as_fen());
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> Result<(), Box<dyn std::error::Error>> {
        for rank in (0..8).rev() {
            write!(writer, "  {} ", rank + 1)?;
            for file in 0..8 {
                let index = rank * 8 + file;
                let mut piece_char = '.';
                for (piece, bitboard) in self.pieces.iter().enumerate() {
                    if bitboard::is_set(*bitboard, index) {
                        piece_char = Self::ASCII_PIECES[piece];
                        break;
                    }
                }
                write!(writer, " {piece_char}")?;
            }
            writeln!(writer)?;
        }
        writeln!(
            writer,
            " {}  a b c d e f g h",
            if self.get_side_to_move() == Color::White {
                "=>"
            } else {
                "  "
            }
        )?;
        writeln!(writer)?;
        writeln!(writer, "FEN: {}", self.as_fen())?;
        Ok(())
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

    // Creates a valid move based on this board.
    // In case of promotion, we promote to a queen.
    // If there are no pieces on the from position, the code will crash.
    pub fn new_move(&self, from: Square, to: Square) -> Move {
        let piece = self.find_piece_on(from);
        let to_bb: BitBoard = bitboard::from_square(to);
        let is_capture = self.occupied & to_bb != 0;
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
