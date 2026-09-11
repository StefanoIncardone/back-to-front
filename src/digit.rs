#![expect(clippy::pub_use)]
// IDEA(stefano): add out of range ranges
// TODO(stefano): add documentations

use crate::{ascii, utf32};
use core::ops::RangeInclusive;


// TODO(stefano): add number-like traits
pub trait Ascii: Copy {}
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

impl<I: Ascii> AsciiRange<I> {
    #[must_use]
    #[inline(always)]
    pub const fn from_range_inclusive(value: RangeInclusive<I>) -> Self {
        return Self { start: *value.start(), end: *value.end() };
    }

    #[must_use]
    #[inline(always)]
    pub const fn to_range_inclusive(self) -> RangeInclusive<I> {
        return RangeInclusive::new(self.start, self.end);
    }
}

impl AsciiRange<ascii> {
    #[must_use]
    #[inline(always)]
    pub const fn from_utf32(other: AsciiRange<utf32>) -> Self {
        return other.to_ascii();
    }

    #[must_use]
    #[inline(always)]
    pub const fn to_utf32(self) -> AsciiRange<utf32> {
        return AsciiRange { start: self.start as utf32, end: self.end as utf32 };
    }
}

impl AsciiRange<utf32> {
    #[must_use]
    #[inline(always)]
    pub const fn from_ascii(other: AsciiRange<ascii>) -> Self {
        return other.to_utf32();
    }

    #[must_use]
    #[inline(always)]
    pub const fn to_ascii(self) -> AsciiRange<ascii> {
        return AsciiRange { start: self.start as ascii, end: self.end as ascii };
    }
}

impl<I: Ascii> From<RangeInclusive<I>> for AsciiRange<I> {
    #[inline(always)]
    fn from(value: RangeInclusive<I>) -> Self {
        return Self::from_range_inclusive(value);
    }
}

impl<I: Ascii> Into<RangeInclusive<I>> for AsciiRange<I> {
    #[inline(always)]
    fn into(self) -> RangeInclusive<I> {
        return self.to_range_inclusive();
    }
}

impl<I: Ascii + core::fmt::Display> core::fmt::Display for AsciiRange<I> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        return write!(f, "{}..{}", self.start, self.end);
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

#[rustfmt::skip]
impl Base {
    #[must_use]
    #[inline]
    pub const fn range(self) -> &'static [AsciiRange<ascii>] {
        return match self {
            Self::Binary      => &binary::RANGES,
            Self::Octal       => &octal::RANGES,
            Self::Decimal     => &decimal::RANGES,
            Self::Hexadecimal => &hexadecimal::RANGES,
        };
    }

    #[must_use]
    #[inline]
    pub const fn range_utf32(self) -> &'static [AsciiRange<utf32>] {
        return match self {
            Self::Binary      => &binary::RANGES_UTF32,
            Self::Octal       => &octal::RANGES_UTF32,
            Self::Decimal     => &decimal::RANGES_UTF32,
            Self::Hexadecimal => &hexadecimal::RANGES_UTF32,
        };
    }

    #[must_use]
    #[inline]
    pub const fn range_ops(self) -> &'static [RangeInclusive<ascii>] {
        return match self {
            Self::Binary      => &binary::RANGES_OPS,
            Self::Octal       => &octal::RANGES_OPS,
            Self::Decimal     => &decimal::RANGES_OPS,
            Self::Hexadecimal => &hexadecimal::RANGES_OPS,
        };
    }

    #[must_use]
    #[inline]
    pub const fn range_ops_utf32(self) -> &'static [RangeInclusive<utf32>] {
        return match self {
            Self::Binary      => &binary::RANGES_OPS_UTF32,
            Self::Octal       => &octal::RANGES_OPS_UTF32,
            Self::Decimal     => &decimal::RANGES_OPS_UTF32,
            Self::Hexadecimal => &hexadecimal::RANGES_OPS_UTF32,
        };
    }
}


// Parsing
pub type Offset = ascii;
pub type OffsetCustomBase = ascii;
pub type Digit = u8;
pub type DigitCustomBase = u8;

// Note: this enum exists just to leverage the enum mechanism of assigning discriminants
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
enum OffsetResult {
    NotADigit = 0b1000_0000,
    OutOfRange,
    BaseMin,
    BaseMax,
}

pub const NOT_A_DIGIT: Offset = OffsetResult::NotADigit as Offset;
pub const OUT_OF_RANGE: Offset = OffsetResult::OutOfRange as Offset;
pub const BASE_MIN: OffsetCustomBase = OffsetResult::BaseMin as OffsetCustomBase;
pub const BASE_MAX: OffsetCustomBase = OffsetResult::BaseMax as OffsetCustomBase;

impl Base {
    #[must_use]
    #[inline]
    pub const fn check(self, ch: ascii) -> Offset {
        #[rustfmt::skip]
        return match self {
            Self::Binary      => binary::check(ch),
            Self::Octal       => octal::check(ch),
            Self::Decimal     => decimal::check(ch),
            Self::Hexadecimal => hexadecimal::check(ch),
        };
    }

    #[must_use]
    #[inline]
    pub const fn parse(self, ch: ascii) -> Digit {
        #[rustfmt::skip]
        return match self {
            Self::Binary      => binary::parse(ch),
            Self::Octal       => octal::parse(ch),
            Self::Decimal     => decimal::parse(ch),
            Self::Hexadecimal => hexadecimal::parse(ch),
        };
    }
}

#[rustfmt::skip]
pub mod binary {
    #[allow(clippy::wildcard_imports)]
    use super::*;

    //# Ranges

    pub const DIGIT:                  AsciiRange<ascii> = AsciiRange { start: b'0',          end: b'1' };
    pub const DIGIT_OUT_OF_RANGE:     AsciiRange<ascii> = AsciiRange { start: DIGIT.end + 1, end: b'9' };
    pub const UPPERCASE_OUT_OF_RANGE: AsciiRange<ascii> = AsciiRange { start: b'A',          end: b'Z' };
    pub const LOWERCASE_OUT_OF_RANGE: AsciiRange<ascii> = AsciiRange { start: b'a',          end: b'z' };

    pub const RANGES:       [AsciiRange<ascii>; 1] = [DIGIT];
    pub const RANGES_UTF32: [AsciiRange<utf32>; 1] = [DIGIT.to_utf32()];

    pub const RANGES_OPS:       [RangeInclusive<ascii>; 1] = [DIGIT.to_range_inclusive()];
    pub const RANGES_OPS_UTF32: [RangeInclusive<utf32>; 1] = [DIGIT.to_utf32().to_range_inclusive()];

    //# Ranges constants

    pub const DIGIT_START:                  u8 = DIGIT.start;
    pub const DIGIT_END:                    u8 = DIGIT.end;
    pub const DIGIT_OUT_OF_RANGE_START:     u8 = DIGIT_OUT_OF_RANGE.start;
    pub const DIGIT_OUT_OF_RANGE_END:       u8 = DIGIT_OUT_OF_RANGE.end;
    pub const UPPERCASE_OUT_OF_RANGE_START: u8 = UPPERCASE_OUT_OF_RANGE.start;
    pub const UPPERCASE_OUT_OF_RANGE_END:   u8 = UPPERCASE_OUT_OF_RANGE.end;
    pub const LOWERCASE_OUT_OF_RANGE_START: u8 = LOWERCASE_OUT_OF_RANGE.start;
    pub const LOWERCASE_OUT_OF_RANGE_END:   u8 = LOWERCASE_OUT_OF_RANGE.end;

    pub const DIGIT_START_UTF32:                  utf32 = DIGIT_START as utf32;
    pub const DIGIT_END_UTF32:                    utf32 = DIGIT_END as utf32;
    pub const DIGIT_OUT_OF_RANGE_START_UTF32:     utf32 = DIGIT_OUT_OF_RANGE_START as utf32;
    pub const DIGIT_OUT_OF_RANGE_END_UTF32:       utf32 = DIGIT_OUT_OF_RANGE_END as utf32;
    pub const UPPERCASE_OUT_OF_RANGE_START_UTF32: utf32 = UPPERCASE_OUT_OF_RANGE_START as utf32;
    pub const UPPERCASE_OUT_OF_RANGE_END_UTF32:   utf32 = UPPERCASE_OUT_OF_RANGE_END as utf32;
    pub const LOWERCASE_OUT_OF_RANGE_START_UTF32: utf32 = LOWERCASE_OUT_OF_RANGE_START as utf32;
    pub const LOWERCASE_OUT_OF_RANGE_END_UTF32:   utf32 = LOWERCASE_OUT_OF_RANGE_END as utf32;

    pub const DIGIT_OFFSET: u8 = DIGIT_START;

    //# Ranges macros

    #[macro_export] macro_rules! binary_digit                        { () => { $crate::digit::binary::DIGIT_START                    ..= $crate::digit::binary::DIGIT_END }; }
    #[macro_export] macro_rules! binary_digit_out_of_range           { () => { $crate::digit::binary::DIGIT_OUT_OF_RANGE_START       ..= $crate::digit::binary::DIGIT_OUT_OF_RANGE_END }; }
    #[macro_export] macro_rules! binary_uppercase_out_of_range       { () => { $crate::digit::binary::UPPERCASE_OUT_OF_RANGE_START   ..= $crate::digit::binary::UPPERCASE_OUT_OF_RANGE_END }; }
    #[macro_export] macro_rules! binary_lowercase_out_of_range       { () => { $crate::digit::binary::LOWERCASE_OUT_OF_RANGE_START   ..= $crate::digit::binary::LOWERCASE_OUT_OF_RANGE_END }; }
    #[macro_export] macro_rules! binary_letter_out_of_range          { () => { $crate::digit::binary::binary_uppercase_out_of_range!() | $crate::digit::binary::binary_lowercase_out_of_range!() }; }
    #[macro_export] macro_rules! binary_out_of_range                 { () => { $crate::digit::binary::binary_digit_out_of_range!()     | $crate::digit::binary::binary_letter_out_of_range!() }; }

    #[macro_export] macro_rules! binary_digit_utf32                  { () => { $crate::digit::binary::DIGIT_START_UTF32                    ..= $crate::digit::binary::DIGIT_END_UTF32 }; }
    #[macro_export] macro_rules! binary_digit_out_of_range_utf32     { () => { $crate::digit::binary::DIGIT_OUT_OF_RANGE_START_UTF32       ..= $crate::digit::binary::DIGIT_OUT_OF_RANGE_END_UTF32 }; }
    #[macro_export] macro_rules! binary_uppercase_out_of_range_utf32 { () => { $crate::digit::binary::UPPERCASE_OUT_OF_RANGE_START_UTF32   ..= $crate::digit::binary::UPPERCASE_OUT_OF_RANGE_END_UTF32 }; }
    #[macro_export] macro_rules! binary_lowercase_out_of_range_utf32 { () => { $crate::digit::binary::LOWERCASE_OUT_OF_RANGE_START_UTF32   ..= $crate::digit::binary::LOWERCASE_OUT_OF_RANGE_END_UTF32 }; }
    #[macro_export] macro_rules! binary_letter_out_of_range_utf32    { () => { $crate::digit::binary::binary_uppercase_out_of_range_utf32!() | $crate::digit::binary::binary_lowercase_out_of_range_utf32!() }; }
    #[macro_export] macro_rules! binary_out_of_range_utf32           { () => { $crate::digit::binary::binary_digit_out_of_range_utf32!()     | $crate::digit::binary::binary_letter_out_of_range_utf32!() }; }

    pub use binary_digit;
    pub use binary_digit_out_of_range;
    pub use binary_uppercase_out_of_range;
    pub use binary_lowercase_out_of_range;
    pub use binary_letter_out_of_range;
    pub use binary_out_of_range;

    pub use binary_digit_utf32;
    pub use binary_digit_out_of_range_utf32;
    pub use binary_uppercase_out_of_range_utf32;
    pub use binary_lowercase_out_of_range_utf32;
    pub use binary_letter_out_of_range_utf32;
    pub use binary_out_of_range_utf32;


    //# Methods

    #[must_use] #[inline]         pub const fn is_digit(ch: ascii)                  -> bool { return matches!(ch, binary_digit!()); }
    #[must_use] #[inline]         pub const fn is_digit_out_of_range(ch: ascii)     -> bool { return matches!(ch, binary_digit_out_of_range!()); }
    #[must_use] #[inline]         pub const fn is_uppercase_out_of_range(ch: ascii) -> bool { return matches!(ch, binary_uppercase_out_of_range!()); }
    #[must_use] #[inline]         pub const fn is_lowercase_out_of_range(ch: ascii) -> bool { return matches!(ch, binary_lowercase_out_of_range!()); }
    #[must_use] #[inline]         pub const fn is_letter_out_of_range(ch: ascii)    -> bool { return is_uppercase_out_of_range(ch) || is_lowercase_out_of_range(ch); }
    #[must_use] #[inline]         pub const fn is_out_of_range(ch: ascii)           -> bool { return is_digit_out_of_range(ch) || is_letter_out_of_range(ch); }
    #[must_use] #[inline(always)] pub const fn is(ch: ascii)                        -> bool { return is_digit(ch); }

    //# Parsing

    #[must_use]
    #[inline]
    pub const fn check(ch: ascii) -> Offset {
        return match ch {
            binary_digit!()        => DIGIT_OFFSET,
            binary_out_of_range!() => return OUT_OF_RANGE,
            _                      => return NOT_A_DIGIT,
        };
    }

    #[must_use]
    #[inline]
    pub const fn parse(ch: ascii) -> Digit {
        let offset = match ch {
            binary_digit!()        => DIGIT_OFFSET,
            binary_out_of_range!() => return OUT_OF_RANGE,
            _                      => return NOT_A_DIGIT,
        };
        return ch - offset;
    }
}

#[rustfmt::skip]
pub mod octal {
    #[allow(clippy::wildcard_imports)]
    use super::*;

    //# Ranges

    pub const DIGIT:                  AsciiRange<ascii> = AsciiRange { start: b'0',          end: b'7' };
    pub const DIGIT_OUT_OF_RANGE:     AsciiRange<ascii> = AsciiRange { start: DIGIT.end + 1, end: b'9' };
    pub const UPPERCASE_OUT_OF_RANGE: AsciiRange<ascii> = AsciiRange { start: b'A',          end: b'Z' };
    pub const LOWERCASE_OUT_OF_RANGE: AsciiRange<ascii> = AsciiRange { start: b'a',          end: b'z' };

    pub const RANGES:       [AsciiRange<ascii>; 1] = [DIGIT];
    pub const RANGES_UTF32: [AsciiRange<utf32>; 1] = [DIGIT.to_utf32()];

    pub const RANGES_OPS:       [RangeInclusive<ascii>; 1] = [DIGIT.to_range_inclusive()];
    pub const RANGES_OPS_UTF32: [RangeInclusive<utf32>; 1] = [DIGIT.to_utf32().to_range_inclusive()];

    //# Ranges constants

    pub const DIGIT_START:                  u8 = DIGIT.start;
    pub const DIGIT_END:                    u8 = DIGIT.end;
    pub const DIGIT_OUT_OF_RANGE_START:     u8 = DIGIT_OUT_OF_RANGE.start;
    pub const DIGIT_OUT_OF_RANGE_END:       u8 = DIGIT_OUT_OF_RANGE.end;
    pub const UPPERCASE_OUT_OF_RANGE_START: u8 = UPPERCASE_OUT_OF_RANGE.start;
    pub const UPPERCASE_OUT_OF_RANGE_END:   u8 = UPPERCASE_OUT_OF_RANGE.end;
    pub const LOWERCASE_OUT_OF_RANGE_START: u8 = LOWERCASE_OUT_OF_RANGE.start;
    pub const LOWERCASE_OUT_OF_RANGE_END:   u8 = LOWERCASE_OUT_OF_RANGE.end;

    pub const DIGIT_START_UTF32:                  utf32 = DIGIT_START as utf32;
    pub const DIGIT_END_UTF32:                    utf32 = DIGIT_END as utf32;
    pub const DIGIT_OUT_OF_RANGE_START_UTF32:     utf32 = DIGIT_OUT_OF_RANGE_START as utf32;
    pub const DIGIT_OUT_OF_RANGE_END_UTF32:       utf32 = DIGIT_OUT_OF_RANGE_END as utf32;
    pub const UPPERCASE_OUT_OF_RANGE_START_UTF32: utf32 = UPPERCASE_OUT_OF_RANGE_START as utf32;
    pub const UPPERCASE_OUT_OF_RANGE_END_UTF32:   utf32 = UPPERCASE_OUT_OF_RANGE_END as utf32;
    pub const LOWERCASE_OUT_OF_RANGE_START_UTF32: utf32 = LOWERCASE_OUT_OF_RANGE_START as utf32;
    pub const LOWERCASE_OUT_OF_RANGE_END_UTF32:   utf32 = LOWERCASE_OUT_OF_RANGE_END as utf32;

    pub const DIGIT_OFFSET: u8 = DIGIT_START;

    //# Ranges macros

    #[macro_export] macro_rules! octal_digit                        { () => { $crate::digit::octal::DIGIT_START                   ..= $crate::digit::octal::DIGIT_END }; }
    #[macro_export] macro_rules! octal_digit_out_of_range           { () => { $crate::digit::octal::DIGIT_OUT_OF_RANGE_START      ..= $crate::digit::octal::DIGIT_OUT_OF_RANGE_END }; }
    #[macro_export] macro_rules! octal_uppercase_out_of_range       { () => { $crate::digit::octal::UPPERCASE_OUT_OF_RANGE_START  ..= $crate::digit::octal::UPPERCASE_OUT_OF_RANGE_END }; }
    #[macro_export] macro_rules! octal_lowercase_out_of_range       { () => { $crate::digit::octal::LOWERCASE_OUT_OF_RANGE_START  ..= $crate::digit::octal::LOWERCASE_OUT_OF_RANGE_END }; }
    #[macro_export] macro_rules! octal_letter_out_of_range          { () => { $crate::digit::octal::octal_uppercase_out_of_range!() | $crate::digit::octal::octal_lowercase_out_of_range!() }; }
    #[macro_export] macro_rules! octal_out_of_range                 { () => { $crate::digit::octal::octal_digit_out_of_range!()     | $crate::digit::octal::octal_letter_out_of_range!() }; }

    #[macro_export] macro_rules! octal_digit_utf32                  { () => { $crate::digit::octal::DIGIT_START_UTF32                   ..= $crate::digit::octal::DIGIT_END_UTF32 }; }
    #[macro_export] macro_rules! octal_digit_out_of_range_utf32     { () => { $crate::digit::octal::DIGIT_OUT_OF_RANGE_START_UTF32      ..= $crate::digit::octal::DIGIT_OUT_OF_RANGE_END_UTF32 }; }
    #[macro_export] macro_rules! octal_uppercase_out_of_range_utf32 { () => { $crate::digit::octal::UPPERCASE_OUT_OF_RANGE_START_UTF32  ..= $crate::digit::octal::UPPERCASE_OUT_OF_RANGE_END_UTF32 }; }
    #[macro_export] macro_rules! octal_lowercase_out_of_range_utf32 { () => { $crate::digit::octal::LOWERCASE_OUT_OF_RANGE_START_UTF32  ..= $crate::digit::octal::LOWERCASE_OUT_OF_RANGE_END_UTF32 }; }
    #[macro_export] macro_rules! octal_letter_out_of_range_utf32    { () => { $crate::digit::octal::octal_uppercase_out_of_range_utf32!() | $crate::digit::octal::octal_lowercase_out_of_range_utf32!() }; }
    #[macro_export] macro_rules! octal_out_of_range_utf32           { () => { $crate::digit::octal::octal_digit_out_of_range_utf32!()     | $crate::digit::octal::octal_letter_out_of_range_utf32!() }; }

    pub use octal_digit;
    pub use octal_digit_out_of_range;
    pub use octal_uppercase_out_of_range;
    pub use octal_lowercase_out_of_range;
    pub use octal_letter_out_of_range;
    pub use octal_out_of_range;

    pub use octal_digit_utf32;
    pub use octal_digit_out_of_range_utf32;
    pub use octal_uppercase_out_of_range_utf32;
    pub use octal_lowercase_out_of_range_utf32;
    pub use octal_letter_out_of_range_utf32;
    pub use octal_out_of_range_utf32;

    //# Methods

    #[must_use] #[inline]         pub const fn is_digit(ch: ascii)                  -> bool { return matches!(ch, octal_digit!()); }
    #[must_use] #[inline]         pub const fn is_digit_out_of_range(ch: ascii)     -> bool { return matches!(ch, octal_digit_out_of_range!()); }
    #[must_use] #[inline]         pub const fn is_uppercase_out_of_range(ch: ascii) -> bool { return matches!(ch, octal_uppercase_out_of_range!()); }
    #[must_use] #[inline]         pub const fn is_lowercase_out_of_range(ch: ascii) -> bool { return matches!(ch, octal_lowercase_out_of_range!()); }
    #[must_use] #[inline]         pub const fn is_letter_out_of_range(ch: ascii)    -> bool { return is_uppercase_out_of_range(ch) || is_lowercase_out_of_range(ch); }
    #[must_use] #[inline]         pub const fn is_out_of_range(ch: ascii)           -> bool { return is_digit_out_of_range(ch) || is_letter_out_of_range(ch); }
    #[must_use] #[inline(always)] pub const fn is(ch: ascii)                        -> bool { return is_digit(ch); }

    //# Parsing

    #[must_use]
    #[inline]
    pub const fn check(ch: ascii) -> Offset {
        return match ch {
            octal_digit!()        => DIGIT_OFFSET,
            octal_out_of_range!() => return OUT_OF_RANGE,
            _                     => return NOT_A_DIGIT,
        };
    }

    #[must_use]
    #[inline]
    pub const fn parse(ch: ascii) -> Digit {
        let offset = match ch {
            octal_digit!()        => DIGIT_OFFSET,
            octal_out_of_range!() => return OUT_OF_RANGE,
            _                     => return NOT_A_DIGIT,
        };
        return ch - offset;
    }
}

#[rustfmt::skip]
pub mod decimal {
    #[allow(clippy::wildcard_imports)]
    use super::*;

    //# Ranges

    pub const DIGIT:                  AsciiRange<ascii> = AsciiRange { start: b'0', end: b'9' };
    pub const UPPERCASE_OUT_OF_RANGE: AsciiRange<ascii> = AsciiRange { start: b'A', end: b'Z' };
    pub const LOWERCASE_OUT_OF_RANGE: AsciiRange<ascii> = AsciiRange { start: b'a', end: b'z' };

    pub const RANGES:       [AsciiRange<ascii>; 1] = [DIGIT];
    pub const RANGES_UTF32: [AsciiRange<utf32>; 1] = [DIGIT.to_utf32()];

    pub const RANGES_OPS:       [RangeInclusive<ascii>; 1] = [DIGIT.to_range_inclusive()];
    pub const RANGES_OPS_UTF32: [RangeInclusive<utf32>; 1] = [DIGIT.to_utf32().to_range_inclusive()];

    //# Ranges constants

    pub const DIGIT_START:                  u8 = DIGIT.start;
    pub const DIGIT_END:                    u8 = DIGIT.end;
    pub const UPPERCASE_OUT_OF_RANGE_START: u8 = UPPERCASE_OUT_OF_RANGE.start;
    pub const UPPERCASE_OUT_OF_RANGE_END:   u8 = UPPERCASE_OUT_OF_RANGE.end;
    pub const LOWERCASE_OUT_OF_RANGE_START: u8 = LOWERCASE_OUT_OF_RANGE.start;
    pub const LOWERCASE_OUT_OF_RANGE_END:   u8 = LOWERCASE_OUT_OF_RANGE.end;

    pub const DIGIT_START_UTF32:                  utf32 = DIGIT_START as utf32;
    pub const DIGIT_END_UTF32:                    utf32 = DIGIT_END as utf32;
    pub const UPPERCASE_OUT_OF_RANGE_START_UTF32: utf32 = UPPERCASE_OUT_OF_RANGE_START as utf32;
    pub const UPPERCASE_OUT_OF_RANGE_END_UTF32:   utf32 = UPPERCASE_OUT_OF_RANGE_END as utf32;
    pub const LOWERCASE_OUT_OF_RANGE_START_UTF32: utf32 = LOWERCASE_OUT_OF_RANGE_START as utf32;
    pub const LOWERCASE_OUT_OF_RANGE_END_UTF32:   utf32 = LOWERCASE_OUT_OF_RANGE_END as utf32;

    pub const DIGIT_OFFSET: u8 = DIGIT_START;

    //# Ranges macros

    #[macro_export] macro_rules! decimal_digit                        { () => { $crate::digit::decimal::DIGIT_START                     ..= $crate::digit::decimal::DIGIT_END }; }
    #[macro_export] macro_rules! decimal_uppercase_out_of_range       { () => { $crate::digit::decimal::UPPERCASE_OUT_OF_RANGE_START    ..= $crate::digit::decimal::UPPERCASE_OUT_OF_RANGE_END }; }
    #[macro_export] macro_rules! decimal_lowercase_out_of_range       { () => { $crate::digit::decimal::LOWERCASE_OUT_OF_RANGE_START    ..= $crate::digit::decimal::LOWERCASE_OUT_OF_RANGE_END }; }
    #[macro_export] macro_rules! decimal_out_of_range                 { () => { $crate::digit::decimal::decimal_uppercase_out_of_range!() | $crate::digit::decimal::decimal_lowercase_out_of_range!() }; }

    #[macro_export] macro_rules! decimal_digit_utf32                  { () => { $crate::digit::decimal::DIGIT_START_UTF32                     ..= $crate::digit::decimal::DIGIT_END_UTF32 }; }
    #[macro_export] macro_rules! decimal_uppercase_out_of_range_utf32 { () => { $crate::digit::decimal::UPPERCASE_OUT_OF_RANGE_START_UTF32    ..= $crate::digit::decimal::UPPERCASE_OUT_OF_RANGE_END_UTF32 }; }
    #[macro_export] macro_rules! decimal_lowercase_out_of_range_utf32 { () => { $crate::digit::decimal::LOWERCASE_OUT_OF_RANGE_START_UTF32    ..= $crate::digit::decimal::LOWERCASE_OUT_OF_RANGE_END_UTF32 }; }
    #[macro_export] macro_rules! decimal_out_of_range_utf32           { () => { $crate::digit::decimal::decimal_uppercase_out_of_range_utf32!() | $crate::digit::decimal::decimal_lowercase_out_of_range_utf32!() }; }

    pub use decimal_digit;
    pub use decimal_uppercase_out_of_range;
    pub use decimal_lowercase_out_of_range;
    pub use decimal_out_of_range;

    pub use decimal_digit_utf32;
    pub use decimal_uppercase_out_of_range_utf32;
    pub use decimal_lowercase_out_of_range_utf32;
    pub use decimal_out_of_range_utf32;

    //# Methods

    #[must_use] #[inline]         pub const fn is_digit(ch: ascii)                  -> bool { return matches!(ch, decimal_digit!()); }
    #[must_use] #[inline]         pub const fn is_uppercase_out_of_range(ch: ascii) -> bool { return matches!(ch, decimal_uppercase_out_of_range!()); }
    #[must_use] #[inline]         pub const fn is_lowercase_out_of_range(ch: ascii) -> bool { return matches!(ch, decimal_lowercase_out_of_range!()); }
    #[must_use] #[inline]         pub const fn is_out_of_range(ch: ascii)           -> bool { return is_uppercase_out_of_range(ch) || is_lowercase_out_of_range(ch); }
    #[must_use] #[inline(always)] pub const fn is(ch: ascii)                        -> bool { return is_digit(ch); }

    //# Parsing

    #[must_use]
    #[inline]
    pub const fn check(ch: ascii) -> Offset {
        return match ch {
            decimal_digit!()        => DIGIT_OFFSET,
            decimal_out_of_range!() => return OUT_OF_RANGE,
            _                       => return NOT_A_DIGIT,
        };
    }

    #[must_use]
    #[inline]
    pub const fn parse(ch: ascii) -> Digit {
        let offset = match ch {
            decimal_digit!()        => DIGIT_OFFSET,
            decimal_out_of_range!() => return OUT_OF_RANGE,
            _                       => return NOT_A_DIGIT,
        };
        return ch - offset;
    }
}

#[rustfmt::skip]
pub mod hexadecimal {
    #[allow(clippy::wildcard_imports)]
    use super::*;

    //# Ranges

    pub const DIGIT:                  AsciiRange<ascii> = AsciiRange { start: b'0', end: b'9' };
    pub const UPPERCASE:              AsciiRange<ascii> = AsciiRange { start: b'A', end: b'F' };
    pub const LOWERCASE:              AsciiRange<ascii> = AsciiRange { start: b'a', end: b'f' };
    pub const UPPERCASE_OUT_OF_RANGE: AsciiRange<ascii> = AsciiRange { start: b'A', end: b'Z' };
    pub const LOWERCASE_OUT_OF_RANGE: AsciiRange<ascii> = AsciiRange { start: b'a', end: b'z' };

    pub const RANGES:       [AsciiRange<ascii>; 3] = [DIGIT,            UPPERCASE,            LOWERCASE];
    pub const RANGES_UTF32: [AsciiRange<utf32>; 3] = [DIGIT.to_utf32(), UPPERCASE.to_utf32(), LOWERCASE.to_utf32()];

    pub const RANGES_OPS:       [RangeInclusive<ascii>; 3] = [DIGIT.to_range_inclusive(),            UPPERCASE.to_range_inclusive(),            LOWERCASE.to_range_inclusive()];
    pub const RANGES_OPS_UTF32: [RangeInclusive<utf32>; 3] = [DIGIT.to_utf32().to_range_inclusive(), UPPERCASE.to_utf32().to_range_inclusive(), LOWERCASE.to_utf32().to_range_inclusive()];

    //# Ranges constants

    pub const DIGIT_START:                  u8 = DIGIT.start;
    pub const DIGIT_END:                    u8 = DIGIT.end;
    pub const UPPERCASE_START:              u8 = UPPERCASE.start;
    pub const UPPERCASE_END:                u8 = UPPERCASE.end;
    pub const LOWERCASE_START:              u8 = LOWERCASE.start;
    pub const LOWERCASE_END:                u8 = LOWERCASE.end;
    pub const UPPERCASE_OUT_OF_RANGE_START: u8 = UPPERCASE_OUT_OF_RANGE.start;
    pub const UPPERCASE_OUT_OF_RANGE_END:   u8 = UPPERCASE_OUT_OF_RANGE.end;
    pub const LOWERCASE_OUT_OF_RANGE_START: u8 = LOWERCASE_OUT_OF_RANGE.start;
    pub const LOWERCASE_OUT_OF_RANGE_END:   u8 = LOWERCASE_OUT_OF_RANGE.end;

    pub const DIGIT_START_UTF32:                  utf32 = DIGIT_START as utf32;
    pub const DIGIT_END_UTF32:                    utf32 = DIGIT_END as utf32;
    pub const UPPERCASE_START_UTF32:              utf32 = UPPERCASE_START as utf32;
    pub const UPPERCASE_END_UTF32:                utf32 = UPPERCASE_END as utf32;
    pub const LOWERCASE_START_UTF32:              utf32 = LOWERCASE_START as utf32;
    pub const LOWERCASE_END_UTF32:                utf32 = LOWERCASE_END as utf32;
    pub const UPPERCASE_OUT_OF_RANGE_START_UTF32: utf32 = UPPERCASE_OUT_OF_RANGE_START as utf32;
    pub const UPPERCASE_OUT_OF_RANGE_END_UTF32:   utf32 = UPPERCASE_OUT_OF_RANGE_END as utf32;
    pub const LOWERCASE_OUT_OF_RANGE_START_UTF32: utf32 = LOWERCASE_OUT_OF_RANGE_START as utf32;
    pub const LOWERCASE_OUT_OF_RANGE_END_UTF32:   utf32 = LOWERCASE_OUT_OF_RANGE_END as utf32;

    pub const DIGIT_OFFSET:     u8 = DIGIT_START;
    pub const UPPERCASE_OFFSET: u8 = UPPERCASE_START - Base::Decimal as u8;
    pub const LOWERCASE_OFFSET: u8 = LOWERCASE_START - Base::Decimal as u8;

    //# Ranges macros

    #[macro_export] macro_rules! hexadecimal_digit                        { () => { $crate::digit::hexadecimal::DIGIT_START                         ..= $crate::digit::hexadecimal::DIGIT_END }; }
    #[macro_export] macro_rules! hexadecimal_uppercase                    { () => { $crate::digit::hexadecimal::UPPERCASE_START                     ..= $crate::digit::hexadecimal::UPPERCASE_END }; }
    #[macro_export] macro_rules! hexadecimal_lowercase                    { () => { $crate::digit::hexadecimal::LOWERCASE_START                     ..= $crate::digit::hexadecimal::LOWERCASE_END }; }
    #[macro_export] macro_rules! hexadecimal_uppercase_out_of_range       { () => { $crate::digit::hexadecimal::UPPERCASE_OUT_OF_RANGE_START        ..= $crate::digit::hexadecimal::UPPERCASE_OUT_OF_RANGE_END }; }
    #[macro_export] macro_rules! hexadecimal_lowercase_out_of_range       { () => { $crate::digit::hexadecimal::LOWERCASE_OUT_OF_RANGE_START        ..= $crate::digit::hexadecimal::LOWERCASE_OUT_OF_RANGE_END }; }
    #[macro_export] macro_rules! hexadecimal_out_of_range                 { () => { $crate::digit::hexadecimal::hexadecimal_uppercase_out_of_range!() | $crate::digit::hexadecimal::hexadecimal_lowercase_out_of_range!() }; }

    #[macro_export] macro_rules! hexadecimal_digit_utf32                  { () => { $crate::digit::hexadecimal::DIGIT_START_UTF32                         ..= $crate::digit::hexadecimal::DIGIT_END_UTF32 }; }
    #[macro_export] macro_rules! hexadecimal_uppercase_utf32              { () => { $crate::digit::hexadecimal::UPPERCASE_START_UTF32                     ..= $crate::digit::hexadecimal::UPPERCASE_END_UT}; }
    #[macro_export] macro_rules! hexadecimal_lowercase_utf32              { () => { $crate::digit::hexadecimal::LOWERCASE_START_UTF32                     ..= $crate::digit::hexadecimal::LOWERCASE_END_UT}; }
    #[macro_export] macro_rules! hexadecimal_uppercase_out_of_range_utf32 { () => { $crate::digit::hexadecimal::UPPERCASE_OUT_OF_RANGE_START_UTF32        ..= $crate::digit::hexadecimal::UPPERCASE_OUT_OF_RANGE_END_UTF32 }; }
    #[macro_export] macro_rules! hexadecimal_lowercase_out_of_range_utf32 { () => { $crate::digit::hexadecimal::LOWERCASE_OUT_OF_RANGE_START_UTF32        ..= $crate::digit::hexadecimal::LOWERCASE_OUT_OF_RANGE_END_UTF32 }; }
    #[macro_export] macro_rules! hexadecimal_out_of_range_utf32           { () => { $crate::digit::hexadecimal::hexadecimal_uppercase_out_of_range_utf32!() | $crate::digit::hexadecimal::hexadecimal_lowercase_out_of_range_utf32!() }; }

    pub use hexadecimal_digit;
    pub use hexadecimal_uppercase;
    pub use hexadecimal_lowercase;
    pub use hexadecimal_uppercase_out_of_range;
    pub use hexadecimal_lowercase_out_of_range;
    pub use hexadecimal_out_of_range;

    pub use hexadecimal_digit_utf32;
    pub use hexadecimal_uppercase_utf32;
    pub use hexadecimal_lowercase_utf32;
    pub use hexadecimal_uppercase_out_of_range_utf32;
    pub use hexadecimal_lowercase_out_of_range_utf32;
    pub use hexadecimal_out_of_range_utf32;

    //# Methods

    #[must_use] #[inline]         pub const fn is_digit(ch: ascii)                  -> bool { return matches!(ch, hexadecimal_digit!()); }
    #[must_use] #[inline]         pub const fn is_uppercase(ch: ascii)              -> bool { return matches!(ch, hexadecimal_uppercase!()); }
    #[must_use] #[inline]         pub const fn is_lowercase(ch: ascii)              -> bool { return matches!(ch, hexadecimal_lowercase!()); }
    #[must_use] #[inline]         pub const fn is_uppercase_out_of_range(ch: ascii) -> bool { return matches!(ch, hexadecimal_uppercase_out_of_range!()); }
    #[must_use] #[inline]         pub const fn is_lowercase_out_of_range(ch: ascii) -> bool { return matches!(ch, hexadecimal_lowercase_out_of_range!()); }
    #[must_use] #[inline]         pub const fn is_out_of_range(ch: ascii)           -> bool { return is_uppercase_out_of_range(ch) || is_lowercase_out_of_range(ch); }
    #[must_use] #[inline(always)] pub const fn is(ch: ascii)                        -> bool { return is_digit(ch); }

    //# Parsing

    #[must_use]
    #[inline]
    pub const fn check(ch: ascii) -> Offset {
        return match ch {
            hexadecimal_digit!()        => DIGIT_OFFSET,
            hexadecimal_out_of_range!() => return OUT_OF_RANGE,
            _                           => return NOT_A_DIGIT,
        };
    }

    #[must_use]
    #[inline]
    pub const fn parse(ch: ascii) -> Digit {
        let offset = match ch {
            hexadecimal_digit!()        => DIGIT_OFFSET,
            hexadecimal_out_of_range!() => return OUT_OF_RANGE,
            _                           => return NOT_A_DIGIT,
        };
        return ch - offset;
    }
}

#[rustfmt::skip]
pub mod any {
    #[allow(clippy::wildcard_imports)]
    use super::*;

    //# Ranges

    pub const DIGIT:     AsciiRange<ascii> = AsciiRange { start: b'0', end: b'9' };
    pub const UPPERCASE: AsciiRange<ascii> = AsciiRange { start: b'A', end: b'Z' };
    pub const LOWERCASE: AsciiRange<ascii> = AsciiRange { start: b'a', end: b'z' };

    pub const RANGES:         [AsciiRange<ascii>; 3] = [DIGIT, UPPERCASE, LOWERCASE];
    pub const DIGIT_RANGES:   [AsciiRange<ascii>; 1] = [DIGIT];
    pub const LETTERS_RANGES: [AsciiRange<ascii>; 2] = [       UPPERCASE, LOWERCASE];

    pub const RANGES_UTF32:         [AsciiRange<utf32>; 3] = [DIGIT.to_utf32(), UPPERCASE.to_utf32(), LOWERCASE.to_utf32()];
    pub const DIGIT_RANGES_UTF32:   [AsciiRange<utf32>; 1] = [DIGIT.to_utf32()];
    pub const LETTERS_RANGES_UTF32: [AsciiRange<utf32>; 2] = [                  UPPERCASE.to_utf32(), LOWERCASE.to_utf32()];

    pub const RANGES_OPS:         [RangeInclusive<ascii>; 3] = [DIGIT.to_range_inclusive(), UPPERCASE.to_range_inclusive(), LOWERCASE.to_range_inclusive()];
    pub const DIGIT_RANGES_OPS:   [RangeInclusive<ascii>; 1] = [DIGIT.to_range_inclusive()];
    pub const LETTERS_RANGES_OPS: [RangeInclusive<ascii>; 2] = [                            UPPERCASE.to_range_inclusive(), LOWERCASE.to_range_inclusive()];

    pub const RANGES_OPS_UTF32:         [RangeInclusive<utf32>; 3] = [DIGIT.to_utf32().to_range_inclusive(), UPPERCASE.to_utf32().to_range_inclusive(), LOWERCASE.to_utf32().to_range_inclusive()];
    pub const DIGIT_RANGES_OPS_UTF32:   [RangeInclusive<utf32>; 1] = [DIGIT.to_utf32().to_range_inclusive()];
    pub const LETTERS_RANGES_OPS_UTF32: [RangeInclusive<utf32>; 2] = [                                       UPPERCASE.to_utf32().to_range_inclusive(), LOWERCASE.to_utf32().to_range_inclusive()];

    //# Ranges constants

    pub const DIGIT_START:                  u8 = DIGIT.start;
    pub const DIGIT_END:                    u8 = DIGIT.end;
    pub const UPPERCASE_START:              u8 = UPPERCASE.start;
    pub const UPPERCASE_END:                u8 = UPPERCASE.end;
    pub const LOWERCASE_START:              u8 = LOWERCASE.start;
    pub const LOWERCASE_END:                u8 = LOWERCASE.end;

    pub const DIGIT_START_UTF32:                  utf32 = DIGIT_START as utf32;
    pub const DIGIT_END_UTF32:                    utf32 = DIGIT_END as utf32;
    pub const UPPERCASE_START_UTF32:              utf32 = UPPERCASE_START as utf32;
    pub const UPPERCASE_END_UTF32:                utf32 = UPPERCASE_END as utf32;
    pub const LOWERCASE_START_UTF32:              utf32 = LOWERCASE_START as utf32;
    pub const LOWERCASE_END_UTF32:                utf32 = LOWERCASE_END as utf32;

    pub const DIGIT_OFFSET:     u8 = DIGIT_START;
    pub const UPPERCASE_OFFSET: u8 = UPPERCASE_START - Base::Decimal as u8;
    pub const LOWERCASE_OFFSET: u8 = LOWERCASE_START - Base::Decimal as u8;

    //# Ranges macros

    #[macro_export] macro_rules! digit                { () => { $crate::digit::any::DIGIT_START       ..= $crate::digit::any::DIGIT_END }; }
    #[macro_export] macro_rules! uppercase            { () => { $crate::digit::any::UPPERCASE_START   ..= $crate::digit::any::UPPERCASE_END }; }
    #[macro_export] macro_rules! lowercase            { () => { $crate::digit::any::LOWERCASE_START   ..= $crate::digit::any::LOWERCASE_END }; }
    #[macro_export] macro_rules! letter               { () => { $crate::digit::any::range_uppercase!()  | $crate::digit::any::range_lowercase!() }; }
    #[macro_export] macro_rules! alphanumerical       { () => { $crate::digit::any::range_digit!()      | $crate::digit::any::range_letter!() }; }

    #[macro_export] macro_rules! digit_utf32          { () => { $crate::digit::any::DIGIT_START_UTF32     ..= $crate::digit::any::DIGIT_END_UTF32 }; }
    #[macro_export] macro_rules! uppercase_utf32      { () => { $crate::digit::any::UPPERCASE_START_UTF32 ..= $crate::digit::any::UPPERCASE_END_UTF32 }; }
    #[macro_export] macro_rules! lowercase_utf32      { () => { $crate::digit::any::LOWERCASE_START_UTF32 ..= $crate::digit::any::LOWERCASE_END_UTF32 }; }
    #[macro_export] macro_rules! letter_utf32         { () => { $crate::digit::any::range_uppercase!()      | $crate::digit::any::range_lowercase!() }; }
    #[macro_export] macro_rules! alphanumerical_utf32 { () => { $crate::digit::any::range_digit!()          | $crate::digit::any::range_letter!() }; }

    //# Methods

    #[must_use] #[inline] pub const fn is_digit(ch: ascii)          -> bool { return matches!(ch, digit!()); }
    #[must_use] #[inline] pub const fn is_uppercase(ch: ascii)      -> bool { return matches!(ch, uppercase!()); }
    #[must_use] #[inline] pub const fn is_lowercase(ch: ascii)      -> bool { return matches!(ch, lowercase!()); }
    #[must_use] #[inline] pub const fn is_letter(ch: ascii)         -> bool { return is_uppercase(ch) || is_lowercase(ch); }
    #[must_use] #[inline] pub const fn is(ch: ascii)                -> bool { return is_digit(ch) || is_letter(ch); }

    //# Parsing

    #[must_use]
    #[inline]
    pub const fn check(ch: ascii, base: u8) -> OffsetCustomBase {
        if base < Base::MIN {
            return BASE_MIN;
        }
        if base > Base::MAX {
            return BASE_MAX;
        }

        let offset = match ch {
            digit!() => DIGIT_OFFSET,
            uppercase!() => UPPERCASE_OFFSET,
            lowercase!() => LOWERCASE_OFFSET,
            _ => return NOT_A_DIGIT,
        };
        let digit = ch - offset;
        if digit >= base {
            return OUT_OF_RANGE;
        }
        return offset;
    }

    #[must_use]
    #[inline]
    pub const fn parse(ch: ascii, base: u8) -> DigitCustomBase {
        if base < Base::MIN {
            return BASE_MIN;
        }
        if base > Base::MAX {
            return BASE_MAX;
        }

        let offset = match ch {
            digit!() => DIGIT_OFFSET,
            uppercase!() => UPPERCASE_OFFSET,
            lowercase!() => LOWERCASE_OFFSET,
            _ => return NOT_A_DIGIT,
        };
        let digit = ch - offset;
        if digit >= base {
            return OUT_OF_RANGE;
        }
        return digit;
    }
}
