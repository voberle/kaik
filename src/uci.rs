//! Handles communication with a UI over UCI.
#![allow(clippy::unused_self)]

use std::{
    collections::VecDeque,
    fmt::Display,
    io::{BufRead, Write},
};

use itertools::Itertools;

use crate::{
    common::{ENGINE_AUTHOR, ENGINE_NAME},
    game::Game,
    moves::Move,
};

pub struct Uci<R, W>
where
    W: std::io::Write,
{
    reader: R,
    writer: W,
    game: Game,
    debug: bool,
}

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

impl<R, W> Uci<R, W>
where
    R: BufRead,
    W: Write,
{
    pub fn new(reader: R, writer: W) -> Self {
        Uci {
            reader,
            writer,
            game: Game::new(),
            debug: false,
        }
    }

    pub fn uci_loop(&mut self) {
        loop {
            let mut line = String::new();
            self.reader
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
                    "uci" => self.handle_uci_cmd(),
                    "debug" => self.handle_debug_cmd(&mut tokens),
                    "isready" => self.handle_isready_cmd(),
                    "setoptions" => self.handle_setoptions_cmd(&mut tokens),
                    "ucinewgame" => self.handle_ucinewgame_cmd(),
                    "position" => self.handle_position_cmd(&mut tokens),
                    "go" => self.handle_go_cmd(&mut tokens),
                    "stop" => self.handle_stop_cmd(),
                    "quit" => return,
                    "register" | "ponderhit" => {} // Command not implemented
                    // Non-standard commands
                    "d" => self.handle_d_cmd(),
                    _ => continue, // Command was unknown, try next token.
                }
                break; // Command was handled.
            }
        }
    }

    fn handle_uci_cmd(&mut self) {
        // Identify.
        outputln!(&mut self.writer, "id name {ENGINE_NAME}");
        outputln!(&mut self.writer, "id author {ENGINE_AUTHOR}");
        // Send the options that can be changed.

        // Ready
        outputln!(&mut self.writer, "uciok");
        // Would a call to flush be needed? self.writer.flush();
    }

    fn handle_debug_cmd(&mut self, tokens: &mut VecDeque<&str>) {
        let val = *tokens.front().expect("No debug value provided");
        let debug = match val {
            "on" => true,
            "off" => false,
            _ => panic!("Invalid debug value"),
        };
        self.debug = debug;
    }

    fn handle_isready_cmd(&mut self) {
        // Ready to start
        // Here we should check that the game is not over.
        // TODO
        outputln!(&mut self.writer, "readyok");
    }

    fn handle_setoptions_cmd(&mut self, _tokens: &mut VecDeque<&str>) {}

    fn handle_ucinewgame_cmd(&mut self) {
        // Not mandatory to be sent by UIs, but most should support it.
        self.game.new_game();
    }

    fn handle_position_cmd(&mut self, tokens: &mut VecDeque<&str>) {
        let pos = *tokens.front().expect("No position provided");
        if pos == "startpos" {
            tokens.pop_front().unwrap();
            self.game.set_to_startpos();
        } else if pos == "fen" {
            tokens.pop_front().unwrap();
            // FEN string is always 6 tokens.
            // Not great to split the string to join it again..
            let fen = tokens.drain(0..6).join(" ");
            self.game.set_to_fen(&fen);
        }

        if matches!(tokens.pop_front(), Some("moves")) {
            self.game.apply_moves(tokens.make_contiguous());
        }
    }

    fn handle_go_cmd(&mut self, _tokens: &mut VecDeque<&str>) {
        let best_move = self.game.start_search();
        self.send_best_move(best_move, None);
    }

    fn handle_stop_cmd(&mut self) {}

    fn handle_d_cmd(&mut self) {
        self.game.display_board(&mut self.writer);
        // self.writer.flush();
    }

    // If best_move is None, it means we are in stale mate.
    pub fn send_best_move(&mut self, mv: Option<Move>, ponder: Option<Move>) {
        if let Some(best_move) = mv {
            if let Some(ponder_move) = ponder {
                outputln!(
                    &mut self.writer,
                    "bestmove {} ponder {}",
                    best_move.pure(),
                    ponder_move.pure()
                );
            } else {
                outputln!(&mut self.writer, "bestmove {}", best_move.pure());
            }
        } else {
            // The protocol doesn't specify what do on stalemates.
            // This is what Stockfish seems to do.
            // <https://github.com/official-stockfish/Stockfish/discussions/5075>
            outputln!(&mut self.writer, "bestmove (none)");
        }
    }

    // As it is currently, the info feature is not very useful,
    // since you need an instance of Uci to call it.
    // Some more practical options could be to implement a custom logger
    // <https://docs.rs/log/latest/log/>
    // Also it depends from where we have to send logs, and how it's controlled.
    // Do we want to have it all over the engine?
    pub fn send_info(&mut self, info: &Info) {
        outputln!(&mut self.writer, "info {}", info);
    }
}

// Whatever the engine wants to send in the UCI info command.
pub struct Info {
    // For now, only string is supported.
    string: String,
}

impl Display for Info {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "string {}", self.string)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::board::Board;

    use super::*;

    #[test]
    fn test_uci_loop_position_startpos() {
        let input = "position startpos\nquit\n";
        let mut reader = Cursor::new(input);
        let mut writer = Vec::new();
        let mut uci = Uci::new(&mut reader, &mut writer);

        uci.uci_loop();

        assert_eq!(uci.game.get_board(), Board::initial_board());
    }

    #[test]
    fn test_uci_loop_position_fen() {
        let input = "position fen r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1\nquit\n";
        let mut reader = Cursor::new(input);
        let mut writer = Vec::new();
        let mut uci = Uci::new(&mut reader, &mut writer);

        uci.uci_loop();

        assert_eq!(
            uci.game.get_board(),
            Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1")
        );
    }

    #[test]
    fn test_uci_loop_position_moves() {
        let input = "position startpos moves e2e4 e7e5\nquit\n";
        let mut reader = Cursor::new(input);
        let mut writer = Vec::new();
        let mut uci = Uci::new(&mut reader, &mut writer);

        uci.uci_loop();

        assert_eq!(
            uci.game.get_board(),
            Board::from_fen("rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq e6 0 1")
        );
    }
}
