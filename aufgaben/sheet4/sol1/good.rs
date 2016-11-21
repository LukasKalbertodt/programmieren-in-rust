/// Prints all happy primes between 1 and 20.
fn main() {
    for i in 1..21 {
        if is_happy_prime(i) {
            println!("{} is a happy prime!", i);
        }
    }
}

/// Determines whether the given number is a happy number AND a prime number.
fn is_happy_prime(n: u64) -> bool {
    is_happy(n) && is_prime(n)
}

/// Determines whether the given, positive number is a [happy number][1].
///
/// [1]: https://en.wikipedia.org/wiki/Happy_number
fn is_happy(mut number: u64) -> bool {
    // Either we end as "1" or in a cycle
    while number > 1 {
        number = {
            // Here we compute the sum of squares of all digits. The trick is that
            // the last digit is `number % 10` and that we can remove the last
            // digit by dividing number by 10.
            let mut sum = 0;
            while number > 0 {
                let digit = number % 10;
                sum += digit * digit;
                number /= 10;
            }

            sum
        };

        // We ended up in a cycle -> not happy
        if number == 4 {
            return false;
        }
    }

    true
}

/// Determines whether the given, positive, non-zero number is a prime number.
fn is_prime(n: u64) -> bool {
    if n == 1 {
        return false;
    }

    for divisor in 2..n {
        if n % divisor == 0 {
            return false;
        }
    }

    true
}
