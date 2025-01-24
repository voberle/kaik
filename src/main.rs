#![allow(dead_code)]

use std::{env, io, time::Instant};

use board::Board;
use common::Square;
use moves::Move;
use uci::Uci;

mod bitboard;
mod board;
mod common;
mod fen;
mod game;
mod moves;
mod uci;

fn main() {
    // Usage: <perft|divide> <depth> <startpos|fen> <moves>
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        let cmd = args[1].to_string();
        let depth: usize = args[2].parse().expect("Invalid depth argument");
        let pos = args.get(3).map_or("startpos", |v| v.as_str());
        let moves = args.get(4).map_or("", |v| v.as_str());

        let mut b: Board = if pos == "startpos" {
            Board::initial_board()
        } else {
            pos.into()
        };

        apply_moves(&mut b, moves);

        // For performance measurement use perft. For debugging, use divide.
        match cmd.as_str() {
            "perft" => println!("{}", b.perft(depth)),
            "perft_time" => perft(&b, depth),
            "divide" => divide(&b, depth),
            _ => panic!("Unsupported command"),
        }

        return;
    }

    start_uci_loop();

    // hacks();
}

#[allow(unused_variables, unused_imports, unused_mut)]
fn hacks() {
    let mut b = Board::initial_board();
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

fn start_uci_loop() {
    let stdio = io::stdin();
    let input = stdio.lock();

    let output = io::stdout();

    let mut uci = Uci::new(input, output, true);

    uci.uci_loop();
}

fn perft(board: &Board, depth: usize) {
    let now = Instant::now();
    let nodes_count = board.perft(depth);
    let elapsed = now.elapsed();

    println!("Perft results for depth {depth}: {nodes_count} nodes.");

    let nodes_secs = nodes_count as u128 / elapsed.as_micros();
    println!("Time: {elapsed:.2?} secs. \t{nodes_secs} millions nodes / secs.");
}

fn divide(board: &Board, depth: usize) {
    // Output format is the same as Stockfish "go perft <depth>" command.
    let nodes = board.divide(depth);

    let total_nodes: usize = nodes.iter().map(|(_, count)| *count).sum();

    for (mv, count) in &nodes {
        println!("{}: {count}", mv.pure());
    }
    println!();
    println!("Nodes searched: {total_nodes}",);
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
