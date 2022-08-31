use crate::{AsyncFiniteMoveIterator, AsyncTerminalMoveIterator};
use async_trait::async_trait;
use either::Either;

pub struct ATMIAdapter<I>(pub(super) I);

#[async_trait]
impl<I> AsyncTerminalMoveIterator for ATMIAdapter<I>
where
    I: AsyncFiniteMoveIterator,
{
    type Terminal = ();
    type Item = <I as AsyncFiniteMoveIterator>::Item;

    async fn into_next(mut self) -> Either<(Self, Self::Item), Self::Terminal> {
        use Either::{Left, Right};

        AsyncFiniteMoveIterator::into_next(self.0)
            .await
            .map(|(inner, x)| Left((ATMIAdapter(inner), x)))
            .unwrap_or(Right(()))
    }
}
