use crate::TerminalMoveIterator;
use either::Either;

/// Convert a [std::iter::Iterator] with `Item = Result<T, E>` into a [TerminalMoveIterator].
///
/// # Example
///
/// A recurring pattern with [std::iter::Iterator] is to pass potential errors through iteration
/// items with [Result]. As a convention, not captured by the type, consuming code should abort
/// iteration and propagate the error.
///
/// ```
/// fn count_empty_lines<R>(input: R) -> std::io::Result<i32>
///     where R: std::io::BufRead,
/// {
///     let mut empty_lines = 0;
///
///     for lineres in input.lines() {
///         let line = lineres?;
///         if line.is_empty() {
///             empty_lines += 1;
///         }
///     }
///
///     Ok(empty_lines)
/// }
/// ```
///
/// This pattern can be converted to a [TerminalMoveIterator] to
/// facilitate the error propagation:
///
/// ```
/// fn count_empty_lines<R>(input: R) -> std::io::Result<i32>
///     where R: std::io::BufRead,
/// {
///     use moveiter::TerminalMoveIterator;
///
///     let it = moveiter::terminal_move_iterator_from_result_iterator(input.lines());
///     let mut empty_lines = 0;
///
///     it.for_each(|line| {
///         if line.is_empty() {
///             empty_lines += 1;
///         }
///     })?;
///
///     Ok(empty_lines)
/// }
/// ```
///
/// An alternative implementation can use [TerminalMoveIterator::fold]:
///
/// ```
/// fn count_empty_lines<R>(input: R) -> std::io::Result<i32>
///     where R: std::io::BufRead,
/// {
///     use moveiter::TerminalMoveIterator;
///
///     moveiter::terminal_move_iterator_from_result_iterator(input.lines()).fold(
///         0,
///         |c, line| c + if line.is_empty() { 1 } else { 0 },
///         |c, res| res.map(|()| c),
///     )
/// }
/// ```
pub fn terminal_move_iterator_from_result_iterator<I, T, E>(
    it: I,
) -> impl TerminalMoveIterator<Item = T, Terminal = Result<(), E>>
where
    I: IntoIterator<Item = Result<T, E>> + Sized,
{
    TmiFromResultIterator(it.into_iter())
}

/// Produces items from the underlying [std::iter::Iterator], terminating with [Err] if
/// encountered, otherwise terminating with [Ok].
///
/// The [TerminalMoveIterator] value returned by [terminal_move_iterator_from_result_iterator].
struct TmiFromResultIterator<I>(I);

impl<I, T, E> TerminalMoveIterator for TmiFromResultIterator<I>
where
    I: Iterator<Item = Result<T, E>> + Sized,
{
    type Terminal = Result<(), E>;
    type Item = T;

    fn into_next(mut self) -> Either<(Self, Self::Item), Self::Terminal> {
        use Either::*;

        if let Some(res) = self.0.next() {
            match res {
                Ok(x) => Left((self, x)),
                Err(e) => Right(Err(e)),
            }
        } else {
            Right(Ok(()))
        }
    }
}
