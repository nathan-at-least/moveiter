mod adapter;

#[cfg(test)]
mod tests;

pub use self::adapter::StdIteratorAdapter;

/// Types which provide iteration over `Item`s with termination enforced by the type system.
///
/// The `MoveIterator` trait is near-isomorphic to `std::iter::Iterator`, and in fact, any type
/// `T: Iterator` is also an instance of `MoveIterator`.
pub trait MoveIterator: Sized {
    /// The type of elements produced by the `MoveIterator`.
    type Item;

    /// The iteration method consumes `self` by move and produces either `None` or else a new state
    /// and the next `Item` element.
    fn into_next_option(self) -> Option<(Self, Self::Item)>;

    /// Any `MoveIter` can be converted into a wrapper type `StdIteratorAdapter` which is an
    /// `Iterator`, which is useful for integrating to existing `Iterator`-based APIs.
    fn into_iter(self) -> StdIteratorAdapter<Self> {
        StdIteratorAdapter::from(self)
    }
}

/// Types which convert into a [`MoveIterator`].
pub trait IntoMoveIterator {
    type Item;
    type IntoMoveIter: MoveIterator<Item = Self::Item>;

    fn into_move_iter(self) -> Self::IntoMoveIter;
}

/// Any `std::iter::Iterator` type is automatically a `MoveIterator` because `into_next_option` can
/// internally mutate the iterator with `next` then return it as the next state.
impl<I> MoveIterator for I
where
    I: Iterator,
{
    type Item = <I as Iterator>::Item;

    fn into_next_option(mut self) -> Option<(Self, Self::Item)> {
        self.next().map(|item| (self, item))
    }
}
