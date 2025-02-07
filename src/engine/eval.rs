//! Evaluation of the position.

use crate::{
    board::Board,
    common::{Color, Score},
};

#[allow(clippy::cast_possible_wrap)]
pub fn eval(board: &Board) -> Score {
    let (white_score, black_score) = material_scores(board);
    // The score is relative to who is moving
    // <https://www.chessprogramming.org/Evaluation#Side_to_move_relative>
    if board.get_side_to_move() == Color::White {
        white_score as i32 - black_score as i32
    } else {
        black_score as i32 - white_score as i32
    }
}

fn material_scores(board: &Board) -> (u32, u32) {
    // From <https://www.chessprogramming.org/Simplified_Evaluation_Function>
    const P_VALUE: u32 = 100;
    const N_VALUE: u32 = 320;
    const B_VALUE: u32 = 330;
    const R_VALUE: u32 = 500;
    const Q_VALUE: u32 = 900;
    const K_VALUE: u32 = 20000;

    board.material_scores(&[P_VALUE, N_VALUE, B_VALUE, R_VALUE, Q_VALUE, K_VALUE])
}
