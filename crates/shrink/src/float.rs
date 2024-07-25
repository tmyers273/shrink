#[derive(Debug, Copy, Clone, PartialEq, Hash)]
pub enum FloatClassification {
    /// Represents both positive and negative zero.
    /// In floating-point arithmetic, +0.0 and -0.0 are distinct values,
    /// but they compare as equal.
    Zero,

    /// Any positive, normal floating-point number.
    /// This includes numbers from the smallest positive normal value
    /// up to (but not including) positive infinity.
    Positive,

    /// Any negative, normal floating-point number.
    /// This includes numbers from the largest negative normal value
    /// down to (but not including) negative infinity.
    Negative,

    /// Represents positive infinity.
    /// This is the result of operations like 1.0 / 0.0 or overflow.
    PositiveInfinity,

    /// Represents negative infinity.
    /// This is the result of operations like -1.0 / 0.0 or underflow.
    NegativeInfinity,

    /// Not a Number (NaN).
    /// This represents undefined or unrepresentable results,
    /// such as 0.0 / 0.0 or sqrt(-1.0).
    NaN,

    /// Subnormal numbers (also known as denormal numbers).
    /// These are very small numbers close to zero that have reduced precision.
    /// They fill the underflow gap around zero in the floating-point representation.
    Subnormal,
}

macro_rules! impl_classify_for_float {
    ($($t:ty),+) => {
        $(
            impl crate::Classify for $t {
                type Output = FloatClassification;

                fn classify(&self) -> FloatClassification {
                    if self.is_nan() {
                        FloatClassification::NaN
                    } else if *self == 0.0 {
                        FloatClassification::Zero
                    } else if self.is_infinite() {
                        if *self > 0.0 {
                            FloatClassification::PositiveInfinity
                        } else {
                            FloatClassification::NegativeInfinity
                        }
                    } else if self.is_subnormal() {
                        FloatClassification::Subnormal
                    } else if *self > 0.0 {
                        FloatClassification::Positive
                    } else {
                        FloatClassification::Negative
                    }
                }
            }
        )+
    }
}

// Implement for float types
impl_classify_for_float!(f32, f64);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Classify;
    use std::fmt::Debug;

    #[test]
    fn test_float_classification() {
        use FloatClassification::*;

        // Helper function to test both f32 and f64
        fn test_both<T: Classify<Output = FloatClassification> + Copy + Debug>(
            value: T,
            expected: FloatClassification,
            type_name: &str,
        ) {
            assert_eq!(
                value.classify(),
                expected,
                "Failed for {type_name} value: {:?}",
                value
            );
        }

        // Test zero
        test_both(0.0f32, Zero, "f32");
        test_both(-0.0f32, Zero, "f32");
        test_both(0.0f64, Zero, "f64");
        test_both(-0.0f64, Zero, "f64");

        // Test positive numbers
        test_both(1.0f32, Positive, "f32");
        test_both(f32::MAX, Positive, "f32");
        test_both(1.0f64, Positive, "f64");
        test_both(f64::MAX, Positive, "f64");

        // Test negative numbers
        test_both(-1.0f32, Negative, "f32");
        test_both(f32::MIN, Negative, "f32");
        test_both(-1.0f64, Negative, "f64");
        test_both(f64::MIN, Negative, "f64");

        // Test infinities
        test_both(f32::INFINITY, PositiveInfinity, "f32");
        test_both(f32::NEG_INFINITY, NegativeInfinity, "f32");
        test_both(f64::INFINITY, PositiveInfinity, "f64");
        test_both(f64::NEG_INFINITY, NegativeInfinity, "f64");

        // Test NaN
        test_both(f32::NAN, NaN, "f32");
        test_both(f64::NAN, NaN, "f64");

        // Test subnormal numbers
        test_both(f32::MIN_POSITIVE / 2.0, Subnormal, "f32");
        test_both(-f32::MIN_POSITIVE / 2.0, Subnormal, "f32");
        test_both(f64::MIN_POSITIVE / 2.0, Subnormal, "f64");
        test_both(-f64::MIN_POSITIVE / 2.0, Subnormal, "f64");

        // Test smallest normal numbers (these should be Positive/Negative, not Subnormal)
        test_both(f32::MIN_POSITIVE, Positive, "f32");
        test_both(-f32::MIN_POSITIVE, Negative, "f32");
        test_both(f64::MIN_POSITIVE, Positive, "f64");
        test_both(-f64::MIN_POSITIVE, Negative, "f64");
    }
}
