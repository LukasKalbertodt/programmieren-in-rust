use term_painter::{Color, ToStyle};


mod state;
mod cell;

pub use self::cell::{Cell, CellId};
pub use self::state::{GameState, Move};
use player::{Player, Role as PlayerRole};


pub fn play<'a>(circle: &'a mut Player, cross: &'a mut Player) {
    use std::mem::swap;

    let mut state = GameState::new();
    let mut active = (cross, PlayerRole::Cross);
    let mut passive = (circle, PlayerRole::Circle);

    while state.status().is_running() {
        let cell_id = match active.0.next_move(&state, active.1) {
            Some(m) => m.id(),
            None => {
                println!(
                    "Player {} ({}) has given up! :(",
                    active.1,
                    active.0.player_kind()
                );
                break;
            }
        };
        state[cell_id] = active.1.marker();

        println!(
            "Player {} ({}) marked cell {}:",
            active.1,
            active.0.player_kind(),
            Color::BrightWhite.bold().paint(cell_id),
        );
        state.print();
        println!("");

        swap(&mut active, &mut passive);
    }

    if let Status::Won(winner) = state.status() {
        println!(
            "Player {} ({}) won! :)",
            winner,
            passive.0.player_kind(),
        );
    } else {
        // Tie
        println!("The game tied :'(");
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    Running,
    Tied,
    Won(PlayerRole),
}

impl Status {
    pub fn is_running(&self) -> bool {
        *self == Status::Running
    }
}
