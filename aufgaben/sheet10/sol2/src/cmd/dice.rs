//! This subcommand rolls a dice with a configurable number of sides, returning
//! the number of the rolled dice.
//!

use clap::{Arg, ArgMatches, App, SubCommand};
use rand::{self, Rng};
use std::error::Error;

pub fn clap() -> App<'static, 'static> {
    SubCommand::with_name("dice")
        .about("Rolls a dice")
        .arg(
            Arg::with_name("sides")
                .long("sides")
                .help("The number of sides of the dice")
                .takes_value(true)
                .validator(|s| {
                    s.parse::<u64>()
                        .map_err(|e| e.to_string())
                        .and_then(|n| {
                            if n <= 1 {
                                Err("number is smaller than 2".into())
                            } else {
                                Ok(())
                            }
                        })
                })
        )
}

pub fn exec(matches: &ArgMatches) -> Result<String, Box<Error>> {
    let sides = value_t!(matches, "sides", u64).unwrap_or(6);

    let num = rand::thread_rng().gen_range(1, sides);
    Ok(num.to_string())
}
