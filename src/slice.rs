#[must_use] #[inline] #[track_caller]
pub const fn get<T>(slice: &[T], index: usize) -> Option<&T> {
    if index >= slice.len() { return None; }
    let item = unsafe { at_unchecked(slice, index) };
    return Some(item);
}

/// # Panics
/// if `index` would overflow the `slice`
#[must_use] #[inline] #[track_caller] #[expect(clippy::panic)]
pub fn at<T>(slice: &[T], index: usize) -> &T {
    if index >= slice.len() { panic!("index {index} out of bounds for an array of length {}", slice.len()); }
    let item = unsafe { at_unchecked(slice, index) };
    return item;
}

/// # Safety
/// Calling this method when `index` overflows `slice` is *[undefined behavior]*.
#[must_use] #[inline] #[track_caller]
pub const unsafe fn at_unchecked<T>(slice: &[T], index: usize) -> &T {
    let slice_ptr = unsafe { slice.as_ptr().add(index) };
    let item = unsafe { slice_ptr.as_ref_unchecked() };
    return item;
}
