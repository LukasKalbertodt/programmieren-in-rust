//! This subcommand flips a coin, returning either 'heads' or 'tails'
//!

use clap::{ArgMatches, App, SubCommand};
use rand::{self, Rng};
use std::error::Error;


pub fn clap() -> App<'static, 'static> {
    SubCommand::with_name("coin")
        .about("Flips a coin, returning either 'heads' or 'tails'")

}

pub fn exec(_: &ArgMatches) -> Result<String, Box<Error>> {
    let out = if rand::thread_rng().gen() {
        "heads"
    } else {
        "tails"
    };

    Ok(out.into())
}
