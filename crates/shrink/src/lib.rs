mod arrays_and_slices;
mod bool;
mod date_time;
mod float;
mod int;
mod naive_date;
mod naive_date_time;
mod option;
mod string;
mod tuple;

use std::hash::Hash;

pub trait Classify {
    type Output: PartialEq + Hash;

    fn classify(&self) -> Self::Output;
}

#[cfg(test)]
mod tests {
    use crate::int::IntClassification;
    use crate::Classify;
    use shrink_macros::Classify;

    #[derive(Classify)]
    struct Test {
        first: u8,
        second: u8,
    }

    #[test]
    fn classify_u8() {
        assert_eq!(0u8.classify(), IntClassification::Zero);
        assert_eq!(1u8.classify(), IntClassification::Positive);
        assert_eq!(u8::MAX.classify(), IntClassification::Max);
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
}
