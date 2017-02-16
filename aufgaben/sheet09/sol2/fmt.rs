struct LoveMachine<T> {
    inner: T,
}

impl<T> LoveMachine<T> {
    pub fn new(t: T) -> Self {
        LoveMachine {
            inner: t,
        }
    }
}

macro_rules! impl_fmt {
    ($fmt_trait:ident, $fmt_str:expr) => {
        impl<T> ::std::fmt::$fmt_trait for LoveMachine<T>
            where T: ::std::fmt::$fmt_trait
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, concat!("♥", $fmt_str, "♥"), self.inner)
            }
        }
    }
}

impl_fmt!(Display,  "{}");
impl_fmt!(Debug,    "{:?}");
impl_fmt!(Octal,    "{:o}");
impl_fmt!(LowerHex, "{:x}");
impl_fmt!(UpperHex, "{:X}");
impl_fmt!(Binary,   "{:b}");
impl_fmt!(LowerExp, "{:e}");
impl_fmt!(UpperExp, "{:E}");

fn main() {
    let lm = LoveMachine::new(27);
    println!("Display:   {}", lm);
    println!("Debug:     {:?}", lm);
    println!("Octal:     {:o}", lm);
    println!("LowerHex:  {:x}", lm);
    println!("UpperHex:  {:X}", lm);
    println!("Binary:    {:b}", lm);
}
