
macro_rules! hash_map {
    ( $($key:expr => $value:expr ,)* ) => {{
        let mut map = ::std::collections::HashMap::new();
        $( map.insert($key, $value); )*
        map
    }};
    ( $($key:expr => $value:expr),* ) => { hash_map!($($key => $value ,)*) };
}


fn main() {
    let ages = hash_map!{ "Sabine" => 26, "Peter" => 32 };
    println!("{:#?}", ages);
}
