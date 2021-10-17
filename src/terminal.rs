//! An Iterator which terminates without any associated value.
mod stditer;

#[cfg(test)]
mod tests;

pub use self::stditer::TerminalStdIter;

/// Types which provide iteration over `Item`s with termination enforced by the type system.
///
/// The `Iterator` trait is near-isomorphic to `std::iter::Iterator`, and in fact, any type
/// `T: Iterator` is also an instance of `Iterator`.
pub trait Iterator: Sized {
    /// The type of elements produced by the `Iterator`.
    type Item;

    /// The iteration method consumes `self` by move and produces either `None` or else a new state
    /// and the next `Item` element.
    fn into_next_option(self) -> Option<(Self, Self::Item)>;

    /// Any `TerminalIter` can be converted into a wrapper type `TerminalStdIter` which is an
    /// `Iterator`, which is useful for integrating to existing `Iterator`-based APIs.
    fn into_iter(self) -> TerminalStdIter<Self> {
        TerminalStdIter::from(self)
    }
}

/// Types which convert into a [`Iterator`].
pub trait IntoIterator {
    type Item;
    type IntoTerminalIter: Iterator<Item = Self::Item>;

    fn into_term_iter(self) -> Self::IntoTerminalIter;
}

/// Any `std::iter::Iterator` type is automatically a `Iterator` because `into_next_option` can
/// internally mutate the iterator with `next` then return it as the next state.
impl<I> Iterator for I
where
    I: std::iter::Iterator,
{
    type Item = <I as std::iter::Iterator>::Item;

    fn into_next_option(mut self) -> Option<(Self, Self::Item)> {
        self.next().map(|item| (self, item))
    }
}
