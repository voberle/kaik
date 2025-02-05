//! Game struct represents the state of a chess game and provides methods
//! for manipulating the game state. It holds the board and other
//! game-related information that is not part of the board itself, like
//! the move history.
//! It's API is non-blocking. Operations that can take a long time such as search
//! are executed in a separate thread.

use std::{
    io::Write,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::Sender,
        Arc,
    },
};

use crate::{
    board::Board,
    moves::Move,
    search::{self, Result},
};

// Events the game can send back to the user / UI.
#[derive(Debug)]
pub enum GameEvent {
    BestMove(Option<Move>),
    Info(String), // TODO Replace with a struct.
}

pub struct Game {
    board: Board,
    debug: bool,
    stop_flag: Arc<AtomicBool>,
    // Should we store the state of the game? Running/Over? Checkmate/Stalemate/etc?
}

impl Game {
    // A game is always initialized to a position, either the starting one or from a FEN string.
    pub fn new() -> Self {
        Self {
            board: Board::initial_board(),
            debug: false,
            stop_flag: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn new_game(&mut self) {
        self.board = Board::initial_board();
    }

    pub fn set_to_startpos(&mut self) {
        self.board = Board::initial_board();
    }

    pub fn set_to_fen(&mut self, fen: &str) {
        self.board = Board::from_fen(fen);
    }

    pub fn get_board(&self) -> Board {
        self.board
    }

    pub fn display_board<W: Write>(&self, writer: &mut W) {
        let _ = self.board.write(writer);
    }

    pub fn apply_moves(&mut self, moves: &[String]) {
        for mv in moves {
            self.board.update_by_move(self.board.new_move_from_pure(mv));
        }
    }

    // Starts a search and returns the best move found.
    // The search is executed in a separate thread started by this function.
    pub fn start_search(&mut self, event_sender: &Sender<GameEvent>) {
        // The spec is not explicit about what to do if we receive a start search
        // when a search is already running.
        // Probably we should stop the current search and start a new one.
        // For now, we ignore the command.
        if self.stop_flag.load(Ordering::Relaxed) {
            warn!("A search is already running, stop it first");
            return;
        }

        let search_thread_stop_flag = self.stop_flag.clone();
        let event_sender_clone = event_sender.clone();
        let board_clone = self.board.clone();

        std::thread::spawn(move || {
            run_search(board_clone, event_sender_clone, search_thread_stop_flag)
        });
    }

    pub fn stop_search(&mut self) {
        self.stop_flag.store(true, Ordering::Relaxed);
    }

    pub fn set_debug(&mut self, val: bool) {
        self.debug = val;
    }
}

fn run_search(board: Board, event_sender: Sender<GameEvent>, stop_flag: Arc<AtomicBool>) {
    if stop_flag.load(Ordering::Relaxed) {
        return; // Stop immediately
    }

    // self.random_move(board)
    let mv = negamax(board, &stop_flag);
    if let Some(m) = mv {
        info!("Move {}", m);
    }

    event_sender.send(GameEvent::BestMove(mv)).unwrap();

    // Search is over, clearing the stop flag.
    stop_flag.store(false, Ordering::Relaxed);
}

fn negamax(board: Board, stop_flag: &Arc<AtomicBool>) -> Option<Move> {
    let result = search::negamax(&board, 5, stop_flag);
    match result {
        Result::BestMove(mv) => Some(mv),
        Result::CheckMate => {
            info!("Checkmate");
            None
        }
        Result::StaleMate => {
            info!("Stalemate");
            None
        }
    }
}

// Looks at all legal moves in depth 1 and returns a random one.
// As it's depth one only, it's very fast and we don't bother with a thread.
fn random_move(board: Board) -> Option<Move> {
    use rand::seq::IteratorRandom;

    // Get pseudo-legal moves
    board
        .generate_moves()
        .iter()
        .filter(|&&mv| {
            // Filter out moves that leave the king in check.
            board.copy_with_move(mv).is_some()
        })
        // Pick a random one
        .choose(&mut rand::thread_rng())
        .copied()
}
