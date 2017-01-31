use rand::{thread_rng, Rng};

use game::{CellId, GameState, Move};
use super::{Player, Role};

pub struct RandomPlayer;

impl Player for RandomPlayer {
    fn new() -> Self {
        RandomPlayer
    }

    fn player_kind(&self) -> &'static str {
        "random player"
    }

    fn next_move<'s>(&mut self, state: &'s GameState, _role: Role)
        -> Option<Move>
    {
        // If we haven't found a valid move after 1000 attempts, we give up
        for _ in 0..1000 {
            let mut rng = thread_rng();
            let row = rng.gen_range(0, 3);
            let col = rng.gen_range(0, 3);
            let id = CellId::new(row, col);

            if let Some(valid_move) = state.verify_move(id) {
                return Some(valid_move);
            }
        }

        None
    }
}
