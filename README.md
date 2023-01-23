# Cube TUI

A pure-rust terminal user-interface for speedcubing

![Welcome screen of cube-tui](images/welcome.jpg)

## Installation (GNU/Linux)

Install rust if not already: https://www.rust-lang.org/tools/install

Clone the repo
```bash
git clone https://github.com/hoehlrich/cube-tui
```

Build it
```bash
cd cube-tui && cargo install --path .
```

## Features

### What it does

1. Time's your solves
2. Generates stats for your solves
3. Generates a random scramble
4. Graph solves
5. Runs lightweight, in the terminal, and with pure rust

### What it will do

1. More tools (scramble display, solver)
2. Multi-stage solves
3. Manage sessions for different cubes
4. Generate a scramble correctly (instead of random turns)

### What it doesn't do

1. Stackmat-esque timer (not possible in ANSI terminal)
2. Integration with a db
