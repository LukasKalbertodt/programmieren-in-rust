//! Contains all subcommands.
//!
//! Each subcommand's module has a `clap()` function which returns the clap
//! structure describing the subcommand's command line use. The `exec()`
//! function of each module executes the actual algorithm.
//!

pub mod dice;
pub mod coin;
pub mod choose;
