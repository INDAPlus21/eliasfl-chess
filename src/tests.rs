#[cfg(test)]
mod tests {
    use crate::*;

    /// Check that game state is in progress after initialisation
    #[test]
    fn game_in_progress_after_init() {
        let game = Game::new();

        assert_eq!(game.state, GameState::InProgress);
    }

    /// Check starting board
    #[test]
    fn valid_starting_board() {
        use Color::*;
        use Piece::*;
        let game = Game::new();

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
            (Position { file: 1, rank: 7 }, Pawn(Black)),
            (Position { file: 2, rank: 7 }, Pawn(Black)),
            (Position { file: 3, rank: 7 }, Pawn(Black)),
            (Position { file: 4, rank: 7 }, Pawn(Black)),
            (Position { file: 5, rank: 7 }, Pawn(Black)),
            (Position { file: 6, rank: 7 }, Pawn(Black)),
            (Position { file: 7, rank: 7 }, Pawn(Black)),
            (Position { file: 8, rank: 7 }, Pawn(Black)),
        ]
        .iter()
        .cloned()
        .collect();
        assert_eq!(game.board, starting_board);
    }

    /// Check that a valid position can be made from a string
    #[test]
    fn position_from_string() {
        let position1 = Position::from_string("d2".to_string());
        assert!(position1.is_ok());
        assert_eq!(position1.unwrap(), Position { file: 4, rank: 2 });

        let position2 = Position::from_string("k15".to_string());
        assert!(!position2.is_ok());
    }

    /// Check for possible moves on a piece
    #[test]
    fn possible_moves() {
        let game = Game::new();
        assert_eq!(game.get_possible_moves("e1".to_string()), Some(vec![]));
        println!("\n{:?}", game.get_possible_moves("e2".to_string()));

        assert!(game.get_possible_moves("c5".to_string()).is_none());
    }

    /// Check that moves can be made
    #[test]
    fn make_move() {
        let game = Game::new();

        println!("{:?}", game);
        // game.make_move("a2".to_string(), "a4".to_string());

        assert_eq!(game.state, GameState::InProgress);
    }
}
