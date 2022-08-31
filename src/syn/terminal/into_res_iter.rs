use crate::optutil::OptionUpdate;
use crate::TerminalMoveIterator;

/// Convert a [TerminalMoveIterator] with `Terminal = Result<(), E>` into a
/// [std::iter::Iterator].
///
/// This is the inverse of [terminal_move_iterator_from_result_iterator](crate::terminal_move_iterator_from_result_iterator).
pub fn terminal_move_iterator_into_result_iterator<I, T, E>(
    tmi: I,
) -> impl Iterator<Item = Result<T, E>>
where
    I: TerminalMoveIterator<Item = T, Terminal = Result<(), E>>,
{
    TmiAsResultIterator(Some(Some(tmi)))
}

// The inner `Option` becomes `None` if we yield the terminal `Err`;
// the outer `Option` becomes `None` when `Iterator` is complete.
struct TmiAsResultIterator<I>(Option<Option<I>>);

impl<I, T, E> Iterator for TmiAsResultIterator<I>
where
    I: TerminalMoveIterator<Item = T, Terminal = Result<(), E>>,
{
    type Item = Result<T, E>;

    fn next(&mut self) -> Option<Self::Item> {
        use either::Either::*;

        self.0.update_in_place(|opttmi| {
            opttmi.and_then(|tmi| match tmi.into_next() {
                Left((next, x)) => Some((Some(next), Ok(x))),
                Right(res) => match res {
                    Ok(()) => None,
                    Err(e) => Some((None, Err(e))),
                },
            })
        })
    }
}
