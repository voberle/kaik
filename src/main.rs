#![allow(dead_code)]

use std::env;

use board::Board;
use common::Square;
use moves::Move;

mod bitboard;
mod board;
mod common;
mod fen;
mod moves;

#[allow(unused_variables, unused_imports, unused_mut)]
fn main() {
    use common::Piece::*;
    use common::Square::*;

    let mut b = Board::initial_board();
    // let mut b: Board = "".into();

    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        let depth: usize = args[1].parse().expect("Invalid depth argument");
        let moves = args.get(2).map_or("", |v| v.as_str());

        apply_moves(&mut b, moves);
        divide(&b, depth);

        return;
    }

    println!("  Kaik Chess Engine");
    println!("         by Vincent");
    println!();
    b.print();
    println!();

    let moves = b.generate_moves();
    print_moves_with_board(&b, &moves);
    print_moves_statistics(&moves);
    // Move::print_list(&moves);
    for mv in &moves {
        println!("{}", mv.pure());
    }
}

fn perft(board: &Board, depth: usize) {
    let nodes_count = board.perft(depth);
    println!("Perft results for depth {depth}: {nodes_count} nodes.");
}

fn divide(board: &Board, depth: usize) {
    // Output format is the same as Stockfish "go perft <depth>" command.
    let nodes = board.divide(depth);
    for (mv, count) in &nodes {
        println!("{}: {count}", mv.pure());
    }
    println!();
    println!(
        "Nodes searched: {}",
        nodes.iter().map(|(_, count)| *count).sum::<usize>()
    );
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

fn apply_moves(board: &mut Board, moves: &str) {
    for mv in moves.split_ascii_whitespace() {
        assert_eq!(mv.len(), 4);
        let from: Square = mv[0..2].try_into().unwrap();
        let to: Square = mv[2..4].try_into().unwrap();
        board.update_by_move(board.new_move(from, to));
    }
}
