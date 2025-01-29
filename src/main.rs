#![allow(dead_code)]

#[macro_use]
extern crate log;
extern crate simplelog;

use clap::{Parser, Subcommand};
use simplelog::{
    ColorChoice, CombinedLogger, Config, LevelFilter, SharedLogger, TermLogger, TerminalMode,
    WriteLogger,
};
use std::fs::File;
use std::path::PathBuf;
use std::{io, time::Instant};

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
mod perft;
mod search;
mod uci;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Arguments {
    /// Sets a log file
    #[arg(short, long, value_name = "FILE")]
    log: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Runs divide command.
    Divide {
        depth: usize,
        position: String,
        moves: Option<String>,
    },
    /// Runs Perft command with result only.
    Perft {
        depth: usize,
        position: String,
        moves: Option<String>,
    },
    /// Runs Perft command with timing information.
    PerftTime {
        depth: usize,
        position: String,
        moves: Option<String>,
    },
    /// Runs a search.
    Search {
        depth: usize,
        position: String,
        moves: Option<String>,
    },
}

fn create_board(position: &String, moves: &Option<String>) -> Board {
    let mut b: Board = if position == "startpos" {
        Board::initial_board()
    } else {
        position.as_str().into()
    };
    if let Some(m) = moves {
        apply_moves(&mut b, m);
    }
    b
}

fn main() {
    let args = Arguments::parse();

    let mut logger: Vec<Box<dyn SharedLogger>> = vec![TermLogger::new(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )];
    if let Some(log_file) = args.log.as_deref() {
        logger.push(WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create(log_file).unwrap(),
        ));
    }
    CombinedLogger::init(logger).unwrap();

    match &args.command {
        Some(Commands::Divide {
            depth,
            position,
            moves,
        }) => {
            divide(&create_board(position, moves), *depth);
            return;
        }
        Some(Commands::Perft {
            depth,
            position,
            moves,
        }) => {
            let nodes_cnt = perft::perft(&create_board(position, moves), *depth);
            println!("{nodes_cnt}");
            return;
        }
        Some(Commands::PerftTime {
            depth,
            position,
            moves,
        }) => {
            perft(&create_board(position, moves), *depth);
            return;
        }
        Some(Commands::Search {
            depth,
            position,
            moves,
        }) => {
            search(&create_board(position, moves), *depth);
            return;
        }
        _ => {}
    }

    info!("Kaik Chess Engine");

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

    let mut uci = Uci::new(input, output);

    uci.uci_loop();
}

fn perft(board: &Board, depth: usize) {
    let now = Instant::now();
    let nodes_count = perft::perft(board, depth);
    let elapsed = now.elapsed();

    println!("Perft results for depth {depth}: {nodes_count} nodes.");

    let nodes_secs = nodes_count as u128 / elapsed.as_micros();
    println!("Time: {elapsed:.2?} secs. \t{nodes_secs} millions nodes / secs.");
}

fn divide(board: &Board, depth: usize) {
    // Output format is the same as Stockfish "go perft <depth>" command.
    let nodes = perft::divide(board, depth);

    let total_nodes: usize = nodes.iter().map(|(_, count)| *count).sum();

    for (mv, count) in &nodes {
        println!("{}: {count}", mv.pure());
    }
    println!();
    println!("Nodes searched: {total_nodes}",);
}

fn search(board: &Board, depth: usize) {
    let now = Instant::now();
    let result = search::negamax(board, depth);
    let elapsed = now.elapsed();

    println!("Search({depth}) {elapsed:.2?} secs: {result}");
    if let search::Result::BestMove(mv) = result {
        board.print_with_move(Some(mv));
    }
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
