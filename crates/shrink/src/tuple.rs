use crate::Classify;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

impl<T: Classify> Classify for (T,) {
    type Output = u64;

    fn classify(&self) -> Self::Output {
        let mut hasher = DefaultHasher::new();
        self.0.classify().hash(&mut hasher);
        hasher.finish()
    }
}

impl<T: Classify, U: Classify> Classify for (T, U) {
    type Output = u64;

    fn classify(&self) -> Self::Output {
        let mut hasher = DefaultHasher::new();
        self.0.classify().hash(&mut hasher);
        self.1.classify().hash(&mut hasher);
        hasher.finish()
    }
}

impl<T: Classify, U: Classify, V: Classify> Classify for (T, U, V) {
    type Output = u64;

    fn classify(&self) -> Self::Output {
        let mut hasher = DefaultHasher::new();
        self.0.classify().hash(&mut hasher);
        self.1.classify().hash(&mut hasher);
        self.2.classify().hash(&mut hasher);
        hasher.finish()
    }
}

impl<T: Classify, U: Classify, V: Classify, W: Classify> Classify for (T, U, V, W) {
    type Output = u64;

    fn classify(&self) -> Self::Output {
        let mut hasher = DefaultHasher::new();
        self.0.classify().hash(&mut hasher);
        self.1.classify().hash(&mut hasher);
        self.2.classify().hash(&mut hasher);
        self.3.classify().hash(&mut hasher);
        hasher.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tuple_classification() {
        let single = (42i32,).classify();
        let pair = (42, "hello").classify();
        let triple = (42, "hello", 3.14f32).classify();
        let quadruple = (42, "hello", 3.14f32, true).classify();

        // Each classification should be unique
        assert_ne!(single, pair);
        assert_ne!(single, triple);
        assert_ne!(single, quadruple);
        assert_ne!(pair, triple);
        assert_ne!(pair, quadruple);
        assert_ne!(triple, quadruple);

        // Same content should yield same classification
        assert_eq!((42,).classify(), (4,).classify());
        assert_eq!((42, "hello").classify(), (4, "hello world").classify());
    }
}
