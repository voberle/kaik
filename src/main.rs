#![allow(dead_code)]

use board::Board;
#[allow(unused_imports)]
use common::Piece;
use moves::Move;

mod bitboard;
mod board;
mod common;
mod fen;
mod moves;

#[allow(unused_variables)]
fn main() {
    println!("  Kaik Chess Engine");
    println!("         by Vincent");
    println!();

    let b = Board::initial_board();
    b.print();
    println!();

    // let b: Board = "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2".into();
    // b.print();
    // println!("FEN: {b}");

    // let b: Board = "2k5/8/8/8/8/8/2Pp4/2K5 w - - 0 1".into();
    // let b: Board = "2k5/2Pp4/8/8/8/8/8/2K5 b - - 0 1".into();
    // let b: Board = "4k3/8/6p1/3P4/5N2/1N6/8/R3K3 w - - 0 1".into();
    // let moves = b.generate_moves();

    // let b: Board = "8/8/8/8/4N3/n1pB2P1/PPPPPPPP/8 w - - 0 1".into();
    // let moves = b.generate_moves_for(&[Piece::WhitePawn]);
    // let b: Board = "8/pppppppp/n1pB2P1/4N3/8/8/8/8 b - - 0 1".into();
    // let moves = b.generate_moves_for(&[Piece::BlackPawn]);

    // let b: Board = fen::TRICKY_POSITION.into();
    // let moves = b.generate_moves();

    // b.print();
    // Move::print_list(&moves);
    // print_moves_with_board(&b, &moves);
    // print_moves_statistics(&moves);

    perft(&b, 3);
}

fn perft(board: &Board, depth: usize) {
    let nodes_count = board.perft(depth);
    println!("Perft results for depth {depth}: {nodes_count} nodes.");
}

fn print_moves_with_board(board: &Board, moves: &[Move]) {
    println!();
    for mv in moves {
        println!("{mv}");
        board.print_with_move(Some(*mv));
    }
}

fn print_moves_statistics(moves: &[Move]) {
    println!(
        "Moves count {} (captures {})",
        moves.len(),
        moves.iter().filter(|m| m.is_capture()).count()
    );
}
