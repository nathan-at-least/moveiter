use crate::{AsyncEndlessMoveIterator, AsyncTerminalMoveIterator};
use async_trait::async_trait;
use either::Either;

/// An adapter providing an [AsyncTerminalMoveIterator] impl for an [AsyncEndlessMoveIterator] value.
pub struct AemiAsTerminal<I>(pub(super) I);

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
