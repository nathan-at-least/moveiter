mod blankets;
mod futati;

// For blanket impl of AsyncTerminalIterator for every TerminalIterator:
pub use self::blankets::TerminalIteration;

// For blanket impl of IntoAsyncTerminalIterator for every Future:
pub use self::futati::{FutureAsyncTerminalIterator, FutureIteration};

use std::future::Future;

pub trait AsyncTerminalIterator: Sized {
    type Item;
    type Terminal;
    type Iteration: Future<Output = Result<(Self, Self::Item), Self::Terminal>>;

    fn into_next_result_async(self) -> Self::Iteration;
}

/// Types which convert into a [`TerminalIterator`].
pub trait IntoAsyncTerminalIterator {
    type Item;
    type Terminal;
    type IntoATI: AsyncTerminalIterator<Item = Self::Item, Terminal = Self::Terminal>;

    fn into_async_term_iter(self) -> Self::IntoATI;
}
