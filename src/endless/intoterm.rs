use crate::{endless, TerminalIterator};

/// A wrapper for any [`endless::Iterator`] type which provides [`TerminalIterator`].
#[derive(Debug)]
pub struct EndlessTerminalIter<EI>(EI)
where
    EI: endless::Iterator;

impl<EI> From<EI> for EndlessTerminalIter<EI>
where
    EI: endless::Iterator,
{
    fn from(endless: EI) -> Self {
        EndlessTerminalIter(endless)
    }
}

impl<EI> TerminalIterator for EndlessTerminalIter<EI>
where
    EI: endless::Iterator,
{
    type Item = <EI as endless::Iterator>::Item;

    fn into_next_option(self) -> Option<(Self, Self::Item)> {
        let (newstate, item) = self.0.into_next();
        Some((EndlessTerminalIter::from(newstate), item))
    }
}
