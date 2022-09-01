use crate::{AsyncEndlessMoveIterator, EndlessMoveIterator};
use async_trait::async_trait;

/// An [AsyncEndlessMoveIterator] impl from [EndlessMoveIterator::into_async].
pub struct EmiAsAsync<I>(pub(crate) I);

#[async_trait]
impl<I> AsyncEndlessMoveIterator for EmiAsAsync<I>
where
    I: EndlessMoveIterator + Sync + Send,
{
    type Item = <I as EndlessMoveIterator>::Item;

    async fn into_next(mut self) -> (Self, Self::Item) {
        let (inner, x) = EndlessMoveIterator::into_next(self.0);
        (EmiAsAsync(inner), x)
    }
}
