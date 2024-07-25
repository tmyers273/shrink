use crate::Classify;

impl<T: Classify> Classify for Option<T> {
    type Output = Option<T::Output>;

    fn classify(&self) -> Self::Output {
        self.as_ref().map(|x| x.classify())
    }
}

#[cfg(test)]
mod tests {
    use crate::Classify;

    #[test]
    fn test_option_classification() {
        let none: Option<i32> = None;
        let some: Option<i32> = Some(42);
        let some2: Option<i32> = Some(4);

        assert_ne!(none.classify(), some.classify());
        assert_eq!(some.classify(), some2.classify());
    }
}
