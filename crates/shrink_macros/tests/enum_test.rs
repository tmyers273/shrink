use shrink::Classify;
use shrink_macros::ClassifyEnum;

#[derive(Debug, ClassifyEnum)]
enum MyEnum {
    First,
    Second,
    Third,
}

#[derive(Debug, ClassifyEnum)]
enum MyEnum2 {
    First(u64),
    Second(u64),
    Third(i64),
}

#[test]
fn enum_works() {
    assert_ne!(MyEnum::First.classify(), MyEnum::Second.classify());
    assert_ne!(MyEnum::First.classify(), MyEnum::Third.classify());
    assert_ne!(MyEnum::Second.classify(), MyEnum::Third.classify());

    assert_ne!(MyEnum2::First(1).classify(), MyEnum2::Second(1).classify());
    assert_ne!(MyEnum2::First(1).classify(), MyEnum2::Third(1).classify());
    assert_ne!(MyEnum2::Second(1).classify(), MyEnum2::Third(1).classify());

    assert_eq!(MyEnum2::First(1).classify(), MyEnum2::First(3).classify());
}
