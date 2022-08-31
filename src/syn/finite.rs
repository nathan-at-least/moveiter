//! The [FiniteMoveIterator] trait.

use crate::adapters::FmiAsAsync;

/// Produce a sequence of 0 or more `Item` values asynchronously, using move semantics.
///
pub trait FiniteMoveIterator: Sized {
    type Item;

    /// Iteration moves `self`, and produces an `Option<(Self, Self::Item)>`.
    fn into_next(self) -> Option<(Self, Self::Item)>;

    /// Convert into the `async` equivalent.
    fn into_async(self) -> FmiAsAsync<Self> {
        FmiAsAsync(self)
    }
}

impl<I> FiniteMoveIterator for I
where
    I: Iterator + Sized,
{
    type Item = I::Item;

    fn into_next(mut self) -> Option<(Self, Self::Item)> {
        self.next().map(|item| (self, item))
    }
}
