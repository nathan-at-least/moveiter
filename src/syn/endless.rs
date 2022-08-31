//! The [EndlessMoveIterator] trait.

use crate::adapters::{EmiAsAsync, EmiAsFinite, EmiAsTerminal};

/// Produce an endless sequence of `Item` values synchronously, using move semantics.
pub trait EndlessMoveIterator: Sized {
    type Item;

    /// Iteration moves `self`, and produces an `Option<(Self, Self::Item)>`.
    fn into_next(self) -> (Self, Self::Item);

    /// Convert into the `async` equivalent.
    fn into_async(self) -> EmiAsAsync<Self> {
        EmiAsAsync(self)
    }

    /// Convert into a value that impls [FiniteMoveIterator](crate::FiniteMoveIterator).
    fn into_finite_move_iterator(self) -> EmiAsFinite<Self> {
        EmiAsFinite(self)
    }

    /// Convert into a value that impls [TerminalMoveIterator](crate::TerminalMoveIterator).
    fn into_terminal_move_iterator(self) -> EmiAsTerminal<Self> {
        EmiAsTerminal(self)
    }
}
