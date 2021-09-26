use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt;
use std::ops::Not;
mod tests;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Check,
    GameOver,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    White,
    Black,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Piece {
    King(Color),
    Queen(Color),
    Rook(Color),
    Bishop(Color),
    Knight(Color),
    Pawn(Color),
}

impl Piece {
    pub fn symbol(&self) -> char {
        use Color::*;
        use Piece::*;
        match *self {
            King(White) => '♔',
            King(Black) => '♚',
            Queen(White) => '♕',
            Queen(Black) => '♛',
            Rook(White) => '♖',
            Rook(Black) => '♜',
            Bishop(White) => '♗',
            Bishop(Black) => '♝',
            Knight(White) => '♘',
            Knight(Black) => '♞',
            Pawn(White) => '♙',
            Pawn(Black) => '♟',
        }
    }

    /// Get valid destinations for a piece in a certain position.
    ///
    /// This function returns all possible destinations on the board, regardless of what is located in that position.
    pub fn valid_destinations(&self, pos: Position) -> HashSet<Position> {
        use Piece::*;
        let mut valid_positions = HashSet::new();

        match *self {
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
            Queen(_) => todo!(),
            Rook(_) => todo!(),
            Bishop(_) => todo!(),
            Knight(_) => todo!(),
            Pawn(_) => {
                if pos.rank == 1 {
                    let new_pos = pos.relative_pos(1, 0);
                    if let Some(x) = new_pos {
                        valid_positions.insert(x);
                    }
                }
            }
        };
        valid_positions.remove(&pos);
        valid_positions
    }

    fn color(&self) -> Color {
        use Color::*;
        use Piece::*;
        match *self {
            King(White) | Queen(White) | Rook(White) | Bishop(White) | Knight(White)
            | Pawn(White) => White,
            _ => Black,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    /// The column: 1-8 -> a-h (king on file "e")
    pub file: u8,
    /// The row: 1-8 -> 1-8 (1 on white side)
    pub rank: u8,
}
impl Position {
    /// Get Position from string with first character as file (a-h) and second char as rank (1-8).
    pub fn from_string(_from: String) -> Result<Position, Box<dyn Error>> {
        let (_file, _rank) = _from.split_at(1);
        let file = match &_file.to_lowercase()[..] {
            "a" => 1,
            "b" => 2,
            "c" => 3,
            "d" => 4,
            "e" => 5,
            "f" => 6,
            "g" => 7,
            "h" => 8,
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
        output.push(match self.file {
            1 => 'a',
            2 => 'b',
            3 => 'c',
            4 => 'd',
            5 => 'e',
            6 => 'f',
            7 => 'g',
            8 => 'h',
            _ => ' ',
        });
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

pub struct Game {
    /// Board HashMap with Position keys and Piece values
    pub board: HashMap<Position, Piece>,
    /// Current game state.
    pub state: GameState,
    /// The color who's turn it is
    pub active_color: Color,
}

impl Game {
    /// Initialises a new board with standard piece positions.
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
        }
    }

    /// Get possible moves from provided Position
    fn _get_possible_moves(&self, position: Position) -> Option<HashSet<Position>> {
        if let Some(piece) = self.board.get(&position) {
            let mut destinations = piece.valid_destinations(position);
            // Filter out moves that land on own piece
            destinations.retain(|destination| {
                if let Some(piece) = self.board.get(&destination) {
                    self.active_color != piece.color()
                } else {
                    true
                }
            });
            Some(destinations)
        } else {
            None
        }
    }

    /// If a piece is standing on the given tile, return all possible
    /// new positions of that piece. Don't forget to the rules for check.
    ///
    /// (optional) Don't forget to include en passent and castling.
    pub fn get_possible_moves(&self, _position: String) -> Option<Vec<String>> {
        if let Ok(position) = Position::from_string(_position) {
            if let Some(moves) = self._get_possible_moves(position) {
                Some(moves.iter().map(|_p| _p.to_string()).collect())
            } else {
                None
            }
        } else {
            None
        }
    }

    /// If the current game state is InProgress and the move is legal,
    /// move a piece and return the resulting state of the game.
    pub fn make_move(&mut self, _from: String, _to: String) -> Option<GameState> {
        if let Some(possible_moves) = self.get_possible_moves(_from) {
            if possible_moves.contains(&_to) {
                self.active_color = !self.active_color;
                Some(self.state)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Set the piece type that a peasant becames following a promotion.
    pub fn set_promotion(&mut self, _piece: String) -> () {
        ()
    }
}

impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        output.push_str("  a b c d e f g h\n");
        for rank in 1..=8 {
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
