use crate::{AsyncEndlessMoveIterator, AsyncFiniteMoveIterator};
use async_trait::async_trait;

/// An [AsyncFiniteMoveIterator] impl from [AsyncEndlessMoveIterator::into_async_finite_move_iterator].
pub struct AemiAsFinite<I>(pub(crate) I);

#[async_trait]
impl<I> AsyncFiniteMoveIterator for AemiAsFinite<I>
where
    I: AsyncEndlessMoveIterator,
{
    type Item = <I as AsyncEndlessMoveIterator>::Item;

    async fn into_next(self) -> Option<(Self, Self::Item)> {
        let (inner, x) = AsyncEndlessMoveIterator::into_next(self.0).await;
        Some((AemiAsFinite(inner), x))
    }
}
