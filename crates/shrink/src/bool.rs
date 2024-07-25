use crate::Classify;

impl Classify for bool {
    type Output = bool;

    fn classify(&self) -> Self::Output {
        *self
    }
}
