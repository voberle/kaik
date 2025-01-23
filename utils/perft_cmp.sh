#!/bin/bash
# Calls perft commands of Stockfish and Kaik, and compares their output.

# Check if at least one argument is provided (depth)
if [ $# -lt 1 ]; then
  echo "Usage: $0 <depth> [position] [moves]"
  exit 1
fi

# Get the depth from the first argument
depth=$1

# Set default values for position and moves
position="startpos"
moves=""

# Override defaults if additional arguments are provided
if [ $# -ge 2 ]; then
  position=$2
  stockfish_position="fen $2"
fi
if [ $# -ge 3 ]; then
  moves=$3
fi

# Construct the Stockfish command string
command="position $stockfish_position moves $moves
go perft $depth"

# Run Stockfish, filter the output, sort it and store it in a variable
stockfish_output=$(echo "$command" | stockfish | grep -v "Stockfish\|info" | sort)

# Run kaik, capture its output and sort it
kaik_output=$(cargo r --release -- divide $depth "$position" "$moves" | sort)

# Compare the outputs and print the diff
diff -B <(echo "$stockfish_output") <(echo "$kaik_output")