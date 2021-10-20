//! An Iterator with move semantics that never terminates.
mod intoterm;
mod iteration;

#[cfg(test)]
mod tests;

pub use self::intoterm::EndlessTerminalIter;
pub use self::iteration::Iteration;
use crate::{residual, terminal};

/// Types which produce an arbitrary number of `Item`s and never terminates.
pub trait Iterator: Sized {
    /// The type of elements produced.
    type Item;

    /// The `into_next` method produces a new state of type `Self` and an `Item`.
    fn into_next(self) -> Iteration<Self, Self::Item>;
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

    fn into_next(self) -> Iteration<Self, Self::Item> {
        match <Self as residual::Iterator>::into_next(self) {
            residual::Next(s, x) => Iteration(s, x),
            _ => unreachable!("Residual is Infallable"),
        }
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
