use crate::{EndlessIterator, MoveIterator};

/// `IntoMoveIter` wraps any `EndlessIterator` type and provides a `MoveIterator` impl.
#[derive(Debug)]
pub struct IntoMoveIter<EI>(EI)
where
    EI: EndlessIterator;

impl<EI> From<EI> for IntoMoveIter<EI>
where
    EI: EndlessIterator,
{
    fn from(endless: EI) -> Self {
        IntoMoveIter(endless)
    }
}

impl<EI> MoveIterator for IntoMoveIter<EI>
where
    EI: EndlessIterator,
{
    type Item = <EI as EndlessIterator>::Item;

    fn into_next_option(self) -> Option<(Self, Self::Item)> {
        let (newstate, item) = self.0.into_next();
        Some((IntoMoveIter::from(newstate), item))
    }
}
