mod adapter;

pub use self::adapter::MoveIteratorAdapter;
use crate::TerminalIterator;

pub trait EndlessIterator: Sized {
    type Item;

    fn into_next(self) -> (Self, Self::Item);

    fn into_moveiter(self) -> MoveIteratorAdapter<Self> {
        MoveIteratorAdapter::from(self)
    }
}

impl<T> EndlessIterator for T
where
    T: TerminalIterator<Terminal = std::convert::Infallible>,
{
    type Item = <Self as TerminalIterator>::Item;

    fn into_next(self) -> (Self, Self::Item) {
        self.into_next_result()
            .expect("TerminalIterator cannot produce Infallible Terminal.")
    }
}
