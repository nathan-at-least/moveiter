use crate::optutil::OptionUpdate;
use crate::FiniteMoveIterator;

/// An [Iterator] impl from [FiniteMoveIterator::into_iter].
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
