use crate::{FiniteMoveIterator, TerminalMoveIterator};
use either::Either;

/// An [TerminalMoveIterator] impl from [FiniteMoveIterator::into_terminal_move_iterator].
pub struct FmiAsTerminal<I>(pub(crate) I);

impl<I> TerminalMoveIterator for FmiAsTerminal<I>
where
    I: FiniteMoveIterator,
{
    type Terminal = ();
    type Item = <I as FiniteMoveIterator>::Item;

    fn into_next(self) -> Either<(Self, Self::Item), Self::Terminal> {
        use Either::{Left, Right};

        FiniteMoveIterator::into_next(self.0)
            .map(|(inner, x)| Left((FmiAsTerminal(inner), x)))
            .unwrap_or(Right(()))
    }
}
