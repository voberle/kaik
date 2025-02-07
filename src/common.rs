//! Basic common definitions for the whole project.
//! Should be mainly enums and such things, with some utils. No actual logic.
//! No dependencies on other parts of the project.

mod colors;
mod moves;
mod pieces;
mod squares;

pub use colors::Color;
pub use moves::Move;
pub use pieces::Piece;
pub use pieces::PieceListBoard;
pub use squares::Square;

// Centipawns
pub type Score = i32;

pub const MIN_SCORE: Score = i32::MIN / 2; // not just taking MIN, so that negating doesn't overflow

pub const ENGINE_NAME: &str = "Kaik";
pub const ENGINE_AUTHOR: &str = "Vincent Oberle";
