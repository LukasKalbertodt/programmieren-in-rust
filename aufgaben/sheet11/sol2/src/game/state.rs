use std::ops::{Index, IndexMut};
use std::str::FromStr;

use term_painter::{Color, ToStyle};

use game::{Cell, CellId, Status};
use player::Role as PlayerRole;

/// Describes the state of the game.
///
/// *Note*: we don't derive/implement `Copy` although we could. This is one
/// example where not implementing `Copy` is useful: we don't want to
/// accidentally copy game state. Thus is shouldn't be easy to do so.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GameState {
    board: [Cell; 9],
}

impl GameState {
    /// Creates an empty board.
    pub fn new() -> Self {
        GameState {
            board: [Cell::Empty; 9],
        }
    }

    /// Pretty prints the board on the terminal. We don't use the `Display`
    /// trait, because I'm using colors and terminal formatting.
    pub fn print(&self) {
        println!("    a   b   c  ");
        println!("  +---+---+---+");

        for row in 0..3 {
            print!("{} ", row + 1);
            for col in 0..3 {
                let cell = self[CellId::new(row, col)];
                let symb = match cell {
                    Cell::Empty => Color::NotSet.paint(" "),
                    Cell::Circle => {
                        PlayerRole::from_marker(cell)
                            .style()
                            .bold()
                            .paint("◯")
                    }
                    Cell::Cross => {
                        PlayerRole::from_marker(cell)
                            .style()
                            .bold()
                            .paint("✗")
                    }
                };

                print!("| {} ", symb);

                // Trailing pipe
                if col == 2 {
                    print!("|");
                }
            }
            println!("");
            println!("  +---+---+---+");
        }
    }

    /// Verifies that a referenced cell is empty or returns None. This is
    /// the only way to change mutate this state.
    pub fn verify_move(&self, id: CellId) -> Option<Move> {
        match self[id] {
            Cell::Empty => Some(Move { id: id }),
            _ => None,
        }
    }

    /// Determines the status of the game (there is a winner, it has tied or
    /// it is still running)
    pub fn status(&self) -> Status {
        macro_rules! check {
            ($( [$a:expr, $b:expr, $c:expr] , )+) => {
                $(
                    let a = self[CellId::from_str($a).unwrap()];
                    let b = self[CellId::from_str($b).unwrap()];
                    let c = self[CellId::from_str($c).unwrap()];
                    if a != Cell::Empty && a == b && a == c {
                        return Status::Won(PlayerRole::from_marker(a));
                    }
                )*
            }
        }

        check![
            // rows
            ["a1", "a2", "a3"],
            ["b1", "b2", "b3"],
            ["c1", "c2", "c3"],

            // columns
            ["a1", "b1", "c1"],
            ["a2", "b2", "c2"],
            ["a3", "b3", "c3"],

            // diagonals
            ["a1", "b2", "c3"],
            ["a3", "b2", "c1"],
        ];

        if self.board.iter().all(|&c| c != Cell::Empty) {
            return Status::Tied;
        }

        Status::Running
    }
}

impl Index<CellId> for GameState {
    type Output = Cell;
    fn index(&self, idx: CellId) -> &Self::Output {
        &self.board[idx.index()]
    }
}

impl IndexMut<CellId> for GameState {
    fn index_mut(&mut self, idx: CellId) -> &mut Self::Output {
        assert_eq!(self.board[idx.index()], Cell::Empty);
        &mut self.board[idx.index()]
    }
}

/// Represents a valid move (referencing a cell which is empty). The only way
/// to modify the game state is by obtaining a `Move` and call
/// `set_cell()` on it.
#[derive(Debug, Eq, PartialEq)]
pub struct Move {
    id: CellId,
}

impl Move {
    pub fn id(&self) -> CellId {
        self.id
    }
}
