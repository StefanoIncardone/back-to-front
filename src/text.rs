use crate::{ascii, src_code::Span};

#[must_use] #[inline(always)]
pub const fn byte(text: &str, index: usize) -> Option<ascii> {
    if index >= text.len() { return None; }
    let byte = text.as_bytes()[index];
    return Some(byte);
}

/// # Panics
///
/// Panics if:
/// - `start_index` is greater than `this.len()`
/// - `start_index` is greater than `end_index`
// IDEA(stefano): return a Result instead
#[must_use] #[inline] #[track_caller]
pub const fn slice(this: &str, start_index: usize, end_index: usize) -> &str {
    assert!(start_index <= this.len(), "start index overflow");
    assert!(start_index <= end_index, "start index cannot be greater than end index");

    let len = end_index - start_index;
    let start_ptr = unsafe { this.as_ptr().add(start_index) };
    let text_bytes = unsafe { core::slice::from_raw_parts(start_ptr, len) };
    let text_slice = unsafe { core::str::from_utf8_unchecked(text_bytes) };
    return text_slice;
}

#[must_use] #[inline(always)]
pub const fn span(this: &str, span: Span) -> &str {
    // NOTE(stefano): the check for 'start_index <= end_index` is never going to fail
    return slice(this, span.start as usize, span.end as usize);
}
