struct Fib {
    curr: u64,
    last: u64,
}

impl Fib {
    fn new() -> Self {
        Fib {
            curr: 1,
            last: 0,
        }
    }
}

impl Iterator for Fib {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        let new = self.last + self.curr;
        self.last = self.curr;
        self.curr = new;

        Some(self.last)
    }
}


fn main() {
    for i in Fib::new().take(20) {
        println!("{}", i);
    }
}
