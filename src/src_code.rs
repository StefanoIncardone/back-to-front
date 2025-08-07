use crate::offset32;

// IDEA(stefano): make generic over the type of start and end
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub struct Span {
    /// Inclusive and less that or equal to [`Self::end()`]
    pub(crate) start: offset32,

    /// Non inclusive and greater than [`Self::start()`]
    pub(crate) end: offset32,
}

// TODO(stefano): add debug_assert/panic/checked/unchecked setters for `start` and `end`
impl Span {
    /// # Safety
    ///
    /// `start` must be less than or equal to `end`
    #[must_use]
    #[inline(always)]
    #[track_caller]
    pub const unsafe fn new_unchecked(start: offset32, end: offset32) -> Self {
        debug_assert!(start <= end, "`start` must be less than or equal to `end`");
        return Self { start, end };
    }

    #[must_use]
    #[inline]
    pub const fn new(start: offset32, end: offset32) -> Option<Self> {
        if start > end {
            return None;
        }
        return Some(Self { start, end });
    }

    #[must_use]
    #[inline(always)]
    pub const fn start(self) -> offset32 {
        return self.start;
    }

    #[must_use]
    #[inline(always)]
    pub const fn end(self) -> offset32 {
        return self.end;
    }
}

pub type Line = Span;
