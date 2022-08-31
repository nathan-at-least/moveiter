//! The [EndlessMoveIterator] trait.

use crate::adapters::EmiAsAsync;

/// Produce an endless sequence of `Item` values synchronously, using move semantics.
pub trait EndlessMoveIterator: Sized {
    type Item;

    /// Iteration moves `self`, and produces an `Option<(Self, Self::Item)>`.
    fn into_next(self) -> (Self, Self::Item);

    /// Convert into the `async` equivalent.
    fn into_async(self) -> EmiAsAsync<Self> {
        EmiAsAsync(self)
    }
}
