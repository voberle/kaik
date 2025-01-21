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

#[allow(unused_variables, unused_imports)]
fn main() {
    use common::Piece::*;
    use common::Square::*;

    let args: Vec<String> = env::args().collect();
    if args.len() == 3 {
        let depth: usize = args[1].parse().expect("Invalid depth argument");
        let moves = &args[2];

        let mut b = Board::initial_board();
        apply_moves(&mut b, moves);
        // b.print();
        divide(&b, depth);

        return;
    }

    println!("  Kaik Chess Engine");
    println!("         by Vincent");
    println!();

    let mut b = Board::initial_board();
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

    // let b: Board = "2r3k1/1q1nbppp/r3p3/3pP3/pPpP4/P1Q2N2/2RN1PPP/2R4K b - b3 0 23".into();
    // let moves = b.generate_moves_for(&[Piece::BlackPawn]);

    // let b: Board = "8/8/8/8/8/8/8/R3K2R w KQkq - 0 1".into(); // Castling
    // let moves = b.generate_moves_for(&[]);

    // let b: Board = "4k3/1P6/8/8/8/8/8/4K3 w - - 0 1".into(); // Promotion
    // let moves = b.generate_moves_for(&[Piece::WhitePawn]);

    apply_moves(&mut b, "a2a4 a7a6 a4a5 b7b5 a5b6");
    b.print();

    let moves = b.generate_moves();
    print_moves_with_board(&b, &moves);
    print_moves_statistics(&moves);
    // Move::print_list(&moves);
    for mv in &moves {
        println!("{}", mv.pure());
    }

    // King attacks
    // let b: Board = "rnbqk2r/pppp1ppp/8/8/1b2PP1P/1PP5/P2p1P2/RNBQKBNR w KQkq - 0 10".into();
    // let b: Board = "rnbqk2r/pppp1ppp/8/3P4/1b2PP1P/1P6/P4P2/RNBQKBNR w KQkq - 0 10".into();
    // b.print();
    // let bb = b.attacks_king(common::Color::White);
    // bb.print();

    // let mut b = Board::initial_board();
    // b.update_by_move(b.new_move(B1, C3));
    // b.update_by_move(b.new_move(D7, D5));
    // b.update_by_move(b.new_move(C3, D5));

    // b.print();
    // divide(&b, 1);

    // b.print_bitboards();

    // let moves = b.generate_moves();
    // print_moves_with_board(&b, &moves);
    // print_moves_statistics(&moves);
}

fn perft(board: &Board, depth: usize) {
    let nodes_count = board.perft(depth);
    println!("Perft results for depth {depth}: {nodes_count} nodes.");
}

fn divide(board: &Board, depth: usize) {
    // Output format is the same as Stockfish "go perft <depth>" command.
    // Save both outputs to p_stockfish and p_kaik, and compare with:
    //   diff <(sort p_stockfish) <(sort p_kaik)
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
