use crate::{AsyncEndlessMoveIterator, AsyncTerminalMoveIterator};
use async_trait::async_trait;
use either::Either;

/// An [AsyncTerminalMoveIterator] impl from [AsyncEndlessMoveIterator::into_async_terminal_move_iterator].
pub struct AemiAsTerminal<I>(pub(crate) I);

#[async_trait]
impl<I> AsyncTerminalMoveIterator for AemiAsTerminal<I>
where
    I: AsyncEndlessMoveIterator,
{
    type Terminal = std::convert::Infallible;
    type Item = <I as AsyncEndlessMoveIterator>::Item;

    async fn into_next(mut self) -> Either<(Self, Self::Item), Self::Terminal> {
        let (inner, x) = AsyncEndlessMoveIterator::into_next(self.0).await;
        Either::Left((AemiAsTerminal(inner), x))
    }
}
