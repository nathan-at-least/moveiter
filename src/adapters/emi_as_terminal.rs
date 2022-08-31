use crate::{EndlessMoveIterator, TerminalMoveIterator};
use either::Either;

/// An adapter providing an [TerminalMoveIterator] impl for an [EndlessMoveIterator] value.
pub struct EmiAsTerminal<I>(pub(crate) I);

impl<I> TerminalMoveIterator for EmiAsTerminal<I>
where
    I: EndlessMoveIterator,
{
    type Terminal = std::convert::Infallible;
    type Item = <I as EndlessMoveIterator>::Item;

    fn into_next(self) -> Either<(Self, Self::Item), Self::Terminal> {
        let (inner, x) = EndlessMoveIterator::into_next(self.0);
        Either::Left((EmiAsTerminal(inner), x))
    }
}
