//! # Chess engine by Elias Floreteng
//! A dependency-free chess engine/library and cli test made by Elias Floreteng during the KTH DD1337 Programming course
//!
//! # How to run the program
//! 1. Download and run the binary (for x86 systems):  
//!     [Windows](https://elias.floreteng.se/chess/bin/eliasfl-chess.exe)  
//!     [Linux](https://elias.floreteng.se/chess/bin/eliasfl-chess)
//!
//! Pass "fancy" as an argument when running to use unicode symbols for the pieces.
//!
// You can alternatively install it on your system with `cargo install eliasfl-chess`
//!
//! # How to use/test the library
//! 1. Clone [the repository](https://github.com/INDAPlus21/eliasfl-chess)
//! 2. To run the program: `cargo run`
//! 3. (Optional) To run tests: `cargo test`
//! 3. (Optional) View documentation locally: `cargo doc --open`
//!
//! # Library usage
//! Parameters to public functions are of type String and consists of a file (a-h) and rank (1-8) eg. "e2" or "d7"  
//! The functionality of the library is encapsulated in the [`Game`] struct:  
//! - [`Game::new`] is used to create a new game with the standard piece arrangement
//! - [`Game::get_possible_moves`] returns the possible moves for a certain square
//! - [`Game::make_move`] moves a piece to a destination
//! - [`Game::set_promotion`] sets the piece to turn pawns into during promotion, applies for current player
//! - [`Game::get_game_state`] returns the current state of the game
//!
//! # Examples
//! ```
//! use eliasfl_chess::Game;
//!
//! fn main() {
//!     let mut game = Game::new();
//!     println!("{:?}", game.get_possible_moves("e2".to_string()));
//!     game.make_move("e2".to_string(), "e3".to_string());
//!     println!("{:?}", game.get_possible_moves("d1".to_string()));
//!     println!("{:?}", game);
//! }
//! ```
//!
//! ### Implementation notes:
//! - Getting moves during the opposite player's turn ignores if move checks their king.
//! - En passant is not possible.
//! - Castling is not possible.
//!
// How to publish https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html
// How to install as binary https://doc.rust-lang.org/book/ch14-04-installing-binaries.html

use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt;
use std::ops::Not;
mod tests;

use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn new_game() -> String {
    let game = Game::new();
    serde_json::to_string(&game).unwrap()
}

#[wasm_bindgen]
pub fn get_possible_moves(json: &str, pos: &str) -> String {
    let game: Game = serde_json::from_str(json).unwrap();
    if let Some(moves) = game.get_possible_moves(pos.to_string()) {
        serde_json::to_string(&moves).unwrap()
    } else {
        let empty: Vec<String> = Vec::new();
        serde_json::to_string(&empty).unwrap()
    }
}

#[wasm_bindgen]
pub fn make_move(json: &str, pos: &str, dest: &str) -> String {
    let mut game: Game = serde_json::from_str(json).unwrap();
    if let Ok(_) = game.make_move(pos.to_string(), dest.to_string()) {
        serde_json::to_string(&game).unwrap()
    } else {
        serde_json::to_string(&game).unwrap()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum GameState {
    InProgress,
    Check,
    CheckMate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Color {
    White,
    Black,
}
impl Color {
    pub fn direction(&self) -> i32 {
        match self {
            Self::White => 1,
            Self::Black => -1,
        }
    }
}
impl Not for Color {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Piece {
    King(Color),
    Queen(Color),
    Rook(Color),
    Bishop(Color),
    Knight(Color),
    Pawn(Color),
}
impl Piece {
    fn color(&self) -> Color {
        use Color::*;
        use Piece::*;
        match *self {
            King(White) | Queen(White) | Rook(White) | Bishop(White) | Knight(White)
            | Pawn(White) => White,
            _ => Black,
        }
    }

    pub fn symbol(&self) -> char {
        use Color::*;
        use Piece::*;
        match *self {
            King(White) => '♚',
            King(Black) => '♔',
            Queen(White) => '♛',
            Queen(Black) => '♕',
            Rook(White) => '♜',
            Rook(Black) => '♖',
            Bishop(White) => '♝',
            Bishop(Black) => '♗',
            Knight(White) => '♞',
            Knight(Black) => '♘',
            Pawn(White) => '♟',
            Pawn(Black) => '♙',
        }
    }

    /// Get valid destinations for a piece in a certain position.
    ///
    /// This function returns all possible destinations on the board, regardless of what is located in that position.
    pub fn valid_destinations(&self, pos: &Position) -> HashSet<Position> {
        use Piece::*;
        let mut valid_positions = HashSet::new();

        match self {
            King(_) => {
                for f in -1..=1 {
                    for r in -1..=1 {
                        let new_pos = pos.relative_pos(f, r);
                        if let Some(x) = new_pos {
                            valid_positions.insert(x);
                        }
                    }
                }
            }
            Queen(_) => {
                // White as arbitrary color (neither are dependent on color)
                valid_positions.extend(Rook(Color::White).valid_destinations(pos));
                valid_positions.extend(Bishop(Color::White).valid_destinations(pos));
            }
            Rook(_) => {
                for file_offset in -8..=8 {
                    let new_pos = pos.relative_pos(file_offset, 0);
                    if let Some(x) = new_pos {
                        valid_positions.insert(x);
                    }
                }
                for rank_offset in -8..=8 {
                    let new_pos = pos.relative_pos(0, rank_offset);
                    if let Some(x) = new_pos {
                        valid_positions.insert(x);
                    }
                }
            }
            Bishop(_) => {
                let directions = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
                for (file_dir, rank_dir) in directions {
                    for diag in 1..=8 {
                        let new_pos = pos.relative_pos(diag * file_dir, diag * rank_dir);
                        if let Some(x) = new_pos {
                            valid_positions.insert(x);
                        }
                    }
                }
            }
            Knight(_) => {
                // (file, rank)
                let offsets = [
                    (2, 1),
                    (2, -1),
                    (-2, 1),
                    (-2, -1),
                    (1, 2),
                    (1, -2),
                    (-1, 2),
                    (-1, -2),
                ];
                for (file_offset, rank_offset) in offsets {
                    let new_pos = pos.relative_pos(file_offset, rank_offset);
                    if let Some(x) = new_pos {
                        valid_positions.insert(x);
                    }
                }
            }
            Pawn(color) => {
                let direction = color.direction();
                let steps = if matches!(pos.rank, 2 | 7) { 2 } else { 1 };
                for i in 1..=steps {
                    let new_pos = pos.relative_pos(0, i * direction);
                    if let Some(x) = new_pos {
                        valid_positions.insert(x);
                    }
                }
                for file_offset in [-1, 1] {
                    let new_pos = pos.relative_pos(file_offset, direction);
                    if let Some(x) = new_pos {
                        valid_positions.insert(x);
                    }
                }
            }
        };
        valid_positions.remove(&pos);
        valid_positions
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Position {
    /// The column: 1-8 -> a-h (king on file "e")
    pub file: u8,
    /// The row: 1-8 -> 1-8 (1 on white side)
    pub rank: u8,
}
impl Position {
    /// Get Position from string with first character as file (a-h) and second char as rank (1-8).
    pub fn from_string(_from: String) -> Result<Position, Box<dyn Error>> {
        if _from.chars().count() < 2 {
            return Err("Position should consist of file and rank (at least 2 characters)".into());
        }
        let (_file, _rank) = _from.split_at(1);
        let file = match _file.to_lowercase().chars().next() {
            // 97 is char code for 'a', 96 is used because file is one-indexed
            Some(c @ 'a'..='h') => c as u8 - 96,
            _ => return Err("Invalid file, should be in range [a, h]".into()),
        };
        let rank = match _rank.parse()? {
            rank @ 1..=8 => rank,
            _ => return Err("Rank out of range: [1, 8]".into()),
        };
        Ok(Position { file, rank })
    }

    /// Get string with first character as file (a-h) and second char as rank (1-8).
    pub fn to_string(&self) -> String {
        let mut output = String::with_capacity(2);
        // 97 is char code for 'a', 96 is used because file is one-indexed
        output.push((self.file + 96) as char);
        output.push(char::from_digit(self.rank as u32, 10).unwrap_or(' '));
        output
    }

    /// Get the position on the board offset by given values or None if it is outside the board
    pub fn relative_pos(&self, file_offset: i32, rank_offset: i32) -> Option<Position> {
        let file = i32::from(self.file) + file_offset;
        let rank = i32::from(self.rank) + rank_offset;
        if (1..=8).contains(&file) && (1..=8).contains(&rank) {
            Some(Position {
                file: file as u8,
                rank: rank as u8,
            })
        } else {
            None
        }
    }

    /// If position is located on the chess board
    pub fn is_valid(&self) -> bool {
        (1..=8).contains(&self.file) && (1..=8).contains(&self.rank)
    }
}

#[serde_as]
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Game {
    /// Board HashMap with Position keys and Piece values
    #[serde_as(as = "Vec<(_, _)>")]
    pub board: HashMap<Position, Piece>,
    /// The color who's turn it is
    pub active_color: Color,
    /// Promotion piece per color
    pub promotion: [Piece; 2],
    /// Current game state. Call `get_game_state` to check for checkmate
    pub state: GameState,
}
impl Game {
    /// Initializes a new board with standard piece positions.
    pub fn new() -> Self {
        use Color::*;
        use Piece::*;
        let mut starting_board: HashMap<Position, Piece> = HashMap::new();
        // Generate starting board
        // Place respective pieces on ranks 1 and 8 for White and Black
        for (r, color) in [(1, White), (8, Black)] {
            for (f, piece) in [
                Rook(color),
                Knight(color),
                Bishop(color),
                Queen(color),
                King(color),
                Bishop(color),
                Knight(color),
                Rook(color),
            ]
            .iter()
            .enumerate()
            {
                starting_board.insert(
                    Position {
                        file: (f + 1) as u8,
                        rank: r,
                    },
                    *piece,
                );
            }
        }
        // Place White and Black Pawns on rank 2 and 7
        for (r, color) in [(2, White), (7, Black)] {
            for f in 1..=8 {
                starting_board.insert(
                    Position {
                        file: f as u8,
                        rank: r,
                    },
                    Pawn(color),
                );
            }
        }

        Self {
            board: starting_board,
            state: GameState::InProgress,
            active_color: Color::White,
            promotion: [Piece::Queen(Color::White), Piece::Queen(Color::Black)],
        }
    }

    /// Detects if there is a piece in the way for a move
    fn _is_piece_in_way(&self, piece: &Piece, position: &Position, destination: &Position) -> bool {
        match piece {
            // Knight and king can move over pieces (king can't move over because he can only move 1)
            Piece::Knight(_) | Piece::King(_) => {}
            Piece::Queen(c) => {
                return self._is_piece_in_way(&Piece::Bishop(*c), position, destination)
                    || self._is_piece_in_way(&Piece::Rook(*c), position, destination);
            }
            Piece::Bishop(_) => {
                let file = destination.file as i32 - position.file as i32;
                let rank = destination.rank as i32 - position.rank as i32;
                for offset in 1..max(file.abs(), rank.abs()) {
                    // Iterate over positions between position and destination (offset is equal with different sign because bishop)
                    if let Some(between_pos) =
                        position.relative_pos(offset * file.signum(), offset * rank.signum())
                    {
                        if let Some(_) = self.board.get(&between_pos) {
                            // If any of the pieces between are occupied
                            return true;
                        }
                    }
                }
            }
            Piece::Rook(_) | Piece::Pawn(_) => {
                if position.file == destination.file {
                    // If in same file: iterate over locations in between
                    // + 1 to ignore current piece and noninclusive range
                    for between in min(position.rank, destination.rank) + 1
                        ..max(position.rank, destination.rank)
                    {
                        let between_pos = Position {
                            rank: between,
                            file: position.file,
                        };
                        if let Some(_) = self.board.get(&between_pos) {
                            // If any of the pieces between are occupied
                            return true;
                        }
                    }
                }
                if position.rank == destination.rank {
                    // If in same rank: iterate over locations in between
                    // + 1 to ignore current piece and noninclusive range
                    for between in min(position.file, destination.file) + 1
                        ..max(position.file, destination.file)
                    {
                        let between_pos = Position {
                            rank: position.rank,
                            file: between,
                        };
                        if let Some(_) = self.board.get(&between_pos) {
                            // If any of the pieces between are occupied
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    /// Get possible moves for provided Position
    ///
    /// Includes destinations that expose king
    ///
    /// None if invalid position or no piece, empty set if no possible moves
    fn _get_possible_moves(&self, position: &Position) -> Option<HashSet<Position>> {
        if let Some(piece) = self.board.get(&position) {
            let mut destinations = piece.valid_destinations(position);
            // Filter out moves that land on own piece or has piece in way
            destinations.retain(|destination| {
                // Keep if destination is opposite color and no pieces are in the way of move
                if let Some(p) = self.board.get(&destination) {
                    // If pawn and dest is occupied -> deny straight move/capture
                    if matches!(piece, Piece::Pawn(_)) && position.file == destination.file {
                        return false;
                    }
                    p.color() != piece.color()
                        && !self._is_piece_in_way(piece, position, destination)
                } else {
                    // If pawn and dest is empty -> deny diagonal capture
                    if matches!(piece, Piece::Pawn(_)) {
                        position.file == destination.file
                            && !self._is_piece_in_way(piece, position, destination)
                    } else {
                        !self._is_piece_in_way(piece, position, destination) // Destination has no piece
                    }
                }
            });

            Some(destinations)
        } else {
            None
        }
    }

    /// If a piece is standing on the given tile, return all possible
    /// new positions of that piece.
    ///
    /// Does not include destinations that expose king
    ///
    /// Returns None if invalid position or no piece there
    ///
    /// Returns empty Vec if no moves are available for piece
    pub fn get_possible_moves(&self, _position: String) -> Option<Vec<String>> {
        if let Ok(position) = Position::from_string(_position) {
            if let Some(mut moves) = self._get_possible_moves(&position) {
                // Cannot move to/capture king -> filter king destinations
                moves.retain(|_p| !matches!(self.board.get(_p), Some(Piece::King(_))));
                // Filter out moves that threaten own king
                moves.retain(|_p| self._ok_to_make_move(&position, _p));
                let mut move_vec: Vec<String> = moves.iter().map(|_p| _p.to_string()).collect();
                move_vec.sort_unstable();
                Some(move_vec)
            } else {
                None // No piece there
            }
        } else {
            None // Invalid position
        }
    }

    /// If ok to make move
    ///
    /// Returns false if own king is threatened by move or if move cannot be made
    fn _ok_to_make_move(&self, from: &Position, to: &Position) -> bool {
        // If getting moves for opposite player -> assume king cannot be threatened
        // Unwrap _should_ never panic
        if self.active_color != self.board.get(from).unwrap().color() {
            return true;
        }
        let mut new_game = self.clone();
        new_game.make_move(from.to_string(), to.to_string()).is_ok()
    }

    /// If the current game state is not CheckMate and the move is legal,
    /// move a piece.
    ///
    /// Return Err if move is illegal or if piece has no possible moves, otherwise Ok with removed piece or None if no piece is removed
    pub fn make_move(&mut self, _from: String, _to: String) -> Result<Option<Piece>, &str> {
        if let (Ok(from), Ok(to)) = (
            Position::from_string(_from.clone()),
            Position::from_string(_to.clone()),
        ) {
            if let Some(piece) = self.board.get(&from) {
                if piece.color() != self.active_color {
                    return Err("Trying to move opponents piece");
                }

                if let Some(possible_moves) =
                    self._get_possible_moves(&Position::from_string(_from).unwrap())
                {
                    if let Some(_) = possible_moves.get(&to) {
                        // Cannot move to/capture king
                        if matches!(self.board.get(&to), Some(Piece::King(_))) {
                            return Err("Cannot capture king");
                        }
                        // Capture piece (or move to square if empty)
                        let new_piece =
                            if matches!(to.rank, 1 | 8) && matches!(piece, Piece::Pawn(_)) {
                                // Set new piece to promotion piece if pawn and dest rank is 1 or 8
                                if let Some(prom_piece) = self
                                    .promotion
                                    .iter()
                                    .find(|p| p.color() == self.active_color)
                                {
                                    *prom_piece
                                } else {
                                    // Promotion piece not found for current player -> use queen
                                    Piece::Queen(self.active_color)
                                }
                            } else {
                                piece.clone() // Not pawn -> clone old piece to new location
                            };
                        // Actual piece move
                        let before_move = self.board.clone();
                        let removed = self.board.insert(to, new_piece); // returns removed piece (or None)
                        self.board.remove(&from);
                        if self._king_is_threatened(self.active_color) {
                            // Own king is threatened -> invalid move
                            self.board = before_move;
                            return Err("Move threatens own king");
                        }
                        // If piece is able to move and doesn't threaten own king -> remove check state
                        self.state = GameState::InProgress;

                        // If oppoiste king is threatened after move -> check other player
                        if self._king_is_threatened(!self.active_color) {
                            self.state = GameState::Check;
                        }

                        // Change to opposite players turn
                        self.active_color = !self.active_color;

                        Ok(removed)
                    } else {
                        Err("Destination move is invalid")
                    }
                } else {
                    Err("No possible moves")
                }
            } else {
                Err("No piece in position(s)")
            }
        } else {
            Err("Invalid position(s)")
        }
    }

    /// Returns true if king with `color` is threatened by piece in `position`
    fn _threatens_king(&self, position: &Position, color: Color) -> bool {
        if let Some(moves) = self._get_possible_moves(position) {
            for mov in moves {
                if let Some(p) = self.board.get(&mov) {
                    match p {
                        Piece::King(_c) if *_c == color => {
                            return true;
                        }
                        _ => continue,
                    }
                }
            }
            false // No piece threatens king
        } else {
            false // No piece in position
        }
    }

    /// Returns if king with provided color is threatened by opposite color
    ///
    /// Iterates over all pieces to find if any of them threatens king with `color`
    fn _king_is_threatened(&self, color: Color) -> bool {
        for (position, piece) in self.board.iter() {
            if piece.color() != color && self._threatens_king(position, color) {
                return true;
            }
        }
        false
    }

    /// Returns if there is a checkmate for the provided color
    ///
    /// Iterates over all moves for `color`'s pieces and if no moves can be made, the game is check mate
    fn _is_checkmate(&self, color: Color) -> bool {
        for (position, _) in self.board.iter().filter(|(_, &p)| p.color() == color) {
            if let Some(moves) = self.get_possible_moves(position.to_string()) {
                if moves.len() > 0 {
                    return false;
                }
            } // Invalid position or no piece, should not be reached
        }
        true
    }

    /// Set promotion piece for the current player.
    ///
    /// String must be "queen", "rook", "bishop" or "knight". Otherwise error is returned
    pub fn set_promotion(&mut self, _piece: String) -> Result<(), &str> {
        let color = self.active_color;
        for prom_piece in self.promotion.iter_mut() {
            if prom_piece.color() == color {
                *prom_piece = match &_piece.to_lowercase()[..] {
                    "queen" => Piece::Queen(color),
                    "rook" => Piece::Rook(color),
                    "bishop" => Piece::Bishop(color),
                    "knight" => Piece::Knight(color),
                    _ => return Err("Invalid promotion piece"),
                };
                break;
            }
        }
        Ok(())
    }

    /// Gets the current game state
    ///
    /// Detects and returns checkmate (private field game.state does not)
    pub fn get_game_state(&mut self) -> GameState {
        if self._is_checkmate(self.active_color) {
            self.state = GameState::CheckMate;
        }
        return self.state;
    }
}

impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        output.push_str(
            &format!(
                "{} a b c d e f g h\n",
                match self.active_color {
                    Color::White => "W", // ⚑
                    Color::Black => "B", // ⚐
                }
            )[..],
        );
        for rank in (1..=8).rev() {
            for file in 1..=8 {
                if file == 1 {
                    output.push(char::from_digit(rank as u32, 10).unwrap_or(' '));
                }
                output.push(' ');
                let pos = Position { file, rank };
                let board = &self.board;
                output.push(if board.contains_key(&pos) {
                    board[&pos].symbol()
                } else {
                    ' '
                });
            }
            output.push_str("\n");
        }
        write!(f, "\n{}", output)
    }
}
