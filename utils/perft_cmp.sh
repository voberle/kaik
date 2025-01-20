#!/bin/bash
# Calls perft commands of Stockfish and Kaik, and compares their output.

# Check if two arguments are provided
if [ $# -ne 2 ]; then
  echo "Usage: $0 <depth> <moves>"
  exit 1
fi

# Get the depth and moves from the arguments
depth=$1
moves=$2

# Construct the Stockfish command string
command="position startpos moves $moves
go perft $depth"

# Run Stockfish, filter the output, sort it and store it in a variable
stockfish_output=$(echo "$command" | stockfish | grep -v "Stockfish\|info" | sort)

# Run kaik, capture its output and sort it
kaik_output=$(cargo r --release -- $depth "$moves" | sort)

# Compare the outputs and print the diff
diff -B <(echo "$stockfish_output") <(echo "$kaik_output")