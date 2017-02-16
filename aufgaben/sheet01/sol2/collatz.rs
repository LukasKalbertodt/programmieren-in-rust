//! Aufgabe 1.2: Collatz

fn main() {
    // Hardcoding the input value is ugly, but it's fine for now.
    let mut n = 27;
    let mut count = 0;

    while n > 1 {
        // We use the feature "everything is an expression" here to slightly
        // shorten the code, although, when the task was due, this feature was
        // not yet introduced to the students.
        n = if n % 2 == 0 {
            n / 2
        } else {
            n * 3 + 1
        };
        count += 1;

        println!("{} -> {}", count, n);
    }
}
