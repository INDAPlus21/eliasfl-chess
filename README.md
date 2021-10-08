# Chess engine by Elias Floreteng

A dependency-free chess engine/library and cli test made by Elias Floreteng during the KTH DD1337 Programming course.  
Playable through the command-line.

> Note: rust-toolchain version at least 1.53.0 is required to run the project with cargo. Update with `rustup update`

## Play chess in the browser

The current program has webassembly bindings and a compiled version can be found on [elias.floreteng.se/chess/play](https://elias.floreteng.se/chess/play/)

Compile to webassembly by running the `build_webassembly.ps1` script and output is in the `pkg` directory.

## Download the program

- [Windows](https://elias.floreteng.se/chess/bin/eliasfl-chess.exe)
- [Linux](https://elias.floreteng.se/chess/bin/eliasfl-chess)

_Run in command-line with argument "fancy" to use unicode piece symbols_

## [Documentation](https://elias.floreteng.se/chess)

View the documentation at [elias.floreteng.se/chess](https://elias.floreteng.se/chess).
This includes examples and run instructions.

## Quick API refrence

To view full documentation locally: `cargo doc --open`

Positions are represented as Strings with file (a-h) and rank (1-8) eg. "e2", "d7"

Following symbols are decendants of the `Game` struct:

| **Symbol**                                                                                | **Description**                                                                                                       |
| ----------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------- |
| `pub fn new() -> Game`                                                                    | Initialises a new board with pieces.                                                                                  |
| `pub fn get_possible_moves(&self, _position: String) -> Optional<Vec<String>>`            | If a piece is standing on the given tile, return all possible new positions of that piece.                            |
| `pub fn make_move(&mut self, _from: String, _to: String ) -> Result<Option<Piece>, &str>` | If the current game state is `InProgress` and the move is legal, move a piece and return the removed piece (or None). |
| `pub fn set_promotion(&mut self, _piece: String) -> Result<(), &str>`                     | Set the piece type that a peasant becames following a promotion. (`_piece` is "queen", "rook", "bishop" or "knight")  |
| `pub fn get_game_state() -> GameState`                                                    | Get the current game state.                                                                                           |
| `pub active_color: Color`                                                                 | Get the color for who's turn it is.                                                                                   |
