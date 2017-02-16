//! Flip -- command line utility for randomness tools
//!
//! This is a simple application which offers several small tools related to
//! randomness. It uses the crates `clap` and `rand` for argument parsing and
//! random number generation, respectively. Example usage:
//!
//! ```
//! $ flip coin
//! heads
//! $ flip dice --sides=10
//! 7
//! $ flip choose --count=2 Peter Ursula Jürgen
//! ["Peter", "Jürgen"]
//! $ flip --times=3 coin
//! heads
//! heads
//! tails
//! ```

#[macro_use]
extern crate clap;
extern crate rand;

mod cmd;

use clap::{Arg, App, AppSettings};
use std::str::FromStr;
use std::error::Error;


/// Does most of the work: parses the command line args and executes the
/// subcommand.
fn parse_and_exec() -> Result<(), Box<Error>> {
    let matches = App::new("Flip")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Offers several tools related to randomness")
        .setting(AppSettings::SubcommandRequired)
        .arg(
            Arg::with_name("times")
                .short("t")
                .long("times")
                .takes_value(true)
                .validator(validate_parse::<u64>)
                .help("Executes the specified subcommand the given number of times")
        )
        .subcommand(cmd::coin::clap())
        .subcommand(cmd::dice::clap())
        .subcommand(cmd::choose::clap())
        .get_matches();

    // Check what subcommand has been executed and fetch the matching
    // function and sub-matches.
    let (cmd_fn, sub_matches): (fn(_) -> _, _) = match matches.subcommand() {
        ("coin",   sub_matches) => (cmd::coin::exec,   sub_matches.unwrap()),
        ("dice",   sub_matches) => (cmd::dice::exec,   sub_matches.unwrap()),
        ("choose", sub_matches) => (cmd::choose::exec, sub_matches.unwrap()),
        _ => unreachable!("We enabled 'SubcommandRequired' for clap!"),
    };

    // We validate the string above. Thus, this will be an `Err` iff the
    // argument wasn't specified. In this case it defaults to 1.
    let times = value_t!(matches, "times", u64).unwrap_or(1);

    // Execute the sub command the specified number of times, printing
    // every result
    for _ in 0..times {
        let s = cmd_fn(sub_matches)?;
        println!("{}", s);
    }

    Ok(())
}

fn main() {
    let res = parse_and_exec();
    if let Err(e) = res {
        println!("{}", e);
    }
}

/// Validator shorthand to check if a string is parsable as given type.
fn validate_parse<T>(s: String) -> Result<(), String>
    where T: FromStr,
          T::Err: ToString,
{
    s.parse::<T>()
        .map(|_| ())
        .map_err(|e| e.to_string())
}
