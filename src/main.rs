use bitboard::BitBoard;
use squares::Square;

mod bitboard;
mod squares;

#[allow(clippy::unreadable_literal)]
fn main() {
    let white_pawns: BitBoard =
        0b0000000000000000000000000000000000000000000000001111111100000000.into();

    white_pawns.print();

    let square = Square::E5;
    let value: u8 = square.into();
    println!("Square: {square:?}, Value: {value}");
}
