//! Search

use itertools::Itertools;

use crate::{board::Board, common::Score, moves::Move};

fn nega_max_rec(board: &Board, depth: usize) -> Score {
    if depth == 0 {
        // We take the negative of the eval, because we do a copy-make approach for move generation
        // so once the move is made, it's the other side that moves.
        return -board.eval();
    }

    let mut max = Score::MIN / 2;

    let move_list = board.generate_moves();
    for mv in move_list {
        if let Some(board_copy) = board.copy_with_move(mv) {
            let s = -nega_max_rec(&board_copy, depth - 1);
            if s > max {
                max = s;
            }
        }
    }
    max
}

// Returns the best moves found via NegaMax.
pub fn negamax(board: &Board, depth: usize) -> Vec<Move> {
    assert!(depth > 0);

    let move_list = board.generate_moves();
    move_list
        .iter()
        .max_set_by_key(|mv| {
            if let Some(board_copy) = board.copy_with_move(**mv) {
                nega_max_rec(&board_copy, depth - 1)
            } else {
                // If the move cannot be made (king is check), return min score.
                // We divide it by 2 as otherwise negating fails with overflow.
                Score::MIN / 2
            }
        })
        .into_iter()
        .copied()
        .collect()
}
