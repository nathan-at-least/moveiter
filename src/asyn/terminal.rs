//! The [AsyncTerminalMoveIterator] trait.
use async_trait::async_trait;
use either::Either;

/// An `AsyncTerminalMoveIterator` type produces a sequence of 0 or more `Item` values asynchronously, then produces a `Terminal` value, using move semantics.
///
/// # Call-site Example
///
/// ```
/// # tokio::runtime::Builder::new_current_thread().build().unwrap().block_on(async {
/// use moveiter::AsyncTerminalMoveIterator;
///
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
#[async_trait]
pub trait AsyncTerminalMoveIterator: Sized + Send {
    type Item;
    type Terminal;

    /// Iteration is async, moves `self`, and produces either a `(Self, Self::Item)` pair, or the
    /// `Self::Terminal` value.
    ///
    /// The simplified interface via [async_trait](::async_trait) looks like:
    ///
    /// ```ignore
    /// async fn atmi_next(self) -> Either<(Self, Self::Item), Self::Terminal>;
    /// ```
    async fn atmi_next(self) -> Either<(Self, Self::Item), Self::Terminal>;
}

/// Any [Iterator] + [Sized] + [Send] is an [AsyncTerminalMoveIterator].
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
