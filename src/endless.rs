mod adapter;

pub use self::adapter::MoveIteratorAdapter;
use crate::TerminalIterator;

/// An `EndlessIterator` produces an arbitrary number of `Item`s and never terminates.
///
/// Any `EndlessIterator` type can be converted to a `MoveIterator` wrapper type with
/// [`EndlessIterator::into_moveiter`].
pub trait EndlessIterator: Sized {
    /// The type of elements produced.
    type Item;

    /// The `into_next` method produces a new state of type `Self` and an `Item`.
    fn into_next(self) -> (Self, Self::Item);

    /// Any `EndlessIterator` type can be converted to a `MoveIterator` with this method.
    fn into_moveiter(self) -> MoveIteratorAdapter<Self> {
        MoveIteratorAdapter::from(self)
    }
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
