use std::io::Write;

use crate::board::Board;

pub struct Game {
    board: Board,
    debug: bool,
}

impl Game {
    // A game is always initialized to a position, either the starting one or from a FEN string.
    pub fn new() -> Self {
        Self {
            board: Board::initial_board(),
            debug: false,
        }
    }

    pub fn new_game(&mut self) {
        self.board = Board::initial_board();
    }

    pub fn set_to_startpos(&mut self) {
        self.board = Board::initial_board();
    }

    pub fn set_to_fen(&mut self, fen: &str) {
        self.board = Board::from_fen(fen);
    }

    pub fn get_board(&self) -> Board {
        self.board
    }

    pub fn display_board<W: Write>(&self, writer: &mut W) {
        let _ = self.board.write(writer);
    }

    pub fn apply_moves(&mut self, moves: &[&str]) {
        for mv in moves {
            self.board.update_by_move(self.board.new_move_from_pure(mv));
        }
    }

    pub fn set_debug(&mut self, val: bool) {
        self.debug = val;
    }
}
