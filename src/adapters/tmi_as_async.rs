use crate::{AsyncTerminalMoveIterator, TerminalMoveIterator};
use async_trait::async_trait;
use either::Either;

/// An adapter providing an [AsyncTerminalMoveIterator] impl for an [AsyncTerminalMoveIterator] value.
pub struct TmiAsAsync<I>(pub(crate) I);

#[async_trait]
impl<I> AsyncTerminalMoveIterator for TmiAsAsync<I>
where
    I: TerminalMoveIterator + Sync + Send,
{
    type Terminal = <I as TerminalMoveIterator>::Terminal;
    type Item = <I as TerminalMoveIterator>::Item;

    async fn into_next(mut self) -> Either<(Self, Self::Item), Self::Terminal> {
        use Either::*;

        match TerminalMoveIterator::into_next(self.0) {
            Left((inner, x)) => Left((TmiAsAsync(inner), x)),
            Right(term) => Right(term),
        }
    }
}
