use crate::TerminalIterator;

pub trait EndlessIterator: Sized {
    type Item;

    fn into_next(self) -> (Self, Self::Item);
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
