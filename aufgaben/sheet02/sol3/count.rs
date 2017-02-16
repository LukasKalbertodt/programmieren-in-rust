//! Aufgabe 2.3

fn main() {
    let s = "The fat cat had a friend! ♥";
    let count = count(s, 'a');
    println!("{}", count);
}


fn count(haystack: &str, needle: char) -> u64 {
    let mut count = 0;
    for c in haystack.chars() {
        if c == needle {
            count += 1;
        }
    }

    count
}
