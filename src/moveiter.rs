mod adapter;

pub use self::adapter::StdIteratorAdapter;

/// The `MoveIterator` trait provides iteration over `Item`s with termination enforced by the type
/// system.
///
/// The `MoveIterator` trait is near-isomorphic to `std::iter::Iterator`, and in fact, any type
/// `T: Iterator` is also an instance of `MoveIterator`.
///
/// While the symmetric correspondence, that any `MoveIterator` should be an `Iterator`, is
/// logically true but not expressable due to the orphan rule. Instead an adapter wrapper type is
/// provided to that any `T: MoveIterator` can be converted to this wrapper type which is
/// `Iterator`.
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

impl<I> MoveIterator for I
where
    I: Iterator,
{
    type Item = <I as Iterator>::Item;

    fn into_next_option(mut self) -> Option<(Self, Self::Item)> {
        self.next().map(|item| (self, item))
    }
}
