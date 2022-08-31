//! The [AsyncFiniteMoveIterator] trait.

mod atmiadapter;

pub use self::atmiadapter::AfmiAsTerminal;
use async_trait::async_trait;

/// Produce a sequence of 0 or more `Item` values asynchronously, using move semantics.
///
/// # `Iterator` Call-site Example
///
/// Any [Iterator] + [Sized] + [Send] is an [AsyncFiniteMoveIterator], such as the
/// [std::ops::Range] in this example:
///
/// ```
/// # tokio::runtime::Builder::new_current_thread().build().unwrap().block_on(async {
/// # use moveiter::AsyncFiniteMoveIterator;
/// let it = 0..2;
///
/// let (it2, a) = it.into_next().await.unwrap();
/// assert_eq!(a, 0);
///
/// let (it3, b) = it2.into_next().await.unwrap();
/// assert_eq!(b, 1);
///
/// let empty = it3.into_next().await;
/// assert!(empty.is_none());
/// # });
/// ```
///
/// # Local `mut` call site:
///
/// To express a loop a common pattern is to use `mut` assignment to the iterator values:
///
/// ```
/// # use moveiter::AsyncFiniteMoveIterator;
/// async fn process_items<I>(mut it: I)
///   where I: AsyncFiniteMoveIterator,
/// {
///     while let Some((nextit, x)) = it.into_next().await {
///         it = nextit;
///         // Process `x`...
///     }
/// }
/// ```
///
/// # Example: Move Semantics
///
/// Suppose we made a mistake in writing the previous example:
///
/// ```compile_fail
/// # use moveiter::AsyncFiniteMoveIterator;
/// async fn process_items<I>(mut it: I)
///     where I: AsyncFiniteMoveIterator,
/// {
///     while let Some((nextit, x)) = it.into_next().await {
///         // Process `x`...
///     }
/// }
/// ```
///
/// Because we attempt to re-use the moved iterator `it`, this is a compile-time error:
///
/// ```text
/// error[E0382]: use of moved value: `it`
///   --> src/asyn/finite.rs:52:35
///    |
/// 5  | async fn process_items<I>(mut it: I)
///    |                           ------ move occurs because `it` has type `I`, which does not implement the `Copy` trait
/// ...
/// 8  |     while let Some((nextit, x)) = it.into_next().await {
///    |                                   ^^ ----------- `it` moved due to this method call, in previous iteration of loop
///    |
/// note: this function takes ownership of the receiver `self`, which moves `it`
///   --> /home/user/hack/moveiter/src/asyn/finite.rs:78:24
///    |
/// 78 |     async fn into_next(self) -> Option<(Self, Self::Item)>;
///    |                        ^^^^
/// ```
#[async_trait]
pub trait AsyncFiniteMoveIterator: Sized + Send {
    type Item;

    /// Iteration is async, moves `self`, and produces an `Option<(Self, Self::Item)>`.
    ///
    /// The simplified interface via [async_trait](::async_trait) looks like:
    ///
    /// ```no_run
    /// # use either::Either;
    /// # #[async_trait::async_trait]
    /// # trait T: Sized {
    /// # type Item;
    /// async fn into_next(self) -> Option<(Self, Self::Item)>;
    /// # }
    /// ```
    async fn into_next(self) -> Option<(Self, Self::Item)>;

    /// Adapt `self` into an [AsyncTerminalMoveIterator](crate::AsyncTerminalMoveIterator) with `Terminal = ()`.
    fn into_async_terminal_move_iterator(self) -> AfmiAsTerminal<Self> {
        AfmiAsTerminal(self)
    }
}

#[async_trait]
impl<I> AsyncFiniteMoveIterator for I
where
    I: Iterator + Sized + Send,
{
    type Item = I::Item;

    async fn into_next(mut self) -> Option<(Self, Self::Item)> {
        self.next().map(|item| (self, item))
    }
}
