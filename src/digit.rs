use crate::{ascii, utf32};

pub trait Ascii {}
impl Ascii for ascii {}
impl Ascii for utf32 {}

pub struct AsciiRange<I: Ascii> {
    pub start: I,
    pub end: I,
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

// Ranges utf32
#[rustfmt::skip]
impl Base {
    pub const BINARY_RANGE:                AsciiRange<utf32> = AsciiRange { start: '0', end: '1' };
    pub const OCTAL_RANGE:                 AsciiRange<utf32> = AsciiRange { start: '0', end: '8' };
    pub const DECIMAL_RANGE:               AsciiRange<utf32> = AsciiRange { start: '0', end: '9' };
    pub const HEXADECIMAL_DIGIT_RANGE:     AsciiRange<utf32> = Self::DECIMAL_RANGE;
    pub const HEXADECIMAL_UPPERCASE_RANGE: AsciiRange<utf32> = AsciiRange { start: 'A', end: 'F' };
    pub const HEXADECIMAL_LOWERCASE_RANGE: AsciiRange<utf32> = AsciiRange { start: 'a', end: 'f' };

    pub const BINARY_RANGES:      [AsciiRange<utf32>; 1] = [Self::BINARY_RANGE];
    pub const OCTAL_RANGES:       [AsciiRange<utf32>; 1] = [Self::OCTAL_RANGE];
    pub const DECIMAL_RANGES:     [AsciiRange<utf32>; 1] = [Self::DECIMAL_RANGE];
    pub const HEXADECIMAL_RANGES: [AsciiRange<utf32>; 3] = [
        Self::HEXADECIMAL_DIGIT_RANGE,
        Self::HEXADECIMAL_UPPERCASE_RANGE,
        Self::HEXADECIMAL_LOWERCASE_RANGE,
    ];

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
}

// Ranges ascii
#[rustfmt::skip]
impl Base {
    pub const BINARY_RANGE_ASCII:                AsciiRange<ascii> = AsciiRange { start: b'0', end: b'1' };
    pub const OCTAL_RANGE_ASCII:                 AsciiRange<ascii> = AsciiRange { start: b'0', end: b'8' };
    pub const DECIMAL_RANGE_ASCII:               AsciiRange<ascii> = AsciiRange { start: b'0', end: b'9' };
    pub const HEXADECIMAL_DIGIT_RANGE_ASCII:     AsciiRange<ascii> = Self::DECIMAL_RANGE_ASCII;
    pub const HEXADECIMAL_UPPERCASE_RANGE_ASCII: AsciiRange<ascii> = AsciiRange { start: b'A', end: b'F' };
    pub const HEXADECIMAL_LOWERCASE_RANGE_ASCII: AsciiRange<ascii> = AsciiRange { start: b'a', end: b'f' };

    pub const BINARY_RANGES_ASCII:      [AsciiRange<ascii>; 1] = [Self::BINARY_RANGE_ASCII];
    pub const OCTAL_RANGES_ASCII:       [AsciiRange<ascii>; 1] = [Self::OCTAL_RANGE_ASCII];
    pub const DECIMAL_RANGES_ASCII:     [AsciiRange<ascii>; 1] = [Self::DECIMAL_RANGE_ASCII];
    pub const HEXADECIMAL_RANGES_ASCII: [AsciiRange<ascii>; 3] = [
        Self::HEXADECIMAL_DIGIT_RANGE_ASCII,
        Self::HEXADECIMAL_UPPERCASE_RANGE_ASCII,
        Self::HEXADECIMAL_LOWERCASE_RANGE_ASCII,
    ];

    pub const BINARY_ASCII_OFFSET:                u8 = Self::BINARY_RANGE_ASCII.start;
    pub const OCTAL_ASCII_OFFSET:                 u8 = Self::OCTAL_RANGE_ASCII.start;
    pub const DECIMAL_ASCII_OFFSET:               u8 = Self::DECIMAL_RANGE_ASCII.start;
    pub const HEXADECIMAL_DIGIT_ASCII_OFFSET:     u8 = Self::DECIMAL_ASCII_OFFSET;
    pub const HEXADECIMAL_UPPERCASE_ASCII_OFFSET: u8 = Self::HEXADECIMAL_UPPERCASE_RANGE_ASCII.start - Self::Decimal as u8;
    pub const HEXADECIMAL_LOWERCASE_ASCII_OFFSET: u8 = Self::HEXADECIMAL_LOWERCASE_RANGE_ASCII.start - Self::Decimal as u8;

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
}

// Parsing
impl Base {
    #[must_use]
    #[inline(always)]
    pub const fn check(self, character: ascii) -> AsciiDigit {
        return check(character, self);
    }

    #[must_use]
    #[inline(always)]
    pub const fn parse(self, character: ascii) -> Digit {
        return parse(character, self);
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum AsciiDigit {
    Ok,
    Underscore,
    Dot,
    OutOfRange,
    Other,
}

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
pub const fn check(character: ascii, base: Base) -> AsciiDigit {
    #[rustfmt::skip]
    return match base {
        Base::Binary      => check_binary(character),
        Base::Octal       => check_octal(character),
        Base::Decimal     => check_decimal(character),
        Base::Hexadecimal => check_hexadecimal(character),
    };
}

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

#[must_use]
#[inline]
pub const fn check_binary(character: ascii) -> AsciiDigit {
    #[rustfmt::skip]
    return match character {
        b'0'..=b'1'                             => AsciiDigit::Ok,
        b'_'                                    => AsciiDigit::Underscore,
        b'.'                                    => AsciiDigit::Dot,
        b'2'..=b'9' | b'A'..=b'Z' | b'a'..=b'z' => AsciiDigit::OutOfRange,
        _                                       => AsciiDigit::Other,
    };
}

#[must_use]
#[inline]
pub const fn parse_binary(character: ascii) -> Digit {
    #[rustfmt::skip]
    let offset = match character {
        b'0'..=b'1'                             => Base::BINARY_ASCII_OFFSET,
        b'_'                                    => return Digit::Underscore,
        b'.'                                    => return Digit::Dot,
        b'2'..=b'9' | b'A'..=b'Z' | b'a'..=b'z' => return Digit::OutOfRange,
        _                                       => return Digit::Other,
    };
    return Digit::Ok(character - offset);
}

#[must_use]
#[inline]
pub const fn check_octal(character: ascii) -> AsciiDigit {
    #[rustfmt::skip]
    return match character {
        b'0'..=b'7'                             => AsciiDigit::Ok,
        b'_'                                    => AsciiDigit::Underscore,
        b'.'                                    => AsciiDigit::Dot,
        b'8'..=b'9' | b'A'..=b'Z' | b'a'..=b'z' => AsciiDigit::OutOfRange,
        _                                       => AsciiDigit::Other,
    };
}

#[must_use]
#[inline]
pub const fn parse_octal(character: ascii) -> Digit {
    #[rustfmt::skip]
    let offset = match character {
        b'0'..=b'7'                             => Base::OCTAL_ASCII_OFFSET,
        b'_'                                    => return Digit::Underscore,
        b'.'                                    => return Digit::Dot,
        b'8'..=b'9' | b'A'..=b'Z' | b'a'..=b'z' => return Digit::OutOfRange,
        _                                       => return Digit::Other,
    };
    return Digit::Ok(character - offset);
}

#[must_use]
#[inline]
pub const fn check_decimal(character: ascii) -> AsciiDigit {
    #[rustfmt::skip]
    return match character {
        b'0'..=b'9'               => AsciiDigit::Ok,
        b'_'                      => AsciiDigit::Underscore,
        b'.'                      => AsciiDigit::Dot,
        b'A'..=b'Z' | b'a'..=b'z' => AsciiDigit::OutOfRange,
        _                         => AsciiDigit::Other,
    };
}

#[must_use]
#[inline]
pub const fn parse_decimal(character: ascii) -> Digit {
    #[rustfmt::skip]
    let offset = match character {
        b'0'..=b'9'               => Base::DECIMAL_ASCII_OFFSET,
        b'_'                      => return Digit::Underscore,
        b'.'                      => return Digit::Dot,
        b'A'..=b'Z' | b'a'..=b'z' => return Digit::OutOfRange,
        _                         => return Digit::Other,
    };
    return Digit::Ok(character - offset);
}

#[must_use]
#[inline]
pub const fn check_hexadecimal(character: ascii) -> AsciiDigit {
    #[rustfmt::skip]
    return match character {
        b'0'..=b'9' | b'A'..=b'F' | b'a'..=b'f' => AsciiDigit::Ok,
        b'_'                                    => AsciiDigit::Underscore,
        b'.'                                    => AsciiDigit::Dot,
        b'G'..=b'Z' | b'g'..=b'z'               => AsciiDigit::OutOfRange,
        _                                       => AsciiDigit::Other,
    };
}

#[must_use]
#[inline]
pub const fn parse_hexadecimal(character: ascii) -> Digit {
    #[rustfmt::skip]
    let offset = match character {
        b'0'..=b'9'               => Base::HEXADECIMAL_DIGIT_ASCII_OFFSET,
        b'A'..=b'F'               => Base::HEXADECIMAL_UPPERCASE_ASCII_OFFSET,
        b'a'..=b'f'               => Base::HEXADECIMAL_LOWERCASE_ASCII_OFFSET,
        b'_'                      => return Digit::Underscore,
        b'.'                      => return Digit::Dot,
        b'G'..=b'Z' | b'g'..=b'z' => return Digit::OutOfRange,
        _                         => return Digit::Other,
    };
    return Digit::Ok(character - offset);
}

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
        b'0'..=b'9' => Base::DECIMAL_ASCII_OFFSET,
        b'A'..=b'Z' => Base::HEXADECIMAL_UPPERCASE_ASCII_OFFSET,
        b'a'..=b'z' => Base::HEXADECIMAL_LOWERCASE_ASCII_OFFSET,
        b'_'        => return AsciiDigitCustomBase::Underscore,
        b'.'        => return AsciiDigitCustomBase::Dot,
        _           => return AsciiDigitCustomBase::Other,
    };

    let digit = character - offset;
    if digit >= base {
        return AsciiDigitCustomBase::OutOfRange;
    }
    return AsciiDigitCustomBase::Ok;
}

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
        b'0'..=b'9' => Base::DECIMAL_ASCII_OFFSET,
        b'A'..=b'Z' => Base::HEXADECIMAL_UPPERCASE_ASCII_OFFSET,
        b'a'..=b'z' => Base::HEXADECIMAL_LOWERCASE_ASCII_OFFSET,
        b'_'        => return DigitCustomBase::Underscore,
        b'.'        => return DigitCustomBase::Dot,
        _           => return DigitCustomBase::Other,
    };

    let digit = character - offset;
    if digit >= base {
        return DigitCustomBase::OutOfRange;
    }
    return DigitCustomBase::Ok(digit);
}

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

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    use super::*;

    macro_rules! test_assert {
        ($expression:expr, == $pattern:pat $(if $guard:expr)? $(,)?) => {
            const _: () = {
                match $expression {
                    $pattern $(if $guard)? => {},
                    _ => panic!(),
                }
            };
        };
        ($expression:expr, != $pattern:pat $(if $guard:expr)? $(,)?) => {
            const _: () = {
                match $expression {
                    $pattern $(if $guard)? => panic!(),
                    _ => {},
                }
            };
        };
    }

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
