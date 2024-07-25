use crate::Classify;

#[derive(Debug, Copy, Clone, PartialEq, Hash)]
pub enum StringClassification {
    /// An empty string
    Empty,
    /// A string containing only whitespace characters
    Whitespace,
    /// A non-empty string containing at least one non-whitespace character
    NonEmpty,
}

impl Classify for String {
    type Output = StringClassification;

    fn classify(&self) -> Self::Output {
        if self.is_empty() {
            StringClassification::Empty
        } else if self.chars().all(char::is_whitespace) {
            StringClassification::Whitespace
        } else {
            StringClassification::NonEmpty
        }
    }
}

// Implement for &str as well
impl Classify for &str {
    type Output = StringClassification;

    fn classify(&self) -> Self::Output {
        if self.is_empty() {
            StringClassification::Empty
        } else if self.chars().all(char::is_whitespace) {
            StringClassification::Whitespace
        } else {
            StringClassification::NonEmpty
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_classification() {
        // Test empty strings
        assert_eq!("".classify(), StringClassification::Empty);
        assert_eq!(String::new().classify(), StringClassification::Empty);

        // Test whitespace strings
        assert_eq!(" ".classify(), StringClassification::Whitespace);
        assert_eq!("\t\n  ".classify(), StringClassification::Whitespace);
        assert_eq!("   ".classify(), StringClassification::Whitespace);

        // Test non-empty strings
        assert_eq!("a".classify(), StringClassification::NonEmpty);
        assert_eq!("Hello, World!".classify(), StringClassification::NonEmpty);
        assert_eq!("123".classify(), StringClassification::NonEmpty);
        assert_eq!(" a ".classify(), StringClassification::NonEmpty);
        assert_eq!("\tHello\n".classify(), StringClassification::NonEmpty);

        // Test with String type
        assert_eq!(
            String::from("Hello").classify(),
            StringClassification::NonEmpty
        );
        assert_eq!(
            String::from(" ").classify(),
            StringClassification::Whitespace
        );

        // Test with non-ASCII characters
        assert_eq!("こんにちは".classify(), StringClassification::NonEmpty);
        assert_eq!("　".classify(), StringClassification::Whitespace); // Full-width space
    }
}
