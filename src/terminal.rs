use crate::MoveIterator;

#[cfg(test)]
mod tests;

/// Types which produces a sequence of `Item`s and then finally a `Terminal` type.
///
/// This is a generalization of `MoveIterator` (and `std::iter::Iterator`) which enables more
/// expressive termination. For example, a type which performs input with `std::io` can produce
/// simple `Item` results and terminate with a `std::io::Result<()>` which ensures that any IO
/// errors terminate iteration.
///
/// Any type which is `MoveIterator` is also an instance of `TerminalIterator` with `Terminal =
/// ()`.
pub trait TerminalIterator: Sized {
    /// The type of the elements produced by the iterator:
    type Item;

    /// A `Terminal` value is produced when iteration terminates:
    type Terminal;

    /// The iteration method produces either a next state and item, or a `Terminal` value. Note
    /// that although this is a `Result`, the `Terminal` value may not represent an error, per-se.
    fn into_next_result(self) -> Result<(Self, Self::Item), Self::Terminal>;

    /// Call `f` on each item, returning the terminal value. Either `f` provides a terminal value,
    /// which is similar to breaking a for-loop, or `Self` does.
    fn for_each<F>(self, mut f: F) -> Self::Terminal
    where
        F: FnMut(Self::Item) -> Option<Self::Terminal>,
    {
        let mut state = self;
        loop {
            match state.into_next_result() {
                Ok((newstate, item)) => {
                    state = newstate;
                    if let Some(term) = f(item) {
                        return term;
                    }
                }
                Err(term) => {
                    return term;
                }
            }
        }
    }

    fn count_and_terminal(self) -> (usize, Self::Terminal) {
        let mut c = 0;
        let term = self.for_each(|_| {
            c += 1;
            None
        });
        (c, term)
    }

    // std::iter::Iterator-inspired methods:
    /// Same semantics as the `std::iter::Iterator` method of the same name.
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }

    /// Same semantics as the `std::iter::Iterator` method of the same name.
    fn count(self) -> usize {
        let (c, _) = self.count_and_terminal();
        c
    }
}

/// Types which convert into a [`TerminalIterator`].
pub trait IntoTerminalIterator {
    type Item;
    type Terminal;
    type IntoTerminal: TerminalIterator<Item = Self::Item, Terminal = Self::Terminal>;

    fn into_term_iter(self) -> Self::IntoTerminal;
}

/// Any `MoveIterator` type is also a `TerminalIterator` with `()` as the `Terminal` type. This is
/// analogous to the isomorphism of `Option<T>` with `Result<T, ()>`.
impl<T> TerminalIterator for T
where
    T: MoveIterator,
{
    type Item = <Self as MoveIterator>::Item;
    type Terminal = ();

    fn into_next_result(self) -> Result<(Self, Self::Item), ()> {
        self.into_next_option().ok_or(())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        <Self as MoveIterator>::size_hint(self)
    }
}
