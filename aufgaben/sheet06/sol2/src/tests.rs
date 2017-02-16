use Vector2;

#[test]
fn constructors() {
    assert_eq!(Vector2::new(27, 42), Vector2 { x: 27, y: 42 });
    assert_eq!(Vector2::origin(), Vector2 { x: 0, y: 0 });
    assert_eq!(Vector2::unit_x(), Vector2 { x: 1, y: 0 });
    assert_eq!(Vector2::unit_y(), Vector2 { x: 0, y: 1 });
}

#[test]
fn operators() {
    let a = Vector2::new(3, 7);
    let b = Vector2::new(-2, 5);

    assert_eq!(a + b, Vector2::new(1, 12));
    assert_eq!(a * 2, Vector2::new(6, 14));
}
