//! Search

use std::{
    fmt::Display,
    sync::{atomic::AtomicBool, Arc},
};

use crate::{
    board::Board,
    common::{Move, Score},
};

use super::negamax::negamax;

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
    depth: usize,
    stop_flag: &Arc<AtomicBool>,
    nodes_count: &mut usize,
) -> Result {
    negamax(board, depth, stop_flag, nodes_count)
}
