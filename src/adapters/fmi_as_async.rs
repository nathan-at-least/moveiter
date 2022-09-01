use crate::{AsyncFiniteMoveIterator, FiniteMoveIterator};
use async_trait::async_trait;

/// An [AsyncFiniteMoveIterator] impl from [FiniteMoveIterator::into_async].
pub struct FmiAsAsync<I>(pub(crate) I);

#[async_trait]
impl<I> AsyncFiniteMoveIterator for FmiAsAsync<I>
where
    I: FiniteMoveIterator + Sync + Send,
{
    type Item = <I as FiniteMoveIterator>::Item;

    async fn into_next(mut self) -> Option<(Self, Self::Item)> {
        FiniteMoveIterator::into_next(self.0).map(|(inner, x)| (FmiAsAsync(inner), x))
    }
}
