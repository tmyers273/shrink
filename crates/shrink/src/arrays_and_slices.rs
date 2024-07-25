use crate::Classify;
use std::collections::{hash_map::DefaultHasher, BTreeSet};
use std::hash::{Hash, Hasher};

impl<T: Classify, const N: usize> Classify for [T; N]
where
    T::Output: Hash + Eq + Ord,
{
    type Output = u64;

    fn classify(&self) -> Self::Output {
        let unique_classifications: BTreeSet<_> = self.iter().map(|item| item.classify()).collect();

        let mut hasher = DefaultHasher::new();
        for classification in unique_classifications {
            classification.hash(&mut hasher);
        }
        hasher.finish()
    }
}

impl<T: Classify> Classify for [T]
where
    T::Output: Hash + Eq + Ord,
{
    type Output = u64;

    fn classify(&self) -> Self::Output {
        let unique_classifications: BTreeSet<_> = self.iter().map(|item| item.classify()).collect();

        let mut hasher = DefaultHasher::new();
        for classification in unique_classifications {
            classification.hash(&mut hasher);
        }
        hasher.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_classification() {
        let arr1 = [1u8, 2, 3];
        let arr2 = [1u8];

        // Arrays with same unique classifications should have same classification
        // In effect, this means that items 2 and 3 are skipped in this case
        assert_eq!(arr1.classify(), arr2.classify());

        // Different unique classifications should have different classifications
        let arr1 = [1u8, 2, 3];
        let arr2 = [0u8];
        assert_ne!(arr1.classify(), arr2.classify());
    }

    #[test]
    fn test_slice_classification() {
        let arr1 = &[1u8, 2, 3];
        let arr2 = &[1u8];

        // Arrays with same unique classifications should have same classification
        // In effect, this means that items 2 and 3 are skipped in this case
        assert_eq!(arr1.classify(), arr2.classify());

        // Different unique classifications should have different classifications
        let arr1 = &[1u8, 2, 3];
        let arr2 = &[0u8];
        assert_ne!(arr1.classify(), arr2.classify());
    }

    #[test]
    fn test_mixed_classification() {
        let arr1 = [1u8, 2, 3];
        let arr2 = &[1u8];

        // Arrays with same unique classifications should have same classification
        // In effect, this means that items 2 and 3 are skipped in this case
        assert_eq!(arr1.classify(), arr2.classify());
    }
}
