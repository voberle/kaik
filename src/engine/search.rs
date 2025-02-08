//! Search

use std::{
    fmt::Display,
    sync::{atomic::AtomicBool, Arc},
};

use crate::{
    board::Board,
    common::{Move, Score},
};

use super::{alpha_beta, game::SearchParams};

#[derive(Debug, PartialEq)]
pub enum Result {
    BestMove(Move, Score),
    CheckMate,
    StaleMate,
}

impl Display for Result {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Result::BestMove(mv, _score) => write!(f, "{mv}"),
            Result::CheckMate => write!(f, "Checkmate"),
            Result::StaleMate => write!(f, "Stalemate"),
        }
    }
}

pub fn run(
    board: &Board,
    search_params: &SearchParams,
    stop_flag: &Arc<AtomicBool>,
    nodes_count: &mut usize,
) -> Result {
    // With the recursive implementation of Negamax, real infinite search isn't an option.
    // const MAX_DEPTH: usize = 4;
    const MAX_DEPTH: usize = 7;
    let depth = match search_params.depth {
        Some(d) => MAX_DEPTH.min(d),
        None => MAX_DEPTH,
    };

    // negamax::negamax(board, depth, stop_flag, nodes_count)
    alpha_beta::alpha_beta(board, depth, stop_flag, nodes_count)
}
