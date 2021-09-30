//! To run tests in order and print output: `cargo test -- --nocapture --test-threads=1`

#[cfg(test)]
mod tests {
    use std::iter::FromIterator;

    use crate::*;

    /// Test that game state is in progress after initialisation
    #[test]
    fn game_in_progress_after_init() {
        let game = Game::new();

        assert_eq!(game.get_game_state(), GameState::InProgress);
    }

    /// Test starting board
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

    /// Test that a valid position can be made from a string
    #[test]
    fn position_from_string() {
        let position1 = Position::from_string("d2".to_string());
        assert!(position1.is_ok());
        assert_eq!(position1.unwrap(), Position { file: 4, rank: 2 });

        let position2 = Position::from_string("k15".to_string());
        assert!(!position2.is_ok());
    }

    /// Test setting a promotion piece
    #[test]
    fn set_promotion_piece() {
        let mut game = Game::new();
        game.set_promotion("Knight".to_string()).unwrap();
        assert!(game.promotion.contains(&Piece::Knight(Color::White)));
        game.active_color = Color::Black;
        game.set_promotion("Rook".to_string()).unwrap();
        assert!(game.promotion.contains(&Piece::Rook(Color::Black)));
    }

    /// Test pawn promotion (and pawn diagonal capture)
    #[test]
    fn promotion() {
        let mut game = Game::new();
        game.set_promotion("knight".to_string()).unwrap();
        let moves = [
            ("a2", "a4"),
            ("b7", "b5"),
            ("a4", "b5"),
            ("b8", "a6"),
            ("b5", "b6"),
            ("a6", "b4"),
            ("b6", "b7"),
            ("b4", "d5"),
            ("b7", "b8"),
        ];
        for (from, to) in moves {
            game.make_move(from.to_string(), to.to_string()).unwrap();
        }
        assert_eq!(
            game.board.get(&Position { file: 2, rank: 8 }),
            Some(&Piece::Knight(Color::White))
        );
    }

    /// Test for possible moves on a piece
    #[test]
    fn possible_moves() {
        let game = Game::new();
        assert_eq!(game.get_possible_moves("e1".to_string()), Some(vec![]));
        // Test c2 white pawn
        assert_eq!(
            game._get_possible_moves(&Position { file: 3, rank: 2 })
                .unwrap(),
            HashSet::from_iter(
                [Position { file: 3, rank: 4 }, Position { file: 3, rank: 3 }]
                    .iter()
                    .cloned()
            )
        );
        // Test f7 black pawn
        assert_eq!(
            game._get_possible_moves(&Position { file: 6, rank: 7 })
                .unwrap(),
            HashSet::from_iter(
                [Position { file: 6, rank: 6 }, Position { file: 6, rank: 5 }]
                    .iter()
                    .cloned()
            )
        );
        // Test empty square
        assert!(game.get_possible_moves("c5".to_string()).is_none());
        // Test blocked king
        assert_eq!(game.get_possible_moves("e1".to_string()).unwrap().len(), 0);
    }

    /// Test if piece in the way
    #[test]
    fn piece_in_way() {
        let game = Game::new();
        let res = game._is_piece_in_way(
            &Piece::Bishop(Color::White),
            &Position { file: 6, rank: 1 },
            &Position { file: 8, rank: 3 },
        );
        assert!(res);
    }

    /// Tests that moves can be made (for each player)
    #[test]
    fn make_move() {
        let mut game = Game::new();

        assert!(game.make_move("a2".to_string(), "a4".to_string()).is_ok());
        assert!(game.make_move("g8".to_string(), "h6".to_string()).is_ok());
        assert!(game.make_move("b1".to_string(), "c3".to_string()).is_ok());
    }

    // Test checkmate with [fool's mate](https://www.chess.com/terms/fools-mate)
    #[test]
    fn fools_mate() {
        let mut game = Game::new();
        let moves = [("f2", "f3"), ("e7", "e5"), ("g2", "g4"), ("d8", "h4")];
        for (from, to) in moves {
            game.make_move(from.to_string(), to.to_string()).unwrap();
        }
        assert_eq!(game.get_game_state(), GameState::CheckMate);
    }

    // Test checkmate with [scholars's mate](https://www.chess.com/terms/fools-mate)
    #[test]
    fn scholars_mate() {
        let mut game = Game::new();
        let moves = [
            ("e2", "e4"),
            ("e7", "e5"),
            ("d1", "h5"),
            ("b8", "c6"),
            ("f1", "c4"),
            ("g8", "f6"),
            ("h5", "f7"),
        ];
        for (from, to) in moves {
            game.make_move(from.to_string(), to.to_string()).unwrap();
        }
        assert_eq!(game.get_game_state(), GameState::CheckMate);
    }
}
