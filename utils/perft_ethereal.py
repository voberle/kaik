#!/usr/bin/python3
#
# Parses the Perft results file found on
# https://github.com/AndyGrant/Ethereal/blob/master/src/perft/standard.epd
# and runs it against Kaik, checking the results.

import subprocess

def parse_line(line):
  """Parses a line of the input file.

  Args:
    line: A string containing a line from the input file.

  Returns:
    A tuple containing the FEN string and a list of (depth, node_count) tuples.
  """
  try:
    parts = line.strip().split(';')
    fen = parts[0].strip()
    depths = []
    for depth_str in parts[1:]:
      depth, node_count = depth_str.split()
      depths.append((int(depth[1:]), int(node_count)))
    return fen, depths
  except ValueError:
    print(f"Error parsing line: {line.strip()}")
    return None, None

def run_cargo(depth, fen):
  """Runs the cargo program and captures the output.

  Args:
    depth: The depth to run the cargo program with.
    fen: The FEN string to run the cargo program with.

  Returns:
    The node count output by the cargo program, or None if an error occurred.
  """
  try:
    result = subprocess.run(
        ['cargo', 'r', '--release', 'perft', str(depth), fen],
        capture_output=True,
        text=True,
        check=True
    )
    return int(result.stdout.strip())
  except subprocess.CalledProcessError as e:
    print(f"Error running cargo: {e}")
    return None

def main(filename):
  """Parses the input file and compares the node counts.

  Args:
    filename: The name of the input file.
  """
  passed = 0
  failed = 0
  with open(filename, 'r') as f:
    for line in f:
      fen, depths = parse_line(line)
      if fen is None:
        continue

      for depth, expected_node_count in depths:
        actual_node_count = run_cargo(depth, fen)
        if actual_node_count is None:
          continue

        if actual_node_count == expected_node_count:
          print(f"PASS: {fen} D{depth} - Expected: {expected_node_count}, Actual: {actual_node_count}")
          passed += 1
        else:
          print(f"FAIL: {fen} D{depth} - Expected: {expected_node_count}, Actual: {actual_node_count}")
          failed += 1

  print(f"\nTotal Passed: {passed}, Total Failed: {failed}")

if __name__ == "__main__":
  filename = "utils/standard.epd"  # Replace with your input filename
  main(filename)