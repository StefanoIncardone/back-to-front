#![allow(deprecated)]
#![expect(clippy::pub_use)]
// TODO(stefano): find a way of only allowing importing of macros from this module and not from the
    // root module: `use crate::foo_macro` should not work, while `use crate::digit::foo_macro`
    // should be the only way of importing these macros
// TODO(stefano): add proper deprecation messages
    // - remove duplication of constants

use crate::{ascii, utf32};
use core::ops::RangeInclusive;

// TODO(stefano): add number-like traits
pub trait Ascii {}
impl Ascii for ascii {}
impl Ascii for utf32 {}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct AsciiRange<I: Ascii> {
    pub start: I,
    pub end: I,
}

impl<I: Ascii> AsciiRange<I> {
    #[must_use]
    #[inline(always)]
    pub const fn new(start: I, end: I) -> Self {
        return Self { start, end };
    }
}

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum Base {
    Binary = 0b10,
    Octal = 0o10,
    #[default]
    Decimal = 10,
    Hexadecimal = 0x10,
}

impl Base {
    pub const MIN: u8 = 2;
    pub const MAX: u8 = Self::Decimal as u8 + b'Z' - b'A' + 1;
}

// IDEA(stefano): bring ranges constants out to global scope or to own modules

// binary
impl Base {
    pub const BINARY_DIGIT_ASCII_START: ascii = b'0';
    pub const BINARY_DIGIT_ASCII_END: ascii = b'1';
    pub const BINARY_DIGIT_ASCII: AsciiRange<ascii> = AsciiRange::new(
        Self::BINARY_DIGIT_ASCII_START,
        Self::BINARY_DIGIT_ASCII_END,
    );

    pub const BINARY_DIGIT_OUT_OF_RANGE_ASCII_START: ascii = Self::BINARY_DIGIT_ASCII_END + 1;
    pub const BINARY_DIGIT_OUT_OF_RANGE_ASCII_END: ascii = b'9';
    pub const BINARY_DIGIT_OUT_OF_RANGE_ASCII: AsciiRange<ascii> = AsciiRange::new(
        Self::BINARY_DIGIT_OUT_OF_RANGE_ASCII_START,
        Self::BINARY_DIGIT_OUT_OF_RANGE_ASCII_END,
    );

    pub const BINARY_UPPERCASE_OUT_OF_RANGE_ASCII_START: ascii = b'A';
    pub const BINARY_UPPERCASE_OUT_OF_RANGE_ASCII_END: ascii = b'Z';
    pub const BINARY_UPPERCASE_OUT_OF_RANGE_ASCII: AsciiRange<ascii> = AsciiRange::new(
        Self::BINARY_UPPERCASE_OUT_OF_RANGE_ASCII_START,
        Self::BINARY_UPPERCASE_OUT_OF_RANGE_ASCII_END,
    );

    pub const BINARY_LOWERCASE_OUT_OF_RANGE_ASCII_START: ascii = b'a';
    pub const BINARY_LOWERCASE_OUT_OF_RANGE_ASCII_END: ascii = b'z';
    pub const BINARY_LOWERCASE_OUT_OF_RANGE_ASCII: AsciiRange<ascii> = AsciiRange::new(
        Self::BINARY_LOWERCASE_OUT_OF_RANGE_ASCII_START,
        Self::BINARY_LOWERCASE_OUT_OF_RANGE_ASCII_END,
    );

    pub const BINARY_DIGIT_ASCII_OPS: RangeInclusive<ascii> = RangeInclusive::new(
        Self::BINARY_DIGIT_ASCII_START,
        Self::BINARY_DIGIT_ASCII_END,
    );

    pub const BINARY_DIGIT_OUT_OF_RANGE_ASCII_OPS: RangeInclusive<ascii> = RangeInclusive::new(
        Self::BINARY_DIGIT_OUT_OF_RANGE_ASCII_START,
        Self::BINARY_DIGIT_OUT_OF_RANGE_ASCII_END,
    );

    pub const BINARY_UPPERCASE_OUT_OF_RANGE_ASCII_OPS: RangeInclusive<ascii> = RangeInclusive::new(
        Self::BINARY_UPPERCASE_OUT_OF_RANGE_ASCII_START,
        Self::BINARY_UPPERCASE_OUT_OF_RANGE_ASCII_END,
    );

    pub const BINARY_LOWERCASE_OUT_OF_RANGE_ASCII_OPS: RangeInclusive<ascii> = RangeInclusive::new(
        Self::BINARY_LOWERCASE_OUT_OF_RANGE_ASCII_START,
        Self::BINARY_LOWERCASE_OUT_OF_RANGE_ASCII_END,
    );

    pub const BINARY_DIGIT_START: utf32 = Self::BINARY_DIGIT_ASCII_START as utf32;
    pub const BINARY_DIGIT_END: utf32 = Self::BINARY_DIGIT_ASCII_END as utf32;
    pub const BINARY_DIGIT: AsciiRange<utf32> = AsciiRange::new(
        Self::BINARY_DIGIT_START,
        Self::BINARY_DIGIT_END,
    );

    pub const BINARY_DIGIT_OUT_OF_RANGE_START: utf32 = Self::BINARY_DIGIT_OUT_OF_RANGE_ASCII_START as utf32;
    pub const BINARY_DIGIT_OUT_OF_RANGE_END: utf32 = Self::BINARY_DIGIT_OUT_OF_RANGE_ASCII_END as utf32;
    pub const BINARY_DIGIT_OUT_OF_RANGE: AsciiRange<utf32> = AsciiRange::new(
        Self::BINARY_DIGIT_OUT_OF_RANGE_START,
        Self::BINARY_DIGIT_OUT_OF_RANGE_END,
    );

    pub const BINARY_UPPERCASE_OUT_OF_RANGE_START: utf32 = Self::BINARY_UPPERCASE_OUT_OF_RANGE_ASCII_START as utf32;
    pub const BINARY_UPPERCASE_OUT_OF_RANGE_END: utf32 = Self::BINARY_UPPERCASE_OUT_OF_RANGE_ASCII_END as utf32;
    pub const BINARY_UPPERCASE_OUT_OF_RANGE: AsciiRange<utf32> = AsciiRange::new(
        Self::BINARY_UPPERCASE_OUT_OF_RANGE_START,
        Self::BINARY_UPPERCASE_OUT_OF_RANGE_END,
    );

    pub const BINARY_LOWERCASE_OUT_OF_RANGE_START: utf32 = Self::BINARY_LOWERCASE_OUT_OF_RANGE_ASCII_START as utf32;
    pub const BINARY_LOWERCASE_OUT_OF_RANGE_END: utf32 = Self::BINARY_LOWERCASE_OUT_OF_RANGE_ASCII_END as utf32;
    pub const BINARY_LOWERCASE_OUT_OF_RANGE: AsciiRange<utf32> = AsciiRange::new(
        Self::BINARY_LOWERCASE_OUT_OF_RANGE_START,
        Self::BINARY_LOWERCASE_OUT_OF_RANGE_END,
    );

    pub const BINARY_DIGIT_OPS: RangeInclusive<utf32> = RangeInclusive::new(
        Self::BINARY_DIGIT_START,
        Self::BINARY_DIGIT_END,
    );

    pub const BINARY_DIGIT_OUT_OF_RANGE_OPS: RangeInclusive<utf32> = RangeInclusive::new(
        Self::BINARY_DIGIT_OUT_OF_RANGE_START,
        Self::BINARY_DIGIT_OUT_OF_RANGE_END,
    );

    pub const BINARY_UPPERCASE_OUT_OF_RANGE_OPS: RangeInclusive<utf32> = RangeInclusive::new(
        Self::BINARY_UPPERCASE_OUT_OF_RANGE_START,
        Self::BINARY_UPPERCASE_OUT_OF_RANGE_END,
    );

    pub const BINARY_LOWERCASE_OUT_OF_RANGE_OPS: RangeInclusive<utf32> = RangeInclusive::new(
        Self::BINARY_LOWERCASE_OUT_OF_RANGE_START,
        Self::BINARY_LOWERCASE_OUT_OF_RANGE_END,
    );

    #[deprecated(since = "0.1.1-dev", note = "will be renamed to `BINARY_DIGIT_ASCII`")]
    pub const BINARY_RANGE_ASCII: AsciiRange<ascii> = Self::BINARY_DIGIT_ASCII;
    #[deprecated(since = "0.1.1-dev", note = "will be renamed to `BINARY_DIGIT`")]
    pub const BINARY_RANGE:       AsciiRange<utf32> = Self::BINARY_DIGIT;

    pub const BINARY_RANGES_ASCII: [AsciiRange<ascii>; 1] = [Self::BINARY_DIGIT_ASCII];
    pub const BINARY_RANGES:       [AsciiRange<utf32>; 1] = [Self::BINARY_RANGE];

    #[deprecated(since = "0.1.1-dev", note = "will be renamed to `BINARY_DIGIT_ASCII_OPS`")]
    pub const BINARY_RANGE_ASCII_OPS: RangeInclusive<ascii> = Self::BINARY_DIGIT_ASCII_OPS;
    #[deprecated(since = "0.1.1-dev", note = "will be renamed to `BINARY_DIGIT_OPS`")]
    pub const BINARY_RANGE_OPS:       RangeInclusive<utf32> = Self::BINARY_DIGIT_OPS;

    pub const BINARY_RANGES_ASCII_OPS: [RangeInclusive<ascii>; 1] = [Self::BINARY_DIGIT_ASCII_OPS];
    pub const BINARY_RANGES_OPS:       [RangeInclusive<utf32>; 1] = [Self::BINARY_DIGIT_OPS];

    pub const BINARY_ASCII_OFFSET: u8 = Self::BINARY_DIGIT_ASCII_START;
}

// octal
#[rustfmt::skip]
impl Base {
    pub const OCTAL_DIGIT_ASCII_START: ascii = b'0';
    pub const OCTAL_DIGIT_ASCII_END: ascii = b'7';
    pub const OCTAL_DIGIT_ASCII: AsciiRange<ascii> = AsciiRange::new(
        Self::OCTAL_DIGIT_ASCII_START,
        Self::OCTAL_DIGIT_ASCII_END,
    );

    pub const OCTAL_DIGIT_OUT_OF_RANGE_ASCII_START: ascii = Self::OCTAL_DIGIT_ASCII_END + 1;
    pub const OCTAL_DIGIT_OUT_OF_RANGE_ASCII_END: ascii = b'9';
    pub const OCTAL_DIGIT_OUT_OF_RANGE_ASCII: AsciiRange<ascii> = AsciiRange::new(
        Self::OCTAL_DIGIT_OUT_OF_RANGE_ASCII_START,
        Self::OCTAL_DIGIT_OUT_OF_RANGE_ASCII_END,
    );

    pub const OCTAL_UPPERCASE_OUT_OF_RANGE_ASCII_START: ascii = b'A';
    pub const OCTAL_UPPERCASE_OUT_OF_RANGE_ASCII_END: ascii = b'Z';
    pub const OCTAL_UPPERCASE_OUT_OF_RANGE_ASCII: AsciiRange<ascii> = AsciiRange::new(
        Self::OCTAL_UPPERCASE_OUT_OF_RANGE_ASCII_START,
        Self::OCTAL_UPPERCASE_OUT_OF_RANGE_ASCII_END,
    );

    pub const OCTAL_LOWERCASE_OUT_OF_RANGE_ASCII_START: ascii = b'a';
    pub const OCTAL_LOWERCASE_OUT_OF_RANGE_ASCII_END: ascii = b'z';
    pub const OCTAL_LOWERCASE_OUT_OF_RANGE_ASCII: AsciiRange<ascii> = AsciiRange::new(
        Self::OCTAL_LOWERCASE_OUT_OF_RANGE_ASCII_START,
        Self::OCTAL_LOWERCASE_OUT_OF_RANGE_ASCII_END,
    );

    pub const OCTAL_DIGIT_ASCII_OPS: RangeInclusive<ascii> = RangeInclusive::new(
        Self::OCTAL_DIGIT_ASCII_START,
        Self::OCTAL_DIGIT_ASCII_END,
    );

    pub const OCTAL_DIGIT_OUT_OF_RANGE_ASCII_OPS: RangeInclusive<ascii> = RangeInclusive::new(
        Self::OCTAL_DIGIT_OUT_OF_RANGE_ASCII_START,
        Self::OCTAL_DIGIT_OUT_OF_RANGE_ASCII_END,
    );

    pub const OCTAL_UPPERCASE_OUT_OF_RANGE_ASCII_OPS: RangeInclusive<ascii> = RangeInclusive::new(
        Self::OCTAL_UPPERCASE_OUT_OF_RANGE_ASCII_START,
        Self::OCTAL_UPPERCASE_OUT_OF_RANGE_ASCII_END,
    );

    pub const OCTAL_LOWERCASE_OUT_OF_RANGE_ASCII_OPS: RangeInclusive<ascii> = RangeInclusive::new(
        Self::OCTAL_LOWERCASE_OUT_OF_RANGE_ASCII_START,
        Self::OCTAL_LOWERCASE_OUT_OF_RANGE_ASCII_END,
    );

    pub const OCTAL_DIGIT_START: utf32 = Self::OCTAL_DIGIT_ASCII_START as utf32;
    pub const OCTAL_DIGIT_END: utf32 = Self::OCTAL_DIGIT_ASCII_END as utf32;
    pub const OCTAL_DIGIT: AsciiRange<utf32> = AsciiRange::new(
        Self::OCTAL_DIGIT_START,
        Self::OCTAL_DIGIT_END,
    );

    pub const OCTAL_DIGIT_OUT_OF_RANGE_START: utf32 = Self::OCTAL_DIGIT_OUT_OF_RANGE_ASCII_START as utf32;
    pub const OCTAL_DIGIT_OUT_OF_RANGE_END: utf32 = Self::OCTAL_DIGIT_OUT_OF_RANGE_ASCII_END as utf32;
    pub const OCTAL_DIGIT_OUT_OF_RANGE: AsciiRange<utf32> = AsciiRange::new(
        Self::OCTAL_DIGIT_OUT_OF_RANGE_START,
        Self::OCTAL_DIGIT_OUT_OF_RANGE_END,
    );

    pub const OCTAL_UPPERCASE_OUT_OF_RANGE_START: utf32 = Self::OCTAL_UPPERCASE_OUT_OF_RANGE_ASCII_START as utf32;
    pub const OCTAL_UPPERCASE_OUT_OF_RANGE_END: utf32 = Self::OCTAL_UPPERCASE_OUT_OF_RANGE_ASCII_END as utf32;
    pub const OCTAL_UPPERCASE_OUT_OF_RANGE: AsciiRange<utf32> = AsciiRange::new(
        Self::OCTAL_UPPERCASE_OUT_OF_RANGE_START,
        Self::OCTAL_UPPERCASE_OUT_OF_RANGE_END,
    );

    pub const OCTAL_LOWERCASE_OUT_OF_RANGE_START: utf32 = Self::OCTAL_LOWERCASE_OUT_OF_RANGE_ASCII_START as utf32;
    pub const OCTAL_LOWERCASE_OUT_OF_RANGE_END: utf32 = Self::OCTAL_LOWERCASE_OUT_OF_RANGE_ASCII_END as utf32;
    pub const OCTAL_LOWERCASE_OUT_OF_RANGE: AsciiRange<utf32> = AsciiRange::new(
        Self::OCTAL_LOWERCASE_OUT_OF_RANGE_START,
        Self::OCTAL_LOWERCASE_OUT_OF_RANGE_END,
    );

    pub const OCTAL_DIGIT_OPS: RangeInclusive<utf32> = RangeInclusive::new(
        Self::OCTAL_DIGIT_START,
        Self::OCTAL_DIGIT_END,
    );

    pub const OCTAL_DIGIT_OUT_OF_RANGE_OPS: RangeInclusive<utf32> = RangeInclusive::new(
        Self::OCTAL_DIGIT_OUT_OF_RANGE_START,
        Self::OCTAL_DIGIT_OUT_OF_RANGE_END,
    );

    pub const OCTAL_UPPERCASE_OUT_OF_RANGE_OPS: RangeInclusive<utf32> = RangeInclusive::new(
        Self::OCTAL_UPPERCASE_OUT_OF_RANGE_START,
        Self::OCTAL_UPPERCASE_OUT_OF_RANGE_END,
    );

    pub const OCTAL_LOWERCASE_OUT_OF_RANGE_OPS: RangeInclusive<utf32> = RangeInclusive::new(
        Self::OCTAL_LOWERCASE_OUT_OF_RANGE_START,
        Self::OCTAL_LOWERCASE_OUT_OF_RANGE_END,
    );

    #[deprecated(since = "0.1.1-dev", note = "will be renamed to `OCTAL_DIGIT_ASCII`")]
    pub const OCTAL_RANGE_ASCII: AsciiRange<ascii> = Self::OCTAL_DIGIT_ASCII;
    #[deprecated(since = "0.1.1-dev", note = "will be renamed to `OCTAL_DIGIT`")]
    pub const OCTAL_RANGE:       AsciiRange<utf32> = Self::OCTAL_DIGIT;

    pub const OCTAL_RANGES_ASCII: [AsciiRange<ascii>; 1] = [Self::OCTAL_DIGIT_ASCII];
    pub const OCTAL_RANGES:       [AsciiRange<utf32>; 1] = [Self::OCTAL_RANGE];

    #[deprecated(since = "0.1.1-dev", note = "will be renamed to `OCTAL_DIGIT_ASCII_OPS`")]
    pub const OCTAL_RANGE_ASCII_OPS: RangeInclusive<ascii> = Self::OCTAL_DIGIT_ASCII_OPS;
    #[deprecated(since = "0.1.1-dev", note = "will be renamed to `OCTAL_DIGIT_OPS`")]
    pub const OCTAL_RANGE_OPS:       RangeInclusive<utf32> = Self::OCTAL_DIGIT_OPS;

    pub const OCTAL_RANGES_ASCII_OPS: [RangeInclusive<ascii>; 1] = [Self::OCTAL_DIGIT_ASCII_OPS];
    pub const OCTAL_RANGES_OPS:       [RangeInclusive<utf32>; 1] = [Self::OCTAL_DIGIT_OPS];

    pub const OCTAL_ASCII_OFFSET: u8 = Self::OCTAL_DIGIT_ASCII_START;
}

// decimal
#[rustfmt::skip]
impl Base {
    pub const DECIMAL_DIGIT_ASCII_START: ascii = b'0';
    pub const DECIMAL_DIGIT_ASCII_END: ascii = b'9';
    pub const DECIMAL_DIGIT_ASCII: AsciiRange<ascii> = AsciiRange::new(
        Self::DECIMAL_DIGIT_ASCII_START,
        Self::DECIMAL_DIGIT_ASCII_END,
    );

    pub const DECIMAL_UPPERCASE_OUT_OF_RANGE_ASCII_START: ascii = b'A';
    pub const DECIMAL_UPPERCASE_OUT_OF_RANGE_ASCII_END: ascii = b'Z';
    pub const DECIMAL_UPPERCASE_OUT_OF_RANGE_ASCII: AsciiRange<ascii> = AsciiRange::new(
        Self::DECIMAL_UPPERCASE_OUT_OF_RANGE_ASCII_START,
        Self::DECIMAL_UPPERCASE_OUT_OF_RANGE_ASCII_END,
    );

    pub const DECIMAL_LOWERCASE_OUT_OF_RANGE_ASCII_START: ascii = b'a';
    pub const DECIMAL_LOWERCASE_OUT_OF_RANGE_ASCII_END: ascii = b'z';
    pub const DECIMAL_LOWERCASE_OUT_OF_RANGE_ASCII: AsciiRange<ascii> = AsciiRange::new(
        Self::DECIMAL_LOWERCASE_OUT_OF_RANGE_ASCII_START,
        Self::DECIMAL_LOWERCASE_OUT_OF_RANGE_ASCII_END,
    );

    pub const DECIMAL_DIGIT_ASCII_OPS: RangeInclusive<ascii> = RangeInclusive::new(
        Self::DECIMAL_DIGIT_ASCII_START,
        Self::DECIMAL_DIGIT_ASCII_END,
    );

    pub const DECIMAL_UPPERCASE_OUT_OF_RANGE_ASCII_OPS: RangeInclusive<ascii> = RangeInclusive::new(
        Self::DECIMAL_UPPERCASE_OUT_OF_RANGE_ASCII_START,
        Self::DECIMAL_UPPERCASE_OUT_OF_RANGE_ASCII_END,
    );

    pub const DECIMAL_LOWERCASE_OUT_OF_RANGE_ASCII_OPS: RangeInclusive<ascii> = RangeInclusive::new(
        Self::DECIMAL_LOWERCASE_OUT_OF_RANGE_ASCII_START,
        Self::DECIMAL_LOWERCASE_OUT_OF_RANGE_ASCII_END,
    );

    pub const DECIMAL_DIGIT_START: utf32 = Self::DECIMAL_DIGIT_ASCII_START as utf32;
    pub const DECIMAL_DIGIT_END: utf32 = Self::DECIMAL_DIGIT_ASCII_END as utf32;
    pub const DECIMAL_DIGIT: AsciiRange<utf32> = AsciiRange::new(
        Self::DECIMAL_DIGIT_START,
        Self::DECIMAL_DIGIT_END,
    );

    pub const DECIMAL_UPPERCASE_OUT_OF_RANGE_START: utf32 = Self::DECIMAL_UPPERCASE_OUT_OF_RANGE_ASCII_START as utf32;
    pub const DECIMAL_UPPERCASE_OUT_OF_RANGE_END: utf32 = Self::DECIMAL_UPPERCASE_OUT_OF_RANGE_ASCII_END as utf32;
    pub const DECIMAL_UPPERCASE_OUT_OF_RANGE: AsciiRange<utf32> = AsciiRange::new(
        Self::DECIMAL_UPPERCASE_OUT_OF_RANGE_START,
        Self::DECIMAL_UPPERCASE_OUT_OF_RANGE_END,
    );

    pub const DECIMAL_LOWERCASE_OUT_OF_RANGE_START: utf32 = Self::DECIMAL_LOWERCASE_OUT_OF_RANGE_ASCII_START as utf32;
    pub const DECIMAL_LOWERCASE_OUT_OF_RANGE_END: utf32 = Self::DECIMAL_LOWERCASE_OUT_OF_RANGE_ASCII_END as utf32;
    pub const DECIMAL_LOWERCASE_OUT_OF_RANGE: AsciiRange<utf32> = AsciiRange::new(
        Self::DECIMAL_LOWERCASE_OUT_OF_RANGE_START,
        Self::DECIMAL_LOWERCASE_OUT_OF_RANGE_END,
    );

    pub const DECIMAL_DIGIT_OPS: RangeInclusive<utf32> = RangeInclusive::new(
        Self::DECIMAL_DIGIT_START,
        Self::DECIMAL_DIGIT_END,
    );

    pub const DECIMAL_UPPERCASE_OUT_OF_RANGE_OPS: RangeInclusive<utf32> = RangeInclusive::new(
        Self::DECIMAL_UPPERCASE_OUT_OF_RANGE_START,
        Self::DECIMAL_UPPERCASE_OUT_OF_RANGE_END,
    );

    pub const DECIMAL_LOWERCASE_OUT_OF_RANGE_OPS: RangeInclusive<utf32> = RangeInclusive::new(
        Self::DECIMAL_LOWERCASE_OUT_OF_RANGE_START,
        Self::DECIMAL_LOWERCASE_OUT_OF_RANGE_END,
    );

    #[deprecated(since = "0.1.1-dev", note = "will be renamed to `DECIMAL_DIGIT_ASCII`")]
    pub const DECIMAL_RANGE_ASCII: AsciiRange<ascii> = Self::DECIMAL_DIGIT_ASCII;
    #[deprecated(since = "0.1.1-dev", note = "will be renamed to `DECIMAL_DIGIT`")]
    pub const DECIMAL_RANGE:       AsciiRange<utf32> = Self::DECIMAL_DIGIT;

    pub const DECIMAL_RANGES_ASCII: [AsciiRange<ascii>; 1] = [Self::DECIMAL_DIGIT_ASCII];
    pub const DECIMAL_RANGES:       [AsciiRange<utf32>; 1] = [Self::DECIMAL_RANGE];

    #[deprecated(since = "0.1.1-dev", note = "will be renamed to `DECIMAL_DIGIT_ASCII_OPS`")]
    pub const DECIMAL_RANGE_ASCII_OPS: RangeInclusive<ascii> = Self::DECIMAL_DIGIT_ASCII_OPS;
    #[deprecated(since = "0.1.1-dev", note = "will be renamed to `DECIMAL_DIGIT_OPS`")]
    pub const DECIMAL_RANGE_OPS:       RangeInclusive<utf32> = Self::DECIMAL_DIGIT_OPS;

    pub const DECIMAL_RANGES_ASCII_OPS: [RangeInclusive<ascii>; 1] = [Self::DECIMAL_DIGIT_ASCII_OPS];
    pub const DECIMAL_RANGES_OPS:       [RangeInclusive<utf32>; 1] = [Self::DECIMAL_DIGIT_OPS];

    pub const DECIMAL_ASCII_OFFSET: u8 = Self::DECIMAL_DIGIT_ASCII_START;
}

// hexadecimal
#[rustfmt::skip]
impl Base {
    pub const HEXADECIMAL_DIGIT_ASCII_START: ascii = b'0';
    pub const HEXADECIMAL_DIGIT_ASCII_END: ascii = b'9';
    pub const HEXADECIMAL_DIGIT_ASCII: AsciiRange<ascii> = AsciiRange::new(
        Self::HEXADECIMAL_DIGIT_ASCII_START,
        Self::HEXADECIMAL_DIGIT_ASCII_END,
    );

    pub const HEXADECIMAL_UPPERCASE_ASCII_START: ascii = b'A';
    pub const HEXADECIMAL_UPPERCASE_ASCII_END: ascii = b'F';
    pub const HEXADECIMAL_UPPERCASE_ASCII: AsciiRange<ascii> = AsciiRange::new(
        Self::HEXADECIMAL_UPPERCASE_ASCII_START,
        Self::HEXADECIMAL_UPPERCASE_ASCII_END,
    );

    pub const HEXADECIMAL_LOWERCASE_ASCII_START: ascii = b'a';
    pub const HEXADECIMAL_LOWERCASE_ASCII_END: ascii = b'f';
    pub const HEXADECIMAL_LOWERCASE_ASCII: AsciiRange<ascii> = AsciiRange::new(
        Self::HEXADECIMAL_LOWERCASE_ASCII_START,
        Self::HEXADECIMAL_LOWERCASE_ASCII_END,
    );

    pub const HEXADECIMAL_UPPERCASE_OUT_OF_RANGE_ASCII_START: ascii = Self::HEXADECIMAL_UPPERCASE_ASCII_END + 1;
    pub const HEXADECIMAL_UPPERCASE_OUT_OF_RANGE_ASCII_END: ascii = b'Z';
    pub const HEXADECIMAL_UPPERCASE_OUT_OF_RANGE_ASCII: AsciiRange<ascii> = AsciiRange::new(
        Self::HEXADECIMAL_UPPERCASE_OUT_OF_RANGE_ASCII_START,
        Self::HEXADECIMAL_UPPERCASE_OUT_OF_RANGE_ASCII_END,
    );

    pub const HEXADECIMAL_LOWERCASE_OUT_OF_RANGE_ASCII_START: ascii = Self::HEXADECIMAL_LOWERCASE_ASCII_END + 1;
    pub const HEXADECIMAL_LOWERCASE_OUT_OF_RANGE_ASCII_END: ascii = b'z';
    pub const HEXADECIMAL_LOWERCASE_OUT_OF_RANGE_ASCII: AsciiRange<ascii> = AsciiRange::new(
        Self::HEXADECIMAL_LOWERCASE_OUT_OF_RANGE_ASCII_START,
        Self::HEXADECIMAL_LOWERCASE_OUT_OF_RANGE_ASCII_END,
    );

    pub const HEXADECIMAL_DIGIT_ASCII_OPS: RangeInclusive<ascii> = RangeInclusive::new(
        Self::HEXADECIMAL_DIGIT_ASCII_START,
        Self::HEXADECIMAL_DIGIT_ASCII_END,
    );

    pub const HEXADECIMAL_UPPERCASE_ASCII_OPS: RangeInclusive<ascii> = RangeInclusive::new(
        Self::HEXADECIMAL_UPPERCASE_ASCII_START,
        Self::HEXADECIMAL_UPPERCASE_ASCII_END,
    );

    pub const HEXADECIMAL_LOWERCASE_ASCII_OPS: RangeInclusive<ascii> = RangeInclusive::new(
        Self::HEXADECIMAL_LOWERCASE_ASCII_START,
        Self::HEXADECIMAL_LOWERCASE_ASCII_END,
    );

    pub const HEXADECIMAL_UPPERCASE_OUT_OF_RANGE_ASCII_OPS: RangeInclusive<ascii> = RangeInclusive::new(
        Self::HEXADECIMAL_UPPERCASE_OUT_OF_RANGE_ASCII_START,
        Self::HEXADECIMAL_UPPERCASE_OUT_OF_RANGE_ASCII_END,
    );

    pub const HEXADECIMAL_LOWERCASE_OUT_OF_RANGE_ASCII_OPS: RangeInclusive<ascii> = RangeInclusive::new(
        Self::HEXADECIMAL_LOWERCASE_OUT_OF_RANGE_ASCII_START,
        Self::HEXADECIMAL_LOWERCASE_OUT_OF_RANGE_ASCII_END,
    );

    pub const HEXADECIMAL_DIGIT_START: utf32 = Self::HEXADECIMAL_DIGIT_ASCII_START as utf32;
    pub const HEXADECIMAL_DIGIT_END: utf32 = Self::HEXADECIMAL_DIGIT_ASCII_END as utf32;
    pub const HEXADECIMAL_DIGIT: AsciiRange<utf32> = AsciiRange::new(
        Self::HEXADECIMAL_DIGIT_START,
        Self::HEXADECIMAL_DIGIT_END,
    );

    pub const HEXADECIMAL_UPPERCASE_START: utf32 = Self::HEXADECIMAL_UPPERCASE_ASCII_START as utf32;
    pub const HEXADECIMAL_UPPERCASE_END: utf32 = Self::HEXADECIMAL_UPPERCASE_ASCII_END as utf32;
    pub const HEXADECIMAL_UPPERCASE: AsciiRange<utf32> = AsciiRange::new(
        Self::HEXADECIMAL_UPPERCASE_START,
        Self::HEXADECIMAL_UPPERCASE_END,
    );

    pub const HEXADECIMAL_LOWERCASE_START: utf32 = Self::HEXADECIMAL_LOWERCASE_ASCII_START as utf32;
    pub const HEXADECIMAL_LOWERCASE_END: utf32 = Self::HEXADECIMAL_LOWERCASE_ASCII_END as utf32;
    pub const HEXADECIMAL_LOWERCASE: AsciiRange<utf32> = AsciiRange::new(
        Self::HEXADECIMAL_LOWERCASE_START,
        Self::HEXADECIMAL_LOWERCASE_END,
    );

    pub const HEXADECIMAL_UPPERCASE_OUT_OF_RANGE_START: utf32 = Self::HEXADECIMAL_UPPERCASE_OUT_OF_RANGE_ASCII_START as utf32;
    pub const HEXADECIMAL_UPPERCASE_OUT_OF_RANGE_END: utf32 = Self::HEXADECIMAL_UPPERCASE_OUT_OF_RANGE_ASCII_END as utf32;
    pub const HEXADECIMAL_UPPERCASE_OUT_OF_RANGE: AsciiRange<utf32> = AsciiRange::new(
        Self::HEXADECIMAL_UPPERCASE_OUT_OF_RANGE_START,
        Self::HEXADECIMAL_UPPERCASE_OUT_OF_RANGE_END,
    );

    pub const HEXADECIMAL_LOWERCASE_OUT_OF_RANGE_START: utf32 = Self::HEXADECIMAL_LOWERCASE_OUT_OF_RANGE_ASCII_START as utf32;
    pub const HEXADECIMAL_LOWERCASE_OUT_OF_RANGE_END: utf32 = Self::HEXADECIMAL_LOWERCASE_OUT_OF_RANGE_ASCII_END as utf32;
    pub const HEXADECIMAL_LOWERCASE_OUT_OF_RANGE: AsciiRange<utf32> = AsciiRange::new(
        Self::HEXADECIMAL_LOWERCASE_OUT_OF_RANGE_START,
        Self::HEXADECIMAL_LOWERCASE_OUT_OF_RANGE_END,
    );

    pub const HEXADECIMAL_DIGIT_OPS: RangeInclusive<utf32> = RangeInclusive::new(
        Self::HEXADECIMAL_DIGIT_START,
        Self::HEXADECIMAL_DIGIT_END,
    );

    pub const HEXADECIMAL_UPPERCASE_OPS: RangeInclusive<utf32> = RangeInclusive::new(
        Self::HEXADECIMAL_UPPERCASE_START,
        Self::HEXADECIMAL_UPPERCASE_END,
    );

    pub const HEXADECIMAL_LOWERCASE_OPS: RangeInclusive<utf32> = RangeInclusive::new(
        Self::HEXADECIMAL_LOWERCASE_START,
        Self::HEXADECIMAL_LOWERCASE_END,
    );

    pub const HEXADECIMAL_UPPERCASE_OUT_OF_RANGE_OPS: RangeInclusive<utf32> = RangeInclusive::new(
        Self::HEXADECIMAL_UPPERCASE_OUT_OF_RANGE_START,
        Self::HEXADECIMAL_UPPERCASE_OUT_OF_RANGE_END,
    );

    pub const HEXADECIMAL_LOWERCASE_OUT_OF_RANGE_OPS: RangeInclusive<utf32> = RangeInclusive::new(
        Self::HEXADECIMAL_LOWERCASE_OUT_OF_RANGE_START,
        Self::HEXADECIMAL_LOWERCASE_OUT_OF_RANGE_END,
    );

    #[deprecated(since = "0.1.1-dev", note = "will be renamed to `HEXADECIMAL_DIGIT_ASCII`")]
    pub const HEXADECIMAL_DIGIT_RANGE_ASCII:     AsciiRange<ascii> = Self::HEXADECIMAL_DIGIT_ASCII;
    #[deprecated(since = "0.1.1-dev", note = "will be renamed to `HEXADECIMAL_UPPERCASE_ASCII`")]
    pub const HEXADECIMAL_UPPERCASE_RANGE_ASCII: AsciiRange<ascii> = Self::HEXADECIMAL_UPPERCASE_ASCII;
    #[deprecated(since = "0.1.1-dev", note = "will be renamed to `HEXADECIMAL_LOWERCASE_ASCII`")]
    pub const HEXADECIMAL_LOWERCASE_RANGE_ASCII: AsciiRange<ascii> = Self::HEXADECIMAL_LOWERCASE_ASCII;

    #[deprecated(since = "0.1.1-dev", note = "will be renamed to `HEXADECIMAL_DIGIT`")]
    pub const HEXADECIMAL_DIGIT_RANGE:     AsciiRange<utf32> = Self::HEXADECIMAL_DIGIT;
    #[deprecated(since = "0.1.1-dev", note = "will be renamed to `HEXADECIMAL_UPPERCASE`")]
    pub const HEXADECIMAL_UPPERCASE_RANGE: AsciiRange<utf32> = Self::HEXADECIMAL_UPPERCASE;
    #[deprecated(since = "0.1.1-dev", note = "will be renamed to `HEXADECIMAL_LOWERCASE`")]
    pub const HEXADECIMAL_LOWERCASE_RANGE: AsciiRange<utf32> = Self::HEXADECIMAL_LOWERCASE;

    pub const HEXADECIMAL_RANGES_ASCII: [AsciiRange<ascii>; 3] = [
        Self::HEXADECIMAL_DIGIT_RANGE_ASCII,
        Self::HEXADECIMAL_UPPERCASE_RANGE_ASCII,
        Self::HEXADECIMAL_LOWERCASE_RANGE_ASCII,
    ];

    pub const HEXADECIMAL_RANGES: [AsciiRange<utf32>; 3] = [
        Self::HEXADECIMAL_DIGIT_RANGE,
        Self::HEXADECIMAL_UPPERCASE_RANGE,
        Self::HEXADECIMAL_LOWERCASE_RANGE,
    ];

    #[deprecated(since = "0.1.1-dev", note = "will be renamed to `HEXADECIMAL_DIGIT_ASCII_OPS`")]
    pub const HEXADECIMAL_DIGIT_RANGE_ASCII_OPS:     RangeInclusive<ascii> = Self::HEXADECIMAL_DIGIT_ASCII_OPS;
    #[deprecated(since = "0.1.1-dev", note = "will be renamed to `HEXADECIMAL_UPPERCASE_ASCII_OPS`")]
    pub const HEXADECIMAL_UPPERCASE_RANGE_ASCII_OPS: RangeInclusive<ascii> = Self::HEXADECIMAL_UPPERCASE_ASCII_OPS;
    #[deprecated(since = "0.1.1-dev", note = "will be renamed to `HEXADECIMAL_LOWERCASE_ASCII_OPS`")]
    pub const HEXADECIMAL_LOWERCASE_RANGE_ASCII_OPS: RangeInclusive<ascii> = Self::HEXADECIMAL_LOWERCASE_ASCII_OPS;

    pub const HEXADECIMAL_RANGES_ASCII_OPS: [RangeInclusive<ascii>; 3] = [
        Self::HEXADECIMAL_DIGIT_ASCII_OPS,
        Self::HEXADECIMAL_UPPERCASE_ASCII_OPS,
        Self::HEXADECIMAL_LOWERCASE_ASCII_OPS,
    ];

    #[deprecated(since = "0.1.1-dev", note = "will be renamed to `HEXADECIMAL_DIGIT_OPS`")]
    pub const HEXADECIMAL_DIGIT_RANGE_OPS:     RangeInclusive<utf32> = Self::HEXADECIMAL_DIGIT_OPS;
    #[deprecated(since = "0.1.1-dev", note = "will be renamed to `HEXADECIMAL_UPPERCASE_OPS`")]
    pub const HEXADECIMAL_UPPERCASE_RANGE_OPS: RangeInclusive<utf32> = Self::HEXADECIMAL_UPPERCASE_OPS;
    #[deprecated(since = "0.1.1-dev", note = "will be renamed to `HEXADECIMAL_LOWERCASE_OPS`")]
    pub const HEXADECIMAL_LOWERCASE_RANGE_OPS: RangeInclusive<utf32> = Self::HEXADECIMAL_LOWERCASE_OPS;

    pub const HEXADECIMAL_RANGES_OPS: [RangeInclusive<utf32>; 3] = [
        Self::HEXADECIMAL_DIGIT_OPS,
        Self::HEXADECIMAL_UPPERCASE_OPS,
        Self::HEXADECIMAL_LOWERCASE_OPS,
    ];

    pub const HEXADECIMAL_DIGIT_ASCII_OFFSET:     u8 = Self::HEXADECIMAL_DIGIT_ASCII_START;
    pub const HEXADECIMAL_UPPERCASE_ASCII_OFFSET: u8 = Self::HEXADECIMAL_UPPERCASE_ASCII_START - Self::Decimal as u8;
    pub const HEXADECIMAL_LOWERCASE_ASCII_OFFSET: u8 = Self::HEXADECIMAL_LOWERCASE_ASCII_START - Self::Decimal as u8;
}

// custom
#[rustfmt::skip]
impl Base {
    pub const DIGIT_ASCII_START: ascii = b'0';
    pub const DIGIT_ASCII_END: ascii = b'9';
    pub const DIGIT_ASCII: AsciiRange<ascii> = AsciiRange::new(
        Self::DIGIT_ASCII_START,
        Self::DIGIT_ASCII_END,
    );

    pub const UPPERCASE_ASCII_START: ascii = b'A';
    pub const UPPERCASE_ASCII_END: ascii = b'Z';
    pub const UPPERCASE_ASCII: AsciiRange<ascii> = AsciiRange::new(
        Self::UPPERCASE_ASCII_START,
        Self::UPPERCASE_ASCII_END,
    );

    pub const LOWERCASE_ASCII_START: ascii = b'a';
    pub const LOWERCASE_ASCII_END: ascii = b'z';
    pub const LOWERCASE_ASCII: AsciiRange<ascii> = AsciiRange::new(
        Self::LOWERCASE_ASCII_START,
        Self::LOWERCASE_ASCII_END,
    );

    pub const DIGIT_ASCII_OPS: RangeInclusive<ascii> = RangeInclusive::new(
        Self::DIGIT_ASCII_START,
        Self::DIGIT_ASCII_END,
    );

    pub const UPPERCASE_ASCII_OPS: RangeInclusive<ascii> = RangeInclusive::new(
        Self::UPPERCASE_ASCII_START,
        Self::UPPERCASE_ASCII_END,
    );

    pub const LOWERCASE_ASCII_OPS: RangeInclusive<ascii> = RangeInclusive::new(
        Self::LOWERCASE_ASCII_START,
        Self::LOWERCASE_ASCII_END,
    );

    pub const DIGIT_START: utf32 = Self::DIGIT_ASCII_START as utf32;
    pub const DIGIT_END: utf32 = Self::DIGIT_ASCII_END as utf32;
    pub const DIGIT: AsciiRange<utf32> = AsciiRange::new(
        Self::DIGIT_START,
        Self::DIGIT_END,
    );

    pub const UPPERCASE_START: utf32 = Self::UPPERCASE_ASCII_START as utf32;
    pub const UPPERCASE_END: utf32 = Self::UPPERCASE_ASCII_END as utf32;
    pub const UPPERCASE: AsciiRange<utf32> = AsciiRange::new(
        Self::UPPERCASE_START,
        Self::UPPERCASE_END,
    );

    pub const LOWERCASE_START: utf32 = Self::LOWERCASE_ASCII_START as utf32;
    pub const LOWERCASE_END: utf32 = Self::LOWERCASE_ASCII_END as utf32;
    pub const LOWERCASE: AsciiRange<utf32> = AsciiRange::new(
        Self::LOWERCASE_START,
        Self::LOWERCASE_END,
    );

    pub const DIGIT_OPS: RangeInclusive<utf32> = RangeInclusive::new(
        Self::DIGIT_START,
        Self::DIGIT_END,
    );

    pub const UPPERCASE_OPS: RangeInclusive<utf32> = RangeInclusive::new(
        Self::UPPERCASE_START,
        Self::UPPERCASE_END,
    );

    pub const LOWERCASE_OPS: RangeInclusive<utf32> = RangeInclusive::new(
        Self::LOWERCASE_START,
        Self::LOWERCASE_END,
    );

    pub const DIGIT_RANGES_ASCII:   [AsciiRange<ascii>; 1] = [Self::DIGIT_ASCII];
    pub const LETTERS_RANGES_ASCII: [AsciiRange<ascii>; 2] = [
        Self::UPPERCASE_ASCII,
        Self::LOWERCASE_ASCII,
    ];
    pub const ALPHANUMERICAL_RANGES_ASCII: [AsciiRange<ascii>; 3] = [
        Self::DIGIT_ASCII,
        Self::UPPERCASE_ASCII,
        Self::LOWERCASE_ASCII,
    ];

    pub const DIGIT_RANGES:   [AsciiRange<utf32>; 1] = [Self::DIGIT];
    pub const LETTERS_RANGES: [AsciiRange<utf32>; 2] = [
        Self::UPPERCASE,
        Self::LOWERCASE,
    ];
    pub const ALPHANUMERICAL_RANGES: [AsciiRange<utf32>; 3] = [
        Self::DIGIT,
        Self::UPPERCASE,
        Self::LOWERCASE,
    ];

    pub const DIGIT_RANGES_ASCII_OPS:   [RangeInclusive<ascii>; 1] = [Self::DIGIT_ASCII_OPS];
    pub const LETTERS_RANGES_ASCII_OPS: [RangeInclusive<ascii>; 2] = [
        Self::UPPERCASE_ASCII_OPS,
        Self::LOWERCASE_ASCII_OPS,
    ];
    pub const ALPHANUMERICAL_RANGES_ASCII_OPS: [RangeInclusive<ascii>; 3] = [
        Self::DIGIT_ASCII_OPS,
        Self::UPPERCASE_ASCII_OPS,
        Self::LOWERCASE_ASCII_OPS,
    ];

    pub const DIGIT_RANGES_OPS:   [RangeInclusive<utf32>; 1] = [Self::DIGIT_OPS];
    pub const LETTERS_RANGES_OPS: [RangeInclusive<utf32>; 2] = [
        Self::UPPERCASE_OPS,
        Self::LOWERCASE_OPS,
    ];
    pub const ALPHANUMERICAL_RANGES_OPS: [RangeInclusive<utf32>; 3] = [
        Self::DIGIT_OPS,
        Self::UPPERCASE_OPS,
        Self::LOWERCASE_OPS,
    ];

    pub const DIGIT_ASCII_OFFSET:     u8 = Self::DIGIT_ASCII_START;
    pub const UPPERCASE_ASCII_OFFSET: u8 = Self::UPPERCASE_ASCII_START - Self::Decimal as u8;
    pub const LOWERCASE_ASCII_OFFSET: u8 = Self::LOWERCASE_ASCII_START - Self::Decimal as u8;
}

#[rustfmt::skip]
impl Base {
    #[must_use]
    #[inline]
    pub const fn range(self) -> &'static [AsciiRange<utf32>] {
        return match self {
            Self::Binary      => &Self::BINARY_RANGES,
            Self::Octal       => &Self::OCTAL_RANGES,
            Self::Decimal     => &Self::DECIMAL_RANGES,
            Self::Hexadecimal => &Self::HEXADECIMAL_RANGES,
        };
    }

    #[must_use]
    #[inline]
    pub const fn range_ops(self) -> &'static [RangeInclusive<utf32>] {
        return match self {
            Self::Binary      => &Self::BINARY_RANGES_OPS,
            Self::Octal       => &Self::OCTAL_RANGES_OPS,
            Self::Decimal     => &Self::DECIMAL_RANGES_OPS,
            Self::Hexadecimal => &Self::HEXADECIMAL_RANGES_OPS,
        };
    }

    #[must_use]
    #[inline]
    pub const fn range_ascii(self) -> &'static [AsciiRange<ascii>] {
        return match self {
            Self::Binary      => &Self::BINARY_RANGES_ASCII,
            Self::Octal       => &Self::OCTAL_RANGES_ASCII,
            Self::Decimal     => &Self::DECIMAL_RANGES_ASCII,
            Self::Hexadecimal => &Self::HEXADECIMAL_RANGES_ASCII,
        };
    }

    #[must_use]
    #[inline]
    pub const fn range_ascii_ops(self) -> &'static [RangeInclusive<ascii>] {
        return match self {
            Self::Binary      => &Self::BINARY_RANGES_ASCII_OPS,
            Self::Octal       => &Self::OCTAL_RANGES_ASCII_OPS,
            Self::Decimal     => &Self::DECIMAL_RANGES_ASCII_OPS,
            Self::Hexadecimal => &Self::HEXADECIMAL_RANGES_ASCII_OPS,
        };
    }
}

// Parsing
pub type Offset = ascii;
pub type OffsetCustomBase = ascii;
// NOTE(stefano): rename to `Digit` and `DigitCustomBase` when removing non offset based parsing
pub type DigitOffset = u8;
pub type DigitOffsetCustomBase = u8;

// Note: this enum exists just to leverage the enum mechanism of assigning discriminants
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
enum OffsetEnum {
    Invalid = 128,
    OutOfRange,
    BaseMin,
    BaseMax,
}

pub const INVALID: Offset = OffsetEnum::Invalid as u8;
pub const OUT_OF_RANGE: Offset = OffsetEnum::OutOfRange as u8;
pub const BASE_MIN: OffsetCustomBase = OffsetEnum::BaseMin as u8;
pub const BASE_MAX: OffsetCustomBase = OffsetEnum::BaseMax as u8;

impl Base {
    #[must_use]
    #[inline(always)]
    pub const fn check_offset(self, character: ascii) -> Offset {
        return check_offset(character, self);
    }

    #[must_use]
    #[inline(always)]
    pub const fn parse_offset(self, character: ascii) -> DigitOffset {
        return parse_offset(character, self);
    }

    #[deprecated(since = "0.1.1-dev", note = "will use offset based checking and parsing")]
    #[must_use]
    #[inline(always)]
    pub const fn check(self, character: ascii) -> AsciiDigit {
        return check(character, self);
    }

    #[deprecated(since = "0.1.1-dev", note = "will use offset based checking and parsing")]
    #[must_use]
    #[inline(always)]
    pub const fn parse(self, character: ascii) -> Digit {
        return parse(character, self);
    }
}

#[deprecated(since = "0.1.1-dev", note = "will use offset based checking and parsing")]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum AsciiDigit {
    Ok,
    Underscore,
    Dot,
    OutOfRange,
    Other,
}

#[deprecated(since = "0.1.1-dev", note = "will use offset based checking and parsing")]
#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum Digit {
    Ok(u8)     = AsciiDigit::Ok as u8,
    Underscore = AsciiDigit::Underscore as u8,
    Dot        = AsciiDigit::Dot as u8,
    OutOfRange = AsciiDigit::OutOfRange as u8,
    Other      = AsciiDigit::Other as u8,
}

#[deprecated(since = "0.1.1-dev", note = "will use offset based checking and parsing")]
#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum AsciiDigitCustomBase {
    Ok         = AsciiDigit::Ok as u8,
    Underscore = AsciiDigit::Underscore as u8,
    Dot        = AsciiDigit::Dot as u8,
    OutOfRange = AsciiDigit::OutOfRange as u8,
    Other      = AsciiDigit::Other as u8,
    BaseMin,
    BaseMax,
}

#[deprecated(since = "0.1.1-dev", note = "will use offset based checking and parsing")]
#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum DigitCustomBase {
    Ok(u8)     = AsciiDigit::Ok as u8,
    Underscore = AsciiDigit::Underscore as u8,
    Dot        = AsciiDigit::Dot as u8,
    OutOfRange = AsciiDigit::OutOfRange as u8,
    Other      = AsciiDigit::Other as u8,
    BaseMin    = AsciiDigitCustomBase::BaseMin as u8,
    BaseMax    = AsciiDigitCustomBase::BaseMax as u8,
}

#[must_use]
#[inline]
pub const fn check_offset(character: ascii, base: Base) -> Offset {
    #[rustfmt::skip]
    return match base {
        Base::Binary      => check_binary_offset(character),
        Base::Octal       => check_octal_offset(character),
        Base::Decimal     => check_decimal_offset(character),
        Base::Hexadecimal => check_hexadecimal_offset(character),
    };
}


#[must_use]
#[inline]
pub const fn parse_offset(character: ascii, base: Base) -> DigitOffset {
    #[rustfmt::skip]
    return match base {
        Base::Binary      => parse_binary_offset(character),
        Base::Octal       => parse_octal_offset(character),
        Base::Decimal     => parse_decimal_offset(character),
        Base::Hexadecimal => parse_hexadecimal_offset(character),
    };
}

#[deprecated(since = "0.1.1-dev", note = "will use offset based checking and parsing")]
#[must_use]
#[inline]
pub const fn check(character: ascii, base: Base) -> AsciiDigit {
    #[rustfmt::skip]
    return match base {
        Base::Binary      => check_binary(character),
        Base::Octal       => check_octal(character),
        Base::Decimal     => check_decimal(character),
        Base::Hexadecimal => check_hexadecimal(character),
    };
}

#[deprecated(since = "0.1.1-dev", note = "will use offset based checking and parsing")]
#[must_use]
#[inline]
pub const fn parse(character: ascii, base: Base) -> Digit {
    #[rustfmt::skip]
    return match base {
        Base::Binary      => parse_binary(character),
        Base::Octal       => parse_octal(character),
        Base::Decimal     => parse_decimal(character),
        Base::Hexadecimal => parse_hexadecimal(character),
    };
}

#[macro_export]
macro_rules! range_binary_digit_ascii {
    () => { Base::BINARY_DIGIT_ASCII_START..=Base::BINARY_DIGIT_ASCII_END };
}
pub use range_binary_digit_ascii;

#[macro_export]
macro_rules! range_binary_digit {
    () => { Base::BINARY_DIGIT_START..=Base::BINARY_DIGIT_END };
}
pub use range_binary_digit;

#[macro_export]
macro_rules! range_binary_digit_out_of_range_ascii {
    () => { Base::BINARY_DIGIT_OUT_OF_RANGE_ASCII_START..=Base::BINARY_DIGIT_OUT_OF_RANGE_ASCII_END };
}
pub use range_binary_digit_out_of_range_ascii;

#[macro_export]
macro_rules! range_binary_digit_out_of_range {
    () => { Base::BINARY_DIGIT_OUT_OF_RANGE_START..=Base::BINARY_DIGIT_OUT_OF_RANGE_END };
}
pub use range_binary_digit_out_of_range;

#[macro_export]
macro_rules! range_binary_uppercase_out_of_range_ascii {
    () => { Base::BINARY_UPPERCASE_OUT_OF_RANGE_ASCII_START..=Base::BINARY_UPPERCASE_OUT_OF_RANGE_ASCII_END };
}
pub use range_binary_uppercase_out_of_range_ascii;

#[macro_export]
macro_rules! range_binary_uppercase_out_of_range {
    () => { Base::BINARY_UPPERCASE_OUT_OF_RANGE_START..=Base::BINARY_UPPERCASE_OUT_OF_RANGE_END };
}
pub use range_binary_uppercase_out_of_range;

#[macro_export]
macro_rules! range_binary_lowercase_out_of_range_ascii {
    () => { Base::BINARY_LOWERCASE_OUT_OF_RANGE_ASCII_START..=Base::BINARY_LOWERCASE_OUT_OF_RANGE_ASCII_END };
}
pub use range_binary_lowercase_out_of_range_ascii;

#[macro_export]
macro_rules! range_binary_lowercase_out_of_range {
    () => { Base::BINARY_LOWERCASE_OUT_OF_RANGE_START..=Base::BINARY_LOWERCASE_OUT_OF_RANGE_END };
}
pub use range_binary_lowercase_out_of_range;

#[macro_export]
macro_rules! range_binary_letter_out_of_range_ascii {
    () => { range_binary_uppercase_out_of_range_ascii!() | range_binary_lowercase_out_of_range_ascii!() };
}
pub use range_binary_letter_out_of_range_ascii;

#[macro_export]
macro_rules! range_binary_letter_out_of_range {
    () => { range_binary_uppercase_out_of_range!() | range_binary_lowercase_out_of_range!() };
}
pub use range_binary_letter_out_of_range;

#[macro_export]
macro_rules! range_binary_out_of_range_ascii {
    () => { range_binary_digit_out_of_range_ascii!() | range_binary_letter_out_of_range_ascii!() };
}
pub use range_binary_out_of_range_ascii;

#[macro_export]
macro_rules! range_binary_out_of_range {
    () => { range_binary_digit_out_of_range!() | range_binary_letter_out_of_range!() };
}
pub use range_binary_out_of_range;

#[must_use]
#[inline]
pub const fn is_binary_digit(ch: ascii) -> bool {
    return matches!(ch, range_binary_digit_ascii!());
}

#[must_use]
#[inline]
pub const fn is_binary_digit_out_of_range(ch: ascii) -> bool {
    return matches!(ch, range_binary_digit_out_of_range_ascii!());
}

#[must_use]
#[inline]
pub const fn is_binary_uppercase_out_of_range(ch: ascii) -> bool {
    return matches!(ch, range_binary_uppercase_out_of_range_ascii!());
}

#[must_use]
#[inline]
pub const fn is_binary_lowercase_out_of_range(ch: ascii) -> bool {
    return matches!(ch, range_binary_lowercase_out_of_range_ascii!());
}

#[must_use]
#[inline]
pub const fn is_binary_letter_out_of_range(ch: ascii) -> bool {
    return is_binary_uppercase_out_of_range(ch) || is_binary_lowercase_out_of_range(ch);
}

#[must_use]
#[inline(always)]
pub const fn is_binary(ch: ascii) -> bool {
    return is_binary_digit(ch);
}

#[must_use]
#[inline]
pub const fn is_binary_out_of_range(ch: ascii) -> bool {
    return is_binary_digit_out_of_range(ch) || is_binary_letter_out_of_range(ch);
}

macro_rules! offset_binary {
    ($ch:expr) => {
        match $ch {
            range_binary_digit_ascii!()        => Base::BINARY_ASCII_OFFSET,
            range_binary_out_of_range_ascii!() => return OUT_OF_RANGE,
            _                                  => return INVALID,
        }
    }
}

#[must_use]
#[inline]
pub const fn check_binary_offset(character: ascii) -> Offset {
    return offset_binary!(character);
}


#[must_use]
#[inline]
pub const fn parse_binary_offset(character: ascii) -> DigitOffset {
    let offset = offset_binary!(character);
    return character - offset;
}

#[deprecated(since = "0.1.1-dev", note = "will be removed in favor of offset based checking and parsing")]
#[must_use]
#[inline]
pub const fn check_binary(character: ascii) -> AsciiDigit {
    #[rustfmt::skip]
    return match character {
        range_binary_digit_ascii!()        => AsciiDigit::Ok,
        b'_'                               => AsciiDigit::Underscore,
        b'.'                               => AsciiDigit::Dot,
        range_binary_out_of_range_ascii!() => AsciiDigit::OutOfRange,
        _                                  => AsciiDigit::Other,
    };
}

#[deprecated(since = "0.1.1-dev", note = "will be removed in favor of offset based checking and parsing")]
#[must_use]
#[inline]
pub const fn parse_binary(character: ascii) -> Digit {
    #[rustfmt::skip]
    let offset = match character {
        range_binary_digit_ascii!()        => Base::BINARY_ASCII_OFFSET,
        b'_'                               => return Digit::Underscore,
        b'.'                               => return Digit::Dot,
        range_binary_out_of_range_ascii!() => return Digit::OutOfRange,
        _                                  => return Digit::Other,
    };
    return Digit::Ok(character - offset);
}

#[macro_export]
macro_rules! range_octal_digit_ascii {
    () => { Base::OCTAL_DIGIT_ASCII_START..=Base::OCTAL_DIGIT_ASCII_END };
}
pub use range_octal_digit_ascii;

#[macro_export]
macro_rules! range_octal_digit {
    () => { Base::OCTAL_DIGIT_START..=Base::OCTAL_DIGIT_END };
}
pub use range_octal_digit;

#[macro_export]
macro_rules! range_octal_digit_out_of_range_ascii {
    () => { Base::OCTAL_DIGIT_OUT_OF_RANGE_ASCII_START..=Base::OCTAL_DIGIT_OUT_OF_RANGE_ASCII_END };
}
pub use range_octal_digit_out_of_range_ascii;

#[macro_export]
macro_rules! range_octal_digit_out_of_range {
    () => { Base::OCTAL_DIGIT_OUT_OF_RANGE_START..=Base::OCTAL_DIGIT_OUT_OF_RANGE_END };
}
pub use range_octal_digit_out_of_range;

#[macro_export]
macro_rules! range_octal_uppercase_out_of_range_ascii {
    () => { Base::OCTAL_UPPERCASE_OUT_OF_RANGE_ASCII_START..=Base::OCTAL_UPPERCASE_OUT_OF_RANGE_ASCII_END };
}
pub use range_octal_uppercase_out_of_range_ascii;

#[macro_export]
macro_rules! range_octal_uppercase_out_of_range {
    () => { Base::OCTAL_UPPERCASE_OUT_OF_RANGE_START..=Base::OCTAL_UPPERCASE_OUT_OF_RANGE_END };
}
pub use range_octal_uppercase_out_of_range;

#[macro_export]
macro_rules! range_octal_lowercase_out_of_range_ascii {
    () => { Base::OCTAL_LOWERCASE_OUT_OF_RANGE_ASCII_START..=Base::OCTAL_LOWERCASE_OUT_OF_RANGE_ASCII_END };
}
pub use range_octal_lowercase_out_of_range_ascii;

#[macro_export]
macro_rules! range_octal_lowercase_out_of_range {
    () => { Base::OCTAL_LOWERCASE_OUT_OF_RANGE_START..=Base::OCTAL_LOWERCASE_OUT_OF_RANGE_END };
}
pub use range_octal_lowercase_out_of_range;

#[macro_export]
macro_rules! range_octal_letter_out_of_range_ascii {
    () => { range_octal_uppercase_out_of_range_ascii!() | range_octal_lowercase_out_of_range_ascii!() };
}
pub use range_octal_letter_out_of_range_ascii;

#[macro_export]
macro_rules! range_octal_letter_out_of_range {
    () => { range_octal_uppercase_out_of_range!() | range_octal_lowercase_out_of_range!() };
}
pub use range_octal_letter_out_of_range;

#[macro_export]
macro_rules! range_octal_out_of_range_ascii {
    () => { range_octal_digit_out_of_range_ascii!() | range_octal_letter_out_of_range_ascii!() };
}
pub use range_octal_out_of_range_ascii;

#[macro_export]
macro_rules! range_octal_out_of_range {
    () => { range_octal_digit_out_of_range!() | range_octal_letter_out_of_range!() };
}
pub use range_octal_out_of_range;

#[must_use]
#[inline]
pub const fn is_octal_digit(ch: ascii) -> bool {
    return matches!(ch, range_octal_digit_ascii!());
}

#[must_use]
#[inline]
pub const fn is_octal_digit_out_of_range(ch: ascii) -> bool {
    return matches!(ch, range_octal_digit_out_of_range_ascii!());
}

#[must_use]
#[inline]
pub const fn is_octal_uppercase_out_of_range(ch: ascii) -> bool {
    return matches!(ch, range_octal_uppercase_out_of_range_ascii!());
}

#[must_use]
#[inline]
pub const fn is_octal_lowercase_out_of_range(ch: ascii) -> bool {
    return matches!(ch, range_octal_lowercase_out_of_range_ascii!());
}

#[must_use]
#[inline]
pub const fn is_octal_letter_out_of_range(ch: ascii) -> bool {
    return is_octal_uppercase_out_of_range(ch) || is_octal_lowercase_out_of_range(ch);
}

#[must_use]
#[inline(always)]
pub const fn is_octal(ch: ascii) -> bool {
    return is_octal_digit(ch);
}

#[must_use]
#[inline]
pub const fn is_octal_out_of_range(ch: ascii) -> bool {
    return is_octal_digit_out_of_range(ch) || is_octal_letter_out_of_range(ch);
}

macro_rules! offset_octal {
    ($ch:expr) => {
        match $ch {
            range_octal_digit_ascii!()        => Base::OCTAL_ASCII_OFFSET,
            range_octal_out_of_range_ascii!() => return OUT_OF_RANGE,
            _                                 => return INVALID,
        }
    }
}

#[must_use]
#[inline]
pub const fn check_octal_offset(character: ascii) -> Offset {
    return offset_octal!(character);
}

#[must_use]
#[inline]
pub const fn parse_octal_offset(character: ascii) -> DigitOffset {
    let offset = offset_octal!(character);
    return character - offset;
}

#[deprecated(since = "0.1.1-dev", note = "will be removed in favor of offset based checking and parsing")]
#[must_use]
#[inline]
pub const fn check_octal(character: ascii) -> AsciiDigit {
    #[rustfmt::skip]
    return match character {
        range_octal_digit_ascii!()        => AsciiDigit::Ok,
        b'_'                              => AsciiDigit::Underscore,
        b'.'                              => AsciiDigit::Dot,
        range_octal_out_of_range_ascii!() => AsciiDigit::OutOfRange,
        _                                 => AsciiDigit::Other,
    };
}

#[deprecated(since = "0.1.1-dev", note = "will be removed in favor of offset based checking and parsing")]
#[must_use]
#[inline]
pub const fn parse_octal(character: ascii) -> Digit {
    #[rustfmt::skip]
    let offset = match character {
        range_octal_digit_ascii!()        => Base::OCTAL_ASCII_OFFSET,
        b'_'                              => return Digit::Underscore,
        b'.'                              => return Digit::Dot,
        range_octal_out_of_range_ascii!() => return Digit::OutOfRange,
        _                                 => return Digit::Other,
    };
    return Digit::Ok(character - offset);
}

#[macro_export]
macro_rules! range_decimal_digit_ascii {
    () => { Base::DECIMAL_DIGIT_ASCII_START..=Base::DECIMAL_DIGIT_ASCII_END };
}
pub use range_decimal_digit_ascii;

#[macro_export]
macro_rules! range_decimal_digit {
    () => { Base::DECIMAL_DIGIT_START..=Base::DECIMAL_DIGIT_END };
}
pub use range_decimal_digit;

#[macro_export]
macro_rules! range_decimal_uppercase_out_of_range_ascii {
    () => { Base::DECIMAL_UPPERCASE_OUT_OF_RANGE_ASCII_START..=Base::DECIMAL_UPPERCASE_OUT_OF_RANGE_ASCII_END };
}
pub use range_decimal_uppercase_out_of_range_ascii;

#[macro_export]
macro_rules! range_decimal_uppercase_out_of_range {
    () => { Base::DECIMAL_UPPERCASE_OUT_OF_RANGE_START..=Base::DECIMAL_UPPERCASE_OUT_OF_RANGE_END };
}
pub use range_decimal_uppercase_out_of_range;

#[macro_export]
macro_rules! range_decimal_lowercase_out_of_range_ascii {
    () => { Base::DECIMAL_LOWERCASE_OUT_OF_RANGE_ASCII_START..=Base::DECIMAL_LOWERCASE_OUT_OF_RANGE_ASCII_END };
}
pub use range_decimal_lowercase_out_of_range_ascii;

#[macro_export]
macro_rules! range_decimal_lowercase_out_of_range {
    () => { Base::DECIMAL_LOWERCASE_OUT_OF_RANGE_START..=Base::DECIMAL_LOWERCASE_OUT_OF_RANGE_END };
}
pub use range_decimal_lowercase_out_of_range;

#[macro_export]
macro_rules! range_decimal_letter_out_of_range_ascii {
    () => { range_decimal_uppercase_out_of_range_ascii!() | range_decimal_lowercase_out_of_range_ascii!() };
}
pub use range_decimal_letter_out_of_range_ascii;

#[macro_export]
macro_rules! range_decimal_letter_out_of_range {
    () => { range_decimal_uppercase_out_of_range!() | range_decimal_lowercase_out_of_range!() };
}
pub use range_decimal_letter_out_of_range;

#[macro_export]
macro_rules! range_decimal_out_of_range_ascii {
    () => { range_decimal_letter_out_of_range_ascii!() };
}
pub use range_decimal_out_of_range_ascii;

#[macro_export]
macro_rules! range_decimal_out_of_range {
    () => { range_decimal_letter_out_of_range!() };
}
pub use range_decimal_out_of_range;

#[must_use]
#[inline]
pub const fn is_decimal_digit(ch: ascii) -> bool {
    return matches!(ch, range_decimal_digit_ascii!());
}

#[must_use]
#[inline]
pub const fn is_decimal_uppercase_out_of_range(ch: ascii) -> bool {
    return matches!(ch, range_decimal_uppercase_out_of_range_ascii!());
}

#[must_use]
#[inline]
pub const fn is_decimal_lowercase_out_of_range(ch: ascii) -> bool {
    return matches!(ch, range_decimal_lowercase_out_of_range_ascii!());
}

#[must_use]
#[inline]
pub const fn is_decimal_letter_out_of_range(ch: ascii) -> bool {
    return is_decimal_uppercase_out_of_range(ch) || is_decimal_lowercase_out_of_range(ch);
}

#[must_use]
#[inline(always)]
pub const fn is_decimal(ch: ascii) -> bool {
    return is_decimal_digit(ch);
}

#[must_use]
#[inline(always)]
pub const fn is_decimal_out_of_range(ch: ascii) -> bool {
    return is_decimal_letter_out_of_range(ch);
}

macro_rules! offset_decimal {
    ($ch:expr) => {
        match $ch {
            range_decimal_digit_ascii!()        => Base::DECIMAL_ASCII_OFFSET,
            range_decimal_out_of_range_ascii!() => return OUT_OF_RANGE,
            _                                   => return INVALID,
        }
    }
}

#[must_use]
#[inline]
pub const fn check_decimal_offset(character: ascii) -> Offset {
    return offset_decimal!(character);
}

#[must_use]
#[inline]
pub const fn parse_decimal_offset(character: ascii) -> DigitOffset {
    let offset = offset_decimal!(character);
    return character - offset;
}

#[deprecated(since = "0.1.1-dev", note = "will be removed in favor of offset based checking and parsing")]
#[must_use]
#[inline]
pub const fn check_decimal(character: ascii) -> AsciiDigit {
    #[rustfmt::skip]
    return match character {
        range_decimal_digit_ascii!()        => AsciiDigit::Ok,
        b'_'                                => AsciiDigit::Underscore,
        b'.'                                => AsciiDigit::Dot,
        range_decimal_out_of_range_ascii!() => AsciiDigit::OutOfRange,
        _                                   => AsciiDigit::Other,
    };
}

#[deprecated(since = "0.1.1-dev", note = "will be removed in favor of offset based checking and parsing")]
#[must_use]
#[inline]
pub const fn parse_decimal(character: ascii) -> Digit {
    #[rustfmt::skip]
    let offset = match character {
        range_decimal_digit_ascii!()        => Base::DECIMAL_ASCII_OFFSET,
        b'_'                                => return Digit::Underscore,
        b'.'                                => return Digit::Dot,
        range_decimal_out_of_range_ascii!() => return Digit::OutOfRange,
        _                                   => return Digit::Other,
    };
    return Digit::Ok(character - offset);
}

#[macro_export]
macro_rules! range_hexadecimal_digit_ascii {
    () => { Base::HEXADECIMAL_DIGIT_ASCII_START..=Base::HEXADECIMAL_DIGIT_ASCII_END };
}
pub use range_hexadecimal_digit_ascii;

#[macro_export]
macro_rules! range_hexadecimal_digit {
    () => { Base::HEXADECIMAL_DIGIT_START..=Base::HEXADECIMAL_DIGIT_END };
}
pub use range_hexadecimal_digit;

#[macro_export]
macro_rules! range_hexadecimal_uppercase_ascii {
    () => { Base::HEXADECIMAL_UPPERCASE_ASCII_START..=Base::HEXADECIMAL_UPPERCASE_ASCII_END };
}
pub use range_hexadecimal_uppercase_ascii;

#[macro_export]
macro_rules! range_hexadecimal_uppercase {
    () => { Base::HEXADECIMAL_UPPERCASE_START..=Base::HEXADECIMAL_UPPERCASE_END };
}
pub use range_hexadecimal_uppercase;

#[macro_export]
macro_rules! range_hexadecimal_lowercase_ascii {
    () => { Base::HEXADECIMAL_LOWERCASE_ASCII_START..=Base::HEXADECIMAL_LOWERCASE_ASCII_END };
}
pub use range_hexadecimal_lowercase_ascii;

#[macro_export]
macro_rules! range_hexadecimal_lowercase {
    () => { Base::HEXADECIMAL_LOWERCASE_START..=Base::HEXADECIMAL_LOWERCASE_END };
}
pub use range_hexadecimal_lowercase;

#[macro_export]
macro_rules! range_hexadecimal_letter_ascii {
    () => { range_hexadecimal_uppercase_ascii!() | range_hexadecimal_lowercase_ascii!() };
}
pub use range_hexadecimal_letter_ascii;

#[macro_export]
macro_rules! range_hexadecimal_letter {
    () => { range_hexadecimal_uppercase!() | range_hexadecimal_lowercase!() };
}
pub use range_hexadecimal_letter;

#[macro_export]
macro_rules! range_hexadecimal_uppercase_out_of_range_ascii {
    () => { Base::HEXADECIMAL_UPPERCASE_OUT_OF_RANGE_ASCII_START..=Base::HEXADECIMAL_UPPERCASE_OUT_OF_RANGE_ASCII_END };
}
pub use range_hexadecimal_uppercase_out_of_range_ascii;

#[macro_export]
macro_rules! range_hexadecimal_uppercase_out_of_range {
    () => { Base::HEXADECIMAL_UPPERCASE_OUT_OF_RANGE_START..=Base::HEXADECIMAL_UPPERCASE_OUT_OF_RANGE_END };
}
pub use range_hexadecimal_uppercase_out_of_range;

#[macro_export]
macro_rules! range_hexadecimal_lowercase_out_of_range_ascii {
    () => { Base::HEXADECIMAL_LOWERCASE_OUT_OF_RANGE_ASCII_START..=Base::HEXADECIMAL_LOWERCASE_OUT_OF_RANGE_ASCII_END };
}
pub use range_hexadecimal_lowercase_out_of_range_ascii;

#[macro_export]
macro_rules! range_hexadecimal_lowercase_out_of_range {
    () => { Base::HEXADECIMAL_LOWERCASE_OUT_OF_RANGE_START..=Base::HEXADECIMAL_LOWERCASE_OUT_OF_RANGE_END };
}
pub use range_hexadecimal_lowercase_out_of_range;

#[macro_export]
macro_rules! range_hexadecimal_letter_out_of_range_ascii {
    () => { range_hexadecimal_uppercase_out_of_range_ascii!() | range_hexadecimal_lowercase_out_of_range_ascii!() };
}
pub use range_hexadecimal_letter_out_of_range_ascii;

#[macro_export]
macro_rules! range_hexadecimal_letter_out_of_range {
    () => { range_hexadecimal_uppercase_out_of_range!() | range_hexadecimal_lowercase_out_of_range!() };
}
pub use range_hexadecimal_letter_out_of_range;

#[macro_export]
macro_rules! range_hexadecimal_ascii {
    () => { range_hexadecimal_digit_ascii!() | range_hexadecimal_letter_ascii!() };
}
pub use range_hexadecimal_ascii;

#[macro_export]
macro_rules! range_hexadecimal {
    () => { range_hexadecimal_digit!() | range_hexadecimal_letter!() };
}
pub use range_hexadecimal;

#[macro_export]
macro_rules! range_hexadecimal_out_of_range_ascii {
    () => { range_hexadecimal_letter_out_of_range_ascii!() };
}
pub use range_hexadecimal_out_of_range_ascii;

#[macro_export]
macro_rules! range_hexadecimal_out_of_range {
    () => { range_hexadecimal_letter_out_of_range!() };
}
pub use range_hexadecimal_out_of_range;

#[must_use]
#[inline]
pub const fn is_hexadecimal_digit(ch: ascii) -> bool {
    return matches!(ch, range_hexadecimal_digit_ascii!());
}

#[must_use]
#[inline]
pub const fn is_hexadecimal_uppercase(ch: ascii) -> bool {
    return matches!(ch, range_hexadecimal_uppercase_ascii!());
}

#[must_use]
#[inline]
pub const fn is_hexadecimal_lowercase(ch: ascii) -> bool {
    return matches!(ch, range_hexadecimal_lowercase_ascii!());
}

#[must_use]
#[inline]
pub const fn is_hexadecimal_letter(ch: ascii) -> bool {
    return is_hexadecimal_uppercase(ch) || is_hexadecimal_lowercase(ch);
}

#[must_use]
#[inline]
pub const fn is_hexadecimal_uppercase_out_of_range(ch: ascii) -> bool {
    return matches!(ch, range_hexadecimal_uppercase_out_of_range_ascii!());
}

#[must_use]
#[inline]
pub const fn is_hexadecimal_lowercase_out_of_range(ch: ascii) -> bool {
    return matches!(ch, range_hexadecimal_lowercase_out_of_range_ascii!());
}

#[must_use]
#[inline]
pub const fn is_hexadecimal_letter_out_of_range(ch: ascii) -> bool {
    return is_hexadecimal_uppercase_out_of_range(ch) || is_hexadecimal_lowercase_out_of_range(ch);
}

#[must_use]
#[inline]
pub const fn is_hexadecimal(ch: ascii) -> bool {
    return is_hexadecimal_digit(ch) || is_hexadecimal_letter(ch);
}

#[must_use]
#[inline(always)]
pub const fn is_hexadecimal_out_of_range(ch: ascii) -> bool {
    return is_hexadecimal_letter_out_of_range(ch);
}

macro_rules! offset_hexadecimal {
    ($ch:expr) => {
        match $ch {
            range_hexadecimal_digit_ascii!()        => Base::HEXADECIMAL_DIGIT_ASCII_OFFSET,
            range_hexadecimal_uppercase_ascii!()    => Base::HEXADECIMAL_UPPERCASE_ASCII_OFFSET,
            range_hexadecimal_lowercase_ascii!()    => Base::HEXADECIMAL_LOWERCASE_ASCII_OFFSET,
            range_hexadecimal_out_of_range_ascii!() => return OUT_OF_RANGE,
            _                                       => return INVALID,
        }
    }
}

#[must_use]
#[inline]
pub const fn check_hexadecimal_offset(character: ascii) -> Offset {
    return offset_hexadecimal!(character);
}

#[must_use]
#[inline]
pub const fn parse_hexadecimal_offset(character: ascii) -> DigitOffset {
    let offset = offset_hexadecimal!(character);
    return character - offset;
}

#[deprecated(since = "0.1.1-dev", note = "will use offset based checking and parsing")]
#[must_use]
#[inline]
pub const fn check_hexadecimal(character: ascii) -> AsciiDigit {
    #[rustfmt::skip]
    return match character {
        range_hexadecimal_ascii!()              => AsciiDigit::Ok,
        b'_'                                    => AsciiDigit::Underscore,
        b'.'                                    => AsciiDigit::Dot,
        range_hexadecimal_out_of_range_ascii!() => AsciiDigit::OutOfRange,
        _                                       => AsciiDigit::Other,
    };
}

#[deprecated(since = "0.1.1-dev", note = "will use offset based checking and parsing")]
#[must_use]
#[inline]
pub const fn parse_hexadecimal(character: ascii) -> Digit {
    #[rustfmt::skip]
    let offset = match character {
        range_hexadecimal_digit_ascii!()        => Base::HEXADECIMAL_DIGIT_ASCII_OFFSET,
        range_hexadecimal_uppercase_ascii!()    => Base::HEXADECIMAL_UPPERCASE_ASCII_OFFSET,
        range_hexadecimal_lowercase_ascii!()    => Base::HEXADECIMAL_LOWERCASE_ASCII_OFFSET,
        b'_'                                    => return Digit::Underscore,
        b'.'                                    => return Digit::Dot,
        range_hexadecimal_out_of_range_ascii!() => return Digit::OutOfRange,
        _                                       => return Digit::Other,
    };
    return Digit::Ok(character - offset);
}

#[macro_export]
macro_rules! range_digit_ascii {
    () => { Base::DIGIT_ASCII_START..=Base::DIGIT_ASCII_END };
}
pub use range_digit_ascii;

#[macro_export]
macro_rules! range_digit {
    () => { Base::DIGIT_START..=Base::DIGIT_END };
}
pub use range_digit;

#[macro_export]
macro_rules! range_uppercase_ascii {
    () => { Base::UPPERCASE_ASCII_START..=Base::UPPERCASE_ASCII_END };
}
pub use range_uppercase_ascii;

#[macro_export]
macro_rules! range_uppercase {
    () => { Base::UPPERCASE_START..=Base::UPPERCASE_END };
}
pub use range_uppercase;

#[macro_export]
macro_rules! range_lowercase_ascii {
    () => { Base::LOWERCASE_ASCII_START..=Base::LOWERCASE_ASCII_END };
}
pub use range_lowercase_ascii;

#[macro_export]
macro_rules! range_lowercase {
    () => { Base::LOWERCASE_START..=Base::LOWERCASE_END };
}
pub use range_lowercase;

#[macro_export]
macro_rules! range_letter_ascii {
    () => { range_uppercase_ascii!() | range_lowercase_ascii!() };
}
pub use range_letter_ascii;

#[macro_export]
macro_rules! range_letter {
    () => { range_uppercase!() | range_lowercase!() };
}
pub use range_letter;

#[macro_export]
macro_rules! range_alphanumerical_ascii {
    () => { range_digit_ascii!() | range_letter_ascii!() };
}
pub use range_alphanumerical_ascii;

#[macro_export]
macro_rules! range_alphanumerical {
    () => { range_digit!() | range_letter!() };
}
pub use range_alphanumerical;

#[must_use]
#[inline]
pub const fn is_digit(ch: ascii) -> bool {
    return matches!(ch, range_digit_ascii!());
}

#[must_use]
#[inline]
pub const fn is_uppercase(ch: ascii) -> bool {
    return matches!(ch, range_uppercase_ascii!());
}

#[must_use]
#[inline]
pub const fn is_lowercase(ch: ascii) -> bool {
    return matches!(ch, range_lowercase_ascii!());
}

#[must_use]
#[inline]
pub const fn is_letter(ch: ascii) -> bool {
    return is_uppercase(ch) || is_lowercase(ch);
}

#[must_use]
#[inline]
pub const fn is_alphanumerical(ch: ascii) -> bool {
    return is_digit(ch) || is_letter(ch);
}

macro_rules! offset_alphanumerical {
    ($ch:expr) => {
        match $ch {
            range_digit_ascii!()     => Base::DIGIT_ASCII_OFFSET,
            range_uppercase_ascii!() => Base::UPPERCASE_ASCII_OFFSET,
            range_lowercase_ascii!() => Base::LOWERCASE_ASCII_OFFSET,
            _                        => return INVALID,
        }
    }
}

#[must_use]
#[inline]
pub const fn check_custom_offset(character: ascii, base: u8) -> OffsetCustomBase {
    if base < Base::MIN { return BASE_MIN; }
    if base > Base::MAX { return BASE_MAX; }

    let offset = offset_alphanumerical!(character);
    let digit = character - offset;
    if digit >= base { return OUT_OF_RANGE; }
    return offset;
}

#[must_use]
#[inline]
pub const fn parse_custom_offset(character: ascii, base: u8) -> DigitOffsetCustomBase {
    if base < Base::MIN { return BASE_MIN; }
    if base > Base::MAX { return BASE_MAX; }

    let offset = offset_alphanumerical!(character);
    let digit = character - offset;
    if digit >= base { return OUT_OF_RANGE; }
    return digit;
}

#[deprecated(since = "0.1.1-dev", note = "will use offset based checking and parsing")]
#[must_use]
#[inline]
pub const fn check_custom(character: ascii, base: u8) -> AsciiDigitCustomBase {
    if base < Base::MIN {
        return AsciiDigitCustomBase::BaseMin;
    }
    if base > Base::MAX {
        return AsciiDigitCustomBase::BaseMax;
    }

    #[rustfmt::skip]
    let offset = match character {
        range_digit_ascii!()     => Base::DIGIT_ASCII_OFFSET,
        range_uppercase_ascii!() => Base::UPPERCASE_ASCII_OFFSET,
        range_lowercase_ascii!() => Base::LOWERCASE_ASCII_OFFSET,
        b'_'                     => return AsciiDigitCustomBase::Underscore,
        b'.'                     => return AsciiDigitCustomBase::Dot,
        _                        => return AsciiDigitCustomBase::Other,
    };

    let digit = character - offset;
    if digit >= base {
        return AsciiDigitCustomBase::OutOfRange;
    }
    return AsciiDigitCustomBase::Ok;
}

#[deprecated(since = "0.1.1-dev", note = "will use offset based checking and parsing")]
#[must_use]
#[inline]
pub const fn parse_custom(character: ascii, base: u8) -> DigitCustomBase {
    if base < Base::MIN {
        return DigitCustomBase::BaseMin;
    }
    if base > Base::MAX {
        return DigitCustomBase::BaseMax;
    }

    #[rustfmt::skip]
    let offset = match character {
        range_digit_ascii!()     => Base::DIGIT_ASCII_OFFSET,
        range_uppercase_ascii!() => Base::UPPERCASE_ASCII_OFFSET,
        range_lowercase_ascii!() => Base::LOWERCASE_ASCII_OFFSET,
        b'_'                     => return DigitCustomBase::Underscore,
        b'.'                     => return DigitCustomBase::Dot,
        _                        => return DigitCustomBase::Other,
    };

    let digit = character - offset;
    if digit >= base {
        return DigitCustomBase::OutOfRange;
    }
    return DigitCustomBase::Ok(digit);
}

#[deprecated(since = "0.1.1-dev", note = "will be removed to avoid inconsistencies")]
#[must_use]
#[inline]
pub const fn check_tally(character: ascii, tally_symbol: ascii) -> AsciiDigit {
    #[rustfmt::skip]
    return match character {
        b'_'                           => AsciiDigit::Underscore,
        b'.'                           => AsciiDigit::Dot,
        _ if character == tally_symbol => AsciiDigit::Ok,
        _                              => AsciiDigit::Other,
    };
}

#[deprecated(since = "0.1.1-dev", note = "will be removed to avoid inconsistencies")]
#[must_use]
#[inline]
pub const fn parse_tally(character: ascii, tally_symbol: ascii) -> Digit {
    #[rustfmt::skip]
    return match character {
        b'_'                           => Digit::Underscore,
        b'.'                           => Digit::Dot,
        _ if character == tally_symbol => Digit::Ok(1),
        _                              => Digit::Other,
    };
}

// IDEA(stefano): introduce loops to check every ascii combination
#[expect(unused_imports, unreachable_patterns, dead_code)]
#[cfg(test)]
#[rustfmt::skip]
mod tests {
    macro_rules! test_assert {
        ($expression:expr, == $pattern:pat $(if $guard:expr)? $(,)?) => {
            const _: () = match $expression {
                $pattern $(if $guard)? => {},
                _ => panic!(),
            };
        };
        ($expression:expr, != $pattern:pat $(if $guard:expr)? $(,)?) => {
            const _: () = match $expression {
                $pattern $(if $guard)? => panic!(),
                _ => {},
            };
        };
    }

    mod _0_1_1_dev_functionality {
        use crate::digit::*;

        test_assert!(check_binary_offset(b'0'), == offset if offset < INVALID && (b'0' - offset) == 0);
        test_assert!(check_binary_offset(b'1'), == offset if offset < INVALID && (b'1' - offset) == 1);
        test_assert!(check_binary_offset(b'2'), == offset if offset == OUT_OF_RANGE);
        test_assert!(check_binary_offset(b'a'), == offset if offset == OUT_OF_RANGE);
        test_assert!(check_binary_offset(b'Z'), == offset if offset == OUT_OF_RANGE);
        test_assert!(check_binary_offset(b'_'), == offset if offset == INVALID);
        test_assert!(check_binary_offset(b'.'), == offset if offset == INVALID);
        test_assert!(check_binary_offset(b'@'), == offset if offset == INVALID);

        test_assert!(parse_binary_offset(b'0'), == 0);
        test_assert!(parse_binary_offset(b'1'), == 1);
        test_assert!(parse_binary_offset(b'2'), == digit if digit == OUT_OF_RANGE);
        test_assert!(parse_binary_offset(b'a'), == digit if digit == OUT_OF_RANGE);
        test_assert!(parse_binary_offset(b'Z'), == digit if digit == OUT_OF_RANGE);
        test_assert!(parse_binary_offset(b'_'), == digit if digit == INVALID);
        test_assert!(parse_binary_offset(b'.'), == digit if digit == INVALID);
        test_assert!(parse_binary_offset(b'@'), == digit if digit == INVALID);

        test_assert!(check_octal_offset(b'0'), == offset if offset < INVALID && (b'0' - offset) == 0);
        test_assert!(check_octal_offset(b'1'), == offset if offset < INVALID && (b'1' - offset) == 1);
        test_assert!(check_octal_offset(b'2'), == offset if offset < INVALID && (b'2' - offset) == 2);
        test_assert!(check_octal_offset(b'3'), == offset if offset < INVALID && (b'3' - offset) == 3);
        test_assert!(check_octal_offset(b'4'), == offset if offset < INVALID && (b'4' - offset) == 4);
        test_assert!(check_octal_offset(b'5'), == offset if offset < INVALID && (b'5' - offset) == 5);
        test_assert!(check_octal_offset(b'6'), == offset if offset < INVALID && (b'6' - offset) == 6);
        test_assert!(check_octal_offset(b'7'), == offset if offset < INVALID && (b'7' - offset) == 7);
        test_assert!(check_octal_offset(b'8'), == OUT_OF_RANGE);
        test_assert!(check_octal_offset(b'9'), == OUT_OF_RANGE);
        test_assert!(check_octal_offset(b'a'), == OUT_OF_RANGE);
        test_assert!(check_octal_offset(b'Z'), == OUT_OF_RANGE);
        test_assert!(check_octal_offset(b'_'), == INVALID);
        test_assert!(check_octal_offset(b'.'), == INVALID);
        test_assert!(check_octal_offset(b'@'), == INVALID);

        test_assert!(parse_octal_offset(b'0'), == 0);
        test_assert!(parse_octal_offset(b'1'), == 1);
        test_assert!(parse_octal_offset(b'2'), == 2);
        test_assert!(parse_octal_offset(b'3'), == 3);
        test_assert!(parse_octal_offset(b'4'), == 4);
        test_assert!(parse_octal_offset(b'5'), == 5);
        test_assert!(parse_octal_offset(b'6'), == 6);
        test_assert!(parse_octal_offset(b'7'), == 7);
        test_assert!(parse_octal_offset(b'8'), == OUT_OF_RANGE);
        test_assert!(parse_octal_offset(b'9'), == OUT_OF_RANGE);
        test_assert!(parse_octal_offset(b'a'), == OUT_OF_RANGE);
        test_assert!(parse_octal_offset(b'Z'), == OUT_OF_RANGE);
        test_assert!(parse_octal_offset(b'_'), == INVALID);
        test_assert!(parse_octal_offset(b'.'), == INVALID);
        test_assert!(parse_octal_offset(b'@'), == INVALID);

        test_assert!(check_decimal_offset(b'0'), == offset if offset < INVALID && (b'0' - offset) == 0);
        test_assert!(check_decimal_offset(b'1'), == offset if offset < INVALID && (b'1' - offset) == 1);
        test_assert!(check_decimal_offset(b'2'), == offset if offset < INVALID && (b'2' - offset) == 2);
        test_assert!(check_decimal_offset(b'3'), == offset if offset < INVALID && (b'3' - offset) == 3);
        test_assert!(check_decimal_offset(b'4'), == offset if offset < INVALID && (b'4' - offset) == 4);
        test_assert!(check_decimal_offset(b'5'), == offset if offset < INVALID && (b'5' - offset) == 5);
        test_assert!(check_decimal_offset(b'6'), == offset if offset < INVALID && (b'6' - offset) == 6);
        test_assert!(check_decimal_offset(b'7'), == offset if offset < INVALID && (b'7' - offset) == 7);
        test_assert!(check_decimal_offset(b'8'), == offset if offset < INVALID && (b'8' - offset) == 8);
        test_assert!(check_decimal_offset(b'9'), == offset if offset < INVALID && (b'9' - offset) == 9);
        test_assert!(check_decimal_offset(b'a'), == OUT_OF_RANGE);
        test_assert!(check_decimal_offset(b'Z'), == OUT_OF_RANGE);
        test_assert!(check_decimal_offset(b'_'), == INVALID);
        test_assert!(check_decimal_offset(b'.'), == INVALID);
        test_assert!(check_decimal_offset(b'@'), == INVALID);

        test_assert!(parse_decimal_offset(b'0'), == 0);
        test_assert!(parse_decimal_offset(b'1'), == 1);
        test_assert!(parse_decimal_offset(b'2'), == 2);
        test_assert!(parse_decimal_offset(b'3'), == 3);
        test_assert!(parse_decimal_offset(b'4'), == 4);
        test_assert!(parse_decimal_offset(b'5'), == 5);
        test_assert!(parse_decimal_offset(b'6'), == 6);
        test_assert!(parse_decimal_offset(b'7'), == 7);
        test_assert!(parse_decimal_offset(b'8'), == 8);
        test_assert!(parse_decimal_offset(b'9'), == 9);
        test_assert!(parse_decimal_offset(b'a'), == OUT_OF_RANGE);
        test_assert!(parse_decimal_offset(b'Z'), == OUT_OF_RANGE);
        test_assert!(parse_decimal_offset(b'_'), == INVALID);
        test_assert!(parse_decimal_offset(b'.'), == INVALID);
        test_assert!(parse_decimal_offset(b'@'), == INVALID);

        test_assert!(check_hexadecimal_offset(b'0'), == offset if offset < INVALID && (b'0' - offset) == 0);
        test_assert!(check_hexadecimal_offset(b'1'), == offset if offset < INVALID && (b'1' - offset) == 1);
        test_assert!(check_hexadecimal_offset(b'2'), == offset if offset < INVALID && (b'2' - offset) == 2);
        test_assert!(check_hexadecimal_offset(b'3'), == offset if offset < INVALID && (b'3' - offset) == 3);
        test_assert!(check_hexadecimal_offset(b'4'), == offset if offset < INVALID && (b'4' - offset) == 4);
        test_assert!(check_hexadecimal_offset(b'5'), == offset if offset < INVALID && (b'5' - offset) == 5);
        test_assert!(check_hexadecimal_offset(b'6'), == offset if offset < INVALID && (b'6' - offset) == 6);
        test_assert!(check_hexadecimal_offset(b'7'), == offset if offset < INVALID && (b'7' - offset) == 7);
        test_assert!(check_hexadecimal_offset(b'8'), == offset if offset < INVALID && (b'8' - offset) == 8);
        test_assert!(check_hexadecimal_offset(b'9'), == offset if offset < INVALID && (b'9' - offset) == 9);
        test_assert!(check_hexadecimal_offset(b'A'), == offset if offset < INVALID && (b'A' - offset) == 10);
        test_assert!(check_hexadecimal_offset(b'a'), == offset if offset < INVALID && (b'a' - offset) == 10);
        test_assert!(check_hexadecimal_offset(b'B'), == offset if offset < INVALID && (b'B' - offset) == 11);
        test_assert!(check_hexadecimal_offset(b'b'), == offset if offset < INVALID && (b'b' - offset) == 11);
        test_assert!(check_hexadecimal_offset(b'C'), == offset if offset < INVALID && (b'C' - offset) == 12);
        test_assert!(check_hexadecimal_offset(b'c'), == offset if offset < INVALID && (b'c' - offset) == 12);
        test_assert!(check_hexadecimal_offset(b'D'), == offset if offset < INVALID && (b'D' - offset) == 13);
        test_assert!(check_hexadecimal_offset(b'd'), == offset if offset < INVALID && (b'd' - offset) == 13);
        test_assert!(check_hexadecimal_offset(b'E'), == offset if offset < INVALID && (b'E' - offset) == 14);
        test_assert!(check_hexadecimal_offset(b'e'), == offset if offset < INVALID && (b'e' - offset) == 14);
        test_assert!(check_hexadecimal_offset(b'F'), == offset if offset < INVALID && (b'F' - offset) == 15);
        test_assert!(check_hexadecimal_offset(b'f'), == offset if offset < INVALID && (b'f' - offset) == 15);
        test_assert!(check_hexadecimal_offset(b'g'), == OUT_OF_RANGE);
        test_assert!(check_hexadecimal_offset(b'Z'), == OUT_OF_RANGE);
        test_assert!(check_hexadecimal_offset(b'_'), == INVALID);
        test_assert!(check_hexadecimal_offset(b'.'), == INVALID);
        test_assert!(check_hexadecimal_offset(b'@'), == INVALID);

        test_assert!(parse_hexadecimal_offset(b'0'), == 0);
        test_assert!(parse_hexadecimal_offset(b'1'), == 1);
        test_assert!(parse_hexadecimal_offset(b'2'), == 2);
        test_assert!(parse_hexadecimal_offset(b'3'), == 3);
        test_assert!(parse_hexadecimal_offset(b'4'), == 4);
        test_assert!(parse_hexadecimal_offset(b'5'), == 5);
        test_assert!(parse_hexadecimal_offset(b'6'), == 6);
        test_assert!(parse_hexadecimal_offset(b'7'), == 7);
        test_assert!(parse_hexadecimal_offset(b'8'), == 8);
        test_assert!(parse_hexadecimal_offset(b'9'), == 9);
        test_assert!(parse_hexadecimal_offset(b'A'), == 10);
        test_assert!(parse_hexadecimal_offset(b'a'), == 10);
        test_assert!(parse_hexadecimal_offset(b'B'), == 11);
        test_assert!(parse_hexadecimal_offset(b'b'), == 11);
        test_assert!(parse_hexadecimal_offset(b'C'), == 12);
        test_assert!(parse_hexadecimal_offset(b'c'), == 12);
        test_assert!(parse_hexadecimal_offset(b'D'), == 13);
        test_assert!(parse_hexadecimal_offset(b'd'), == 13);
        test_assert!(parse_hexadecimal_offset(b'E'), == 14);
        test_assert!(parse_hexadecimal_offset(b'e'), == 14);
        test_assert!(parse_hexadecimal_offset(b'F'), == 15);
        test_assert!(parse_hexadecimal_offset(b'f'), == 15);
        test_assert!(parse_hexadecimal_offset(b'g'), == OUT_OF_RANGE);
        test_assert!(parse_hexadecimal_offset(b'Z'), == OUT_OF_RANGE);
        test_assert!(parse_hexadecimal_offset(b'_'), == INVALID);
        test_assert!(parse_hexadecimal_offset(b'.'), == INVALID);
        test_assert!(parse_hexadecimal_offset(b'@'), == INVALID);

        test_assert!(check_custom_offset(b'g', 00), == BASE_MIN);
        test_assert!(check_custom_offset(b'g', 01), == BASE_MIN);
        test_assert!(check_custom_offset(b'g', 02), != BASE_MIN);
        test_assert!(check_custom_offset(b'g', 36), != BASE_MAX);
        test_assert!(check_custom_offset(b'g', 37), == BASE_MAX);
        test_assert!(check_custom_offset(b'g', 17), == offset if offset < INVALID && (b'g' - offset) == 16);
        test_assert!(check_custom_offset(b'z', 36), == offset if offset < INVALID && (b'z' - offset) == 35);
        test_assert!(check_custom_offset(b'z', 21), == OUT_OF_RANGE);
        test_assert!(check_custom_offset(b'_', 36), == INVALID);
        test_assert!(check_custom_offset(b'.', 36), == INVALID);
        test_assert!(check_custom_offset(b'@', 36), == INVALID);

        test_assert!(parse_custom_offset(b'g', 00), == BASE_MIN);
        test_assert!(parse_custom_offset(b'g', 01), == BASE_MIN);
        test_assert!(parse_custom_offset(b'g', 02), != BASE_MIN);
        test_assert!(parse_custom_offset(b'g', 36), != BASE_MAX);
        test_assert!(parse_custom_offset(b'g', 37), == BASE_MAX);
        test_assert!(parse_custom_offset(b'g', 17), == 16);
        test_assert!(parse_custom_offset(b'z', 36), == 35);
        test_assert!(parse_custom_offset(b'z', 21), == OUT_OF_RANGE);
        test_assert!(parse_custom_offset(b'_', 36), == INVALID);
        test_assert!(parse_custom_offset(b'.', 36), == INVALID);
        test_assert!(parse_custom_offset(b'@', 36), == INVALID);
    }

    mod _0_1_1_dev_backwards_compatibility {
        use core::ops::RangeInclusive;
        use crate::{ascii, utf32};
        use crate::digit::{
            Ascii, AsciiRange,

            Offset, OffsetCustomBase, DigitOffset, DigitOffsetCustomBase,
            BASE_MAX, BASE_MIN, INVALID, OUT_OF_RANGE,

            Base,

            check_offset,
            parse_offset,

            range_binary_digit_ascii,
            range_binary_digit,
            range_binary_digit_out_of_range_ascii,
            range_binary_digit_out_of_range,
            range_binary_uppercase_out_of_range_ascii,
            range_binary_uppercase_out_of_range,
            range_binary_lowercase_out_of_range_ascii,
            range_binary_lowercase_out_of_range,
            range_binary_letter_out_of_range_ascii,
            range_binary_letter_out_of_range,
            range_binary_out_of_range_ascii,
            range_binary_out_of_range,
            is_binary_digit,
            is_binary_digit_out_of_range,
            is_binary_uppercase_out_of_range,
            is_binary_lowercase_out_of_range,
            is_binary_letter_out_of_range,
            is_binary,
            is_binary_out_of_range,
            check_binary_offset,
            parse_binary_offset,

            range_octal_digit_ascii,
            range_octal_digit,
            range_octal_digit_out_of_range_ascii,
            range_octal_digit_out_of_range,
            range_octal_uppercase_out_of_range_ascii,
            range_octal_uppercase_out_of_range,
            range_octal_lowercase_out_of_range_ascii,
            range_octal_lowercase_out_of_range,
            range_octal_letter_out_of_range_ascii,
            range_octal_letter_out_of_range,
            range_octal_out_of_range_ascii,
            range_octal_out_of_range,
            is_octal_digit,
            is_octal_digit_out_of_range,
            is_octal_uppercase_out_of_range,
            is_octal_lowercase_out_of_range,
            is_octal_letter_out_of_range,
            is_octal,
            is_octal_out_of_range,
            check_octal_offset,
            parse_octal_offset,

            range_decimal_digit_ascii,
            range_decimal_digit,
            range_decimal_uppercase_out_of_range_ascii,
            range_decimal_uppercase_out_of_range,
            range_decimal_lowercase_out_of_range_ascii,
            range_decimal_lowercase_out_of_range,
            range_decimal_letter_out_of_range_ascii,
            range_decimal_letter_out_of_range,
            range_decimal_out_of_range_ascii,
            range_decimal_out_of_range,
            is_decimal_digit,
            is_decimal_uppercase_out_of_range,
            is_decimal_lowercase_out_of_range,
            is_decimal_letter_out_of_range,
            is_decimal,
            is_decimal_out_of_range,
            check_decimal_offset,
            parse_decimal_offset,

            range_hexadecimal_digit_ascii,
            range_hexadecimal_digit,
            range_hexadecimal_uppercase_ascii,
            range_hexadecimal_uppercase,
            range_hexadecimal_lowercase_ascii,
            range_hexadecimal_lowercase,
            range_hexadecimal_letter_ascii,
            range_hexadecimal_letter,
            range_hexadecimal_uppercase_out_of_range_ascii,
            range_hexadecimal_uppercase_out_of_range,
            range_hexadecimal_lowercase_out_of_range_ascii,
            range_hexadecimal_lowercase_out_of_range,
            range_hexadecimal_letter_out_of_range_ascii,
            range_hexadecimal_letter_out_of_range,
            range_hexadecimal_ascii,
            range_hexadecimal,
            range_hexadecimal_out_of_range_ascii,
            range_hexadecimal_out_of_range,
            is_hexadecimal_digit,
            is_hexadecimal_uppercase,
            is_hexadecimal_lowercase,
            is_hexadecimal_letter,
            is_hexadecimal_uppercase_out_of_range,
            is_hexadecimal_lowercase_out_of_range,
            is_hexadecimal_letter_out_of_range,
            is_hexadecimal,
            is_hexadecimal_out_of_range,
            check_hexadecimal_offset,
            parse_hexadecimal_offset,

            range_digit_ascii,
            range_digit,
            range_uppercase,
            range_uppercase_ascii,
            range_lowercase,
            range_lowercase_ascii,
            range_letter,
            range_letter_ascii,
            range_alphanumerical,
            range_alphanumerical_ascii,
            is_digit,
            is_uppercase,
            is_lowercase,
            is_letter,
            is_alphanumerical,
            check_custom_offset,
            parse_custom_offset,
        };

        const _: () = {
            fn new<I: Ascii>(start: I, end: I) -> AsciiRange<I> {
                return AsciiRange::<I>::new(start, end);
            }
        };

        const _: ascii                 = Base::BINARY_DIGIT_ASCII_START;
        const _: ascii                 = Base::BINARY_DIGIT_ASCII_END;
        const _: AsciiRange<ascii>     = Base::BINARY_DIGIT_ASCII;
        const _: ascii                 = Base::BINARY_DIGIT_OUT_OF_RANGE_ASCII_START;
        const _: ascii                 = Base::BINARY_DIGIT_OUT_OF_RANGE_ASCII_END;
        const _: AsciiRange<ascii>     = Base::BINARY_DIGIT_OUT_OF_RANGE_ASCII;
        const _: ascii                 = Base::BINARY_UPPERCASE_OUT_OF_RANGE_ASCII_START;
        const _: ascii                 = Base::BINARY_UPPERCASE_OUT_OF_RANGE_ASCII_END;
        const _: AsciiRange<ascii>     = Base::BINARY_UPPERCASE_OUT_OF_RANGE_ASCII;
        const _: ascii                 = Base::BINARY_LOWERCASE_OUT_OF_RANGE_ASCII_START;
        const _: ascii                 = Base::BINARY_LOWERCASE_OUT_OF_RANGE_ASCII_END;
        const _: AsciiRange<ascii>     = Base::BINARY_LOWERCASE_OUT_OF_RANGE_ASCII;
        const _: RangeInclusive<ascii> = Base::BINARY_DIGIT_ASCII_OPS;
        const _: RangeInclusive<ascii> = Base::BINARY_DIGIT_OUT_OF_RANGE_ASCII_OPS;
        const _: RangeInclusive<ascii> = Base::BINARY_UPPERCASE_OUT_OF_RANGE_ASCII_OPS;
        const _: RangeInclusive<ascii> = Base::BINARY_LOWERCASE_OUT_OF_RANGE_ASCII_OPS;
        const _: utf32                 = Base::BINARY_DIGIT_START;
        const _: utf32                 = Base::BINARY_DIGIT_END;
        const _: AsciiRange<utf32>     = Base::BINARY_DIGIT;
        const _: utf32                 = Base::BINARY_DIGIT_OUT_OF_RANGE_START;
        const _: utf32                 = Base::BINARY_DIGIT_OUT_OF_RANGE_END;
        const _: AsciiRange<utf32>     = Base::BINARY_DIGIT_OUT_OF_RANGE;
        const _: utf32                 = Base::BINARY_UPPERCASE_OUT_OF_RANGE_START;
        const _: utf32                 = Base::BINARY_UPPERCASE_OUT_OF_RANGE_END;
        const _: AsciiRange<utf32>     = Base::BINARY_UPPERCASE_OUT_OF_RANGE;
        const _: utf32                 = Base::BINARY_LOWERCASE_OUT_OF_RANGE_START;
        const _: utf32                 = Base::BINARY_LOWERCASE_OUT_OF_RANGE_END;
        const _: AsciiRange<utf32>     = Base::BINARY_LOWERCASE_OUT_OF_RANGE;
        const _: RangeInclusive<utf32> = Base::BINARY_DIGIT_OPS;
        const _: RangeInclusive<utf32> = Base::BINARY_DIGIT_OUT_OF_RANGE_OPS;
        const _: RangeInclusive<utf32> = Base::BINARY_UPPERCASE_OUT_OF_RANGE_OPS;
        const _: RangeInclusive<utf32> = Base::BINARY_LOWERCASE_OUT_OF_RANGE_OPS;

        const _: ascii                 = Base::OCTAL_DIGIT_ASCII_START;
        const _: ascii                 = Base::OCTAL_DIGIT_ASCII_END;
        const _: AsciiRange<ascii>     = Base::OCTAL_DIGIT_ASCII;
        const _: ascii                 = Base::OCTAL_DIGIT_OUT_OF_RANGE_ASCII_START;
        const _: ascii                 = Base::OCTAL_DIGIT_OUT_OF_RANGE_ASCII_END;
        const _: AsciiRange<ascii>     = Base::OCTAL_DIGIT_OUT_OF_RANGE_ASCII;
        const _: ascii                 = Base::OCTAL_UPPERCASE_OUT_OF_RANGE_ASCII_START;
        const _: ascii                 = Base::OCTAL_UPPERCASE_OUT_OF_RANGE_ASCII_END;
        const _: AsciiRange<ascii>     = Base::OCTAL_UPPERCASE_OUT_OF_RANGE_ASCII;
        const _: ascii                 = Base::OCTAL_LOWERCASE_OUT_OF_RANGE_ASCII_START;
        const _: ascii                 = Base::OCTAL_LOWERCASE_OUT_OF_RANGE_ASCII_END;
        const _: AsciiRange<ascii>     = Base::OCTAL_LOWERCASE_OUT_OF_RANGE_ASCII;
        const _: RangeInclusive<ascii> = Base::OCTAL_DIGIT_ASCII_OPS;
        const _: RangeInclusive<ascii> = Base::OCTAL_DIGIT_OUT_OF_RANGE_ASCII_OPS;
        const _: RangeInclusive<ascii> = Base::OCTAL_UPPERCASE_OUT_OF_RANGE_ASCII_OPS;
        const _: RangeInclusive<ascii> = Base::OCTAL_LOWERCASE_OUT_OF_RANGE_ASCII_OPS;
        const _: utf32                 = Base::OCTAL_DIGIT_START;
        const _: utf32                 = Base::OCTAL_DIGIT_END;
        const _: AsciiRange<utf32>     = Base::OCTAL_DIGIT;
        const _: utf32                 = Base::OCTAL_DIGIT_OUT_OF_RANGE_START;
        const _: utf32                 = Base::OCTAL_DIGIT_OUT_OF_RANGE_END;
        const _: AsciiRange<utf32>     = Base::OCTAL_DIGIT_OUT_OF_RANGE;
        const _: utf32                 = Base::OCTAL_UPPERCASE_OUT_OF_RANGE_START;
        const _: utf32                 = Base::OCTAL_UPPERCASE_OUT_OF_RANGE_END;
        const _: AsciiRange<utf32>     = Base::OCTAL_UPPERCASE_OUT_OF_RANGE;
        const _: utf32                 = Base::OCTAL_LOWERCASE_OUT_OF_RANGE_START;
        const _: utf32                 = Base::OCTAL_LOWERCASE_OUT_OF_RANGE_END;
        const _: AsciiRange<utf32>     = Base::OCTAL_LOWERCASE_OUT_OF_RANGE;
        const _: RangeInclusive<utf32> = Base::OCTAL_DIGIT_OPS;
        const _: RangeInclusive<utf32> = Base::OCTAL_DIGIT_OUT_OF_RANGE_OPS;
        const _: RangeInclusive<utf32> = Base::OCTAL_UPPERCASE_OUT_OF_RANGE_OPS;
        const _: RangeInclusive<utf32> = Base::OCTAL_LOWERCASE_OUT_OF_RANGE_OPS;

        const _: ascii                 = Base::DECIMAL_DIGIT_ASCII_START;
        const _: ascii                 = Base::DECIMAL_DIGIT_ASCII_END;
        const _: AsciiRange<ascii>     = Base::DECIMAL_DIGIT_ASCII;
        const _: ascii                 = Base::DECIMAL_UPPERCASE_OUT_OF_RANGE_ASCII_START;
        const _: ascii                 = Base::DECIMAL_UPPERCASE_OUT_OF_RANGE_ASCII_END;
        const _: AsciiRange<ascii>     = Base::DECIMAL_UPPERCASE_OUT_OF_RANGE_ASCII;
        const _: ascii                 = Base::DECIMAL_LOWERCASE_OUT_OF_RANGE_ASCII_START;
        const _: ascii                 = Base::DECIMAL_LOWERCASE_OUT_OF_RANGE_ASCII_END;
        const _: AsciiRange<ascii>     = Base::DECIMAL_LOWERCASE_OUT_OF_RANGE_ASCII;
        const _: RangeInclusive<ascii> = Base::DECIMAL_DIGIT_ASCII_OPS;
        const _: RangeInclusive<ascii> = Base::DECIMAL_UPPERCASE_OUT_OF_RANGE_ASCII_OPS;
        const _: RangeInclusive<ascii> = Base::DECIMAL_LOWERCASE_OUT_OF_RANGE_ASCII_OPS;
        const _: utf32                 = Base::DECIMAL_DIGIT_START;
        const _: utf32                 = Base::DECIMAL_DIGIT_END;
        const _: AsciiRange<utf32>     = Base::DECIMAL_DIGIT;
        const _: utf32                 = Base::DECIMAL_UPPERCASE_OUT_OF_RANGE_START;
        const _: utf32                 = Base::DECIMAL_UPPERCASE_OUT_OF_RANGE_END;
        const _: AsciiRange<utf32>     = Base::DECIMAL_UPPERCASE_OUT_OF_RANGE;
        const _: utf32                 = Base::DECIMAL_LOWERCASE_OUT_OF_RANGE_START;
        const _: utf32                 = Base::DECIMAL_LOWERCASE_OUT_OF_RANGE_END;
        const _: AsciiRange<utf32>     = Base::DECIMAL_LOWERCASE_OUT_OF_RANGE;
        const _: RangeInclusive<utf32> = Base::DECIMAL_DIGIT_OPS;
        const _: RangeInclusive<utf32> = Base::DECIMAL_UPPERCASE_OUT_OF_RANGE_OPS;
        const _: RangeInclusive<utf32> = Base::DECIMAL_LOWERCASE_OUT_OF_RANGE_OPS;

        const _: ascii                 = Base::HEXADECIMAL_DIGIT_ASCII_START;
        const _: ascii                 = Base::HEXADECIMAL_DIGIT_ASCII_END;
        const _: AsciiRange<ascii>     = Base::HEXADECIMAL_DIGIT_ASCII;
        const _: ascii                 = Base::HEXADECIMAL_UPPERCASE_ASCII_START;
        const _: ascii                 = Base::HEXADECIMAL_UPPERCASE_ASCII_END;
        const _: AsciiRange<ascii>     = Base::HEXADECIMAL_UPPERCASE_ASCII;
        const _: ascii                 = Base::HEXADECIMAL_LOWERCASE_ASCII_START;
        const _: ascii                 = Base::HEXADECIMAL_LOWERCASE_ASCII_END;
        const _: AsciiRange<ascii>     = Base::HEXADECIMAL_LOWERCASE_ASCII;
        const _: ascii                 = Base::HEXADECIMAL_UPPERCASE_OUT_OF_RANGE_ASCII_START;
        const _: ascii                 = Base::HEXADECIMAL_UPPERCASE_OUT_OF_RANGE_ASCII_END;
        const _: AsciiRange<ascii>     = Base::HEXADECIMAL_UPPERCASE_OUT_OF_RANGE_ASCII;
        const _: ascii                 = Base::HEXADECIMAL_LOWERCASE_OUT_OF_RANGE_ASCII_START;
        const _: ascii                 = Base::HEXADECIMAL_LOWERCASE_OUT_OF_RANGE_ASCII_END;
        const _: AsciiRange<ascii>     = Base::HEXADECIMAL_LOWERCASE_OUT_OF_RANGE_ASCII;
        const _: RangeInclusive<ascii> = Base::HEXADECIMAL_DIGIT_ASCII_OPS;
        const _: RangeInclusive<ascii> = Base::HEXADECIMAL_UPPERCASE_ASCII_OPS;
        const _: RangeInclusive<ascii> = Base::HEXADECIMAL_LOWERCASE_ASCII_OPS;
        const _: RangeInclusive<ascii> = Base::HEXADECIMAL_UPPERCASE_OUT_OF_RANGE_ASCII_OPS;
        const _: RangeInclusive<ascii> = Base::HEXADECIMAL_LOWERCASE_OUT_OF_RANGE_ASCII_OPS;
        const _: utf32                 = Base::HEXADECIMAL_DIGIT_START;
        const _: utf32                 = Base::HEXADECIMAL_DIGIT_END;
        const _: AsciiRange<utf32>     = Base::HEXADECIMAL_DIGIT;
        const _: utf32                 = Base::HEXADECIMAL_UPPERCASE_START;
        const _: utf32                 = Base::HEXADECIMAL_UPPERCASE_END;
        const _: AsciiRange<utf32>     = Base::HEXADECIMAL_UPPERCASE;
        const _: utf32                 = Base::HEXADECIMAL_LOWERCASE_START;
        const _: utf32                 = Base::HEXADECIMAL_LOWERCASE_END;
        const _: AsciiRange<utf32>     = Base::HEXADECIMAL_LOWERCASE;
        const _: utf32                 = Base::HEXADECIMAL_UPPERCASE_OUT_OF_RANGE_START;
        const _: utf32                 = Base::HEXADECIMAL_UPPERCASE_OUT_OF_RANGE_END;
        const _: AsciiRange<utf32>     = Base::HEXADECIMAL_UPPERCASE_OUT_OF_RANGE;
        const _: utf32                 = Base::HEXADECIMAL_LOWERCASE_OUT_OF_RANGE_START;
        const _: utf32                 = Base::HEXADECIMAL_LOWERCASE_OUT_OF_RANGE_END;
        const _: AsciiRange<utf32>     = Base::HEXADECIMAL_LOWERCASE_OUT_OF_RANGE;
        const _: RangeInclusive<utf32> = Base::HEXADECIMAL_DIGIT_OPS;
        const _: RangeInclusive<utf32> = Base::HEXADECIMAL_UPPERCASE_OPS;
        const _: RangeInclusive<utf32> = Base::HEXADECIMAL_LOWERCASE_OPS;
        const _: RangeInclusive<utf32> = Base::HEXADECIMAL_UPPERCASE_OUT_OF_RANGE_OPS;
        const _: RangeInclusive<utf32> = Base::HEXADECIMAL_LOWERCASE_OUT_OF_RANGE_OPS;

        const _: ascii                      = Base::DIGIT_ASCII_START;
        const _: ascii                      = Base::DIGIT_ASCII_END;
        const _: AsciiRange<ascii>          = Base::DIGIT_ASCII;
        const _: ascii                      = Base::UPPERCASE_ASCII_START;
        const _: ascii                      = Base::UPPERCASE_ASCII_END;
        const _: AsciiRange<ascii>          = Base::UPPERCASE_ASCII;
        const _: ascii                      = Base::LOWERCASE_ASCII_START;
        const _: ascii                      = Base::LOWERCASE_ASCII_END;
        const _: AsciiRange<ascii>          = Base::LOWERCASE_ASCII;
        const _: RangeInclusive<ascii>      = Base::DIGIT_ASCII_OPS;
        const _: RangeInclusive<ascii>      = Base::UPPERCASE_ASCII_OPS;
        const _: RangeInclusive<ascii>      = Base::LOWERCASE_ASCII_OPS;
        const _: utf32                      = Base::DIGIT_START;
        const _: utf32                      = Base::DIGIT_END;
        const _: AsciiRange<utf32>          = Base::DIGIT;
        const _: utf32                      = Base::UPPERCASE_START;
        const _: utf32                      = Base::UPPERCASE_END;
        const _: AsciiRange<utf32>          = Base::UPPERCASE;
        const _: utf32                      = Base::LOWERCASE_START;
        const _: utf32                      = Base::LOWERCASE_END;
        const _: AsciiRange<utf32>          = Base::LOWERCASE;
        const _: RangeInclusive<utf32>      = Base::DIGIT_OPS;
        const _: RangeInclusive<utf32>      = Base::UPPERCASE_OPS;
        const _: RangeInclusive<utf32>      = Base::LOWERCASE_OPS;
        const _: [AsciiRange<ascii>; 1]     = Base::DIGIT_RANGES_ASCII;
        const _: [AsciiRange<ascii>; 2]     = Base::LETTERS_RANGES_ASCII;
        const _: [AsciiRange<ascii>; 3]     = Base::ALPHANUMERICAL_RANGES_ASCII;
        const _: [AsciiRange<utf32>; 1]     = Base::DIGIT_RANGES;
        const _: [AsciiRange<utf32>; 2]     = Base::LETTERS_RANGES;
        const _: [AsciiRange<utf32>; 3]     = Base::ALPHANUMERICAL_RANGES;
        const _: [RangeInclusive<ascii>; 1] = Base::DIGIT_RANGES_ASCII_OPS;
        const _: [RangeInclusive<ascii>; 2] = Base::LETTERS_RANGES_ASCII_OPS;
        const _: [RangeInclusive<ascii>; 3] = Base::ALPHANUMERICAL_RANGES_ASCII_OPS;
        const _: [RangeInclusive<utf32>; 1] = Base::DIGIT_RANGES_OPS;
        const _: [RangeInclusive<utf32>; 2] = Base::LETTERS_RANGES_OPS;
        const _: [RangeInclusive<utf32>; 3] = Base::ALPHANUMERICAL_RANGES_OPS;
        const _: u8                         = Base::DIGIT_ASCII_OFFSET;
        const _: u8                         = Base::UPPERCASE_ASCII_OFFSET;
        const _: u8                         = Base::LOWERCASE_ASCII_OFFSET;

        const _: fn(Base, ascii) -> Offset = Base::check_offset;
        const _: fn(Base, ascii) -> DigitOffset = Base::parse_offset;
        const _: OffsetCustomBase = BASE_MAX;
        const _: OffsetCustomBase = BASE_MIN;
        const _: Offset = INVALID;
        const _: Offset = OUT_OF_RANGE;

        const _: () = match b'0' {
            range_binary_digit_ascii!() => {},
            range_binary_digit_out_of_range_ascii!() => {},
            range_binary_uppercase_out_of_range_ascii!() => {},
            range_binary_lowercase_out_of_range_ascii!() => {},
            range_binary_letter_out_of_range_ascii!() => {},
            range_binary_out_of_range_ascii!() => {},
            _ => {},
        };
        const _: () = match '0' {
            range_binary_digit!() => {},
            range_binary_digit_out_of_range!() => {},
            range_binary_uppercase_out_of_range!() => {},
            range_binary_lowercase_out_of_range!() => {},
            range_binary_letter_out_of_range!() => {},
            range_binary_out_of_range!() => {},
            _ => {},
        };
        const _: fn(ascii) -> bool        = is_binary_digit;
        const _: fn(ascii) -> bool        = is_binary_digit_out_of_range;
        const _: fn(ascii) -> bool        = is_binary_uppercase_out_of_range;
        const _: fn(ascii) -> bool        = is_binary_lowercase_out_of_range;
        const _: fn(ascii) -> bool        = is_binary_letter_out_of_range;
        const _: fn(ascii) -> bool        = is_binary;
        const _: fn(ascii) -> bool        = is_binary_out_of_range;
        const _: fn(ascii) -> Offset      = check_binary_offset;
        const _: fn(ascii) -> DigitOffset = parse_binary_offset;

        const _: () = match b'0' {
            range_octal_digit_ascii!() => {},
            range_octal_digit_out_of_range_ascii!() => {},
            range_octal_uppercase_out_of_range_ascii!() => {},
            range_octal_lowercase_out_of_range_ascii!() => {},
            range_octal_letter_out_of_range_ascii!() => {},
            range_octal_out_of_range_ascii!() => {},
            _ => {},
        };
        const _: () = match '0' {
            range_octal_digit!() => {},
            range_octal_digit_out_of_range!() => {},
            range_octal_uppercase_out_of_range!() => {},
            range_octal_lowercase_out_of_range!() => {},
            range_octal_letter_out_of_range!() => {},
            range_octal_out_of_range!() => {},
            _ => {},
        };
        const _: fn(ascii) -> bool        = is_octal_digit;
        const _: fn(ascii) -> bool        = is_octal_digit_out_of_range;
        const _: fn(ascii) -> bool        = is_octal_uppercase_out_of_range;
        const _: fn(ascii) -> bool        = is_octal_lowercase_out_of_range;
        const _: fn(ascii) -> bool        = is_octal_letter_out_of_range;
        const _: fn(ascii) -> bool        = is_octal;
        const _: fn(ascii) -> bool        = is_octal_out_of_range;
        const _: fn(ascii) -> Offset      = check_octal_offset;
        const _: fn(ascii) -> DigitOffset = parse_octal_offset;

        const _: () = match b'0' {
            range_decimal_digit_ascii!() => {},
            range_decimal_uppercase_out_of_range_ascii!() => {},
            range_decimal_lowercase_out_of_range_ascii!() => {},
            range_decimal_letter_out_of_range_ascii!() => {},
            range_decimal_out_of_range_ascii!() => {},
            _ => {},
        };
        const _: () = match '0' {
            range_decimal_digit!() => {},
            range_decimal_uppercase_out_of_range!() => {},
            range_decimal_lowercase_out_of_range!() => {},
            range_decimal_letter_out_of_range!() => {},
            range_decimal_out_of_range!() => {},
            _ => {},
        };
        const _: fn(ascii) -> bool        = is_decimal_digit;
        const _: fn(ascii) -> bool        = is_decimal_uppercase_out_of_range;
        const _: fn(ascii) -> bool        = is_decimal_lowercase_out_of_range;
        const _: fn(ascii) -> bool        = is_decimal_letter_out_of_range;
        const _: fn(ascii) -> bool        = is_decimal;
        const _: fn(ascii) -> bool        = is_decimal_out_of_range;
        const _: fn(ascii) -> Offset      = check_decimal_offset;
        const _: fn(ascii) -> DigitOffset = parse_decimal_offset;

        const _: () = match b'0' {
            range_hexadecimal_digit_ascii!() => {},
            range_hexadecimal_uppercase_ascii!() => {},
            range_hexadecimal_lowercase_ascii!() => {},
            range_hexadecimal_letter_ascii!() => {},
            range_hexadecimal_uppercase_out_of_range_ascii!() => {},
            range_hexadecimal_lowercase_out_of_range_ascii!() => {},
            range_hexadecimal_letter_out_of_range_ascii!() => {},
            range_hexadecimal_ascii!() => {},
            range_hexadecimal_out_of_range_ascii!() => {},
            _ => {},
        };
        const _: () = match '0' {
            range_hexadecimal_digit!() => {},
            range_hexadecimal_uppercase!() => {},
            range_hexadecimal_lowercase!() => {},
            range_hexadecimal_letter!() => {},
            range_hexadecimal_uppercase_out_of_range!() => {},
            range_hexadecimal_lowercase_out_of_range!() => {},
            range_hexadecimal_letter_out_of_range!() => {},
            range_hexadecimal!() => {},
            range_hexadecimal_out_of_range!() => {},
            _ => {},
        };

        const _: fn(ascii) -> bool        = is_hexadecimal_digit;
        const _: fn(ascii) -> bool        = is_hexadecimal_uppercase;
        const _: fn(ascii) -> bool        = is_hexadecimal_lowercase;
        const _: fn(ascii) -> bool        = is_hexadecimal_letter;
        const _: fn(ascii) -> bool        = is_hexadecimal_uppercase_out_of_range;
        const _: fn(ascii) -> bool        = is_hexadecimal_lowercase_out_of_range;
        const _: fn(ascii) -> bool        = is_hexadecimal_letter_out_of_range;
        const _: fn(ascii) -> bool        = is_hexadecimal;
        const _: fn(ascii) -> bool        = is_hexadecimal_out_of_range;
        const _: fn(ascii) -> Offset      = check_hexadecimal_offset;
        const _: fn(ascii) -> DigitOffset = parse_hexadecimal_offset;

        const _: () = match b'0' {
            range_digit_ascii!() => {},
            range_uppercase_ascii!() => {},
            range_lowercase_ascii!() => {},
            range_letter_ascii!() => {},
            range_alphanumerical_ascii!() => {},
            _ => {},
        };
        const _: () = match '0' {
            range_digit!() => {},
            range_uppercase!() => {},
            range_lowercase!() => {},
            range_letter!() => {},
            range_alphanumerical!() => {},
            _ => {},
        };
        const _: fn(ascii) -> bool = is_digit;
        const _: fn(ascii) -> bool = is_uppercase;
        const _: fn(ascii) -> bool = is_lowercase;
        const _: fn(ascii) -> bool = is_letter;
        const _: fn(ascii) -> bool = is_alphanumerical;
        const _: fn(ascii, u8) -> OffsetCustomBase = check_custom_offset;
        const _: fn(ascii, u8) -> DigitOffsetCustomBase = parse_custom_offset;
    }

    mod _0_1_0_functionality {
        use crate::digit::*;

        test_assert!(Base::Binary.check(b'1'),      == AsciiDigit::Ok);
        test_assert!(Base::Octal.check(b'7'),       == AsciiDigit::Ok);
        test_assert!(Base::Decimal.check(b'9'),     == AsciiDigit::Ok);
        test_assert!(Base::Hexadecimal.check(b'f'), == AsciiDigit::Ok);
        test_assert!(Base::Hexadecimal.check(b'F'), == AsciiDigit::Ok);

        test_assert!(Base::Binary.parse(b'1'),      == Digit::Ok(1));
        test_assert!(Base::Octal.parse(b'7'),       == Digit::Ok(7));
        test_assert!(Base::Decimal.parse(b'9'),     == Digit::Ok(9));
        test_assert!(Base::Hexadecimal.parse(b'f'), == Digit::Ok(15));
        test_assert!(Base::Hexadecimal.parse(b'F'), == Digit::Ok(15));

        test_assert!(check(b'1', Base::Binary),      == AsciiDigit::Ok);
        test_assert!(check(b'7', Base::Octal),       == AsciiDigit::Ok);
        test_assert!(check(b'9', Base::Decimal),     == AsciiDigit::Ok);
        test_assert!(check(b'f', Base::Hexadecimal), == AsciiDigit::Ok);
        test_assert!(check(b'F', Base::Hexadecimal), == AsciiDigit::Ok);

        test_assert!(parse(b'1', Base::Binary),      == Digit::Ok(1));
        test_assert!(parse(b'7', Base::Octal),       == Digit::Ok(7));
        test_assert!(parse(b'9', Base::Decimal),     == Digit::Ok(9));
        test_assert!(parse(b'f', Base::Hexadecimal), == Digit::Ok(15));
        test_assert!(parse(b'F', Base::Hexadecimal), == Digit::Ok(15));

        test_assert!(check_binary(b'0'), == AsciiDigit::Ok);
        test_assert!(check_binary(b'1'), == AsciiDigit::Ok);
        test_assert!(check_binary(b'2'), == AsciiDigit::OutOfRange);
        test_assert!(check_binary(b'a'), == AsciiDigit::OutOfRange);
        test_assert!(check_binary(b'Z'), == AsciiDigit::OutOfRange);
        test_assert!(check_binary(b'_'), == AsciiDigit::Underscore);
        test_assert!(check_binary(b'.'), == AsciiDigit::Dot);
        test_assert!(check_binary(b'@'), == AsciiDigit::Other);

        test_assert!(parse_binary(b'0'), == Digit::Ok(0));
        test_assert!(parse_binary(b'1'), == Digit::Ok(1));
        test_assert!(parse_binary(b'2'), == Digit::OutOfRange);
        test_assert!(parse_binary(b'a'), == Digit::OutOfRange);
        test_assert!(parse_binary(b'Z'), == Digit::OutOfRange);
        test_assert!(parse_binary(b'_'), == Digit::Underscore);
        test_assert!(parse_binary(b'.'), == Digit::Dot);
        test_assert!(parse_binary(b'@'), == Digit::Other);

        test_assert!(check_octal(b'0'), == AsciiDigit::Ok);
        test_assert!(check_octal(b'1'), == AsciiDigit::Ok);
        test_assert!(check_octal(b'2'), == AsciiDigit::Ok);
        test_assert!(check_octal(b'3'), == AsciiDigit::Ok);
        test_assert!(check_octal(b'4'), == AsciiDigit::Ok);
        test_assert!(check_octal(b'5'), == AsciiDigit::Ok);
        test_assert!(check_octal(b'6'), == AsciiDigit::Ok);
        test_assert!(check_octal(b'7'), == AsciiDigit::Ok);
        test_assert!(check_octal(b'8'), == AsciiDigit::OutOfRange);
        test_assert!(check_octal(b'9'), == AsciiDigit::OutOfRange);
        test_assert!(check_octal(b'a'), == AsciiDigit::OutOfRange);
        test_assert!(check_octal(b'Z'), == AsciiDigit::OutOfRange);
        test_assert!(check_octal(b'_'), == AsciiDigit::Underscore);
        test_assert!(check_octal(b'.'), == AsciiDigit::Dot);
        test_assert!(check_octal(b'@'), == AsciiDigit::Other);

        test_assert!(parse_octal(b'0'), == Digit::Ok(0));
        test_assert!(parse_octal(b'1'), == Digit::Ok(1));
        test_assert!(parse_octal(b'2'), == Digit::Ok(2));
        test_assert!(parse_octal(b'3'), == Digit::Ok(3));
        test_assert!(parse_octal(b'4'), == Digit::Ok(4));
        test_assert!(parse_octal(b'5'), == Digit::Ok(5));
        test_assert!(parse_octal(b'6'), == Digit::Ok(6));
        test_assert!(parse_octal(b'7'), == Digit::Ok(7));
        test_assert!(parse_octal(b'8'), == Digit::OutOfRange);
        test_assert!(parse_octal(b'9'), == Digit::OutOfRange);
        test_assert!(parse_octal(b'a'), == Digit::OutOfRange);
        test_assert!(parse_octal(b'Z'), == Digit::OutOfRange);
        test_assert!(parse_octal(b'_'), == Digit::Underscore);
        test_assert!(parse_octal(b'.'), == Digit::Dot);
        test_assert!(parse_octal(b'@'), == Digit::Other);

        test_assert!(check_decimal(b'0'), == AsciiDigit::Ok);
        test_assert!(check_decimal(b'1'), == AsciiDigit::Ok);
        test_assert!(check_decimal(b'2'), == AsciiDigit::Ok);
        test_assert!(check_decimal(b'3'), == AsciiDigit::Ok);
        test_assert!(check_decimal(b'4'), == AsciiDigit::Ok);
        test_assert!(check_decimal(b'5'), == AsciiDigit::Ok);
        test_assert!(check_decimal(b'6'), == AsciiDigit::Ok);
        test_assert!(check_decimal(b'7'), == AsciiDigit::Ok);
        test_assert!(check_decimal(b'8'), == AsciiDigit::Ok);
        test_assert!(check_decimal(b'9'), == AsciiDigit::Ok);
        test_assert!(check_decimal(b'a'), == AsciiDigit::OutOfRange);
        test_assert!(check_decimal(b'Z'), == AsciiDigit::OutOfRange);
        test_assert!(check_decimal(b'_'), == AsciiDigit::Underscore);
        test_assert!(check_decimal(b'.'), == AsciiDigit::Dot);
        test_assert!(check_decimal(b'@'), == AsciiDigit::Other);

        test_assert!(parse_decimal(b'0'), == Digit::Ok(0));
        test_assert!(parse_decimal(b'1'), == Digit::Ok(1));
        test_assert!(parse_decimal(b'2'), == Digit::Ok(2));
        test_assert!(parse_decimal(b'3'), == Digit::Ok(3));
        test_assert!(parse_decimal(b'4'), == Digit::Ok(4));
        test_assert!(parse_decimal(b'5'), == Digit::Ok(5));
        test_assert!(parse_decimal(b'6'), == Digit::Ok(6));
        test_assert!(parse_decimal(b'7'), == Digit::Ok(7));
        test_assert!(parse_decimal(b'8'), == Digit::Ok(8));
        test_assert!(parse_decimal(b'9'), == Digit::Ok(9));
        test_assert!(parse_decimal(b'a'), == Digit::OutOfRange);
        test_assert!(parse_decimal(b'Z'), == Digit::OutOfRange);
        test_assert!(parse_decimal(b'_'), == Digit::Underscore);
        test_assert!(parse_decimal(b'.'), == Digit::Dot);
        test_assert!(parse_decimal(b'@'), == Digit::Other);

        test_assert!(check_hexadecimal(b'0'), == AsciiDigit::Ok);
        test_assert!(check_hexadecimal(b'1'), == AsciiDigit::Ok);
        test_assert!(check_hexadecimal(b'2'), == AsciiDigit::Ok);
        test_assert!(check_hexadecimal(b'3'), == AsciiDigit::Ok);
        test_assert!(check_hexadecimal(b'4'), == AsciiDigit::Ok);
        test_assert!(check_hexadecimal(b'5'), == AsciiDigit::Ok);
        test_assert!(check_hexadecimal(b'6'), == AsciiDigit::Ok);
        test_assert!(check_hexadecimal(b'7'), == AsciiDigit::Ok);
        test_assert!(check_hexadecimal(b'8'), == AsciiDigit::Ok);
        test_assert!(check_hexadecimal(b'9'), == AsciiDigit::Ok);
        test_assert!(check_hexadecimal(b'A'), == AsciiDigit::Ok);
        test_assert!(check_hexadecimal(b'a'), == AsciiDigit::Ok);
        test_assert!(check_hexadecimal(b'B'), == AsciiDigit::Ok);
        test_assert!(check_hexadecimal(b'b'), == AsciiDigit::Ok);
        test_assert!(check_hexadecimal(b'C'), == AsciiDigit::Ok);
        test_assert!(check_hexadecimal(b'c'), == AsciiDigit::Ok);
        test_assert!(check_hexadecimal(b'D'), == AsciiDigit::Ok);
        test_assert!(check_hexadecimal(b'd'), == AsciiDigit::Ok);
        test_assert!(check_hexadecimal(b'E'), == AsciiDigit::Ok);
        test_assert!(check_hexadecimal(b'e'), == AsciiDigit::Ok);
        test_assert!(check_hexadecimal(b'F'), == AsciiDigit::Ok);
        test_assert!(check_hexadecimal(b'f'), == AsciiDigit::Ok);
        test_assert!(check_hexadecimal(b'g'), == AsciiDigit::OutOfRange);
        test_assert!(check_hexadecimal(b'Z'), == AsciiDigit::OutOfRange);
        test_assert!(check_hexadecimal(b'_'), == AsciiDigit::Underscore);
        test_assert!(check_hexadecimal(b'.'), == AsciiDigit::Dot);
        test_assert!(check_hexadecimal(b'@'), == AsciiDigit::Other);

        test_assert!(parse_hexadecimal(b'0'), == Digit::Ok(0));
        test_assert!(parse_hexadecimal(b'1'), == Digit::Ok(1));
        test_assert!(parse_hexadecimal(b'2'), == Digit::Ok(2));
        test_assert!(parse_hexadecimal(b'3'), == Digit::Ok(3));
        test_assert!(parse_hexadecimal(b'4'), == Digit::Ok(4));
        test_assert!(parse_hexadecimal(b'5'), == Digit::Ok(5));
        test_assert!(parse_hexadecimal(b'6'), == Digit::Ok(6));
        test_assert!(parse_hexadecimal(b'7'), == Digit::Ok(7));
        test_assert!(parse_hexadecimal(b'8'), == Digit::Ok(8));
        test_assert!(parse_hexadecimal(b'9'), == Digit::Ok(9));
        test_assert!(parse_hexadecimal(b'A'), == Digit::Ok(10));
        test_assert!(parse_hexadecimal(b'a'), == Digit::Ok(10));
        test_assert!(parse_hexadecimal(b'B'), == Digit::Ok(11));
        test_assert!(parse_hexadecimal(b'b'), == Digit::Ok(11));
        test_assert!(parse_hexadecimal(b'C'), == Digit::Ok(12));
        test_assert!(parse_hexadecimal(b'c'), == Digit::Ok(12));
        test_assert!(parse_hexadecimal(b'D'), == Digit::Ok(13));
        test_assert!(parse_hexadecimal(b'd'), == Digit::Ok(13));
        test_assert!(parse_hexadecimal(b'E'), == Digit::Ok(14));
        test_assert!(parse_hexadecimal(b'e'), == Digit::Ok(14));
        test_assert!(parse_hexadecimal(b'F'), == Digit::Ok(15));
        test_assert!(parse_hexadecimal(b'f'), == Digit::Ok(15));
        test_assert!(parse_hexadecimal(b'g'), == Digit::OutOfRange);
        test_assert!(parse_hexadecimal(b'Z'), == Digit::OutOfRange);
        test_assert!(parse_hexadecimal(b'_'), == Digit::Underscore);
        test_assert!(parse_hexadecimal(b'.'), == Digit::Dot);
        test_assert!(parse_hexadecimal(b'@'), == Digit::Other);

        test_assert!(check_custom(b'g', 00), == AsciiDigitCustomBase::BaseMin);
        test_assert!(check_custom(b'g', 37), == AsciiDigitCustomBase::BaseMax);
        test_assert!(check_custom(b'g', 17), == AsciiDigitCustomBase::Ok);
        test_assert!(check_custom(b'z', 36), == AsciiDigitCustomBase::Ok);
        test_assert!(check_custom(b'z', 21), == AsciiDigitCustomBase::OutOfRange);
        test_assert!(check_custom(b'_', 36), == AsciiDigitCustomBase::Underscore);
        test_assert!(check_custom(b'.', 36), == AsciiDigitCustomBase::Dot);
        test_assert!(check_custom(b'@', 36), == AsciiDigitCustomBase::Other);

        test_assert!(parse_custom(b'g', 00), == DigitCustomBase::BaseMin);
        test_assert!(parse_custom(b'g', 01), == DigitCustomBase::BaseMin);
        test_assert!(parse_custom(b'g', 02), != DigitCustomBase::BaseMin);
        test_assert!(parse_custom(b'g', 36), != DigitCustomBase::BaseMax);
        test_assert!(parse_custom(b'g', 37), == DigitCustomBase::BaseMax);
        test_assert!(parse_custom(b'g', 17), == DigitCustomBase::Ok(16));
        test_assert!(parse_custom(b'z', 36), == DigitCustomBase::Ok(35));
        test_assert!(parse_custom(b'z', 21), == DigitCustomBase::OutOfRange);
        test_assert!(parse_custom(b'_', 36), == DigitCustomBase::Underscore);
        test_assert!(parse_custom(b'.', 36), == DigitCustomBase::Dot);
        test_assert!(parse_custom(b'@', 36), == DigitCustomBase::Other);

        test_assert!(check_tally(b'0', b'0'), == AsciiDigit::Ok);
        test_assert!(check_tally(b'_', b'0'), == AsciiDigit::Underscore);
        test_assert!(check_tally(b'.', b'0'), == AsciiDigit::Dot);
        test_assert!(check_tally(b'@', b'0'), == AsciiDigit::Other);

        test_assert!(parse_tally(b'0', b'0'), == Digit::Ok(1));
        test_assert!(parse_tally(b'_', b'0'), == Digit::Underscore);
        test_assert!(parse_tally(b'.', b'0'), == Digit::Dot);
        test_assert!(parse_tally(b'@', b'0'), == Digit::Other);
    }

    mod _0_1_0_backwards_compatibility {
        use core::ops::RangeInclusive;
        use crate::{ascii, utf32};
        use crate::digit::{
            Ascii, AsciiRange,
            Base::{self, Binary, Octal, Decimal, Hexadecimal},
            AsciiDigit, AsciiDigitCustomBase, Digit, DigitCustomBase,

            check,
            parse,
            check_binary,
            parse_binary,
            check_octal,
            parse_octal,
            check_decimal,
            parse_decimal,
            check_hexadecimal,
            parse_hexadecimal,
            check_custom,
            parse_custom,
            check_tally,
            parse_tally,
        };

        const _: () = {use AsciiDigit::{Ok, Underscore, Dot, OutOfRange, Other};};
        const _: () = {use Digit::{Ok, Underscore, Dot, OutOfRange, Other};};
        const _: () = {use AsciiDigitCustomBase::{Ok, Underscore, Dot, OutOfRange, Other, BaseMin, BaseMax};};
        const _: () = {use DigitCustomBase::{Ok, Underscore, Dot, OutOfRange, Other, BaseMin, BaseMax};};

        const _: u8 = Base::MIN;
        const _: u8 = Base::MAX;

        const _: AsciiRange<utf32>          = Base::BINARY_RANGE;
        const _: AsciiRange<utf32>          = Base::OCTAL_RANGE;
        const _: AsciiRange<utf32>          = Base::DECIMAL_RANGE;
        const _: AsciiRange<utf32>          = Base::HEXADECIMAL_DIGIT_RANGE;
        const _: AsciiRange<utf32>          = Base::HEXADECIMAL_UPPERCASE_RANGE;
        const _: AsciiRange<utf32>          = Base::HEXADECIMAL_LOWERCASE_RANGE;
        const _: [AsciiRange<utf32>; 1]     = Base::BINARY_RANGES;
        const _: [AsciiRange<utf32>; 1]     = Base::OCTAL_RANGES;
        const _: [AsciiRange<utf32>; 1]     = Base::DECIMAL_RANGES;
        const _: [AsciiRange<utf32>; 3]     = Base::HEXADECIMAL_RANGES;
        const _: RangeInclusive<utf32>      = Base::BINARY_RANGE_OPS;
        const _: RangeInclusive<utf32>      = Base::OCTAL_RANGE_OPS;
        const _: RangeInclusive<utf32>      = Base::DECIMAL_RANGE_OPS;
        const _: RangeInclusive<utf32>      = Base::HEXADECIMAL_DIGIT_RANGE_OPS;
        const _: RangeInclusive<utf32>      = Base::HEXADECIMAL_UPPERCASE_RANGE_OPS;
        const _: RangeInclusive<utf32>      = Base::HEXADECIMAL_LOWERCASE_RANGE_OPS;
        const _: [RangeInclusive<utf32>; 1] = Base::BINARY_RANGES_OPS;
        const _: [RangeInclusive<utf32>; 1] = Base::OCTAL_RANGES_OPS;
        const _: [RangeInclusive<utf32>; 1] = Base::DECIMAL_RANGES_OPS;
        const _: [RangeInclusive<utf32>; 3] = Base::HEXADECIMAL_RANGES_OPS;

        const _: AsciiRange<ascii>          = Base::BINARY_RANGE_ASCII;
        const _: AsciiRange<ascii>          = Base::OCTAL_RANGE_ASCII;
        const _: AsciiRange<ascii>          = Base::DECIMAL_RANGE_ASCII;
        const _: AsciiRange<ascii>          = Base::HEXADECIMAL_DIGIT_RANGE_ASCII;
        const _: AsciiRange<ascii>          = Base::HEXADECIMAL_UPPERCASE_RANGE_ASCII;
        const _: AsciiRange<ascii>          = Base::HEXADECIMAL_LOWERCASE_RANGE_ASCII;
        const _: [AsciiRange<ascii>; 1]     = Base::BINARY_RANGES_ASCII;
        const _: [AsciiRange<ascii>; 1]     = Base::OCTAL_RANGES_ASCII;
        const _: [AsciiRange<ascii>; 1]     = Base::DECIMAL_RANGES_ASCII;
        const _: [AsciiRange<ascii>; 3]     = Base::HEXADECIMAL_RANGES_ASCII;
        const _: RangeInclusive<ascii>      = Base::BINARY_RANGE_ASCII_OPS;
        const _: RangeInclusive<ascii>      = Base::OCTAL_RANGE_ASCII_OPS;
        const _: RangeInclusive<ascii>      = Base::DECIMAL_RANGE_ASCII_OPS;
        const _: RangeInclusive<ascii>      = Base::HEXADECIMAL_DIGIT_RANGE_ASCII_OPS;
        const _: RangeInclusive<ascii>      = Base::HEXADECIMAL_UPPERCASE_RANGE_ASCII_OPS;
        const _: RangeInclusive<ascii>      = Base::HEXADECIMAL_LOWERCASE_RANGE_ASCII_OPS;
        const _: [RangeInclusive<ascii>; 1] = Base::BINARY_RANGES_ASCII_OPS;
        const _: [RangeInclusive<ascii>; 1] = Base::OCTAL_RANGES_ASCII_OPS;
        const _: [RangeInclusive<ascii>; 1] = Base::DECIMAL_RANGES_ASCII_OPS;
        const _: [RangeInclusive<ascii>; 3] = Base::HEXADECIMAL_RANGES_ASCII_OPS;
        const _: u8                         = Base::BINARY_ASCII_OFFSET;
        const _: u8                         = Base::OCTAL_ASCII_OFFSET;
        const _: u8                         = Base::DECIMAL_ASCII_OFFSET;
        const _: u8                         = Base::HEXADECIMAL_DIGIT_ASCII_OFFSET;
        const _: u8                         = Base::HEXADECIMAL_UPPERCASE_ASCII_OFFSET;
        const _: u8                         = Base::HEXADECIMAL_LOWERCASE_ASCII_OFFSET;

        const _: fn(Base) -> &'static [AsciiRange<utf32>]     = Base::range;
        const _: fn(Base) -> &'static [RangeInclusive<utf32>] = Base::range_ops;
        const _: fn(Base) -> &'static [AsciiRange<ascii>]     = Base::range_ascii;
        const _: fn(Base) -> &'static [RangeInclusive<ascii>] = Base::range_ascii_ops;

        const _: fn(Base, ascii) -> AsciiDigit = Base::check;
        const _: fn(Base, ascii) -> Digit      = Base::parse;

        const _: fn(ascii, Base) -> AsciiDigit         = check;
        const _: fn(ascii, Base) -> Digit              = parse;
        const _: fn(ascii) -> AsciiDigit               = check_binary;
        const _: fn(ascii) -> Digit                    = parse_binary;
        const _: fn(ascii) -> AsciiDigit               = check_octal;
        const _: fn(ascii) -> Digit                    = parse_octal;
        const _: fn(ascii) -> AsciiDigit               = check_decimal;
        const _: fn(ascii) -> Digit                    = parse_decimal;
        const _: fn(ascii) -> AsciiDigit               = check_hexadecimal;
        const _: fn(ascii) -> Digit                    = parse_hexadecimal;
        const _: fn(ascii, u8) -> AsciiDigitCustomBase = check_custom;
        const _: fn(ascii, u8) -> DigitCustomBase      = parse_custom;
        const _: fn(ascii, ascii) -> AsciiDigit        = check_tally;
        const _: fn(ascii, ascii) -> Digit             = parse_tally;
    }
}
