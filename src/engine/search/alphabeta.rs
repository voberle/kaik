//! Alpha Beta search
//! Good explanation <http://web.archive.org/web/20070704121716/http://www.brucemo.com/compchess/programming/alphabeta.htm>

use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc::Sender,
    Arc,
};

use crate::{
    board::Board,
    common::{format_moves_as_pure_string, Move, Score, MAX_SCORE, MIN_SCORE},
    engine::{
        eval::eval,
        game::{Event, InfoData, SearchParams},
    },
    search::{
        self,
        Result::{self, BestMove, CheckMate, StaleMate},
    },
};

fn alphabeta_rec(
    board: &Board,
    depth: usize,
    mut alpha: Score,
    beta: Score,
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
                depth - 1,
                -beta,
                -alpha,
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

    if legal_moves {
        best_score
    } else if board.in_check() {
        MIN_SCORE // Checkmate
    } else {
        0 // Stalemate
    }
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
                depth - 1,
                -beta,
                -alpha,
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
        BestMove(best_move.unwrap(), best_score)
    } else if board.in_check() {
        CheckMate
    } else {
        StaleMate
    }
}

pub fn run(
    board: &Board,
    search_params: &SearchParams,
    event_sender: &Sender<Event>,
    stop_flag: &Arc<AtomicBool>,
) -> Result {
    // With the recursive implementation, real infinite search isn't an option.
    const MAX_DEPTH: usize = 7;
    let depth = match search_params.depth {
        Some(d) => MAX_DEPTH.min(d),
        None => MAX_DEPTH,
    };

    let mut nodes_count = 0;
    let mut pv_line = Vec::new();
    let result = alphabeta(board, depth, stop_flag, &mut nodes_count, &mut pv_line);

    info!("PV: {}", format_moves_as_pure_string(&pv_line));

    // TODO we could simply take the best move from the MV and not return it as part of the result.
    // So have alphabeta just return a score.
    // Checkmate/stalemate can be detected if PV is empty.

    let mut info_data = vec![
        InfoData::Depth(depth),
        InfoData::Nodes(nodes_count),
        InfoData::Pv(pv_line),
    ];

    if let BestMove(_mv, score) = result {
        info_data.push(InfoData::Score(score));
    }
    event_sender.send(Event::Info(info_data)).unwrap();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::common::Move;
    use crate::common::Piece::*;
    use crate::common::Square::*;

    #[test]
    fn test_startpos_depth_4() {
        let board = Board::initial_board();
        let stop_flag = Arc::new(AtomicBool::new(false));

        let mut nodes_count = 0;
        let mut pv_line = Vec::new();
        let r = alphabeta(&board, 4, &stop_flag, &mut nodes_count, &mut pv_line);
        assert_eq!(r, BestMove(Move::quiet(A2, A3, WhitePawn), 0));
        assert_eq!(nodes_count, 2024);
        assert_eq!(
            pv_line,
            [
                Move::quiet(A2, A3, WhitePawn),
                Move::quiet(A7, A5, BlackPawn),
                Move::quiet(B2, B3, WhitePawn),
                Move::quiet(A5, A4, BlackPawn),
            ]
        );
    }

    #[test]
    fn test_mate_minus_1() {
        // Not yet mate but mate on next move.
        let board: Board = "2kr1b2/Rp3pp1/8/8/2b1K2r/4P1pP/8/1NB1nBNR w - - 0 40".into();
        let stop_flag = Arc::new(AtomicBool::new(false));

        let mut nodes_count = 0;
        let r = alphabeta(&board, 4, &stop_flag, &mut nodes_count, &mut Vec::new());
        assert_eq!(r, BestMove(Move::quiet(E4, E5, WhiteKing), MIN_SCORE));
    }

    #[test]
    fn test_smothered_mate() {
        // Has both a smothered mate via a queen sacrifice and simpler
        // one via a knight sacrifice, in 2 moves.
        let board: Board = "2r4k/6pp/8/4N3/8/1Q6/B5PP/7K w - - 0 1".into();
        let stop_flag = Arc::new(AtomicBool::new(false));

        let mut nodes_count = 0;
        let r = alphabeta(&board, 4, &stop_flag, &mut nodes_count, &mut Vec::new());
        assert_eq!(r, BestMove(Move::quiet(E5, G6, WhiteKnight), MAX_SCORE));
    }
}
