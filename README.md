# ChessBro - UCI Chess Engine

A simple UCI-compatible chess engine written in Rust that plays random moves.

## Features

- UCI (Universal Chess Interface) protocol support
- Random move selection from all legal moves
- Compatible with any UCI chess GUI (Arena, Cute Chess, etc.)

## Building

```bash
cargo build --release
```

The compiled binary will be located at `target/release/chessbro-rust`.

## Usage

### Manual Testing

You can test the engine manually via stdin/stdout:

```bash
cargo run --release
```

Then type UCI commands:

```
uci
isready
position startpos
go
position startpos moves e2e4 e7e5
go
quit
```

### Using with Chess GUIs

1. Build the release binary: `cargo build --release`
2. In your chess GUI (e.g., Cute Chess, Arena):
   - Add a new engine
   - Point to the binary at `target/release/chessbro-rust`
   - The engine should now be available to play against

### Supported UCI Commands

- `uci` - Initialize UCI mode
- `isready` - Check if engine is ready
- `ucinewgame` - Start a new game
- `position [startpos | fen <fenstring>] moves <move1> ... <movei>` - Set up position
- `go` - Start calculating and return best move
- `quit` - Exit the engine

## Implementation

The engine uses:
- [chess](https://crates.io/crates/chess) crate for move generation and board representation
- [rand](https://crates.io/crates/rand) crate for random move selection

## Future Enhancements

- Add actual search algorithms (minimax, alpha-beta pruning)
- Implement position evaluation
- Add time management
- Support for UCI options
- Opening book support
