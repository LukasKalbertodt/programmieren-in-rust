fn main() {
    3.times(|i| {
        println!("Ferris ate {} cookies", i);
    });
}

trait ChristmasExt<T> {
    fn times<F: FnMut(T)>(&self, f: F);
}

// We would somehow implement this for all integer types, but this is just
// a proof of concept, so one impl for `i32` is fine.
impl ChristmasExt<i32> for i32 {
    fn times<F: FnMut(i32)>(&self, mut f: F) {
        for i in 0..*self {
            f(i);
        }
    }
}
