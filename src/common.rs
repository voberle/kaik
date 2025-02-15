//! Basic common definitions for the whole project.
//! Should be mainly enums and such things, with some utils. No actual logic.
//! No dependencies on other parts of the project.

mod colors;
mod moves;
mod pieces;
mod squares;

pub use colors::Color;
pub use moves::format_moves_as_pure_string;
pub use moves::Move;
pub use pieces::Piece;
pub use pieces::PieceListBoard;
pub use squares::Square;

// Centipawns
pub type Score = i32;

pub const MIN_SCORE: Score = i32::MIN / 2; // not just taking MIN, so that negating doesn't overflow
pub const MAX_SCORE: Score = -MIN_SCORE;

pub const ENGINE_NAME: &str = "Kaik";
pub const ENGINE_AUTHOR: &str = "Vincent Oberle";

#[cfg(test)]
mod tests {
    use crate::common::{MAX_SCORE, MIN_SCORE};

    #[test]
    fn test_min_max_score() {
        assert_eq!(-MIN_SCORE, MAX_SCORE);
        assert_eq!(-MAX_SCORE, MIN_SCORE);
    }
}
