extern crate term_painter;

mod db;
mod engine;
mod game;

use game::{choose_pokemon, fight};
use engine::Pokemon;

fn main() {
    // Let both players choose their Pokemon
    let model_red = choose_pokemon("Player Red");
    let poki_red = Pokemon::with_level(model_red, 5);

    let model_blue = choose_pokemon("Player Blue");
    let poki_blue = Pokemon::with_level(model_blue, 5);

    // Let both pokis fight!
    fight(poki_red, poki_blue);
}
