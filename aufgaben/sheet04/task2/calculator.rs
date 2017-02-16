
fn main() {
    loop {
        // Read input from the user and just do nothing when the input is empty
        let input = read_string();
        if input.is_empty() {
            continue;
        }

        // Debug output
        println!("{}", input);
    }
}


/// Reads a string from the user (with a nice prompt).
fn read_string() -> String {
    use std::io::Write;

    // Print prompt
    print!("calc > ");
    std::io::stdout().flush().unwrap();

    // Read line
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("something went horribly wrong...");

    // Discard trailing newline
    let new_len = buffer.trim_right().len();
    buffer.truncate(new_len);

    buffer
}
