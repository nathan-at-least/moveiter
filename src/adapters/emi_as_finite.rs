use crate::{EndlessMoveIterator, FiniteMoveIterator};

/// An adapter providing an [FiniteMoveIterator] impl for an [EndlessMoveIterator] value.
pub struct EmiAsFinite<I>(pub(crate) I);

impl<I> FiniteMoveIterator for EmiAsFinite<I>
where
    I: EndlessMoveIterator,
{
    type Item = <I as EndlessMoveIterator>::Item;

    fn into_next(self) -> Option<(Self, Self::Item)> {
        let (inner, x) = EndlessMoveIterator::into_next(self.0);
        Some((EmiAsFinite(inner), x))
    }
}
