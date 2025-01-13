#![allow(dead_code)]

use board::Board;

mod bitboard;
mod board;
mod common;
mod constants;
mod fen;
mod movements;
mod moves;

fn main() {
    println!("Kaik Chess Engine");
    println!();

    let b = Board::initial_board();
    b.print();

    let b: Board = "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2".into();
    b.print();
    println!("FEN: {b}");
}
