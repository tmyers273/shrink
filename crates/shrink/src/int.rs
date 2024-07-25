#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IntClassification {
    Zero,
    Positive,
    Negative,
    Max,
    Min,
}

macro_rules! impl_classify_for_uint {
    ($($t:ty),+) => {
        $(
            impl crate::Classify for $t {
                type Output = IntClassification;

                fn classify(&self) -> IntClassification {
                    match *self {
                        0 => IntClassification::Zero,
                        x if x == <$t>::MAX => IntClassification::Max,
                        _ => IntClassification::Positive,
                    }
                }
            }

            impl crate::Classify for &$t {
                type Output = IntClassification;

                fn classify(&self) -> IntClassification {
                    match *self {
                        0 => IntClassification::Zero,
                        &x if x == <$t>::MAX => IntClassification::Max,
                        _ => IntClassification::Positive,
                    }
                }
            }
        )+
    }
}

macro_rules! impl_classify_for_int {
    ($($t:ty),+) => {
        $(
            impl crate::Classify for $t {
                type Output = IntClassification;

                fn classify(&self) -> IntClassification {
                    match *self {
                        0 => IntClassification::Zero,
                        x if x == <$t>::MAX => IntClassification::Max,
                        x if x == <$t>::MIN => IntClassification::Min,
                        x if x < 0 => IntClassification::Negative,
                        _ => IntClassification::Positive,
                    }
                }
            }

            impl crate::Classify for &$t {
                type Output = IntClassification;

                fn classify(&self) -> IntClassification {
                    match *self {
                        0 => IntClassification::Zero,
                        &x if x == <$t>::MAX => IntClassification::Max,
                        &x if x == <$t>::MIN => IntClassification::Min,
                        &x if x < 0 => IntClassification::Negative,
                        _ => IntClassification::Positive,
                    }
                }
            }
        )+
    }
}

impl_classify_for_uint!(u8, u16, u32, u64, usize);
impl_classify_for_int!(i8, i16, i32, i64, isize);
