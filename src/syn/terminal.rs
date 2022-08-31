//! The [TerminalMoveIterator] trait.

use either::Either;

/// Produce a sequence of 0 or more `Item` values asynchronously, then produce a `Terminal` value, using move semantics.
pub trait TerminalMoveIterator: Sized {
    type Item;
    type Terminal;

    /// Iteration is async, moves `self`, and produces either a `(Self, Self::Item)` pair, or the
    /// `Self::Terminal` value.
    fn into_next(self) -> Either<(Self, Self::Item), Self::Terminal>;
}

impl<I> TerminalMoveIterator for I
where
    I: Iterator + Sized + Send,
{
    type Item = I::Item;
    type Terminal = ();

    fn into_next(mut self) -> Either<(Self, Self::Item), Self::Terminal> {
        use Either::*;

        self.next()
            .map(|item| Left((self, item)))
            .unwrap_or(Right(()))
    }
}
