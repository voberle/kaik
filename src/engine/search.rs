//! Search

use std::fmt::Display;

use crate::common::{Move, Score};

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

mod alphabeta;

// If we have multiple search implementation they can be chosen via features.
// The default search implementation is specified in Cargo.toml.
// It can be changed at the command-line:
//     cargo r --no-default-features --features negamax
// #[cfg(feature = "alphabeta")]
pub use alphabeta::run;
