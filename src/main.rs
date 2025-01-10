use bitboard::print_bitboard;

mod bitboard;

fn main() {
    let white_pawns = 0b0000000000000000000000000000000000000000000000001111111100000000;

    print_bitboard(white_pawns);
}
