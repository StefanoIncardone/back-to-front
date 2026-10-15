use crate::{ascii, usize32};


#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[must_use] #[repr(u8)]
pub enum LineEnd {
    LF   = Self::LF_STR.len() as u8,
    CRLF = Self::CRLF_STR.len() as u8,
}

impl LineEnd {
    pub const CR: Self = Self::LF;

    pub const LF_CHAR:  ascii = b'\n';
    pub const CR_CHAR:  ascii = b'\r';
    pub const LF_STR:   &str  = "\n";
    pub const CR_STR:   &str  = "\r";
    pub const CRLF_STR: &str  = "\r\n";
}

impl LineEnd {
    #[must_use] #[inline(always)]
    pub const fn len(self) -> usize {
        return self as usize;
    }

    #[must_use] #[inline(always)]
    pub const fn to_str(self) -> &'static str {
        #[rustfmt::skip]
        return match self {
            Self::LF   => Self::LF_STR,
            Self::CRLF => Self::CRLF_STR,
        };
    }
}


// IDEA(stefano): make generic over the type of start and end
// - constrain the generic type to be number-like
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
#[must_use]
pub struct Span {
    /// Inclusive and less that or equal to [`Self::end()`]
    pub(crate) start: usize32,

    /// Non inclusive and greater than [`Self::start()`]
    pub(crate) end: usize32,
}

// TODO(stefano): add debug_assert/panic/checked/unchecked setters for `start` and `end`
// Getters
impl Span {
    /// Inclusive and less that or equal to [`Self::end()`]
    #[must_use] #[inline(always)]
    pub const fn start(self) -> usize32 {
        return self.start;
    }

    /// Non inclusive and greater than [`Self::start()`]
    #[must_use] #[inline(always)]
    pub const fn end(self) -> usize32 {
        return self.end;
    }
}

impl Span {
    /// # Safety
    ///
    /// `start` must be less than or equal to `end`
    #[inline(always)] #[track_caller]
    pub const unsafe fn new_unchecked(start: usize32, end: usize32) -> Self {
        debug_assert!(start <= end, "`start` must be less than or equal to `end`");
        return Self { start, end };
    }

    #[must_use] #[inline]
    pub const fn new(start: usize32, end: usize32) -> Option<Self> {
        if start > end { return None; }
        return Some(Self { start, end });
    }
}

pub type Line = Span;
