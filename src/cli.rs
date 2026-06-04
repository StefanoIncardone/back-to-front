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
    assert!(start_index <= end_index, "start index cannot be greater than end index");
    let slice_len = end_index - start_index;
    let array_ptr = unsafe { array.as_ptr().add(start_index) };
    let array_bytes = unsafe { core::slice::from_raw_parts(array_ptr, slice_len) };
    let array_slice = unsafe { core::str::from_utf8_unchecked(array_bytes) };
    return array_slice;
}

#[must_use]
#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum FlagPrefix {
    Empty    = 0b0000_0000,
    Dash     = 0b0000_0001,
    DashDash = 0b0000_0010,
    Slash    = 0b0000_0011,
}

impl FlagPrefix {
    pub const MASK: u8 = 0b0000_0011;
}

impl FlagPrefix {
    #[must_use]
    #[inline]
    pub const fn to_str(self) -> &'static str {
        return match self {
            Self::Empty => "",
            Self::Dash => "-",
            Self::DashDash => "--",
            Self::Slash => "/",
        }
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

impl FlagSeparator {
    pub const MASK: u8 = 0b0000_0011;
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

    pub const fn parse(arg: &str) -> Self {
        let prefix = match get_byte(arg, 0) {
            None => return Self::empty(),
            Some(b'/') => FlagPrefix::Slash,
            Some(b'-') => match get_byte(arg, 1) {
                Some(b'-') => FlagPrefix::DashDash,
                Some(_) | None => FlagPrefix::Dash,
            },
            Some(_) => FlagPrefix::Empty,
        };
        let prefix_len = prefix.to_str().len();

        let mut separator_index = prefix_len;
        let mut separator = FlagSeparator::Empty;
        loop {
            match get_byte(arg, separator_index) {
                None => break,
                Some(b'-') => separator = FlagSeparator::Dash,
                Some(b':') => separator = FlagSeparator::Colon,
                Some(b'=') => separator = FlagSeparator::Equals,
                Some(_) => separator_index += 1,
            }
        }

        let key_text_len = separator_index - prefix_len;
        assert!(key_text_len <= uoffset16::MAX as usize, "key length cannot be greater than u16::MAX");
        return Self { prefix, key_text_len: key_text_len as uoffset16, separator };
    }

    #[must_use]
    /// # Safety
    /// Expected to be called on the same argument that was previously passed to [`Self::parse`]
    pub const unsafe fn key_value(self, arg: &str) -> (&str, &str) {
        let key_start_index = self.prefix.to_str().len();
        let key_len = self.key_text_len as usize;
        let key_text = get_slice(arg, key_start_index, key_len);

        let value_start_index = key_start_index + key_len + self.separator.to_str().len();
        let value_len = arg.len() - value_start_index;
        let value_text = get_slice(arg, value_start_index, value_len);

        return (key_text, value_text);
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
