//! Handles communication with a UI over UCI.

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
    common::{format_moves_as_pure_string, Move, ENGINE_AUTHOR, ENGINE_NAME},
    engine::game::{Event, Game, InfoData, SearchParams},
};

// Writes the UCI output to the writer and logs it.
#[macro_export]
macro_rules! outputln {
    ($writer:expr, $($arg:tt)*) => {
        let msg = format!($($arg)*);
        info!("> {}", msg);
        // If we fail to write, we can just panic, as we don't have anything better to do anyway.
        let _ = writeln!($writer, "{}", msg).unwrap();
    };
}

// GUI to Engine
#[derive(Debug)]
enum UciCommand {
    Uci,
    Debug(bool),
    IsReady,
    SetOption(String, Option<String>),
    Register,
    UciNewGame,
    Position(Option<String>, Vec<String>),
    Go(Vec<GoCommand>),
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
    Info(Vec<InfoData>),
    Option,
    DisplayBoard(String), // Non-standard (response to d)
}

#[derive(Debug)]
enum GoCommand {
    SearchMoves(Vec<Move>),
    Ponder,
    WTime(u32),
    BTime(u32),
    WInc(u32),
    BInc(u32),
    MovesToGo(u32),
    Depth(usize),
    Nodes(u32),
    Mate(u32),
    MoveTime(u32),
    Infinite, // search until the stop command.
}

// Set up the various threads that run the engine.
pub fn run<R, W>(game: &mut Game, reader: Arc<Mutex<R>>, writer: Arc<Mutex<W>>)
where
    R: BufRead + Send + 'static,
    W: Write + Send + 'static,
{
    let (cmd_sender, cmd_receiver): (Sender<UciCommand>, Receiver<UciCommand>) = mpsc::channel();
    let (evt_sender, evt_receiver): (Sender<UciEvent>, Receiver<UciEvent>) = mpsc::channel();
    let (game_event_sender, game_event_receiver): (Sender<Event>, Receiver<Event>) =
        mpsc::channel();

    spawn_ui_input_handler(reader, cmd_sender);
    spawn_ui_event_handler(writer, evt_receiver);
    spawn_game_event_handler(game_event_receiver, evt_sender.clone());
    spawn_game_commands_handler(game, cmd_receiver, evt_sender, game_event_sender);
}

// Spawn a thread to handle UI input.
fn spawn_ui_input_handler<R>(reader: Arc<Mutex<R>>, cmd_sender: Sender<UciCommand>)
where
    R: BufRead + Send + 'static,
{
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
                        assert_eq!(tokens.pop_front().unwrap(), "name");
                        let name = tokens.pop_front().unwrap().to_string();
                        let value = if let Some(v) = tokens.pop_front() {
                            assert_eq!(v, "value");
                            Some(tokens.pop_front().unwrap().to_string())
                        } else {
                            None
                        };
                        cmd_sender.send(UciCommand::SetOption(name, value)).unwrap();
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
                        let mut go_cmds = Vec::new();
                        while let Some(p) = tokens.pop_front() {
                            match p {
                                "infinite" => go_cmds.push(GoCommand::Infinite),
                                "depth" => {
                                    let d = tokens.pop_front().unwrap().parse().unwrap();
                                    go_cmds.push(GoCommand::Depth(d));
                                }
                                _ => {}
                            }
                        }
                        cmd_sender.send(UciCommand::Go(go_cmds)).unwrap();
                    }
                    "stop" => cmd_sender.send(UciCommand::Stop).unwrap(),
                    "quit" | "q" => cmd_sender.send(UciCommand::Quit).unwrap(), // Only "quit" is standard.
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
            while let Ok(cmd) = evt_receiver.recv() {
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
                    UciEvent::Info(infos) => {
                        // Sorting the keys for readability.
                        outputln!(
                            &mut writer,
                            "info {}",
                            infos
                                .iter()
                                .sorted_unstable_by_key(|i| info_data_sort_order(i))
                                .join(" ")
                        );
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
fn spawn_game_event_handler(game_event_receiver: Receiver<Event>, evt_sender: Sender<UciEvent>) {
    std::thread::spawn(move || {
        loop {
            // Receive messages from the Game thread (info messages, bestmove)
            while let Ok(evt) = game_event_receiver.recv() {
                // Convert to UCI event.
                let uci_event = match evt {
                    Event::BestMove(mv, ponder) => UciEvent::BestMove(mv, ponder),
                    Event::Info(info) => UciEvent::Info(info),
                };
                // Send to UciCommand handler.
                evt_sender.send(uci_event).unwrap();
            }
        }
    });
}

// Handle game commands (not in a thread).
#[allow(clippy::needless_pass_by_value)]
fn spawn_game_commands_handler(
    game: &mut Game,
    cmd_receiver: Receiver<UciCommand>,
    evt_sender: Sender<UciEvent>,
    game_event_sender: Sender<Event>,
) {
    loop {
        // Receive messages from the Game thread (info messages, bestmove)
        while let Ok(cmd) = cmd_receiver.recv() {
            match cmd {
                // UI to Engine: Standard commands
                UciCommand::Uci => handle_uci_cmd(&evt_sender),
                UciCommand::Debug(val) => handle_debug_cmd(game, val),
                UciCommand::IsReady => handle_isready_cmd(&evt_sender),
                UciCommand::SetOption(name, value) => handle_setoptions_cmd(&name, &value),
                UciCommand::UciNewGame => handle_ucinewgame_cmd(game),
                UciCommand::Position(position, moves) => {
                    handle_position_cmd(game, position, &moves);
                }
                UciCommand::Go(go_cmds) => handle_go_cmd(game, &go_cmds, &game_event_sender),
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

fn handle_setoptions_cmd(name: &str, value: &Option<String>) {
    info!("Setting option {name} to {:?}", value);
}

fn handle_ucinewgame_cmd(game: &mut Game) {
    // Not mandatory to be sent by UIs, but most should support it.
    game.new_game();
}

fn handle_position_cmd(game: &mut Game, position: Option<String>, moves: &[String]) {
    if let Some(fen) = position {
        game.set_to_fen(&fen);
    } else {
        game.set_to_startpos();
    }

    if !moves.is_empty() {
        game.apply_moves(moves);
    }
}

fn handle_go_cmd(game: &mut Game, go_cmds: &[GoCommand], game_event_sender: &Sender<Event>) {
    let mut sp = SearchParams::default();
    for c in go_cmds {
        match c {
            GoCommand::Infinite => sp.depth = None,
            GoCommand::Depth(d) => sp.depth = Some(*d),
            GoCommand::SearchMoves(_) => todo!(),
            GoCommand::Ponder => todo!(),
            GoCommand::WTime(_) => todo!(),
            GoCommand::BTime(_) => todo!(),
            GoCommand::WInc(_) => todo!(),
            GoCommand::BInc(_) => todo!(),
            GoCommand::MovesToGo(_) => todo!(),
            GoCommand::Nodes(_) => todo!(),
            GoCommand::Mate(_) => todo!(),
            GoCommand::MoveTime(_) => todo!(),
        }
    }
    game.start_search(sp, game_event_sender);
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

impl Display for InfoData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InfoData::Depth(x) => write!(f, "depth {x}"),
            InfoData::Score(x) => write!(f, "score cp {x}"),
            InfoData::ScoreMate(y) => write!(f, "score mate {y}"),
            InfoData::Nodes(x) => write!(f, "nodes {x}"),
            InfoData::Pv(moves) => write!(f, "pv {}", format_moves_as_pure_string(moves)),
            InfoData::String(s) => write!(f, "string {s}"),
        }
    }
}

fn info_data_sort_order(info: &InfoData) -> u8 {
    match info {
        InfoData::Score(_) => 1,
        InfoData::ScoreMate(_) => 2,
        InfoData::Depth(_) => 3,
        InfoData::Nodes(_) => 4,
        InfoData::Pv(_) => 5,
        InfoData::String(_) => 6,
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::{board::Board, uci};

    use super::*;

    #[test]
    fn test_position_startpos() {
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
    fn test_position_fen() {
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
    fn test_position_moves() {
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
            Board::from_fen("rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq e6 0 2")
        );
    }
}
