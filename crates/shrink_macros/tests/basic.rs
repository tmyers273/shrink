use shrink::Classify;
use shrink_macros::Classify;

#[derive(Classify)]
struct Test {
    first: u8,
    second: u8,
}

#[test]
fn classify_test() {
    let mut t = Test {
        first: 1,
        second: 2,
    };
    let a = t.classify();
    t.first = 2;
    let b = t.classify();
    assert_eq!(a, b);

    t.first = 0;
    let c = t.classify();
    assert_ne!(a, c);
    assert_ne!(b, c);
}
