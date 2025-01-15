#![allow(dead_code)]

use board::Board;
use moves::Move;

mod bitboard;
mod board;
mod common;
mod fen;
mod moves;

fn main() {
    println!("Kaik Chess Engine");
    println!();

    let b = Board::initial_board();
    b.print();

    // let b: Board = "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2".into();
    // b.print();
    // println!("FEN: {b}");

    let _b: Board = "2k5/8/8/8/8/8/2Pp4/2K5 w - - 0 1".into();
    let b: Board = "2k5/2Pp4/8/8/8/8/8/2K5 b - - 0 1".into();
    b.print();
    let moves = b.generate_moves();
    // Move::print_list(&moves);
    print_moves_with_board(&b, &moves);
}

fn print_moves_with_board(board: &Board, moves: &[Move]) {
    for mv in moves {
        println!("{mv}");
        board.print_with_move(Some(*mv));
    }
}
