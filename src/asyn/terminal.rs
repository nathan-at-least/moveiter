//! The [AsyncTerminalMoveIterator] trait.
use async_trait::async_trait;
use either::Either;

/// An `AsyncTerminalMoveIterator` type produces a sequence of 0 or more `Item` values asynchronously, then produces a `Terminal` value, using move semantics.
///
/// # `Iterator` Call-site Example
///
/// Any [Iterator] + [Sized] + [Send] is an [AsyncTerminalMoveIterator], such as the
/// [std::ops::Range] in this example:
///
/// ```
/// # tokio::runtime::Builder::new_current_thread().build().unwrap().block_on(async {
/// # use moveiter::AsyncTerminalMoveIterator;
/// let it = 0..2;
///
/// let (it2, a) = it.atmi_next().await.left().unwrap();
/// assert_eq!(a, 0);
///
/// let (it3, b) = it2.atmi_next().await.left().unwrap();
/// assert_eq!(b, 1);
///
/// let term = it3.atmi_next().await.right().unwrap();
/// assert_eq!((), term);
/// # });
/// ```
///
/// # Local `mut` call site:
///
/// To express a loop a common pattern is to use `mut` assignment to the iterator values:
///
/// ```
/// # use moveiter::AsyncTerminalMoveIterator;
/// async fn process_items<I, R>(mut it: I) -> R
///   where I: AsyncTerminalMoveIterator<Terminal = R>,
/// {
///     use either::Either::{Left, Right};
///
///     loop {
///         match it.atmi_next().await {
///             Left((nextit, x)) => {
///                 it = nextit;
///                 // Process `x`...
///             }
///             Right(term) => {
///                 return term;
///             }
///         }
///     }
/// }
/// ```
///
/// # Example: Move Semantics
///
/// Suppose we made a mistake in writing the previous example:
///
/// ```compile_fail
/// # use moveiter::AsyncTerminalMoveIterator;
/// async fn process_items<I, R>(mut it: I) -> R
///   where I: AsyncTerminalMoveIterator<Terminal = R>,
/// {
///     use either::Either::{Left, Right};
///
///     loop {
///         match it.atmi_next().await {
///             Left((_, x)) => {
///                 // Process `x`...
///             }
///             Right(term) => {
///                 return term;
///             }
///         }
///     }
/// }
/// ```
///
/// Because we attempt to re-use the moved iterator `it`, this is a compile-time error:
///
/// ```text
/// error[E0382]: use of moved value: `it`
///    --> src/asyn/terminal.rs:67:15
///     |
/// 5   | async fn process_items<I, R>(mut it: I) -> R
///     |                              ------ move occurs because `it` has type `I`, which does not implement the `Copy` trait
/// ...
/// 11  |         match it.atmi_next().await {
///     |               ^^ ----------- `it` moved due to this method call, in previous iteration of loop
///     |
/// note: this function takes ownership of the receiver `self`, which moves `it`
///    --> /home/user/hack/moveiter/src/asyn/terminal.rs:142:24
///     |
/// 142 |     async fn atmi_next(self) -> Either<(Self, Self::Item), Self::Terminal>;
///     |                        ^^^^
/// help: consider further restricting this bound
///     |
/// 6   |   where I: AsyncTerminalMoveIterator<Terminal = R> + Copy,
///     |                                                    ++++++
/// ```
///
/// # `Result` Terminal Call-site Example
///
/// A common pattern is for `Terminal` to be a `Result`:
///
/// ```
/// # use moveiter::AsyncTerminalMoveIterator;
/// # type Error = ();
/// async fn read_inputs_and_sum<I>(mut it: I) -> Result<i32, Error>
///   where I: AsyncTerminalMoveIterator<Item = i32, Terminal = Result<(), Error>>,
/// {
///     use either::Either::{Left, Right};
///
///     let mut sum = 0;
///
///     loop {
///         match it.atmi_next().await {
///             Left((nextit, x)) => {
///                 it = nextit;
///                 sum += x;
///             }
///             Right(term) => {
///                 // The `Result::map` returns our sum if the iterator terminated successfully.
///                 // Otherwise it propagates the error.
///                 return term.map(|()| sum);
///             }
///         }
///     }
/// }
/// ```
#[async_trait]
pub trait AsyncTerminalMoveIterator: Sized + Send {
    type Item;
    type Terminal;

    /// Iteration is async, moves `self`, and produces either a `(Self, Self::Item)` pair, or the
    /// `Self::Terminal` value.
    ///
    /// The simplified interface via [async_trait](::async_trait) looks like:
    ///
    /// ```no_run
    /// # use either::Either;
    /// # #[async_trait::async_trait]
    /// # trait T: Sized {
    /// # type Item;
    /// # type Terminal;
    /// async fn atmi_next(self) -> Either<(Self, Self::Item), Self::Terminal>;
    /// # }
    /// ```
    async fn atmi_next(self) -> Either<(Self, Self::Item), Self::Terminal>;
}

#[async_trait]
impl<I> AsyncTerminalMoveIterator for I
where
    I: Iterator + Sized + Send,
{
    type Item = I::Item;
    type Terminal = ();

    async fn atmi_next(mut self) -> Either<(Self, Self::Item), Self::Terminal> {
        use Either::*;

        if let Some(item) = self.next() {
            Left((self, item))
        } else {
            Right(())
        }
    }
}
