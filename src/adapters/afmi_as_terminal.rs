use crate::{AsyncFiniteMoveIterator, AsyncTerminalMoveIterator};
use async_trait::async_trait;
use either::Either;

/// An [AsyncTerminalMoveIterator] impl from [AsyncFiniteMoveIterator::into_async_terminal_move_iterator].
pub struct AfmiAsTerminal<I>(pub(crate) I);

#[async_trait]
impl<I> AsyncTerminalMoveIterator for AfmiAsTerminal<I>
where
    I: AsyncFiniteMoveIterator,
{
    type Terminal = ();
    type Item = <I as AsyncFiniteMoveIterator>::Item;

    async fn into_next(self) -> Either<(Self, Self::Item), Self::Terminal> {
        use Either::{Left, Right};

        AsyncFiniteMoveIterator::into_next(self.0)
            .await
            .map(|(inner, x)| Left((AfmiAsTerminal(inner), x)))
            .unwrap_or(Right(()))
    }
}
