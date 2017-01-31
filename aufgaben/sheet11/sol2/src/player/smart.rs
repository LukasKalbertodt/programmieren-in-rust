use game::{CellId, GameState, Move, Status};
use super::{Player, Role};
use super::random::RandomPlayer;
use boolinator::Boolinator;

pub struct SmartPlayer(RandomPlayer);

impl Player for SmartPlayer {
    fn new() -> Self {
        SmartPlayer(RandomPlayer)
    }

    fn player_kind(&self) -> &'static str {
        "smart player"
    }

    fn next_move<'s>(&mut self, state: &GameState, role: Role)
        -> Option<Move>
    {

        // Just a few useful things for later...
        let valid_moves = || (0..9)
            .map(|i| CellId::new(i / 3, i % 3))
            .filter_map(move |id| state.verify_move(id));

        let ca = CellId::new(0, 0);
        let cb = CellId::new(0, 2);
        let cc = CellId::new(2, 0);
        let cd = CellId::new(2, 2);

        // We use a few simple tactics here. The rules below are taken from
        // Wikipedia [1] (note: not all are implemented). This is not a perfect
        // player! You can beat it.
        //
        // [1]: https://en.wikipedia.org/wiki/Tic-tac-toe#Strategy
        None.or(
            // Rule 1: If we can win with the next move, we use that move.
            valid_moves().find(|valid_move| {
                let mut new_state = state.clone();
                new_state[valid_move.id()] = role.marker();
                new_state.status() == Status::Won(role)
            })

        ).or(
            // Rule 2: If the enemy could win with their next move, we
            // try to avoid that by marking that field
            valid_moves().find(|valid_move| {
                let mut new_state = state.clone();
                new_state[valid_move.id()] = role.enemy().marker();
                new_state.status() == Status::Won(role.enemy())
            })
        ).or(
            // Rule 5: If the field is empty, we choose a corner cell,
            // otherwise we will choose the center.
            if valid_moves().count() == 9 {
                state.verify_move(CellId::new(0, 0))
            } else {
                state.verify_move(CellId::new(1, 1))
            }
        ).or_else(|| {
            // Rule 6: If the enemy has a marker in one corner and the
            // opposite corner is free, put a marker there.
            let enemy = role.enemy().marker();

            None
                .or((state[ca] == enemy).and_option(state.verify_move(cd)))
                .or((state[cd] == enemy).and_option(state.verify_move(ca)))
                .or((state[cb] == enemy).and_option(state.verify_move(cc)))
                .or((state[cc] == enemy).and_option(state.verify_move(cb)))
        }).or(
            // Rule 7: If there is an empty corner, use it
            None
                .or(state.verify_move(ca))
                .or(state.verify_move(cb))
                .or(state.verify_move(cc))
                .or(state.verify_move(cd))
        ).or(
            // If none of those rules results in a valid move, we just choose
            // something at random.
            self.0.next_move(state, role)
        )
    }
}
