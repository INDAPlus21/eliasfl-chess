//! Interactive chess game through terminal

use eliasfl_chess::*;
use std::env;
use std::io::{self, BufRead};

const ASCII_SYMBOLS: [(&str, &str); 12] = [
    ("♚", "K"),
    ("♔", "k"),
    ("♛", "Q"),
    ("♕", "q"),
    ("♜", "R"),
    ("♖", "r"),
    ("♝", "B"),
    ("♗", "b"),
    ("♞", "N"),
    ("♘", "n"),
    ("♟", "P"),
    ("♙", "p"),
];

fn rerender(game: &Game) {
    let mut gameboard = format!("{:?}", game);
    match env::args().nth(1) {
        Some(arg) if arg.contains("fancy") => {
            // Clear terminal screen
            println!("\x1B[2J\x1B[1;1H");
            // Print gameboard
            print!("{}", gameboard);
        }
        _ => {
            for (from, to) in ASCII_SYMBOLS {
                gameboard = gameboard.replace(from, to);
            }
            print!("{}", gameboard);
        }
    }
}

fn main() {
    let mut game = Game::new();

    let help = r#"
Possible commands:
Enter one coordinate (eg. "e2") to get possible moves
Enter two coordinates (eg. "e2 e3") to try to move piece
Type name of piece to be set as promotion piece for current player (eg. "knight")
Type "state" to get current game state
Type "color" to get which color's turn it is (also shown in upper left corner of board)
Type "restart" to restart the game
Type "help" to show this again
Type "q", "quit" or "exit" anytime to quit
Press enter to start game or update board
        "#
    .trim();
    println!("{}", help);

    let stdin = io::stdin();
    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        rerender(&game);

        match &line.to_lowercase()[..] {
            "q" | "quit" | "exit" | "\u{4}" => {
                break;
            }
            "help" | "?" => {
                println!("{}", help);
                continue;
            }
            "restart" => {
                game = Game::new();
                rerender(&game);
                continue;
            }
            "state" => {
                println!("{:?}", game.get_game_state());
                continue;
            }
            "color" => {
                println!("{:?}", game.active_color);
                continue;
            }
            "queen" | "rook" | "bishop" | "knight" => {
                if game.set_promotion(line).is_ok() {
                    println!("Promotion piece set to {:?}", game.promotion);
                }
                continue;
            }
            _ => {}
        }

        let positions: Vec<Option<Position>> = line
            .split_whitespace()
            .map(|f| Position::from_string(f.to_string()).ok())
            .collect();
        match &positions[..] {
            // Single position provided -> get moves
            [Some(x)] => {
                if let Some(moves) = game.get_possible_moves(x.to_string()) {
                    if !moves.is_empty() {
                        println!("Moves for {}: [{}]", x.to_string(), moves.join(", "));
                    } else {
                        println!("No valid moves for {}", x.to_string());
                    }
                } else {
                    println!("There is no piece on {}", x.to_string());
                }
            }
            // Two positions provided -> movie piece
            [Some(x), Some(y)] => match game.make_move(x.to_string(), y.to_string()) {
                Ok(_) => {
                    rerender(&game);
                    print!("Moved piece from {} to {}", x.to_string(), y.to_string());
                    if game.get_game_state() != GameState::InProgress {
                        print!(", new game state: {:?}", game.get_game_state());
                    }
                    println!();
                    continue;
                }
                Err(err) => println!("Illegal move: {}", err),
            },
            _ => {
                println!();
            }
        }
    }
}
