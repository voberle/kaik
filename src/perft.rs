//! Perft <https://www.chessprogramming.org/Perft>

use crate::board::Board;

impl Board {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perft_initial() {
        let board = Board::initial_board();
        assert_eq!(board.perft(1), 20);
        assert_eq!(board.perft(2), 400);
        assert_eq!(board.perft(3), 8902);
    }
}
