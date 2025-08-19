# Feature Ideas

>[!WARNING]
> no feature is final, modifications can happen at any moment

## 0.1.2 - Index iterator

```rust
trait IndexType {}
impl IndexType for u8 {}
impl IndexType for i8 {}
impl IndexType for ... {}

struct Indices<I: IndexType> {
    start: I,
    end: I,
}

pub struct Indices<I: IndexType> {
    pub start: I,
    pub end: I,
}

impl<I: IndexType> Iterator for Indices<I> {
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            let index = self.start;
            self.start += 1;
            return Some(index);
        } else {
            return None;
        }
    }
}
```

## 0.1.? - More ergonomic constants

```rust
pub mod binary {
    use super::*;

    pub const DIGIT_ASCII_START: ascii = b'0';
    pub const DIGIT_ASCII_END: ascii = b'1';
    pub const DIGIT_ASCII: AsciiRange<ascii> = AsciiRange::new(
        DIGIT_ASCII_START,
        DIGIT_ASCII_END,
    );
}

pub use binary::{
    DIGIT_ASCII_START as BINARY_DIGIT_ASCII_START,
    DIGIT_ASCII_END as BINARY_DIGIT_ASCII_END,
    DIGIT_ASCII as BINARY_DIGIT_ASCII,
};

const _: ascii = BINARY_DIGIT_ASCII_START;
const _: ascii = binary::DIGIT_ASCII_START;

// instead of

impl Base {
    pub const BINARY_DIGIT_ASCII_START: ascii = b'0';
    pub const BINARY_DIGIT_ASCII_END: ascii = b'1';
    pub const BINARY_DIGIT_ASCII: AsciiRange<ascii> = AsciiRange::new(
        Self::BINARY_DIGIT_ASCII_START,
        Self::BINARY_DIGIT_ASCII_END,
    );
}

const _: ascii = Base::BINARY_DIGIT_ASCII_START;
```
