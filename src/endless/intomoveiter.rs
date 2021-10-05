use crate::{EndlessIterator, MoveIterator};

/// A wrapper for any [`EndlessIterator`] type which provides [`MoveIterator`].
#[derive(Debug)]
pub struct EndlessMoveIter<EI>(EI)
where
    EI: EndlessIterator;

impl<EI> From<EI> for EndlessMoveIter<EI>
where
    EI: EndlessIterator,
{
    fn from(endless: EI) -> Self {
        EndlessMoveIter(endless)
    }
}

impl<EI> MoveIterator for EndlessMoveIter<EI>
where
    EI: EndlessIterator,
{
    type Item = <EI as EndlessIterator>::Item;

    fn into_next_option(self) -> Option<(Self, Self::Item)> {
        let (newstate, item) = self.0.into_next();
        Some((EndlessMoveIter::from(newstate), item))
    }
}
