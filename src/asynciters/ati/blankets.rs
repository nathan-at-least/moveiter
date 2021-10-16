use crate::{AsyncTerminalIterator, TerminalIterator};
use std::future::Future;
use std::marker::Unpin;
use std::pin::Pin;
use std::task::{Context, Poll};

impl<T> AsyncTerminalIterator for T
where
    T: TerminalIterator + Unpin,
{
    type Item = <T as TerminalIterator>::Item;
    type Terminal = <T as TerminalIterator>::Terminal;
    type Iteration = TerminalIteration<T>;

    fn into_next_result_async(self) -> Self::Iteration {
        TerminalIteration(Some(self))
    }
}

pub struct TerminalIteration<T>(Option<T>)
where
    T: TerminalIterator + Unpin;

impl<T> Future for TerminalIteration<T>
where
    T: TerminalIterator + Unpin,
{
    type Output = Result<(T, <T as TerminalIterator>::Item), <T as TerminalIterator>::Terminal>;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mutself = Pin::into_inner(self);
        let state = mutself.0.take().unwrap();
        Poll::Ready(state.into_next_result())
    }
}
