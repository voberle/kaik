//! Handles communication with a UI over UCI.
#![allow(clippy::unused_self)]

use std::{
    collections::VecDeque,
    fmt::Display,
    io::{BufRead, Write},
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
};

use itertools::Itertools;

use crate::{
    common::{ENGINE_AUTHOR, ENGINE_NAME},
    game::{Game, GameEvent},
    moves::Move,
};

// We use a writer for the UCI output instead of just println!, as this
// allows among other thing to test it.
// But if we fail to write, we can just panic, as we don't have anything better to do anyway.
// This macro does that.
#[macro_export]
macro_rules! outputln {
    ($writer:expr, $($arg:tt)*) => {
        let msg = format!($($arg)*);
        info!("> {}", msg);
        let _ = writeln!($writer, "{}", msg).unwrap();
        // writeln!($writer, $($arg)*).unwrap();
    };
}

// GUI to Engine
#[derive(Debug)]
enum UciCommand {
    Uci,
    Debug(bool),
    IsReady,
    SetOption(String),
    Register,
    UciNewGame,
    Position(Option<String>, Vec<String>),
    Go,
    Stop,
    PonderHit,
    Quit,
    Print, // Non-standard: "d"
}

// Engine to GUI
#[derive(Debug)]
enum UciEvent {
    Id(String, String),
    UciOk,
    ReadyOk,
    BestMove(Option<Move>, Option<Move>), // move, ponder
    CopyProtection,
    Registration,
    Info(InfoData),
    Option,
    DisplayBoard(String), // Non-standard (response to d)
}

pub fn run<R, W>(game: &mut Game, reader: Arc<Mutex<R>>, writer: Arc<Mutex<W>>)
where
    R: BufRead + Send + 'static,
    W: Write + Send + 'static,
{
    let (cmd_sender, cmd_receiver): (Sender<UciCommand>, Receiver<UciCommand>) = mpsc::channel();
    let (evt_sender, evt_receiver): (Sender<UciEvent>, Receiver<UciEvent>) = mpsc::channel();
    let (game_event_sender, game_event_receiver): (Sender<GameEvent>, Receiver<GameEvent>) =
        mpsc::channel();

    spawn_ui_input_handler(reader, cmd_sender);
    spawn_ui_event_handler(writer, evt_receiver);
    spawn_game_event_handler(game_event_receiver, evt_sender.clone());
    spawn_game_commands_handler(game, cmd_receiver, evt_sender, game_event_sender);
}

fn spawn_ui_input_handler<R>(reader: Arc<Mutex<R>>, cmd_sender: Sender<UciCommand>)
where
    R: BufRead + Send + 'static,
{
    // Spawn a thread to handle UI input.
    std::thread::spawn(move || {
        loop {
            let mut line = String::new();
            reader
                .lock()
                .unwrap()
                .read_line(&mut line)
                .expect("Could not read line");
            if line.is_empty() {
                continue;
            }

            info!("< {}", line.trim());

            // Split the input into tokens
            let mut tokens: VecDeque<_> = line.split_ascii_whitespace().collect();
            if tokens.is_empty() {
                continue;
            }

            while let Some(cmd) = tokens.pop_front() {
                match cmd.to_lowercase().as_str() {
                    // Standard commands
                    "uci" => cmd_sender.send(UciCommand::Uci).unwrap(),
                    "debug" => {
                        let val = *tokens.front().expect("No debug value provided");
                        let debug = match val {
                            "on" => true,
                            "off" => false,
                            _ => panic!("Invalid debug value"),
                        };
                        cmd_sender.send(UciCommand::Debug(debug)).unwrap();
                    }
                    "isready" => cmd_sender.send(UciCommand::IsReady).unwrap(),
                    "setoptions" => {
                        // TODO
                    }
                    "ucinewgame" => cmd_sender.send(UciCommand::UciNewGame).unwrap(),
                    "position" => {
                        let pos = *tokens.front().expect("No position provided");

                        let position = if pos == "startpos" {
                            tokens.pop_front().unwrap();
                            None // means start pos
                        } else if pos == "fen" {
                            tokens.pop_front().unwrap();
                            // FEN string is always 6 tokens.
                            // Not great to split the string to join it again..
                            let fen = tokens.drain(0..6).join(" ");
                            Some(fen)
                        } else {
                            panic!("Missing position")
                        };

                        let moves: Vec<String> = if matches!(tokens.pop_front(), Some("moves")) {
                            tokens.into_iter().map(String::from).collect()
                        } else {
                            vec![]
                        };

                        cmd_sender
                            .send(UciCommand::Position(position, moves))
                            .unwrap();
                    }
                    "go" => {
                        // TODO handle go parpas
                        cmd_sender.send(UciCommand::Go).unwrap();
                    }
                    "stop" => cmd_sender.send(UciCommand::Stop).unwrap(),
                    "quit" => return,
                    "register" | "ponderhit" => {} // Command not implemented
                    // Non-standard commands
                    "d" => cmd_sender.send(UciCommand::Print).unwrap(),
                    _ => continue, // Command was unknown, try next token.
                }
                break; // Command was handled.
            }
        }
    });
}

// Handle UCI commands..
fn spawn_ui_event_handler<W>(writer: Arc<Mutex<W>>, evt_receiver: Receiver<UciEvent>)
where
    W: Write + Send + 'static,
{
    std::thread::spawn(move || {
        let mut writer = writer.lock().unwrap();
        loop {
            while let Ok(cmd) = evt_receiver.try_recv() {
                match cmd {
                    UciEvent::Id(param, value) => {
                        outputln!(&mut writer, "id {param} {value}");
                    }
                    UciEvent::UciOk => {
                        outputln!(&mut writer, "uciok");
                    }
                    UciEvent::ReadyOk => {
                        outputln!(&mut writer, "readyok");
                    }
                    UciEvent::BestMove(mv, ponder) => {
                        // If best_move is None, it means we are in stale mate.
                        if let Some(best_move) = mv {
                            if let Some(ponder_move) = ponder {
                                outputln!(
                                    &mut writer,
                                    "bestmove {} ponder {}",
                                    best_move.pure(),
                                    ponder_move.pure()
                                );
                            } else {
                                outputln!(&mut writer, "bestmove {}", best_move.pure());
                            }
                        } else {
                            // The protocol doesn't specify what do on stalemates.
                            // This is what Stockfish seems to do.
                            // <https://github.com/official-stockfish/Stockfish/discussions/5075>
                            outputln!(&mut writer, "bestmove (none)");
                        }
                    }
                    UciEvent::Info(info) => {
                        outputln!(&mut writer, "info {info}");
                    }
                    UciEvent::Option => {
                        // TODO
                    }
                    UciEvent::DisplayBoard(b) => {
                        outputln!(&mut writer, "{b}");
                    }
                    UciEvent::CopyProtection | UciEvent::Registration => {
                        unimplemented!();
                    }
                }
            }
        }
    });
}

// Spawn a thread to handle game events.
fn spawn_game_event_handler(
    game_event_receiver: Receiver<GameEvent>,
    evt_sender: Sender<UciEvent>,
) {
    std::thread::spawn(move || {
        loop {
            // Receive messages from the Game thread (info messages, bestmove)
            while let Ok(evt) = game_event_receiver.try_recv() {
                // Convert to UCI event.
                let uci_event = match evt {
                    GameEvent::BestMove(mv) => UciEvent::BestMove(mv, None),
                    GameEvent::Info(info) => UciEvent::Info(InfoData { string: info }),
                };
                // Send to UciCommand handler.
                evt_sender.send(uci_event).unwrap();
            }
        }
    });
}

// Handle game commands (not in a thread).
fn spawn_game_commands_handler(
    game: &mut Game,
    cmd_receiver: Receiver<UciCommand>,
    evt_sender: Sender<UciEvent>,
    game_event_sender: Sender<GameEvent>,
) {
    loop {
        // Receive messages from the Game thread (info messages, bestmove)
        while let Ok(cmd) = cmd_receiver.try_recv() {
            match cmd {
                // UI to Engine: Standard commands
                UciCommand::Uci => handle_uci_cmd(&evt_sender),
                UciCommand::Debug(val) => handle_debug_cmd(game, val),
                UciCommand::IsReady => handle_isready_cmd(&evt_sender),
                UciCommand::SetOption(options) => handle_setoptions_cmd(),
                UciCommand::UciNewGame => handle_ucinewgame_cmd(game),
                UciCommand::Position(position, moves) => handle_position_cmd(game, position, moves),
                UciCommand::Go => handle_go_cmd(game, &game_event_sender),
                UciCommand::Stop => handle_stop_cmd(game),
                UciCommand::Quit => return,
                UciCommand::Register | UciCommand::PonderHit => {} // Command not implemented
                // UI to Engine: Non-standard commands
                UciCommand::Print => handle_d_cmd(game, &evt_sender),
            }
        }
    }
}

fn handle_uci_cmd(evt_sender: &Sender<UciEvent>) {
    // Identify.
    evt_sender
        .send(UciEvent::Id("name".to_string(), ENGINE_NAME.to_string()))
        .unwrap();
    evt_sender
        .send(UciEvent::Id(
            "author".to_string(),
            ENGINE_AUTHOR.to_string(),
        ))
        .unwrap();

    // Send the options that can be changed.

    // Ready
    evt_sender.send(UciEvent::UciOk).unwrap();
}

fn handle_debug_cmd(game: &mut Game, debug: bool) {
    game.set_debug(debug);
}

fn handle_isready_cmd(evt_sender: &Sender<UciEvent>) {
    // Ready to start
    // Here we should check that the game is not over.
    // TODO
    evt_sender.send(UciEvent::ReadyOk).unwrap();
}

fn handle_setoptions_cmd() {}

fn handle_ucinewgame_cmd(game: &mut Game) {
    // Not mandatory to be sent by UIs, but most should support it.
    game.new_game();
}

fn handle_position_cmd(game: &mut Game, position: Option<String>, moves: Vec<String>) {
    if let Some(fen) = position {
        game.set_to_fen(&fen);
    } else {
        game.set_to_startpos();
    }

    if !moves.is_empty() {
        game.apply_moves(&moves);
    }
}

fn handle_go_cmd(game: &mut Game, game_event_sender: &Sender<GameEvent>) {
    game.start_search(game_event_sender);
}

fn handle_stop_cmd(game: &mut Game) {
    game.stop_search();
}

fn handle_d_cmd(game: &mut Game, evt_sender: &Sender<UciEvent>) {
    let mut out = Vec::new();
    game.display_board(&mut out);
    let output = String::from_utf8(out).expect("Invalid UTF-8 sequence");
    evt_sender.send(UciEvent::DisplayBoard(output)).unwrap();
}

// Whatever the engine wants to send in the UCI info command.
#[derive(Debug)]
pub struct InfoData {
    // For now, only string is supported.
    string: String,
}

impl Display for InfoData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "string {}", self.string)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::{board::Board, uci};

    use super::*;

    #[test]
    fn test_uci_loop_position_startpos() {
        let input = "position startpos\nquit\n";
        let mut game = Game::new();
        let input = Cursor::new(input);
        let output = Vec::new();
        uci::run(
            &mut game,
            Arc::new(Mutex::new(input)),
            Arc::new(Mutex::new(output)),
        );

        assert_eq!(game.get_board(), Board::initial_board());
    }

    #[test]
    fn test_uci_loop_position_fen() {
        let input = "position fen r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1\nquit\n";
        let mut game = Game::new();
        let input = Cursor::new(input);
        let output = Vec::new();
        uci::run(
            &mut game,
            Arc::new(Mutex::new(input)),
            Arc::new(Mutex::new(output)),
        );

        assert_eq!(
            game.get_board(),
            Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1")
        );
    }

    #[test]
    fn test_uci_loop_position_moves() {
        let input = "position startpos moves e2e4 e7e5\nquit\n";
        let mut game = Game::new();
        let input = Cursor::new(input);
        let output = Vec::new();
        uci::run(
            &mut game,
            Arc::new(Mutex::new(input)),
            Arc::new(Mutex::new(output)),
        );

        assert_eq!(
            game.get_board(),
            Board::from_fen("rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq e6 0 1")
        );
    }
}
