//! Basic common definitions for the whole project.
//! Should be mainly enums and such things, with some utils. No actual logic.
//! No dependencies on other parts of the project.

mod color;
mod pieces;
mod squares;

pub use color::Color;
pub use pieces::Piece;
pub use pieces::PieceListBoard;
pub use squares::Square;
