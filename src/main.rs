use bitboard::BitBoard;

mod bitboard;

#[allow(clippy::unreadable_literal)]
fn main() {
    let white_pawns: BitBoard =
        0b0000000000000000000000000000000000000000000000001111111100000000.into();

    white_pawns.print();
}
