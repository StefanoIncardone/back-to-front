use core::fmt::Display;
use crate::{ascii, uoffset16};

#[must_use]
#[inline(always)]
const fn get_byte(array: &str, index: usize) -> Option<ascii> {
    if index >= array.len() {
        return None;
    }
    return Some(array.as_bytes()[index]);
}

#[track_caller]
#[must_use]
#[inline(always)]
const fn get_slice(array: &str, start_index: usize, end_index: usize) -> &str {
    assert!(start_index <= array.len(), "start index overflow");
    assert!(start_index <= end_index, "start index cannot be greater than end index");

    let array_len = end_index - start_index;
    let array_ptr = unsafe { array.as_ptr().add(start_index) };
    let array_bytes = unsafe { core::slice::from_raw_parts(array_ptr, array_len) };
    let array_slice = unsafe { core::str::from_utf8_unchecked(array_bytes) };
    return array_slice;
}

#[must_use]
pub const fn bits(masks: &[u8]) -> u8 {
    let mut mask = 0;

    let mut sub_mask_index = 0;
    while sub_mask_index < masks.len() {
        let sub_mask = masks[sub_mask_index];
        sub_mask_index += 1;

        let collides = mask & sub_mask > 0;
        assert!(!collides, "two bits collided");

        mask |= sub_mask;
    }

    return mask;
}

#[must_use]
pub const fn mask(masks: &[u8]) -> u8 {
    let mut mask = 0;

    let mut sub_mask_index = 0;
    while sub_mask_index < masks.len() {
        let sub_mask = masks[sub_mask_index];
        sub_mask_index += 1;
        mask |= sub_mask;
    }

    return mask;
}

#[must_use]
pub const fn mask_shift(mask: u8) -> u8 {
    return (u8::BITS - mask.leading_zeros()) as u8;
}


pub trait Mask: Sized {
    const MASK: u8;
    const MASK_SHIFT: u8 = mask_shift(Self::MASK);
}

#[must_use]
#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum FlagPrefix {
    Empty    = 0b0000_0000,
    Dash     = 0b0000_0001,
    DashDash = 0b0000_0010,
}

impl Mask for FlagPrefix {
    const MASK: u8 = mask(&[
        Self::Empty as u8,
        Self::Dash as u8,
        Self::DashDash as u8,
    ]);
}

impl FlagPrefix {
    #[must_use]
    pub const fn combine(self, tag: u8) -> u8 {
        return bits(&[self as u8, tag << Self::MASK_SHIFT]);
    }

    #[must_use]
    #[inline]
    pub const fn to_str(self) -> &'static str {
        return match self {
            Self::Empty => "",
            Self::Dash => "-",
            Self::DashDash => "--",
        }
    }
}

impl Display for FlagPrefix {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        return f.write_str(self.to_str());
    }
}


#[must_use]
#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum FlagSeparator {
    Empty  = 0b0000_0000,
    Dash   = 0b0000_0001,
    Equals = 0b0000_0010,
    Colon  = 0b0000_0011,
}

impl Mask for FlagSeparator {
    const MASK: u8 = mask(&[
        Self::Empty as u8,
        Self::Dash as u8,
        Self::Equals as u8,
        Self::Colon as u8,
    ]);
}

impl FlagSeparator {
    #[must_use]
    #[inline]
    pub const fn to_str(self) -> &'static str {
        return match self {
            Self::Empty => "",
            Self::Dash => "-",
            Self::Equals => "=",
            Self::Colon => ":",
        }
    }
}

impl Display for FlagSeparator {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        return f.write_str(self.to_str());
    }
}


#[must_use]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum ArgResult {
    Ok(Arg),
    MaxLen,
}


#[must_use]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Arg {
    pub prefix: FlagPrefix,
    pub key_text_len: uoffset16,
    pub separator: FlagSeparator,
}

impl Arg {
    #[inline(always)]
    pub const fn empty() -> Self {
        return Self { prefix: FlagPrefix::Empty, key_text_len: 0, separator: FlagSeparator::Empty };
    }

    pub const fn parse(mut arg: &str) -> ArgResult {
        arg = arg.trim_ascii();

        let prefix = match get_byte(arg, 0) {
            None => return ArgResult::Ok(Self::empty()),
            Some(b'-') => match get_byte(arg, 1) {
                Some(b'-') => FlagPrefix::DashDash,
                Some(_) | None => FlagPrefix::Dash,
            },
            Some(_) => FlagPrefix::Empty,
        };
        let prefix_len = prefix.to_str().len();

        let mut separator_index = prefix_len;
        let separator = loop {
            match get_byte(arg, separator_index) {
                None =>       break FlagSeparator::Empty,
                Some(b'-') => break FlagSeparator::Dash,
                Some(b':') => break FlagSeparator::Colon,
                Some(b'=') => break FlagSeparator::Equals,
                Some(_) => separator_index += 1,
            }
        };

        let key_text_len = separator_index - prefix_len;
        if key_text_len > uoffset16::MAX as usize {
            return ArgResult::MaxLen;
        }
        return ArgResult::Ok(Self { prefix, key_text_len: key_text_len as uoffset16, separator });
    }


    #[must_use]
    pub const fn start_of_key_index(self) -> u16 {
        return self.prefix.to_str().len() as u16;
    }

    #[must_use]
    pub const fn start_of_separator_index(self) -> u16 {
        let start_of_key_index = self.start_of_key_index();
        return start_of_key_index + self.key_text_len;
    }

    #[must_use]
    pub const fn start_of_value_index(self) -> Option<u16> {
        if let FlagSeparator::Empty = self.separator {
            return None;
        }

        let start_of_separator_index = self.start_of_separator_index();
        return Some(start_of_separator_index + self.separator.to_str().len() as u16);
    }


    #[must_use]
    /// # Safety
    /// Expected to be called on the same argument that was previously passed to [`Self::parse`]
    pub const unsafe fn key(self, arg: &str) -> &str {
        let key_start_index = self.start_of_key_index() as usize;
        let key_end_index = key_start_index + self.key_text_len as usize;
        let key_text = get_slice(arg, key_start_index, key_end_index);
        return key_text;
    }

    #[must_use]
    /// # Safety
    /// Expected to be called on the same argument that was previously passed to [`Self::parse`]
    pub const unsafe fn key_value(self, arg: &str) -> (&str, &str) {
        let key_start_index = self.start_of_key_index() as usize;
        let key_end_index = key_start_index + self.key_text_len as usize;
        let key_text = get_slice(arg, key_start_index, key_end_index);

        let value_start_index = key_end_index + self.separator.to_str().len();
        let value_end_index = arg.len();
        let value_text = get_slice(arg, value_start_index, value_end_index);

        return (key_text, value_text);
    }

    #[must_use]
    pub const fn is_empty(self) -> bool {
        if let Self { prefix: FlagPrefix::Empty, key_text_len: 0, separator: FlagSeparator::Empty } = self {
            return true;
        }
        return false;
    }
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum AnsiFg {
    #[default]
    Default      = 0,
    Black        = 30,
    Red          = 31,
    Green        = 32,
    Yellow       = 33,
    Blue         = 34,
    Magenta      = 35,
    Cyan         = 36,
    LightGray    = 37,
    DarkGray     = 90,
    LightRed     = 91,
    LightGreen   = 92,
    LightYellow  = 93,
    LightBlue    = 94,
    LightMagenta = 95,
    LightCyan    = 96,
    White        = 97,
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum AnsiBg {
    #[default]
    Default       = 0,
    Black         = 40,
    DarkRed       = 41,
    DarkGreen     = 42,
    DarkYellow    = 43,
    DarkBlue      = 44,
    DarkMagenta   = 45,
    DarkCyan      = 46,
    DarkWhite     = 47,
    BrightBlack   = 100,
    BrightRed     = 101,
    BrightGreen   = 102,
    BrightYellow  = 103,
    BrightBlue    = 104,
    BrightMagenta = 105,
    BrightCyan    = 106,
    White         = 107,
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum AnsiCode {
    #[default]
    Default      = 0,
    Bold         = 1,
    Underline    = 4,
    NoUnderline  = 24,
    ReverseText  = 7,
    PositiveText = 27,
}

// TODO: backwards compatibility tests
