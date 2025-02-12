//! Alpha Beta search
//! Good explanation <http://web.archive.org/web/20070704121716/http://www.brucemo.com/compchess/programming/alphabeta.htm>

use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc::Sender,
    Arc,
};

use crate::{
    board::Board,
    common::{Move, Score, MAX_SCORE, MIN_SCORE},
    engine::{
        eval::eval,
        game::{Event, InfoData, SearchParams},
    },
    search,
};

fn alphabeta_rec(
    board: &Board,
    mut alpha: Score,
    beta: Score,
    depth: usize,
    stop_flag: &Arc<AtomicBool>,
    nodes_count: &mut usize,
    pv_line: &mut Vec<Move>,
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
            *nodes_count += 1;
            let mut child_line = Vec::new();
            let score = -alphabeta_rec(
                &board_copy,
                -beta,
                -alpha,
                depth - 1,
                stop_flag,
                nodes_count,
                &mut child_line,
            );
            legal_moves = true;

            if score > best_score {
                best_score = score;
                if score > alpha {
                    alpha = score;
                    pv_line.clear();
                    pv_line.push(mv);
                    pv_line.extend_from_slice(&child_line);
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
    nodes_count: &mut usize,
    pv_line: &mut Vec<Move>,
) -> search::Result {
    debug_assert!(depth > 0);

    let mut best_score = MIN_SCORE;
    let mut best_move = None;

    let mut alpha = MIN_SCORE;
    let beta = MAX_SCORE;

    let mut legal_moves = false;
    let move_list = board.generate_moves();
    for mv in move_list {
        if let Some(board_copy) = board.copy_with_move(mv) {
            *nodes_count += 1;
            let mut child_line = Vec::new();
            let score = -alphabeta_rec(
                &board_copy,
                -beta,
                -alpha,
                depth - 1,
                stop_flag,
                nodes_count,
                &mut child_line,
            );
            legal_moves = true;

            if score > best_score || best_move.is_none() {
                best_score = score;
                best_move = Some(mv);

                if score > alpha {
                    alpha = score;
                    pv_line.clear();
                    pv_line.push(mv);
                    pv_line.extend_from_slice(&child_line);
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
    // With the recursive implementation, real infinite search isn't an option.
    const MAX_DEPTH: usize = 7;
    let depth = match search_params.depth {
        Some(d) => MAX_DEPTH.min(d),
        None => MAX_DEPTH,
    };

    let mut nodes_count = 0;
    let mut pv_line = Vec::new();
    let result = alphabeta(board, depth, stop_flag, &mut nodes_count, &mut pv_line);

    info!(
        "PV: {}",
        crate::common::format_moves_as_pure_string(&pv_line)
    );

    // TODO we could simply take the best move from the MV and not return it as part of the result.
    // So have alphabeta just return a score.

    if let search::Result::BestMove(_mv, score) = result {
        let info_data = vec![
            InfoData::Depth(depth),
            InfoData::Score(score),
            InfoData::Nodes(nodes_count),
            InfoData::Pv(pv_line),
        ];
        event_sender.send(Event::Info(info_data)).unwrap();
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::common::Move;
    use crate::common::Piece::*;
    use crate::common::Square::*;

    #[test]
    fn test_mate_minus_1() {
        // Not yet mate but mate on next move.
        let board: Board = "2kr1b2/Rp3pp1/8/8/2b1K2r/4P1pP/8/1NB1nBNR w - - 0 40".into();
        let stop_flag = Arc::new(AtomicBool::new(false));

        let mut nodes_count = 0;
        let r = alphabeta(&board, 4, &stop_flag, &mut nodes_count, &mut Vec::new());
        assert_eq!(
            r,
            search::Result::BestMove(Move::quiet(E4, E5, WhiteKing), MIN_SCORE)
        );
    }
}
