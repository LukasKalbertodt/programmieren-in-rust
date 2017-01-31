extern crate rand;
extern crate term_painter;
extern crate boolinator;

mod game;
mod player;

use player::Player;
use term_painter::{ToStyle, Color};
use std::process;


fn main() {
    // We do argument parsing manually... which might not be the best idea,
    // because it's annoying to extend it. It's probably the best to use `clap`
    // or something like that from the very beginning.
    let args: Vec<_> = std::env::args().skip(1).take(3).collect();
    if args.len() != 2 {
        Color::Red.with(|| {
            println!("You have to specify the players as command line parameters!");
            println!("");
            println!("  ttt <cross-player> <circle-player>");
            println!("");
            println!("Player: one of 'human', 'random' or 'smart'.");
        });
    }

    let mut cross = get_player(&args[0]).unwrap_or_else(|| process::exit(1));
    let mut circle = get_player(&args[1]).unwrap_or_else(|| process::exit(1));

    // Pass a mutable reference to the box contents into the main function
    game::play(&mut *circle, &mut *cross);
}

/// This will return a trait object corresponding to the given player name
/// or `None` if the name is invalid. Because this function is returning a
/// trait object, we have to box it.
fn get_player(name: &str) -> Option<Box<Player>> {
    use player::human::HumanPlayer;
    use player::random::RandomPlayer;
    use player::smart::SmartPlayer;

    // Note that automatic conversions are used here: for example, the type
    // `Box<HumanPlayer>` is converted to `Box<Player>`. This is the
    // "type erasure" step.
    match name {
        "human" => Some(Box::new(HumanPlayer::new())),
        "random" => Some(Box::new(RandomPlayer::new())),
        "smart" => Some(Box::new(SmartPlayer::new())),
        invalid_name => {
            Color::Red.with(|| {
                println!(
                    "'{}' is not a valid player! Use 'human', 'random' or 'smart'.",
                    invalid_name,
                );
            });
            None
        }
    }
}
