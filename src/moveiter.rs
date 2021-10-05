mod stditer;

#[cfg(test)]
mod tests;

pub use self::stditer::MoveStdIter;

/// Types which provide iteration over `Item`s with termination enforced by the type system.
///
/// The `MoveIterator` trait is near-isomorphic to `std::iter::Iterator`, and in fact, any type
/// `T: Iterator` is also an instance of `MoveIterator`.
pub trait MoveIterator: Sized {
    /// The type of elements produced by the `MoveIterator`.
    type Item;

    /// The iteration method consumes `self` by move and produces either `None` or else a new state
    /// and the next `Item` element.
    fn into_next_option(self) -> Option<(Self, Self::Item)>;

    /// Any `MoveIter` can be converted into a wrapper type `MoveStdIter` which is an
    /// `Iterator`, which is useful for integrating to existing `Iterator`-based APIs.
    fn into_iter(self) -> MoveStdIter<Self> {
        MoveStdIter::from(self)
    }

    // std::iter::Iterator-inspired methods:
    /// Same semantics as the `std::iter::Iterator` method of the same name.
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }

    /// Same semantics as the `std::iter::Iterator` method of the same name.
    fn count(self) -> usize {
        let mut state = self;
        let mut c = 0;
        while let Some((newstate, _)) = state.into_next_option() {
            c += 1;
            state = newstate;
        }
        c
    }
}

/// Types which convert into a [`MoveIterator`].
pub trait IntoMoveIterator {
    type Item;
    type IntoMoveIter: MoveIterator<Item = Self::Item>;

    fn into_move_iter(self) -> Self::IntoMoveIter;
}

/// Any `std::iter::Iterator` type is automatically a `MoveIterator` because `into_next_option` can
/// internally mutate the iterator with `next` then return it as the next state.
impl<I> MoveIterator for I
where
    I: Iterator,
{
    type Item = <I as Iterator>::Item;

    fn into_next_option(mut self) -> Option<(Self, Self::Item)> {
        self.next().map(|item| (self, item))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        <Self as Iterator>::size_hint(self)
    }
}
