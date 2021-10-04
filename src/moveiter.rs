mod adapter;

pub use self::adapter::StdIteratorAdapter;
use crate::EndlessIterator;

pub trait MoveIterator: Sized {
    type Item;

    fn into_next_option(self) -> Option<(Self, Self::Item)>;

    fn into_iter(self) -> StdIteratorAdapter<Self> {
        StdIteratorAdapter::from(self)
    }
}

impl<T> MoveIterator for T
where
    T: EndlessIterator,
{
    type Item = <Self as EndlessIterator>::Item;

    fn into_next_option(self) -> Option<(Self, Self::Item)> {
        Some(self.into_next())
    }
}
