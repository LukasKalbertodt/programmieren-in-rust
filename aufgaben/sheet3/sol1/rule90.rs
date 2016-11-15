//! Task 3.1: Rule 90

const NUM_ITERATIONS: u64 = 20;

fn main() {
    /// Helper function to pretty print the automaton state
    fn print_state(state: &[bool]) {
        for &cell in state {
            print!("{}", if cell { "██" } else { "  " });
        }
        println!("");
    }

    // Read initial state and print it
    let mut old_state = read_input();
    print_state(&old_state);

    // Simulate automaton for 20 steps
    for _ in 0..NUM_ITERATIONS {
        let new_state = next_step(&old_state);
        print_state(&new_state);

        // The new state is now the old one
        old_state = new_state;
    }
}

/// Reads a valid initial configuration for our automaton from the terminal.
fn read_input() -> Vec<bool> {
    // This tries to read a string from the terminal, checks whether it's
    // valid (only contains 1's and 0's). If the user fails to input a correct
    // string, this routine will ask again until the user finally manages to
    // give us a correct string.
    //
    // You don't need to understand this routine yet; that's why I've written
    // it already ;-)
    //
    // You only need to use the `input` variable (of type `String`). You can
    // also assume that it only contains '0' and '1' chars.
    let input = {
        let mut buffer = String::new();

        loop {
            println!("Please give me the initial configuration (a string of '0' and '1'!):");
            buffer.clear();

            // `read_line` returns an error if the input isn't valid UTF8 or if
            // a strange IO error occured. We just panic in that case...
            std::io::stdin()
                .read_line(&mut buffer)
                .expect("something went seriously wrong :O");

            if buffer.trim().chars().all(|c| c == '1' || c == '0') {
                break;
            }
        }

        buffer.trim().to_string()
    };

    // TODO: Task 1a)

    // We can reduce the number of reallocations, because we know exactly how
    // long our vector will be.
    let mut out = Vec::with_capacity(input.len());

    // We iterate through the whole string, pushing the corresponding boolean
    // value.
    for c in input.chars() {
        out.push(c == '1');
    }

    out
}

/// Given the state of the automaton at time n, this function will return the
/// automaton's state at time n+1.
fn next_step(old: &[bool]) -> Vec<bool> {
    // Note: this function signature is not optimal in terms of heap
    // allocations. The signature alone already implies that this function
    // always allocates a new vector. We could instead also pass a
    // `new: &mut Vec<bool>` argument to reuse a buffer.

    // We know the final length, so we can reduce the number of reallocations
    // to one :)
    let mut out = Vec::with_capacity(old.len());

    for i in 0..old.len() {
        // We need to handle the first and last cell.
        let right_index = (i + 1) % old.len();
        let left_index = (i + old.len() - 1) % old.len();

        // The rules of Rule90 are actually just an XOR between the neighbor
        // cells.
        out.push(old[left_index] ^ old[right_index]);
    }

    out
}

#[test]
fn rule90_rules() {
    assert_eq!(next_step(&[false, false, false]), vec![false, false, false]);
    assert_eq!(next_step(&[ true, false, false]), vec![false,  true,  true]);
    assert_eq!(next_step(&[ true,  true, false]), vec![ true,  true, false]);
    assert_eq!(next_step(&[ true,  true,  true]), vec![false, false, false]);
}
