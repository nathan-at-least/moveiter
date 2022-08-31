use crate::optutil::OptionUpdate;
use crate::FiniteMoveIterator;

/// An adapter providing an [Iterator] impl for an [FiniteMoveIterator] value.
pub struct FmiAsIterator<I>(Option<I>);

impl<I> FmiAsIterator<I> {
    pub(crate) fn new(x: I) -> Self {
        FmiAsIterator(Some(x))
    }
}

impl<I> Iterator for FmiAsIterator<I>
where
    I: FiniteMoveIterator,
{
    type Item = <I as FiniteMoveIterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.update_in_place(|previnner| previnner.into_next())
    }
}
