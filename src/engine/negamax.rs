//! Negamax search

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use crate::{
    board::Board,
    common::{Score, MIN_SCORE},
};

use super::eval::eval;
use super::search::Result;

fn nega_max_rec(
    board: &Board,
    depth: usize,
    stop_flag: &Arc<AtomicBool>,
    nodes_count: &mut usize,
) -> Score {
    if depth == 0 || stop_flag.load(Ordering::Relaxed) {
        return eval(board);
    }

    let mut legal_moves = false;
    let mut max = MIN_SCORE;

    let move_list = board.generate_moves();
    for mv in move_list {
        if let Some(board_copy) = board.copy_with_move(mv) {
            *nodes_count += 1;
            let s = -nega_max_rec(&board_copy, depth - 1, stop_flag, nodes_count);
            legal_moves = true;

            if s > max {
                max = s;
            }
        }
    }

    if !legal_moves {
        // Either checkmage or stalemate
        return if board.attacks_king(board.get_side_to_move()) != 0 {
            MIN_SCORE
        } else {
            0
        };
    }
    max
}

// Returns the best moves found via NegaMax.
// The stop_flag should be checked regularly. When true, the search should be interrupted
// and return the best move found so far.
pub fn negamax(
    board: &Board,
    depth: usize,
    stop_flag: &Arc<AtomicBool>,
    nodes_count: &mut usize,
) -> Result {
    assert!(depth > 0);

    let mut best_score = MIN_SCORE;
    let mut best_move = None;

    let mut legal_moves = false;
    let move_list = board.generate_moves();
    for mv in move_list {
        if let Some(board_copy) = board.copy_with_move(mv) {
            *nodes_count += 1;
            let score = -nega_max_rec(&board_copy, depth - 1, stop_flag, nodes_count);
            legal_moves = true;

            if score > best_score || best_move.is_none() {
                best_score = score;
                best_move = Some(mv);
            }
        }

        if stop_flag.load(Ordering::Relaxed) {
            break;
        }
    }

    if legal_moves {
        Result::BestMove(best_move.unwrap(), best_score)
    } else {
        // Either checkmage or stalemate
        if board.attacks_king(board.get_side_to_move()) != 0 {
            Result::CheckMate
        } else {
            Result::StaleMate
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::common::Move;
    use crate::common::Piece::*;
    use crate::common::Square::*;

    #[test]
    fn test_negamax_mate_minus_1() {
        // Not yet mate but mate on next move.
        let board: Board = "2kr1b2/Rp3pp1/8/8/2b1K2r/4P1pP/8/1NB1nBNR w - - 0 40".into();
        let stop_flag = Arc::new(AtomicBool::new(false));

        let mut nodes_count = 0;
        let r = negamax(&board, 4, &stop_flag, &mut nodes_count);
        assert_eq!(
            r,
            Result::BestMove(Move::quiet(E4, E5, WhiteKing), MIN_SCORE)
        );
    }
}
