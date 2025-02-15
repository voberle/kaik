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
    search::Result::{self, BestMove, CheckMate, StaleMate},
};

const MATE_SCORE: Score = 40_000;

fn mate_in(score: Score) -> Option<i32> {
    // Handle up to mate in 500 or so.
    if score >= MATE_SCORE - 1000 {
        let dist = (MATE_SCORE - score + 1) / 2;
        info!("Mate in {dist}");
        Some(dist)
    } else {
        None
    }
}

fn mated_in(score: Score) -> Option<i32> {
    if score <= -MATE_SCORE + 1000 {
        let dist = (MATE_SCORE + score) / 2;
        info!("Mated in {dist}");
        Some(dist)
    } else {
        None
    }
}

// The stop_flag should be checked regularly. When true, the search should be interrupted
// and return the best move found so far.
// Mate scoring logic from <http://web.archive.org/web/20070707035457/www.brucemo.com/compchess/programming/matescore.htm>
#[allow(clippy::too_many_arguments)] // TODO Fix with a Search struct (stop_flag, nodes_count)
fn alphabeta(
    board: &Board,
    depth: usize,
    mut alpha: Score,
    beta: Score,
    mate: Score,
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
            let score = -alphabeta(
                &board_copy,
                depth - 1,
                -beta,
                -alpha,
                mate - 1,
                stop_flag,
                nodes_count,
                &mut child_line,
            );
            legal_moves = true;

            if score > best_score {
                best_score = score;
                if score > alpha {
                    alpha = score;
                    // PV update.
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
        -mate // Checkmate
    } else {
        0 // Stalemate
          // Doesn't have to be 0, see <http://web.archive.org/web/20070707023203/http://www.brucemo.com/compchess/programming/contempt.htm>
    }
}

// Executes an alpha-beta search with iterative deepening.
pub fn run(
    board: &Board,
    search_params: &SearchParams,
    event_sender: &Sender<Event>,
    stop_flag: &Arc<AtomicBool>,
) -> Result {
    // usize::MAX is for infinite search
    let max_depth = search_params.depth.unwrap_or(usize::MAX);

    let mut nodes_count = 0;
    let mut pv_line = Vec::new();

    let mut result = StaleMate; // Dummy init val.
    let mut depth = 1;
    loop {
        let score = alphabeta(
            board,
            depth,
            MIN_SCORE,
            MAX_SCORE,
            MATE_SCORE,
            stop_flag,
            &mut nodes_count,
            &mut pv_line,
        );
        if depth > 1 && stop_flag.load(Ordering::Relaxed) {
            // If we got interrupted during a search at any depth beyond the first,
            // we ignore the incomplete results from that depth and use the previous one.
            break;
        }

        info!("PV: {}", format_moves_as_pure_string(&pv_line));

        let mut info_data = vec![
            InfoData::Depth(depth),
            InfoData::Nodes(nodes_count),
            InfoData::Pv(pv_line.clone()),
        ];

        if let Some(mate_in) = mate_in(score) {
            info_data.push(InfoData::ScoreMate(mate_in));
        } else if let Some(mated_in) = mated_in(score) {
            if mated_in == 0 {
                debug_assert!(pv_line.is_empty());
                return CheckMate;
            }
            // Use negative values if we are getting mated.
            info_data.push(InfoData::ScoreMate(-mated_in));
        } else {
            info_data.push(InfoData::Score(score));
        }

        event_sender.send(Event::Info(info_data)).unwrap();

        if pv_line.is_empty() {
            return StaleMate;
        }

        result = BestMove(pv_line[0], score);

        depth += 1;
        if depth >= max_depth || stop_flag.load(Ordering::Relaxed) {
            break;
        }
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
    fn test_startpos_depth_4() {
        let board = Board::initial_board();
        let mut nodes_count = 0;
        let mut pv_line = Vec::new();
        let score = alphabeta(
            &board,
            4,
            MIN_SCORE,
            MAX_SCORE,
            MATE_SCORE,
            &Arc::new(AtomicBool::new(false)),
            &mut nodes_count,
            &mut pv_line,
        );

        assert_eq!(pv_line[0], Move::quiet(A2, A3, WhitePawn));
        assert_eq!(score, 0);
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
        assert_eq!(mate_in(score), None);
        assert_eq!(mated_in(score), None);
    }

    #[test]
    fn test_mated_minus_1() {
        // Mated on next move.
        let board: Board = "2kr1b2/Rp3pp1/8/8/2b1K2r/4P1pP/8/1NB1nBNR w - - 0 40".into();
        let mut nodes_count = 0;
        let mut pv_line = Vec::new();
        let score = alphabeta(
            &board,
            4,
            MIN_SCORE,
            MAX_SCORE,
            MATE_SCORE,
            &Arc::new(AtomicBool::new(false)),
            &mut nodes_count,
            &mut pv_line,
        );

        assert_eq!(pv_line[0], Move::quiet(E4, E5, WhiteKing));
        assert_eq!(mated_in(score), Some(1));
        assert_eq!(mate_in(score), None);
        assert_eq!(score, -MATE_SCORE + 2);
    }

    #[test]
    fn test_smothered_mate() {
        // Has both a smothered mate via a queen sacrifice and simpler
        // one via a knight sacrifice, in 2 moves.
        let board: Board = "2r4k/6pp/8/4N3/8/1Q6/B5PP/7K w - - 0 1".into();
        let mut nodes_count = 0;
        let mut pv_line = Vec::new();
        let score = alphabeta(
            &board,
            4,
            MIN_SCORE,
            MAX_SCORE,
            MATE_SCORE,
            &Arc::new(AtomicBool::new(false)),
            &mut nodes_count,
            &mut pv_line,
        );

        assert_eq!(pv_line[0], Move::quiet(E5, G6, WhiteKnight));
        assert_eq!(mate_in(score), Some(2));
        assert_eq!(mated_in(score), None);
        assert_eq!(score, MATE_SCORE - 3);
    }

    #[test]
    fn test_stalemate() {
        // Black to move, but it cannot, stalemate.
        let board: Board = "4k3/4P3/4Q3/8/8/8/8/5K2 b - - 0 1".into();
        let mut nodes_count = 0;
        let mut pv_line = Vec::new();
        let score = alphabeta(
            &board,
            4,
            MIN_SCORE,
            MAX_SCORE,
            MATE_SCORE,
            &Arc::new(AtomicBool::new(false)),
            &mut nodes_count,
            &mut pv_line,
        );

        assert!(pv_line.is_empty());
        assert_eq!(score, 0);
        assert_eq!(mate_in(score), None);
        assert_eq!(mated_in(score), None);
    }
}
