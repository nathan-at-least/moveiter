//! The [TerminalMoveIterator] trait.

mod resiter;

use crate::adapters::TmiAsAsync;
use either::Either;

pub use self::resiter::terminal_move_iterator_from_result_iterator;

/// Produce a sequence of 0 or more `Item` values asynchronously, then produce a `Terminal` value, using move semantics.
pub trait TerminalMoveIterator: Sized {
    type Item;
    type Terminal;

    /// Iteration is async, moves `self`, and produces either a `(Self, Self::Item)` pair, or the
    /// `Self::Terminal` value.
    fn into_next(self) -> Either<(Self, Self::Item), Self::Terminal>;

    /// Convert into the `async` equivalent.
    fn into_async(self) -> TmiAsAsync<Self> {
        TmiAsAsync(self)
    }

    /// Call a closure on each `Item`, then return the `Terminal`.
    fn for_each<F>(self, mut f: F) -> Self::Terminal
    where
        F: FnMut(Self::Item),
    {
        use Either::*;

        let mut it = self;

        loop {
            match it.into_next() {
                Left((next, x)) => {
                    it = next;
                    f(x);
                }
                Right(term) => {
                    return term;
                }
            }
        }
    }

    /// Folds every `Item` into an accumulator with `fitem`, then integrates the final accumulator
    /// value with `Terminal` in `fterm`.
    fn fold<A, F, T, R>(self, mut acc: A, fitem: F, fterm: T) -> R
    where
        F: Fn(A, Self::Item) -> A,
        T: FnOnce(A, Self::Terminal) -> R,
    {
        use Either::*;

        let mut it = self;

        loop {
            match it.into_next() {
                Left((next, x)) => {
                    it = next;
                    acc = fitem(acc, x);
                }
                Right(term) => {
                    return fterm(acc, term);
                }
            }
        }
    }
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
