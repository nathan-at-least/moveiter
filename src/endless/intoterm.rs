use crate::{EndlessIterator, TerminalIterator};

/// A wrapper for any [`EndlessIterator`] type which provides [`TerminalIterator`].
#[derive(Debug)]
pub struct EndlessTerminalIter<EI>(EI)
where
    EI: EndlessIterator;

impl<EI> From<EI> for EndlessTerminalIter<EI>
where
    EI: EndlessIterator,
{
    fn from(endless: EI) -> Self {
        EndlessTerminalIter(endless)
    }
}

impl<EI> TerminalIterator for EndlessTerminalIter<EI>
where
    EI: EndlessIterator,
{
    type Item = <EI as EndlessIterator>::Item;

    fn into_next_option(self) -> Option<(Self, Self::Item)> {
        let (newstate, item) = self.0.into_next();
        Some((EndlessTerminalIter::from(newstate), item))
    }
}
