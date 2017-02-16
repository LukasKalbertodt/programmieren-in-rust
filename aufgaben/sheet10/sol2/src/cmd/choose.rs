//! This subcommand chooses items from a list.
//!

use clap::{Arg, ArgMatches, App, SubCommand};
use rand::{self, Rng};
use std::error::Error;

pub fn clap() -> App<'static, 'static> {
    SubCommand::with_name("choose")
        .about("Chooses one or more values from a list")
        .arg(
            Arg::with_name("count")
                .long("count")
                .short("c")
                .help("How many elements to choose")
                .takes_value(true)
                .validator(|s| {
                    s.parse::<u64>()
                        .map_err(|e| e.to_string())
                        .and_then(|n| {
                            if n == 1 {
                                Err("number is smaller than 2".into())
                            } else {
                                Ok(())
                            }
                        })
                })
        )
        .arg(
            Arg::with_name("ELEMENTS")
                .help("The list to choose one item from (comma separated)")
                .required(true)
                .multiple(true)
        )
}

pub fn exec(matches: &ArgMatches) -> Result<String, Box<Error>> {
    // Retrieve arguments
    let count = value_t!(matches, "count", u64).unwrap_or(1);
    let mut elements = values_t!(matches, "ELEMENTS", String).unwrap();

    // Sanity check arguments
    if count > elements.len() as u64 {
        return Err("'count' argument is greater than the number of given elements".into());
    }

    let mut rng = rand::thread_rng();

    // Choose a random valid index, remove the respective element from the
    // input vector and insert it into the out vector. Repeat specified number
    // of times.
    let mut out = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let idx = rng.gen_range(0, elements.len());
        out.push(elements.swap_remove(idx));
    }

    if count == 1 {
        Ok(out[0].to_string())
    } else {
        Ok(format!("{:?}", out))
    }
}
