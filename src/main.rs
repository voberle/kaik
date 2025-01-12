#![allow(dead_code)]

use board::Board;

mod bitboard;
mod board;
mod constants;
mod fen;
mod movements;
mod pieces;
mod side;
mod squares;

fn main() {
    println!("Kaik Chess Engine");
    println!();

    let b = Board::initial_board();
    b.print();
}
