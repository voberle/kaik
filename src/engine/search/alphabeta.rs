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
    search::Result::{self, BestMove, CheckMate},
};

const MATE_SCORE: Score = 40_000;

fn mate_in(score: Score) -> i32 {
    (MATE_SCORE - score + 1) / 2
}

fn mated_in(score: Score) -> i32 {
    (MATE_SCORE + score) / 2
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

    info!("PV: {}", format_moves_as_pure_string(&pv_line));

    // TODO we could simply take the best move from the MV and not return it as part of the result.
    // So have alphabeta just return a score.
    // Checkmate/stalemate can be detected if PV is empty.

    let mut info_data = vec![
        InfoData::Depth(depth),
        InfoData::Nodes(nodes_count),
        InfoData::Pv(pv_line.clone()),
    ];

    if score >= MATE_SCORE - 1000 {
        // Handle up to mate in 500 or so.
        let mate_in = (MATE_SCORE - score + 1) / 2;
        // println!("Mate in {mate_in}");
        info_data.push(InfoData::ScoreMate(mate_in));
    } else if score <= -MATE_SCORE + 1000 {
        let mated_in = (MATE_SCORE + score) / 2;
        // println!("Mated in {mated_in}");
        if mated_in == 0 {
            return CheckMate;
        }
        info_data.push(InfoData::ScoreMate(-mated_in));
    } else {
        info_data.push(InfoData::Score(score));
    }

    event_sender.send(Event::Info(info_data)).unwrap();

    // TODO handle stalemate

    BestMove(pv_line[0], score)
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
        let score = alphabeta(
            &board,
            4,
            MIN_SCORE,
            MAX_SCORE,
            MATE_SCORE,
            &stop_flag,
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
    }

    #[test]
    fn test_mate_minus_1() {
        // Not yet mate but mate on next move.
        let board: Board = "2kr1b2/Rp3pp1/8/8/2b1K2r/4P1pP/8/1NB1nBNR w - - 0 40".into();
        let stop_flag = Arc::new(AtomicBool::new(false));

        let mut nodes_count = 0;
        let mut pv_line = Vec::new();
        let score = alphabeta(
            &board,
            4,
            MIN_SCORE,
            MAX_SCORE,
            MATE_SCORE,
            &stop_flag,
            &mut nodes_count,
            &mut pv_line,
        );

        assert_eq!(pv_line[0], Move::quiet(E4, E5, WhiteKing));
        assert_eq!(mated_in(score), 1);
        assert_eq!(score, -MATE_SCORE + 2);
    }

    #[test]
    fn test_smothered_mate() {
        // Has both a smothered mate via a queen sacrifice and simpler
        // one via a knight sacrifice, in 2 moves.
        let board: Board = "2r4k/6pp/8/4N3/8/1Q6/B5PP/7K w - - 0 1".into();
        let stop_flag = Arc::new(AtomicBool::new(false));

        let mut nodes_count = 0;
        let mut pv_line = Vec::new();
        let score = alphabeta(
            &board,
            4,
            MIN_SCORE,
            MAX_SCORE,
            MATE_SCORE,
            &stop_flag,
            &mut nodes_count,
            &mut pv_line,
        );

        assert_eq!(pv_line[0], Move::quiet(E5, G6, WhiteKnight));
        assert_eq!(mate_in(score), 2);
        assert_eq!(score, MATE_SCORE - 3);
    }
}
