# Kaik Chess Engine.

Kaik is a chess engine.

## Features

- Bitboards based move generation.

## Perft

The move generation is verified using [Perft tests](https://www.chessprogramming.org/Perft).

Some are implemented as Rust tests and executed as part of the test suite. Not all are enabled by default, run all with:

    cargo t --release --features perft

Others can be executed separately.

The [Ethereal test cases](https://github.com/AndyGrant/Ethereal/blob/master/src/perft/standard.epd):

    ./utils/perft_ethereal.py

Comparing with Stockfish results:

    ./utils/perft_cmp.sh 2 "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1" "a2a3"

## Resources

- [Chess Programming Wiki](https://www.chessprogramming.org)