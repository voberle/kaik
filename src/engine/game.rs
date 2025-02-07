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
    common::Move,
    common::Score,
    search::{self, Result},
};

// Parameters passed to the search.
#[derive(Debug, Clone, Copy)]
pub struct SearchParams {
    pub depth: Option<usize>,
}

// Events the game can send back to the user / UI.
#[derive(Debug)]
pub enum Event {
    BestMove(Option<Move>, Option<Move>),
    Info(InfoData),
}

// Whatever the engine wants to send to the UI.
#[derive(Debug)]
pub enum InfoData {
    Score(Score),
    ScoreMate(i32), // mate in y moves. If the engine is getting mated use negative values.
    String(String),
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
    pub fn start_search(&mut self, search_params: SearchParams, event_sender: &Sender<Event>) {
        // The spec is not explicit about what to do if we receive a start search
        // when a search is already running.
        // Probably we should stop the current search and start a new one.
        // For now, we ignore the command.
        if self.stop_flag.load(Ordering::Relaxed) {
            warn!("A search is already running, stop it first");
            return;
        }

        let board_clone = self.board;
        let search_params_clone = search_params;
        let event_sender_clone = event_sender.clone();
        let search_thread_stop_flag = self.stop_flag.clone();

        std::thread::spawn(move || {
            run_search(
                board_clone,
                search_params_clone,
                event_sender_clone,
                search_thread_stop_flag,
            );
        });
    }

    pub fn stop_search(&mut self) {
        self.stop_flag.store(true, Ordering::Relaxed);
    }

    pub fn set_debug(&mut self, val: bool) {
        self.debug = val;
    }
}

#[allow(clippy::needless_pass_by_value)]
fn run_search(
    board: Board,
    search_params: SearchParams,
    event_sender: Sender<Event>,
    stop_flag: Arc<AtomicBool>,
) {
    if stop_flag.load(Ordering::Relaxed) {
        return; // Stop immediately
    }

    // self.random_move(board)
    let r = negamax(board, &search_params, &stop_flag);
    if let Some((mv, score)) = r {
        info!("Move {}", mv);
        event_sender
            .send(Event::Info(InfoData::Score(score)))
            .unwrap();

        event_sender.send(Event::BestMove(Some(mv), None)).unwrap();
    } else {
        // event_sender
        //     .send(Event::Info(InfoData::ScoreMate(-1)))
        //     .unwrap();

        event_sender.send(Event::BestMove(None, None)).unwrap();
    }

    // Search is over, clearing the stop flag.
    stop_flag.store(false, Ordering::Relaxed);
}

fn negamax(
    board: Board,
    search_params: &SearchParams,
    stop_flag: &Arc<AtomicBool>,
) -> Option<(Move, Score)> {
    // With the recursive implementation of Negamax, real infinite search isn't an option.
    const MAX_NEGAMAX_DEPTH: usize = 4;
    let depth = match search_params.depth {
        Some(d) => MAX_NEGAMAX_DEPTH.min(d),
        None => MAX_NEGAMAX_DEPTH,
    };

    let result = search::negamax(&board, depth, stop_flag);
    match result {
        Result::BestMove(mv, score) => Some((mv, score)),
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
