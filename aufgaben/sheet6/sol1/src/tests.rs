#[test]
fn clamp() {
    use clamp;

    assert_eq!(clamp(3,  5, 10), 5);
    assert_eq!(clamp(6,  5, 10), 6);
    assert_eq!(clamp(11, 5, 10), 10);

    assert_eq!(clamp(3.0,  5.0, 10.0), 5.0);
    assert_eq!(clamp(6.0,  5.0, 10.0), 6.0);
    assert_eq!(clamp(11.0, 5.0, 10.0), 10.0);
}

#[test]
fn sum_product() {
    use sum_product;

    assert_eq!(sum_product(3, 4), (7, 12));
    assert_eq!(sum_product(3.0, 4.0), (7.0, 12.0));
}

#[test]
fn bool_option() {
    use BoolOptionExt;

    assert_eq!(false.into_option(3), None);
    assert_eq!( true.into_option(3), Some(3));
}
