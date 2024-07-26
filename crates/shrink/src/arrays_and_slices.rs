use crate::Classify;
use std::collections::{hash_map::DefaultHasher, BTreeSet};
use std::hash::{Hash, Hasher};

impl<T: Classify, const N: usize> Classify for [T; N]
where
    T::Output: Hash + Eq + Ord,
{
    type Output = u64;

    fn classify(&self) -> Self::Output {
        self.as_slice().classify()
    }
}

impl<T: Classify> Classify for [T]
where
    T::Output: Hash + Eq + Ord,
{
    type Output = u64;

    fn classify(&self) -> Self::Output {
        let mut hasher = DefaultHasher::new();

        // Hash the length of the slice, classifying all
        // items with two or more elements as the same
        match self.len() {
            0 => 0.hash(&mut hasher),
            1 => 1.hash(&mut hasher),
            _ => 2.hash(&mut hasher),
        }

        // Get a unique, sorted list of classifications
        let unique_classifications = self.iter().map(Classify::classify).collect::<BTreeSet<_>>();

        // Then hash those
        for classification in unique_classifications {
            classification.hash(&mut hasher);
        }

        hasher.finish()
    }
}

impl<T: Classify> Classify for Vec<T>
where
    T::Output: Hash + Eq + Ord,
{
    type Output = u64;

    fn classify(&self) -> Self::Output {
        self.as_slice().classify()
    }
}

#[cfg(test)]
mod tests {
    use crate::Classify;

    #[test]
    fn test_array_classification_length() {
        let arr1: [u8; 0] = [];
        let arr2 = [1u8];
        let arr3 = [1u8, 1u8];

        assert_ne!(arr1.classify(), arr2.classify());
        assert_ne!(arr1.classify(), arr3.classify());
        assert_ne!(arr2.classify(), arr3.classify());
    }

    #[test]
    fn test_array_classification() {
        let arr1 = [1u8, 2, 3];
        let arr2 = [1u8, 2, 3, 4];

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
        let arr2 = &[1u8, 2, 3, 4];

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
        let arr2 = &[1u8, 2];

        // Arrays with same unique classifications should have same classification
        // In effect, this means that items 2 and 3 are skipped in this case
        assert_eq!(arr1.classify(), arr2.classify());
    }
}
