# Ant Test

## Overview

`ant_test` is a Rust project that provides a way to explore a 2D grid starting from a given cell, considering the digits of the coordinates of each cell.

## Features

- **Cell Representation**: Represents each cell with `FieldCell` that contains coordinates `(x, y)`.
- **Cell Counter**: Counts the number of cells available to visit based on the sum of the digits of the coordinates.
- **Cell Movement**: Allows movement to adjacent cells (top, bottom, left, right).

## Installation

Make sure you have Rust and Cargo installed. If not, download them [here](https://rustup.rs/).

Clone the repository:

```bash
git clone https://github.com/qqrm/ant_test.git
```

Navigate to the project directory:

```bash
cd ant_test
```

Compile the project:

```bash
cargo build
```

## Usage

You can run the tests by using:

```bash
cargo test
```

## Structure

- `mod cell`: Contains the `FieldCell` struct and its methods.
- `mod cell_counter`: Contains the `CellCounter` struct and its methods.

