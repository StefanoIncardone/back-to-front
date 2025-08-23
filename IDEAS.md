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

## 0.1.? - More type-safe number base

```rust
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum Base {
    // Common bases
    Binary = 0b10,
    Octal = 0o10,
    #[default]
    Decimal = 10,
    Hexadecimal = 0x10,

    // Extended bases
    Three = 3,
    Four = 4,
    ...
    Duodecimal = 12,
    ...
    Vigesimal = 20,
    ...
    ThirtySix = 36,
}

// usefull aliases
impl Base {
    pub const Base2: Self = Self::Binary;
    pub const Base8: Self = Self::Octal;
    pub const Base10: Self = Self::Decimal;
    pub const Base16: Self = Self::Hexadecimal;
    pub const Base3: Self = Self::Three;
    pub const Base20: Self = Self::Vigesimal;

    pub const MIN: Self = Self::Base2;
    pub const MAX: Self = Self::Base36;
}

// Would allow for the removal of errors related to base min and max
#[deprecated(since = "0.1.1", note = "will use offset based checking and parsing")]
#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum AsciiDigitCustomBase {
    Ok         = AsciiDigit::Ok as u8,
    Underscore = AsciiDigit::Underscore as u8,
    Dot        = AsciiDigit::Dot as u8,
    OutOfRange = AsciiDigit::OutOfRange as u8,
    Other      = AsciiDigit::Other as u8,
    // cannot happen anymore
    BaseMin,
    BaseMax,
}

#[must_use]
#[inline]
pub const fn check_custom_offset(character: ascii, base: Base) -> OffsetCustomBase {
    // cannot happen anymore
    // if base < Base::MIN { return BASE_MIN; }
    // if base > Base::MAX { return BASE_MAX; }

    let offset = offset_alphanumerical!(character);
    let digit = character - offset;
    if digit >= base as u8 { return OUT_OF_RANGE; }
    return offset;
}

// could also create constants similar to the ones for the common bases, and maybe provide impl
// macros to reduce the boilerplate
```
