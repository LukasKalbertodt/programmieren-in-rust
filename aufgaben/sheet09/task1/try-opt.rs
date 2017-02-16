fn main() {
    println!("consistent: {:?}", is_home_consistent());
    println!("dummy: {:?}", foo());
}

fn foo() -> Option<u8> {
    let a: u8 = try_opt!(Some(50));
    let b = try_opt!(a.checked_mul(6));

    Some(b / 2)
}

fn is_home_consistent() -> Option<bool> {
    use std::env;

    let home_dir = try_opt!(env::home_dir());
    let home_var = try_opt!(env::var_os("HOME"));

    Some(home_dir == home_var)
}
