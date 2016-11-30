use std::str::FromStr;
use std::fmt::Display;

fn main() {
    // let x = read::<usize>();
    let x: usize = read();
    println!("{}", x);

}

fn read<T>() -> T
    where T: FromStr,
          T::Err: Display
{
    use std::io;

    loop {
        let mut s = String::new();
        io::stdin()
            .read_line(&mut s)
            .expect("unexpected io error");

        match s.trim_right().parse() {
            Ok(value) => return value,
            Err(e) => println!("error: {}", e),
        }
    }
}
