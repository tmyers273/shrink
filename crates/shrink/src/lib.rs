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

    #[test]
    fn classify_u8() {
        assert_eq!(0u8.classify(), IntClassification::Zero);
        assert_eq!(1u8.classify(), IntClassification::Positive);
        assert_eq!(u8::MAX.classify(), IntClassification::Max);
    }
}
