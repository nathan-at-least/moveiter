use crate::EndlessIterator;

pub trait MoveIterator: Sized {
    type Item;

    fn into_next_option(self) -> Option<(Self, Self::Item)>;
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
