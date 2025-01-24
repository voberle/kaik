use std::io::Write;

use crate::board::Board;

#[derive(Debug)]
pub struct Game {
    board: Board,
}

impl Game {
    // A game is always initialized to a position, either the starting one or from a FEN string.
    pub fn startpos() -> Self {
        Self {
            board: Board::initial_board(),
        }
    }

    pub fn from_fen(fen: &str) -> Self {
        Self {
            board: Board::from_fen(fen),
        }
    }

    pub fn get_board(&self) -> Board {
        self.board
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> Result<(), Box<dyn std::error::Error>> {
        self.board.write(writer)
    }

    pub fn apply_moves(&mut self, moves: &[&str]) {
        for mv in moves {
            self.board.update_by_move(self.board.new_move_from_pure(mv));
        }
    }
}
