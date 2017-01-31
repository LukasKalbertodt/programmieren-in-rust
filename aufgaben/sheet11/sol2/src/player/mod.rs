use std::fmt;

use term_painter::{Color, Style, ToStyle};

use game::{Cell, GameState, Move};

pub mod random;
pub mod human;
pub mod smart;

/// A player that may be able to play tic tac toe.
pub trait Player {
    /// Returns a new player. This is useful to have in this trait, but we
    /// can't use it as virtual method on a trait object. Thus we manually
    /// exclude it from the set of virtual methods by requiring `Self: Sized`.
    fn new() -> Self where Self: Sized;

    /// Returns the kind of player in human readable form, e.g.
    /// "random player".
    fn player_kind(&self) -> &'static str;

    /// Returns the player's next move or `None` if the player gives up.
    ///
    /// Note that a valid move has to be returned. The only way to obtain an
    /// instance of `Move` is to use the `GameState::verify_move()`.
    fn next_move<'s>(&mut self, state: &'s GameState, role: Role)
        -> Option<Move>;
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Role {
    Circle,
    Cross,
}

impl Role {
    pub fn marker(&self) -> Cell {
        match *self {
            Role::Circle => Cell::Circle,
            Role::Cross => Cell::Cross,
        }
    }

    pub fn enemy(&self) -> Role {
        match *self {
            Role::Circle => Role::Cross,
            Role::Cross => Role::Circle,
        }
    }

    pub fn from_marker(c: Cell) -> Self {
        match c {
            Cell::Circle => Role::Circle,
            Cell::Cross => Role::Cross,
            _ => panic!("empty cell cannot be converted into role"),
        }
    }

    pub fn style(&self) -> Style {
        match *self {
            Role::Circle => Color::BrightGreen.to_style(),
            Role::Cross => Color::BrightBlue.to_style(),
        }
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            Role::Circle => "circle",
            Role::Cross => "cross",
        };
        self.style().paint(s).fmt(f)
    }
}
