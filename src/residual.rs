use crate::MoveIterator;

#[cfg(test)]
mod tests;

/// Types which produces a sequence of `Item`s and then finally a `Residual` type.
///
/// This is a generalization of `MoveIterator` (and `std::iter::Iterator`) which enables more
/// expressive termination. For example, a type which performs input with `std::io` can produce
/// simple `Item` results and terminate with a `std::io::Result<()>` which ensures that any IO
/// errors terminate iteration.
///
/// Any type which is `MoveIterator` is also an instance of `ResidualIterator` with `Residual =
/// ()`.
pub trait ResidualIterator: Sized {
    /// The type of the elements produced by the iterator:
    type Item;

    /// A `Residual` value is produced when iteration terminates:
    type Residual;

    /// The iteration method produces either a next state and item, or a `Residual` value. Note
    /// that although this is a `Result`, the `Residual` value may not represent an error, per-se.
    fn into_next_result(self) -> Result<(Self, Self::Item), Self::Residual>;
}

/// Types which convert into a [`ResidualIterator`].
pub trait IntoResidualIterator {
    type Item;
    type Residual;
    type IntoResidual: ResidualIterator<Item = Self::Item, Residual = Self::Residual>;

    fn into_res_iter(self) -> Self::IntoResidual;
}

/// Any `MoveIterator` type is also a `ResidualIterator` with `()` as the `Residual` type. This is
/// analogous to the isomorphism of `Option<T>` with `Result<T, ()>`.
impl<T> ResidualIterator for T
where
    T: MoveIterator,
{
    type Item = <Self as MoveIterator>::Item;
    type Residual = ();

    fn into_next_result(self) -> Result<(Self, Self::Item), ()> {
        self.into_next_option().ok_or(())
    }
}
