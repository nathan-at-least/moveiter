mod intoterm;

#[cfg(test)]
mod tests;

pub use self::intoterm::EndlessTerminalIter;
use crate::{residual, terminal};

/// Types which produce an arbitrary number of `Item`s and never terminates.
pub trait Iterator: Sized {
    /// The type of elements produced.
    type Item;

    /// The `into_next` method produces a new state of type `Self` and an `Item`.
    fn into_next(self) -> (Self, Self::Item);
}

/// Types which convert into an [`Iterator`].
pub trait IntoIterator {
    type Item;
    type IntoEndless: Iterator<Item = Self::Item>;

    fn into_endless_iter(self) -> Self::IntoEndless;
}

/// Any `residual::Iterator` type with a `Residual` type of `std::convert::Infallible` can never
/// terminate as guaranteed by the type system, so it is automatically an `Iterator`.
impl<T> Iterator for T
where
    T: residual::Iterator<Residual = std::convert::Infallible>,
{
    type Item = <Self as residual::Iterator>::Item;

    fn into_next(self) -> (Self, Self::Item) {
        self.into_next_result()
            .expect("residual::Iterator cannot produce Infallible Residual.")
    }
}

impl<T> terminal::IntoIterator for T
where
    T: Iterator,
{
    type Item = <T as Iterator>::Item;
    type IntoTerminalIter = EndlessTerminalIter<Self>;

    fn into_term_iter(self) -> EndlessTerminalIter<Self> {
        EndlessTerminalIter::from(self)
    }
}
