//! Aufgabe 1.2: Primzahltest

fn main() {
    for i in 1..20 {
        print!("{}", i);
        if is_prime(i) {
            print!("*");
        }
        println!("");
    }
}

/// Tests if the given number `n` is prime.
fn is_prime(n: u64) -> bool {
    // Note: there are nicer *and* faster ways to do a primality test. But that
    // was not necessarily the point of this task.

    // 1 is special and yes: 1 is not a prime!
    if n <= 1 {
        return false;
    }

    // Iterator through all possible divisors
    for d in 2..n {
        if n % d == 0 {
            return false;
        }
    }

    true
}

#[test]
fn small_primes() {
    assert!(is_prime(2));
    assert!(is_prime(3));
    assert!(is_prime(5));
    assert!(is_prime(7));
}

#[test]
fn small_composites() {
    assert!(!is_prime(1));
    assert!(!is_prime(4));
    assert!(!is_prime(6));
    assert!(!is_prime(8));
    assert!(!is_prime(9));
}

#[test]
fn large_primes() {
    assert!(is_prime(1_300_769));
    assert!(is_prime(1_300_297));
    assert!(is_prime(7_367_287));
}

#[test]
fn large_composites() {
    assert!(!is_prime(908_209));
    assert!(!is_prime(3_073_009));
    assert!(!is_prime(4_897_369));
}
