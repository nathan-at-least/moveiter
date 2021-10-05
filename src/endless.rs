mod intomoveiter;

#[cfg(test)]
mod tests;

pub use self::intomoveiter::IntoMoveIter;
use crate::{IntoMoveIterator, TerminalIterator};

/// Types which produce an arbitrary number of `Item`s and never terminates.
pub trait EndlessIterator: Sized {
    /// The type of elements produced.
    type Item;

    /// The `into_next` method produces a new state of type `Self` and an `Item`.
    fn into_next(self) -> (Self, Self::Item);
}

/// Types which convert into an [`EndlessIterator`].
pub trait IntoEndlessIterator {
    type Item;
    type IntoEndless: EndlessIterator<Item = Self::Item>;

    fn into_endless_iter(self) -> Self::IntoEndless;
}

/// Any `TerminalIterator` type with a `Terminal` type of `std::convert::Infallible` can never
/// terminate as guaranteed by the type system, so it is automatically an `EndlessIterator`.
impl<T> EndlessIterator for T
where
    T: TerminalIterator<Terminal = std::convert::Infallible>,
{
    type Item = <Self as TerminalIterator>::Item;

    fn into_next(self) -> (Self, Self::Item) {
        self.into_next_result()
            .expect("TerminalIterator cannot produce Infallible Terminal.")
    }
}

impl<T> IntoMoveIterator for T
where
    T: EndlessIterator,
{
    type Item = <T as EndlessIterator>::Item;
    type IntoMoveIter = IntoMoveIter<Self>;

    fn into_move_iter(self) -> IntoMoveIter<Self> {
        IntoMoveIter::from(self)
    }
}
