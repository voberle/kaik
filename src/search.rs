//! Search

use std::{
    fmt::Display,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use crate::{board::Board, common::Score, moves::Move};

pub enum Result {
    BestMove(Move),
    CheckMate,
    StaleMate,
}

impl Display for Result {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Result::BestMove(mv) => write!(f, "{mv}"),
            Result::CheckMate => write!(f, "Checkmate"),
            Result::StaleMate => write!(f, "Stalemate"),
        }
    }
}

fn nega_max_rec(board: &Board, depth: usize, stop_flag: &Arc<AtomicBool>) -> Score {
    if depth == 0 || stop_flag.load(Ordering::Relaxed) {
        return board.eval();
    }

    let mut legal_moves = false;
    let mut max = Score::MIN / 2;

    let move_list = board.generate_moves();
    for mv in move_list {
        if let Some(board_copy) = board.copy_with_move(mv) {
            let s = -nega_max_rec(&board_copy, depth - 1, stop_flag);
            if s > max {
                max = s;
            }
            legal_moves = true;
        }
    }

    if !legal_moves {
        // println!("No legal moves for this board");
        // board.print();
        // Either checkmage or stalemate
        return if board.attacks_king(board.get_side_to_move()) != 0 {
            Score::MIN / 2
        } else {
            0
        };
    }
    max
}

// Returns the best moves found via NegaMax.
// The stop_flag should be checked regularly. When true, the search should be interrupted
// and return the best move found so far.
pub fn negamax(board: &Board, depth: usize, stop_flag: &Arc<AtomicBool>) -> Result {
    assert!(depth > 0);

    let mut best_score = Score::MIN / 2;
    let mut best_move = None;

    let move_list = board.generate_moves();
    for mv in move_list {
        if let Some(board_copy) = board.copy_with_move(mv) {
            let score = -nega_max_rec(&board_copy, depth - 1, stop_flag);
            if score > best_score {
                best_score = score;
                best_move = Some(mv);
            }
        }

        if stop_flag.load(Ordering::Relaxed) {
            break;
        }
    }

    if let Some(mv) = best_move {
        Result::BestMove(mv)
    } else {
        // Either checkmage or stalemate
        if board.attacks_king(board.get_side_to_move()) != 0 {
            Result::CheckMate
        } else {
            Result::StaleMate
        }
    }
}
