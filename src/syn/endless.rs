//! The [EndlessMoveIterator] trait.

// TODO: use crate::adapters::{AmiAsFinite, AmiAsTerminal};

/// Produce an endless sequence of `Item` values synchronously, using move semantics.
pub trait EndlessMoveIterator: Sized {
    type Item;

    /// Iteration moves `self`, and produces an `Option<(Self, Self::Item)>`.
    fn into_next(self) -> (Self, Self::Item);
}
