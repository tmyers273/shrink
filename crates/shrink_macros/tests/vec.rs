use shrink::Classify;

#[derive(shrink_macros::Classify)]
struct Test {
    a: Vec<u8>,
    b: Vec<u8>,
}

#[test]
fn can_classify_macro_vec() {
    let a = Test {
        a: vec![1, 2, 3],
        b: vec![1, 2, 3],
    };

    let b = Test {
        a: vec![3, 4, 5, 6],
        b: vec![4, 5, 6, 7],
    };

    assert_eq!(a.classify(), b.classify());
}
