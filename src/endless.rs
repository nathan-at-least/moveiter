mod intomoveiter;

#[cfg(test)]
mod tests;

pub use self::intomoveiter::EndlessMoveIter;
use crate::{IntoMoveIterator, ResidualIterator};

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

/// Any `ResidualIterator` type with a `Residual` type of `std::convert::Infallible` can never
/// terminate as guaranteed by the type system, so it is automatically an `EndlessIterator`.
impl<T> EndlessIterator for T
where
    T: ResidualIterator<Residual = std::convert::Infallible>,
{
    type Item = <Self as ResidualIterator>::Item;

    fn into_next(self) -> (Self, Self::Item) {
        self.into_next_result()
            .expect("ResidualIterator cannot produce Infallible Residual.")
    }
}

impl<T> IntoMoveIterator for T
where
    T: EndlessIterator,
{
    type Item = <T as EndlessIterator>::Item;
    type IntoMoveIter = EndlessMoveIter<Self>;

    fn into_move_iter(self) -> EndlessMoveIter<Self> {
        EndlessMoveIter::from(self)
    }
}
