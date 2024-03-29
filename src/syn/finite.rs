//! The [FiniteMoveIterator] trait.

use crate::adapters::{FmiAsAsync, FmiAsIterator, FmiAsTerminal};

/// Produce a sequence of 0 or more `Item` values asynchronously, using move semantics.
///
pub trait FiniteMoveIterator: Sized {
    type Item;

    /// Iteration moves `self`, and produces an `Option<(Self, Self::Item)>`.
    fn into_next(self) -> Option<(Self, Self::Item)>;

    /// Convert into a [std::iter::Iterator] value.
    fn into_iter(self) -> FmiAsIterator<Self> {
        FmiAsIterator::new(self)
    }

    /// Convert into the `async` equivalent.
    fn into_async(self) -> FmiAsAsync<Self> {
        FmiAsAsync(self)
    }

    /// Convert into a value that impls [TerminalMoveIterator](crate::TerminalMoveIterator).
    fn into_terminal_move_iterator(self) -> FmiAsTerminal<Self> {
        FmiAsTerminal(self)
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
