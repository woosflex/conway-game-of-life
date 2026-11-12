# Conway's Game of Life in Rust

A terminal-based implementation of [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) written in Rust.

## Overview

Conway's Game of Life is a zero-player cellular automaton devised by mathematician John Horton Conway in 1970. Its evolution is determined solely by its initial state. This implementation provides an interactive CLI to configure the grid size and initial population, randomly populates the board, and renders the generations continuously in your terminal.

## Rules of the Game

Each cell on the grid is in one of two states: **alive** (`0`, rendered in green) or **dead** (`O`, rendered in red). At each time step, the following transitions occur based on the cell's 8 neighbours (horizontal, vertical, diagonal):

1. **Underpopulation:** Any live cell with fewer than two live neighbours dies.
2. **Survival:** Any live cell with two or three live neighbours lives on to the next generation.
3. **Overpopulation:** Any live cell with more than three live neighbours dies.
4. **Reproduction:** Any dead cell with exactly three live neighbours becomes a live cell.

## Features

- **Interactive Configuration:** Prompt-driven input for grid height, width, and starting live cell count.
- **Input Validation:** Handles invalid numerical entries gracefully and prevents setting more alive cells than available grid spaces.
- **Random Seeding:** Randomly distributes the initial live cells across the grid using the `rand` crate.
- **ANSI Terminal Rendering:** Clears the terminal screen (`\x1b[2J`) between frames and renders cells with ANSI color highlights for a smooth terminal animation.
- **Double-Buffered State Updates:** Computes the next generation into a temporary buffer to ensure all cell transitions occur synchronously.

## Prerequisites

- [Rust & Cargo](https://www.rust-lang.org/tools/install) (2024 edition supported; Rust 1.85+ recommended)

## Getting Started

### Run with Cargo

```bash
cargo run
```

### Build for Release

To compile an optimized binary:

```bash
cargo build --release
./target/release/gol
```

### Example Usage

```text
Enter the height of the grid: 
20
Enter the width of the grid: 
40
Height: 20
Width: 40
Enter the number of alive cells in the start: 
150
```

The simulation will start animating in the terminal. To exit the program at any time, press `Ctrl + C`.

## Project Structure

```text
.
├── Cargo.lock
├── Cargo.toml
├── README.md
└── src
    └── main.rs       # Main game loop, input parsing, rules logic, and terminal renderer
```

## Dependencies

- [`rand`](https://crates.io/crates/rand) - Fast, lightweight random number generation for initial cell distribution.
