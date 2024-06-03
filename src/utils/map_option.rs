//! The [`map_option`] function.

/// A simple function to use instead of [`Option<_>::map`] which allows using `?` inside
/// of it.
#[inline]
pub fn map_option<I, O>(option: I, map_fn: impl Fn(I) -> Option<O>) -> Option<O> {
    map_fn(option)
}
