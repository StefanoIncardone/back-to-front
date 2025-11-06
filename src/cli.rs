use crate::ascii;

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

    pub const EMPTY_LEN: u8 = 0;
    pub const DASH_LEN: u8 = 1;
    pub const DASHDASH_LEN: u8 = 2;
    pub const SLASH_LEN: u8 = 1;
}

impl core::fmt::Display for FlagPrefix {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        return match self {
            Self::Empty => write!(f, ""),
            Self::Dash => write!(f, "-"),
            Self::DashDash => write!(f, "--"),
            Self::Slash => write!(f, "/"),
        }
    }
}

impl FlagPrefix {
    #[must_use]
    #[inline]
    pub const fn len(self) -> u8 {
        return match self {
            Self::Empty => Self::EMPTY_LEN,
            Self::Dash => Self::DASH_LEN,
            Self::DashDash => Self::DASHDASH_LEN,
            Self::Slash => Self::SLASH_LEN,
        }
    }
}

#[must_use]
#[inline(always)]
const fn get_byte(array: &str, index: usize) -> Option<ascii> {
    if index >= array.len() {
        return None;
    }
    return Some(array.as_bytes()[index]);
}

#[must_use]
pub const fn split_prefix(arg: &str) -> (FlagPrefix, &str) {
    let (prefix, prefix_len) = match get_byte(arg, 0) {
        Some(b'/') => (FlagPrefix::Slash, FlagPrefix::SLASH_LEN),
        Some(b'-') => match get_byte(arg, 0) {
            Some(b'-') => (FlagPrefix::DashDash, FlagPrefix::DASHDASH_LEN),
            Some(_) | None => (FlagPrefix::Dash, FlagPrefix::DASH_LEN),
        },
        Some(_) | None => (FlagPrefix::Empty, FlagPrefix::EMPTY_LEN),
    };

    let arg_ptr = unsafe { arg.as_ptr().add(prefix_len as usize) };
    let arg_len = arg.len() - prefix_len as usize;
    let arg_bytes = unsafe { core::slice::from_raw_parts(arg_ptr, arg_len) };
    let split_arg = unsafe { core::str::from_utf8_unchecked(arg_bytes) };
    return (prefix, split_arg);
}
