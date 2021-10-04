mod adapter;

pub use self::adapter::StdIteratorAdapter;

pub trait MoveIterator: Sized {
    type Item;

    fn into_next_option(self) -> Option<(Self, Self::Item)>;

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
