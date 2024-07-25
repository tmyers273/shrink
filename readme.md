# Shrink

This crate provides a single trait `Classify`, which is used to shrink a list of items.

# What is shrinking?

This crate will use the term "shrink" to mean the process of reducing a list of items to
a smaller list of items, with the intention of reducing the list to the smallest number
of items that represent all meaningfully different items.

So, what is a meaningful difference? This crate defines it for unsigned integers as

```rust
pub enum UnsignedIntClassification {
    Zero,
    Max,
    /// Any other value that is not zero or the maximum value
    Positive,
}
```

and for strings as

```rust
pub enum StringClassification {
    /// An empty string
    Empty,
    /// A string containing only whitespace characters
    Whitespace,
    /// A non-empty string containing at least one non-whitespace character
    NonEmpty,
}
```

So if we have the following struct:

```rust
struct Item {
    first: u8,
    second: String,
}
```

The `first` field could be classified into 3 types. The `second` field could be classified into
3 types. So, we have 3 * 3 = 9 possible classifications.

This means that, no matter how many items we have, we can reduce the list to at most 9 meaningfully
different items.

# Usage

```rust
#[derive(Classify)]
struct Item {
    first: u8,
    second: String,
    third: SomeEnum,
}

#[derive(ClassifyEnum)]
enum SomeEnum {
    A,
    B,
    C,
}

fn shrink(items: Vec<Item>) -> Vec<Item> {
    let hm = FxHashMap::default();

    for item in items {
        let key = item.classify();
        hm.entry(key).or_insert(item);
    }

    hm.into_iter().map(|(_, item)| item).collect()
}
```

# Differences

- unsigned ints
    - zero
    - max
    - positive
- signed ints
    - zero
    - max
    - min
    - positive
    - negative
- floats
    - zero
    - positive
    - negative
    - `+` inf
    - `-` int
    - nan
    - subnormal
- strings
    - empty
    - whitespace
    - non-empty
- bools
    - true
    - false
- naive date / naive date time / date time
    - probably error - for years < 1970 and > 2050
    - default - for the epoch
    - normal - for all other dates

## Tuples

Tuples of up to 4 items are supported. Each item needs to implement
the Classify trait.

## Options

Options effectively add another classification of None to the list of
classifications available on the underlying type T. Requires T to be
Classify.

## Arrays, Slices, and Vecs

These are supported for any type that implements Classify.

Duplicate classifications will be removed. So for example

```rust
let t = vec![1u8, 2, 3];
```

will be classified as

```rust
vec![UnsignedIntClassification::Positive];
```

The remaining classifications will be sorted.

```rust
// -> [Positive, Positive, Zero] 
// -> dedupes to [Positive, Zero]
// -> sorts to [Zero, Positive]
let a = vec![1u8, 2, 0];
// -> [Zero, Positive, Positive] 
// -> dedupes to [Zero, Positive] 
// -> sorts to [Zero, Positive]
let b = vec![0, 100u8, 200];

// Allowing these to be equal
assert_eq!(a.classify(), b.classify());
```

## Enums

Enums can be derived with the `#[derive(ClassifyEnum)]` proc macro. All
variants will be considered unique classifications.

If the variant has named or unnamed fields, they must implement Classify.
