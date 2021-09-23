use std::collections::HashMap;
use std::fmt;

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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    /// The column: 1-8 -> a-h
    file: u8,
    /// The row: 1-8 -> 1-8 (1 on white side)
    rank: u8,
}

/* IMPORTANT:
* - Document well!
 * - Write well structured and clean code!
 */

pub struct Game {
    /* save board, active colour, ... */
    state: GameState,
    board: HashMap<Position, Piece>,
}

impl Game {
    /// Initialises a new board with pieces.
    pub fn new() -> Game {
        use Color::*;
        use Piece::*;
        let starting_board: HashMap<Position, Piece> = [
            // White
            (Position { file: 1, rank: 1 }, Rook(White)),
            (Position { file: 2, rank: 1 }, Knight(White)),
            (Position { file: 3, rank: 1 }, Bishop(White)),
            (Position { file: 4, rank: 1 }, Queen(White)),
            (Position { file: 5, rank: 1 }, King(White)),
            (Position { file: 6, rank: 1 }, Bishop(White)),
            (Position { file: 7, rank: 1 }, Knight(White)),
            (Position { file: 8, rank: 1 }, Rook(White)),
            // White Pawns
            (Position { file: 1, rank: 2 }, Pawn(White)),
            (Position { file: 2, rank: 2 }, Pawn(White)),
            (Position { file: 3, rank: 2 }, Pawn(White)),
            (Position { file: 4, rank: 2 }, Pawn(White)),
            (Position { file: 5, rank: 2 }, Pawn(White)),
            (Position { file: 6, rank: 2 }, Pawn(White)),
            (Position { file: 7, rank: 2 }, Pawn(White)),
            (Position { file: 8, rank: 2 }, Pawn(White)),
            // Black
            (Position { file: 1, rank: 8 }, Rook(Black)),
            (Position { file: 2, rank: 8 }, Knight(Black)),
            (Position { file: 3, rank: 8 }, Bishop(Black)),
            (Position { file: 4, rank: 8 }, Queen(Black)),
            (Position { file: 5, rank: 8 }, King(Black)),
            (Position { file: 6, rank: 8 }, Bishop(Black)),
            (Position { file: 7, rank: 8 }, Knight(Black)),
            (Position { file: 8, rank: 8 }, Rook(Black)),
            // Black Pawns
            (Position { file: 1, rank: 7 }, Pawn(White)),
            (Position { file: 2, rank: 7 }, Pawn(White)),
            (Position { file: 3, rank: 7 }, Pawn(White)),
            (Position { file: 4, rank: 7 }, Pawn(White)),
            (Position { file: 5, rank: 7 }, Pawn(White)),
            (Position { file: 6, rank: 7 }, Pawn(White)),
            (Position { file: 7, rank: 7 }, Pawn(White)),
            (Position { file: 8, rank: 7 }, Pawn(White)),
        ]
        .iter()
        .cloned()
        .collect();
        Game {
            /* initialise board, set active colour to white, ... */
            state: GameState::InProgress,
            board: starting_board,
        }
    }

    /// If the current game state is InProgress and the move is legal,
    /// move a piece and return the resulting state of the game.
    pub fn make_move(&mut self, _from: String, _to: String) -> Option<GameState> {
        None
    }

    /// Set the piece type that a peasant becames following a promotion.
    pub fn set_promotion(&mut self, _piece: String) -> () {
        ()
    }

    /// Get the current game state.
    pub fn get_game_state(&self) -> GameState {
        self.state
    }

    /// If a piece is standing on the given tile, return all possible
    /// new positions of that piece. Don't forget to the rules for check.
    ///
    /// (optional) Don't forget to include en passent and castling.
    pub fn get_possible_moves(&self, _postion: String) -> Option<Vec<String>> {
        None
    }
}

// Implement print routine for Game.
//
// Output example:
// |:----------------------:|
// | R  Kn B  K  Q  B  Kn R |
// | P  P  P  P  P  P  P  P |
// | *  *  *  *  *  *  *  * |
// | *  *  *  *  *  *  *  * |
// | *  *  *  *  *  *  *  * |
// | *  *  *  *  *  *  *  * |
// | P  P  P  P  P  P  P  P |
// | R  Kn B  K  Q  B  Kn R |
// |:----------------------:|

impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /* build board representation string */

        let mut output = String::from("");

        for f in 1..=8 {
            for r in 1..=8 {
                output.push_str("\n")
            }
            output.push_str("\n")
        }
        write!(f, output)
    }
}

// --------------------------
// ######### TESTS ##########
// --------------------------

#[cfg(test)]
mod tests {
    use super::Game;
    use super::GameState;

    // check test framework
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // example test
    // check that game state is in progress after initialisation
    #[test]
    fn game_in_progress_after_init() {
        let game = Game::new();

        println!("{:?}", game);

        assert_eq!(game.get_game_state(), GameState::InProgress);
    }
}
