# Advent of Code 2023
My solutions to the [Advent of Code 2023](https://adventofcode.com/2023) done in Rust, again.

## Running
This year I decided upon using [cargo-aoc](https://github.com/gobanos/cargo-aoc), so follow the `README.md` there for 
specifics. In short, use the following commands
```sh
# If input not provided, set your own aoc token to be able to retrieve the input
cargo aoc credentials -s {token}

# Build the project, get the input of the latest day if not present, and run the latest day
cargo aoc

# Like previous, but run a specific day and part
cargo aoc -d {day} -p {part}

# Run the benchmarks for all days, optionally add -o flag to automatically open it in your browser
cargo aoc bench
```

