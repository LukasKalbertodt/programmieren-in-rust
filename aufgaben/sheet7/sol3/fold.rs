fn main() {
    // product
    let fac_5 = (1..5).fold(1, |acc, x| acc * x);
    assert_eq!(fac_5, (1..5).product());
    println!("5! = {}", fac_5);

    // max
    let max = vec![3, 1, 4, 1, 5, 9, 2, 6]
        .into_iter()
        .fold(0, |acc, x| std::cmp::max(acc, x));
    assert_eq!(max, 9);
    println!("max: {}", max);

    // all
    let all_even = (1..9)
        .fold(true, |acc, x| acc && (x % 2 == 0));
    assert!(!all_even);
    println!("all_even: {}", all_even);
}
