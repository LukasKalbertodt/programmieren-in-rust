use std::ffi::OsString;
use std::{fmt, io};

const USAGE: &'static str = "
Partial `cp` implementation in Rust. Copies files.

Usage:
  mycp <source> <destination>
";

fn main() {
    // First, try to parse the cmd args (we require exactly two args!)
    let (from, to) = match parse_args() {
        Err(e) => {
            println!("{}", USAGE);
            print_and_exit(e);
        }
        Ok(t) => t,
    };

    // Next, just copy from the first file to the second
    if let Err(e) = copy(&from, &to) {
        print_and_exit(e);
    }
}

/// Prints the given error, the USAGE string and exits with a non-zero exit
/// status.
fn print_and_exit<E: fmt::Display>(e: E) -> ! {
    println!("error: {}", e);
    std::process::exit(1);
}

/// Tries to open file `from` for reading and file `to` for writing. Then
/// tries copy all contents from `from` to `to`. Returns first IO error.
fn copy(from: &str, to: &str) -> io::Result<()> {
    use std::fs::File;

    let mut from = match File::open(from) {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    let mut to = match File::create(to) {
        Ok(f) => f,
        Err(e) => return Err(e),
    };

    io::copy(&mut from, &mut to)
        // we are not interested in the number of bytes written
        .map(|_| ())
}

/// Tries to parse the command line args as two arguments.
fn parse_args() -> Result<(String, String), ArgsError> {
    use std::env;

    match env::args_os().count() {
        n if n > 4 => return Err(ArgsError::TooManyArgs(n - 1)),
        n if n < 3 => return Err(ArgsError::NotEnoughArgs),
        _ => {}
    }

    env::args_os()
        // only interested in the first two "real" arguments
        .skip(1)
        .take(2)
        // try to convert each OsString into a proper Utf8 String
        .map(|oss| oss.into_string())
        // collect to get the Result on the outside
        .collect::<Result<Vec<_>, _>>()
        // convert vector into tuple of Strings
        .map(|mut v| (v.remove(0), v.remove(0)))
        // wrap conversion error into our error type
        .map_err(|oss| ArgsError::NotUtf8(oss))
}

/// Stuff that can go wrong while parsing command line args.
enum ArgsError {
    NotEnoughArgs,
    TooManyArgs(usize),
    NotUtf8(OsString),
}

impl fmt::Display for ArgsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ArgsError::NotEnoughArgs => {
                write!(f, "not enough arguments given")
            }
            ArgsError::TooManyArgs(count) => {
                write!(
                    f,
                    "too many arguments were given (expected 2, found {})",
                    count
                )
            }
            ArgsError::NotUtf8(ref oss) => {
                write!(
                    f,
                    "given argument '{}' is not valid UTF8",
                    oss.to_string_lossy()
                )
            }
        }
    }
}
