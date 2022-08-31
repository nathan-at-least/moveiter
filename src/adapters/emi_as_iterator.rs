use crate::optutil::OptionUpdate;
use crate::EndlessMoveIterator;

/// An adapter providing an [Iterator] impl for an [EndlessMoveIterator] value.
pub struct EmiAsIterator<I>(Option<I>);

impl<I> EmiAsIterator<I> {
    pub(crate) fn new(x: I) -> Self {
        EmiAsIterator(Some(x))
    }
}

impl<I> Iterator for EmiAsIterator<I>
where
    I: EndlessMoveIterator,
{
    type Item = <I as EndlessMoveIterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.update_in_place(|inner| Some(inner.into_next()))
    }
}
