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
            if let Some(board_copy) = self.copy_with_move(mv) {
                nodes += board_copy.perft(depth - 1);
            }
        }
        nodes
    }

    // Listing all moves and for each move, the perft of the decremented depth.
    pub fn divide(&self, depth: usize) -> Vec<(Move, usize)> {
        assert!(depth > 0);
        let mut nodes = Vec::new();
        let move_list = self.generate_moves();
        for mv in move_list {
            if let Some(board_copy) = self.copy_with_move(mv) {
                nodes.push((mv, board_copy.perft(depth - 1)));
            }
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

    #[test]
    fn test_peterellisjones() {
        // Test cases from <https://gist.github.com/peterellisjones/8c46c28141c162d1d8a0f0badbc9cff9>
        let b: Board = "r6r/1b2k1bq/8/8/7B/8/8/R3K2R b KQ - 3 2".into();
        assert_eq!(b.perft(1), 8);

        let b: Board = "8/8/8/2k5/2pP4/8/B7/4K3 b - d3 0 3".into();
        assert_eq!(b.perft(1), 8);

        let b: Board = "r1bqkbnr/pppppppp/n7/8/8/P7/1PPPPPPP/RNBQKBNR w KQkq - 2 2".into();
        assert_eq!(b.perft(1), 19);

        // let b: Board = "r3k2r/p1pp1pb1/bn2Qnp1/2qPN3/1p2P3/2N5/PPPBBPPP/R3K2R b KQkq - 3 2".into();
        // assert_eq!(b.perft(1), 5);

        let b: Board = "2kr3r/p1ppqpb1/bn2Qnp1/3PN3/1p2P3/2N5/PPPBBPPP/R3K2R b KQ - 3 2".into();
        assert_eq!(b.perft(1), 44);

        let b: Board = "rnb2k1r/pp1Pbppp/2p5/q7/2B5/8/PPPQNnPP/RNB1K2R w KQ - 3 9".into();
        assert_eq!(b.perft(1), 39);

        let b: Board = "2r5/3pk3/8/2P5/8/2K5/8/8 w - - 5 4".into();
        assert_eq!(b.perft(1), 9);

        // let b: Board = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8".into();
        // assert_eq!(b.perft(3), 62379);

        let b: Board =
            "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10".into();
        assert_eq!(b.perft(3), 89890);

        // let b: Board = "3k4/3p4/8/K1P4r/8/8/8/8 b - - 0 1".into();
        // assert_eq!(b.perft(6), 1134888);

        // let b: Board = "8/8/4k3/8/2p5/8/B2P2K1/8 w - - 0 1".into();
        // assert_eq!(b.perft(6), 1015133);

        // let b: Board = "8/8/1k6/2b5/2pP4/8/5K2/8 b - d3 0 1".into();
        // assert_eq!(b.perft(6), 1440467);

        // let b: Board = "5k2/8/8/8/8/8/8/4K2R w K - 0 1".into();
        // assert_eq!(b.perft(6), 661072);

        // let b: Board = "3k4/8/8/8/8/8/8/R3K3 w Q - 0 1".into();
        // assert_eq!(b.perft(6), 803711);

        // let b: Board = "r3k2r/1b4bq/8/8/8/8/7B/R3K2R w KQkq - 0 1".into();
        // assert_eq!(b.perft(4), 1274206);

        // let b: Board = "r3k2r/8/3Q4/8/8/5q2/8/R3K2R b KQkq - 0 1".into();
        // assert_eq!(b.perft(4), 1720476);

        // let b: Board = "2K2r2/4P3/8/8/8/8/8/3k4 w - - 0 1".into();
        // assert_eq!(b.perft(6), 3821001);

        // let b: Board = "8/8/1P2K3/8/2n5/1q6/8/5k2 b - - 0 1".into();
        // assert_eq!(b.perft(5), 1004658);

        // let b: Board = "4k3/1P6/8/8/8/8/K7/8 w - - 0 1".into();
        // assert_eq!(b.perft(6), 217342);

        // let b: Board = "8/P1k5/K7/8/8/8/8/8 w - - 0 1".into();
        // assert_eq!(b.perft(6), 92683);

        // let b: Board = "K1k5/8/P7/8/8/8/8/8 w - - 0 1".into();
        // assert_eq!(b.perft(6), 2217);

        // let b: Board = "8/k1P5/8/1K6/8/8/8/8 w - - 0 1".into();
        // assert_eq!(b.perft(7), 567584);

        // let b: Board = "8/8/2k5/5q2/5n2/8/5K2/8 b - - 0 1".into();
        // assert_eq!(b.perft(4), 23527);
    }
}
