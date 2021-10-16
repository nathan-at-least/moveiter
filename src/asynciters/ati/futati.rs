use crate::{AsyncTerminalIterator, IntoAsyncTerminalIterator};
use std::future::Future;
use std::marker::Unpin;
use std::pin::Pin;
use std::task::{Context, Poll};

impl<F> IntoAsyncTerminalIterator for F
where
    F: Future + Unpin,
{
    type Item = <F as Future>::Output;
    type Terminal = ();
    type IntoATI = FutureAsyncTerminalIterator<F>;

    fn into_async_term_iter(self) -> Self::IntoATI {
        FutureAsyncTerminalIterator(Some(self))
    }
}

pub struct FutureAsyncTerminalIterator<F>(Option<F>)
where
    F: Future + Unpin;

impl<F> AsyncTerminalIterator for FutureAsyncTerminalIterator<F>
where
    F: Future + Unpin,
{
    type Item = <F as Future>::Output;
    type Terminal = ();
    type Iteration = FutureIteration<F>;

    fn into_next_result_async(self) -> Self::Iteration {
        FutureIteration(self.0)
    }
}

pub struct FutureIteration<F>(Option<F>)
where
    F: Future + Unpin;

impl<F> Future for FutureIteration<F>
where
    F: Future + Unpin,
{
    type Output = Result<(FutureAsyncTerminalIterator<F>, <F as Future>::Output), ()>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match Pin::into_inner(self).0.take() {
            Some(mut fut) => {
                let futpin = Pin::new(&mut fut);
                match Future::poll(futpin, cx) {
                    Poll::Pending => Poll::Pending,
                    Poll::Ready(x) => Poll::Ready(Ok((FutureAsyncTerminalIterator(None), x))),
                }
            }
            None => Poll::Ready(Err(())),
        }
    }
}
