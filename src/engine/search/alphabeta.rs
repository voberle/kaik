//! Alpha Beta search

use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc::Sender,
    Arc,
};

use crate::{
    board::Board,
    common::{Score, MAX_SCORE, MIN_SCORE},
    engine::{
        eval::eval,
        game::{Event, InfoData, SearchParams},
    },
    search,
};

use super::Info;

fn alphabeta_rec(
    board: &Board,
    mut alpha: Score,
    beta: Score,
    depth: usize,
    stop_flag: &Arc<AtomicBool>,
    info: &mut Info,
) -> Score {
    if depth == 0 || stop_flag.load(Ordering::Relaxed) {
        // TODO here we should do a quiescence search, which makes the alpha-beta search much more stable.
        // <https://www.chessprogramming.org/Quiescence_Search>
        return eval(board);
    }

    let mut legal_moves = false;
    let mut best_score = MIN_SCORE;

    let move_list = board.generate_moves();
    for mv in move_list {
        if let Some(board_copy) = board.copy_with_move(mv) {
            info.nodes += 1;
            let score = -alphabeta_rec(&board_copy, -beta, -alpha, depth - 1, stop_flag, info);
            legal_moves = true;

            if score > best_score {
                best_score = score;
                if score > alpha {
                    alpha = score;
                }
            }
            if score >= beta {
                break; // fail soft beta-cutoff
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
    best_score
}

// Returns the best moves found via NegaMax.
// The stop_flag should be checked regularly. When true, the search should be interrupted
// and return the best move found so far.
fn alphabeta(
    board: &Board,
    depth: usize,
    stop_flag: &Arc<AtomicBool>,
    info: &mut Info,
) -> search::Result {
    assert!(depth > 0);

    let mut best_score = MIN_SCORE;
    let mut best_move = None;

    let mut alpha = MIN_SCORE;
    let beta = MAX_SCORE;

    let mut legal_moves = false;
    let move_list = board.generate_moves();
    for mv in move_list {
        if let Some(board_copy) = board.copy_with_move(mv) {
            info.nodes += 1;
            let score = -alphabeta_rec(&board_copy, -beta, -alpha, depth - 1, stop_flag, info);
            legal_moves = true;

            if score > best_score || best_move.is_none() {
                best_score = score;
                best_move = Some(mv);

                if score > alpha {
                    alpha = score;
                }
            }
            if score >= beta {
                break; // fail soft beta-cutoff
            }
        }

        if stop_flag.load(Ordering::Relaxed) {
            break;
        }
    }

    if legal_moves {
        search::Result::BestMove(best_move.unwrap(), best_score)
    } else {
        // Either checkmage or stalemate
        if board.attacks_king(board.get_side_to_move()) != 0 {
            search::Result::CheckMate
        } else {
            search::Result::StaleMate
        }
    }
}

pub fn run(
    board: &Board,
    search_params: &SearchParams,
    event_sender: &Sender<Event>,
    stop_flag: &Arc<AtomicBool>,
) -> search::Result {
    // With the recursive implementation of Negamax, real infinite search isn't an option.
    const MAX_DEPTH: usize = 7;
    let depth = match search_params.depth {
        Some(d) => MAX_DEPTH.min(d),
        None => MAX_DEPTH,
    };

    let mut info = Info::default();
    let result = alphabeta(board, depth, stop_flag, &mut info);

    if let search::Result::BestMove(_mv, score) = result {
        let info_data = vec![
            InfoData::Depth(depth),
            InfoData::Score(score),
            InfoData::Nodes(info.nodes),
        ];
        event_sender.send(Event::Info(info_data)).unwrap();
    }

    result
}
