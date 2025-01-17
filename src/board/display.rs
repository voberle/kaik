//! Visualization of a Board

use std::fmt::Display;

use crate::{
    common::{Color, Square},
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
            if self.get_side_to_move() == Color::White {
                "=>"
            } else {
                "  "
            }
        );
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_fen())
    }
}
