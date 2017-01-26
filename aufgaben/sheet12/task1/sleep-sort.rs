
fn main() {
    let mut arr = [83, 12, 13, 35, 91, 71, 75, 58, 26, 38, 2, 23, 10];
    sleep_sort(&mut arr);
    assert_eq!(arr, [2, 10, 12, 13, 23, 26, 35, 38, 58, 71, 75, 83, 91]);
}

fn sleep_sort(arr: &mut [u64]) {}
