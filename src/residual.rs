//! An Iterator which produces a value with termination.
use crate::terminal;

mod iteration;

#[cfg(test)]
mod tests;

pub use self::iteration::{Iteration, Next, Residual};

/// Types which produces a sequence of `Item`s and then finally a `Residual` type.
///
/// This is a generalization of `terminal::Iterator` (and `std::iter::Iterator`) which enables more
/// expressive termination. For example, a type which performs input with `std::io` can produce
/// simple `Item` results and terminate with a `std::io::Result<()>` which ensures that any IO
/// errors terminate iteration.
///
/// Any type which is `terminal::Iterator` is also an instance of `Iterator` with `Residual =
/// ()`.
pub trait Iterator: Sized {
    /// The type of the elements produced by the iterator:
    type Item;

    /// A `Residual` value is produced when iteration terminates:
    type Residual;

    /// The iteration method produces either a next state and item, or a `Residual` value. Note
    /// that although this is a `Result`, the `Residual` value may not represent an error, per-se.
    fn into_next(self) -> Iteration<Self, Self::Item, Self::Residual>;
}

/// Types which convert into a [`Iterator`].
pub trait IntoIterator {
    type Item;
    type Residual;
    type IntoResidual: Iterator<Item = Self::Item, Residual = Self::Residual>;

    fn into_res_iter(self) -> Self::IntoResidual;
}

/// Any `terminal::Iterator` type is also a `Iterator` with `()` as the `Residual` type. This is
/// analogous to the isomorphism of `Option<T>` with `Result<T, ()>`.
impl<T> Iterator for T
where
    T: terminal::Iterator,
{
    type Item = <Self as terminal::Iterator>::Item;
    type Residual = ();

    fn into_next(self) -> Iteration<Self, Self::Item, ()> {
        match self.into_next() {
            terminal::Terminal => Residual(()),
            terminal::Next(s, x) => Next(s, x),
        }
    }
}
