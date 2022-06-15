//! The [AsyncTerminalMoveIterator] trait.
use async_trait::async_trait;
use either::Either;

/// An `AsyncTerminalMoveIterator` type produces a sequence of 0 or more `Item` values asynchronously, then produces a `Terminal` value, using move semantics.
#[async_trait]
pub trait AsyncTerminalMoveIterator: Sized + Send {
    type Item;
    type Terminal;

    /// Iteration is async, moves `self`, and produces either a `(Self, Self::Item)` pair, or the
    /// `Self::Terminal` value.
    ///
    /// The simplified interface via [async_trait](::async_trait) looks like:
    ///
    /// ```ignore
    /// async fn next(self) -> Either<(Self, Self::Item), Self::Terminal>;
    /// ```
    async fn next(self) -> Either<(Self, Self::Item), Self::Terminal>;
}

/// Any [Iterator] + [Sized] + [Send] is an [AsyncTerminalMoveIterator].
#[async_trait]
impl<I> AsyncTerminalMoveIterator for I
where
    I: Iterator + Sized + Send,
{
    type Item = I::Item;
    type Terminal = ();

    async fn next(mut self) -> Either<(Self, Self::Item), Self::Terminal> {
        use Either::*;

        if let Some(item) = Iterator::next(&mut self) {
            Left((self, item))
        } else {
            Right(())
        }
    }
}
