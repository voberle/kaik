#!/bin/bash
# Runs the perft command of Stockfish.

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

# Run Stockfish, filter the output, and print the result
echo "$command" | stockfish | grep -v "Stockfish\|info"