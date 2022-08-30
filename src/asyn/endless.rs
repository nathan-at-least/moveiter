//! The [AsyncEndlessMoveIterator] trait.
use async_trait::async_trait;

/// An `AsyncEndlessMoveIterator` type produces an endless sequence of `Item` values asynchronously, using move semantics.
///
/// # Example: Local `mut` call site:
///
/// To express a loop a common pattern is to use `mut` assignment to the iterator values:
///
/// ```
/// # use moveiter::AsyncEndlessMoveIterator;
/// /// This function never terminates:
/// async fn process_items<I>(mut it: I)
///   where I: AsyncEndlessMoveIterator,
/// {
///     while let (nextit, x) = it.aemi_next().await {
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
/// # use moveiter::AsyncEndlessMoveIterator;
/// /// This function never terminates:
/// async fn process_items<I>(mut it: I)
///     where I: AsyncEndlessMoveIterator,
/// {
///     while let (nextit, x) = it.aemi_next().await {
///         // Process `x`...
///     }
/// }
/// ```
///
/// Because we attempt to re-use the moved iterator `it`, this is a compile-time error:
///
/// ```text
/// error[E0382]: use of moved value: `it`
///   --> src/asyn/endless.rs:33:29
///    |
/// 6  | async fn process_items<I>(mut it: I)
///    |                           ------ move occurs because `it` has type `I`, which does not implement the `Copy` trait
/// ...
/// 9  |     while let (nextit, x) = it.aemi_next().await {
///    |                             ^^ ----------- `it` moved due to this method call, in previous iteration of loop
///    |
/// note: this function takes ownership of the receiver `self`, which moves `it`
///   --> /home/user/hack/moveiter/src/asyn/endless.rs:60:24
///    |
/// 60 |     async fn aemi_next(self) -> (Self, Self::Item);
///    |                        ^^^^
/// ```
#[async_trait]
pub trait AsyncEndlessMoveIterator: Sized + Send {
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
    /// async fn aemi_next(self) -> (Self, Self::Item);
    /// # }
    /// ```
    async fn aemi_next(self) -> (Self, Self::Item);
}
