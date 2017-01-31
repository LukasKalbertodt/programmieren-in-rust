use std::io::{self, Write};

use term_painter::{Color, ToStyle};

use game::{CellId, GameState, Move};
use super::{Player, Role};

pub struct HumanPlayer;

impl Player for HumanPlayer {
    fn new() -> Self {
        HumanPlayer
    }

    fn player_kind(&self) -> &'static str {
        "human"
    }

    fn next_move<'s>(&mut self, state: &'s GameState, _role: Role)
        -> Option<Move>
    {
        loop {
            print!("Put mark on: ");
            io::stdout().flush().expect("failed flushing stdout");

            let mut s = String::new();
            io::stdin()
                .read_line(&mut s)
                .expect("failed reading from stdin");

            let id: CellId = match s.trim().parse() {
                Ok(id) => id,
                Err(e) => {
                    Color::Red.with(|| println!("invalid input: {}", e));
                    continue;
                }
            };

            match state.verify_move(id) {
                Some(valid_move) => return Some(valid_move),
                None => {
                    Color::Red.with(|| {
                        println!("invalid input: field {} is already occupied", id);
                    });
                    continue;
                }
            }
        }
    }
}
