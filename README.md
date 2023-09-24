# redkar-chess

# Usage

## Creating a game/board
You can create a new game by calling:
```rust
let mut game = Game::new_game();
```

You can also create a gameboard using FEN string:
```rust
let mut game = Game::game_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
```
However, the move history will be empty when this method is used. 

The board is a 2D-array of Option\<Piece\>, Some() indicating the existence of a piece, and None indicating the absence of a piece. 

## Moves
To make a move, you call the function do\_move:
```rust
pub fn do_move(&mut self, mv: Move) -> Result<Option<Decision>, MoveError>
```
As you can see, you must pass in an instance of the struct Move which is defined as so:
```rust 
pub struct Move {
    start_x: usize,
    start_y: usize,
    end_x: usize,
    end_y: usize,
}
```

The chessboard coordinates are 0-indexed and are as so: 
Files: a-h = 7, 6, ... 0 
Ranks: 1-8 = 0, 1, ... 7

The function do\_move will perform the move if possible and return either a Err(MoveError), or an Ok(Option\<Decision\>);
If Option\<Decision\> is Some(Decision), then the game has ended, and the decision will be given:
```rust
pub enum Decision {
    White, 
    Black, 
    Tie,
}
```

