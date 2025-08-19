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

#[expect(unused_macro_rules, clippy::should_panic_without_expect, unused_imports)]
#[cfg(test)]
#[rustfmt::skip]
mod tests {
    use super::*;

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

    mod _0_1_0_functionality {
        use super::*;

        test_assert!(unsafe { Span::new_unchecked(0, 0) }, == Span { start: 0, end: 0 });

        #[should_panic]
        #[test]
        const fn span_new_unchecked_should_panic() {
            let _ = unsafe { Span::new_unchecked(1, 0) };
        }

        test_assert!(Span::new(0, 0), == Some(Span { start: 0, end: 0 }));
        test_assert!(Span::new(1, 0), == None);
    }

    mod _0_1_0_backwards_compatibility {
        use crate::offset32;
        use super::{
            Span, Line,
        };

        const _: unsafe fn(offset32, offset32) -> Span = Span::new_unchecked;
        const _: fn(offset32, offset32) -> Option<Span> = Span::new;
        const _: fn(Span) -> offset32 = Span::start;
        const _: fn(Span) -> offset32 = Span::end;
    }
}
