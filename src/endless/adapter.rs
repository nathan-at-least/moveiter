use crate::{EndlessIterator, MoveIterator};

/// `MoveIteratorAdapter` wraps any `EndlessIterator` type and provides a `MoveIterator` impl.
#[derive(Debug)]
pub struct MoveIteratorAdapter<EI>(EI)
where
    EI: EndlessIterator;

impl<EI> From<EI> for MoveIteratorAdapter<EI>
where
    EI: EndlessIterator,
{
    fn from(endless: EI) -> Self {
        MoveIteratorAdapter(endless)
    }
}

impl<EI> MoveIterator for MoveIteratorAdapter<EI>
where
    EI: EndlessIterator,
{
    type Item = <EI as EndlessIterator>::Item;

    fn into_next_option(self) -> Option<(Self, Self::Item)> {
        let (newstate, item) = self.0.into_next();
        Some((MoveIteratorAdapter::from(newstate), item))
    }
}
