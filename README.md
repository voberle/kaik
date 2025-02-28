# Kaik Chess Engine.

![Kaik logo](kaik_small.jpeg)

Kaik is a chess engine written in Rust.

[Read more about the story of its development](https://docs.google.com/document/d/e/2PACX-1vQY7HSYDMW61Dagpkt2_ApORg0S4KayXvj3mwOpUI-OoNZVcaMjGGsVzT7NiYJ3Isv3cW5KeT_oVwDN/pub).

You can [play it on Lichess](https://lichess.org/@/kaik_bot) (it might not always be online).

## Features

Current features:

- Move generation:
  - Bitboard based.
  - Hyperbola Quintessence approach for sliding pieces.
  - Perft tests to validate move generation.
- Board identification with Zobrist key.
- Search:
  - Alpha-beta.
- Evaluation:
  - Material based.
- UCI interface.

## Architecture

[Design overview](DESIGN.md).

## Testing

### Using from the command-line

Since UCI is a simple text protocol using stdout/stdin, one can just run the engine and enter UCI commands directly.

Tip: Use the cool [rlwrap](https://github.com/hanslub42/rlwrap) tool to get history and command line completion.

    rlwrap -r cargo r --release

### With a UI

Any Chess UI that supports adding UCI based engines should work.

On Mac, I found that [BanksiaGUI](https://banksiagui.com) was working the best.

### Running games against other engines

The [command line interface c-chess-cli](https://github.com/lucasart/c-chess-cli) is a convenient way to have Kaik play against itself:

    c-chess-cli -engine cmd="target/release/kaik --log-discriminant=k1" -engine cmd="target/release/kaik --log-discriminant=k2" -pgn out.pgn 1

For best performance, remember to add:

    RUSTFLAGS="-C target-cpu=native"

### Perft

The move generation is verified using [Perft tests](https://www.chessprogramming.org/Perft).

Some are implemented as Rust tests and executed as part of the test suite. Not all are enabled by default, run all with:

    cargo t --release --features perft

Others can be executed separately.

The [Ethereal test cases](https://github.com/AndyGrant/Ethereal/blob/master/src/perft/standard.epd):

    ./utils/perft_ethereal.py

Comparing with Stockfish results:

    ./utils/perft_cmp.sh 2 "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1" "a2a3"

### UCI Compliance

The [Fastchess tool](https://github.com/Disservin/fastchess) has a UCI compliance checker

    fastchess --compliance target/release/kaik

## Resources

- [Chess Programming Wiki](https://www.chessprogramming.org)