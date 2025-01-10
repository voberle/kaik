use bitboard::BitBoard;
use board::Board;
use initial_board::INITIAL_BOARD;
use squares::Square;

mod bitboard;
mod board;
mod initial_board;
mod squares;

#[allow(clippy::unreadable_literal)]
fn main() {
    let white_pawns: BitBoard =
        0b0000000000000000000000000000000000000000000000001111111100000000.into();

    white_pawns.print();

    let square = Square::E5;
    let value: u8 = square.into();
    println!("Square: {square:?}, Value: {value}");

    // let mut board = Board::empty();
    // board.white[Board::PAWNS].set(Square::E2.into());
    // board.print();

    INITIAL_BOARD.print();
}
