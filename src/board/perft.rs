//! Perft <https://www.chessprogramming.org/Perft>

use crate::{board::Board, moves::Move};

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

    // Listing all moves and for each move, the perft of the decremented depth.
    pub fn divide(&self, depth: usize) -> Vec<(Move, usize)> {
        assert!(depth > 0);
        let mut nodes = Vec::new();
        let move_list = self.generate_moves();
        for mv in move_list {
            let mut board_copy = *self;
            board_copy.update_by_move(mv);
            nodes.push((mv, board_copy.perft(depth - 1)));
        }
        nodes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perft_divide() {
        let board = Board::initial_board();
        assert_eq!(
            board.perft(2),
            board
                .divide(2)
                .iter()
                .map(|(_, count)| *count)
                .sum::<usize>()
        );
    }

    #[test]
    fn test_perft_initial() {
        let board = Board::initial_board();
        assert_eq!(board.perft(1), 20);
        assert_eq!(board.perft(2), 400);
        assert_eq!(board.perft(3), 8902);
    }
}
