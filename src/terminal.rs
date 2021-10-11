mod stepby;

use crate::MoveIterator;

pub use self::stepby::StepBy;

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

    fn terminal_and_count(self) -> (Self::Terminal, usize) {
        let mut c = 0;
        let term = self.for_each(|_| {
            c += 1;
            None
        });
        (term, c)
    }

    fn terminal_and_last(self) -> (Self::Terminal, Option<Self::Item>) {
        let mut item = None;
        let term = self.for_each(|x| {
            item = Some(x);
            None
        });
        (term, item)
    }

    // std::iter::Iterator-inspired methods:
    /// Same semantics as the `std::iter::Iterator` method of the same name.
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }

    /// Same semantics as the `std::iter::Iterator` method of the same name.
    fn count(self) -> usize {
        let (_, c) = self.terminal_and_count();
        c
    }

    fn last(self) -> Option<Self::Item> {
        let (_, x) = self.terminal_and_last();
        x
    }

    fn nth(self, n: usize) -> Result<(Self, Self::Item), Self::Terminal> {
        self.skip(n)?.into_next_result()
    }

    fn skip(self, n: usize) -> Result<Self, Self::Terminal> {
        let mut state = self;
        for _ in 0..n {
            let (nextstate, _) = state.into_next_result()?;
            state = nextstate;
        }
        Ok(state)
    }

    fn step_by(self, step: usize) -> StepBy<Self> {
        StepBy::new(self, step)
    }

    /*

        pub fn chain<U>(
            self,
            other: U
        ) -> Chain<Self, <U as IntoIterator>::IntoIter>ⓘ
        where
            U: IntoIterator<Item = Self::Item>,
        { ... }
        pub fn zip<U>(self, other: U) -> Zip<Self, <U as IntoIterator>::IntoIter>ⓘ
        where
            U: IntoIterator,
        { ... }
        pub fn intersperse(self, separator: Self::Item) -> Intersperse<Self>ⓘ
        where
            Self::Item: Clone,
        { ... }
        pub fn intersperse_with<G>(self, separator: G) -> IntersperseWith<Self, G>ⓘ
        where
            G: FnMut() -> Self::Item,
        { ... }
        pub fn map<B, F>(self, f: F) -> Map<Self, F>ⓘ
        where
            F: FnMut(Self::Item) -> B,
        { ... }
        pub fn for_each<F>(self, f: F)
        where
            F: FnMut(Self::Item),
        { ... }
        pub fn filter<P>(self, predicate: P) -> Filter<Self, P>ⓘ
        where
            P: FnMut(&Self::Item) -> bool,
        { ... }
        pub fn filter_map<B, F>(self, f: F) -> FilterMap<Self, F>ⓘ
        where
            F: FnMut(Self::Item) -> Option<B>,
        { ... }
        pub fn enumerate(self) -> Enumerate<Self>ⓘ { ... }
        pub fn peekable(self) -> Peekable<Self>ⓘ { ... }
        pub fn skip_while<P>(self, predicate: P) -> SkipWhile<Self, P>ⓘ
        where
            P: FnMut(&Self::Item) -> bool,
        { ... }
        pub fn take_while<P>(self, predicate: P) -> TakeWhile<Self, P>ⓘ
        where
            P: FnMut(&Self::Item) -> bool,
        { ... }
        pub fn map_while<B, P>(self, predicate: P) -> MapWhile<Self, P>ⓘ
        where
            P: FnMut(Self::Item) -> Option<B>,
        { ... }
        pub fn skip(self, n: usize) -> Skip<Self>ⓘ { ... }
        pub fn take(self, n: usize) -> Take<Self>ⓘ { ... }
        pub fn scan<St, B, F>(self, initial_state: St, f: F) -> Scan<Self, St, F>ⓘ
        where
            F: FnMut(&mut St, Self::Item) -> Option<B>,
        { ... }
        pub fn flat_map<U, F>(self, f: F) -> FlatMap<Self, U, F>ⓘ
        where
            F: FnMut(Self::Item) -> U,
            U: IntoIterator,
        { ... }
        pub fn flatten(self) -> Flatten<Self>ⓘ
        where
            Self::Item: IntoIterator,
        { ... }
        pub fn fuse(self) -> Fuse<Self>ⓘ { ... }
        pub fn inspect<F>(self, f: F) -> Inspect<Self, F>ⓘ
        where
            F: FnMut(&Self::Item),
        { ... }
        pub fn by_ref(&mut self) -> &mut Selfⓘ { ... }
    [+] Expand attributes
        pub fn collect<B>(self) -> B
        where
            B: FromIterator<Self::Item>,
        { ... }
        pub fn partition<B, F>(self, f: F) -> (B, B)
        where
            F: FnMut(&Self::Item) -> bool,
            B: Default + Extend<Self::Item>,
        { ... }
        pub fn partition_in_place<'a, T, P>(self, predicate: P) -> usize
        where
            Self: DoubleEndedIterator<Item = &'a mut T>,
            T: 'a,
            P: FnMut(&T) -> bool,
        { ... }
        pub fn is_partitioned<P>(self, predicate: P) -> bool
        where
            P: FnMut(Self::Item) -> bool,
        { ... }
        pub fn try_fold<B, F, R>(&mut self, init: B, f: F) -> R
        where
            F: FnMut(B, Self::Item) -> R,
            R: Try<Ok = B>,
        { ... }
        pub fn try_for_each<F, R>(&mut self, f: F) -> R
        where
            F: FnMut(Self::Item) -> R,
            R: Try<Ok = ()>,
        { ... }
        pub fn fold<B, F>(self, init: B, f: F) -> B
        where
            F: FnMut(B, Self::Item) -> B,
        { ... }
        pub fn reduce<F>(self, f: F) -> Option<Self::Item>
        where
            F: FnMut(Self::Item, Self::Item) -> Self::Item,
        { ... }
        pub fn all<F>(&mut self, f: F) -> bool
        where
            F: FnMut(Self::Item) -> bool,
        { ... }
        pub fn any<F>(&mut self, f: F) -> bool
        where
            F: FnMut(Self::Item) -> bool,
        { ... }
        pub fn find<P>(&mut self, predicate: P) -> Option<Self::Item>
        where
            P: FnMut(&Self::Item) -> bool,
        { ... }
        pub fn find_map<B, F>(&mut self, f: F) -> Option<B>
        where
            F: FnMut(Self::Item) -> Option<B>,
        { ... }
        pub fn try_find<F, R>(
            &mut self,
            f: F
        ) -> Result<Option<Self::Item>, <R as Try>::Error>
        where
            F: FnMut(&Self::Item) -> R,
            R: Try<Ok = bool>,
        { ... }
        pub fn position<P>(&mut self, predicate: P) -> Option<usize>
        where
            P: FnMut(Self::Item) -> bool,
        { ... }
        pub fn rposition<P>(&mut self, predicate: P) -> Option<usize>
        where
            Self: ExactSizeIterator + DoubleEndedIterator,
            P: FnMut(Self::Item) -> bool,
        { ... }
        pub fn max(self) -> Option<Self::Item>
        where
            Self::Item: Ord,
        { ... }
        pub fn min(self) -> Option<Self::Item>
        where
            Self::Item: Ord,
        { ... }
        pub fn max_by_key<B, F>(self, f: F) -> Option<Self::Item>
        where
            F: FnMut(&Self::Item) -> B,
            B: Ord,
        { ... }
        pub fn max_by<F>(self, compare: F) -> Option<Self::Item>
        where
            F: FnMut(&Self::Item, &Self::Item) -> Ordering,
        { ... }
        pub fn min_by_key<B, F>(self, f: F) -> Option<Self::Item>
        where
            F: FnMut(&Self::Item) -> B,
            B: Ord,
        { ... }
        pub fn min_by<F>(self, compare: F) -> Option<Self::Item>
        where
            F: FnMut(&Self::Item, &Self::Item) -> Ordering,
        { ... }
        pub fn rev(self) -> Rev<Self>ⓘ
        where
            Self: DoubleEndedIterator,
        { ... }
        pub fn unzip<A, B, FromA, FromB>(self) -> (FromA, FromB)
        where
            Self: Iterator<Item = (A, B)>,
            FromA: Default + Extend<A>,
            FromB: Default + Extend<B>,
        { ... }
        pub fn copied<'a, T>(self) -> Copied<Self>ⓘ
        where
            Self: Iterator<Item = &'a T>,
            T: 'a + Copy,
        { ... }
        pub fn cloned<'a, T>(self) -> Cloned<Self>ⓘ
        where
            Self: Iterator<Item = &'a T>,
            T: 'a + Clone,
        { ... }
        pub fn cycle(self) -> Cycle<Self>ⓘ
        where
            Self: Clone,
        { ... }
        pub fn sum<S>(self) -> S
        where
            S: Sum<Self::Item>,
        { ... }
        pub fn product<P>(self) -> P
        where
            P: Product<Self::Item>,
        { ... }
        pub fn cmp<I>(self, other: I) -> Ordering
        where
            I: IntoIterator<Item = Self::Item>,
            Self::Item: Ord,
        { ... }
        pub fn cmp_by<I, F>(self, other: I, cmp: F) -> Ordering
        where
            F: FnMut(Self::Item, <I as IntoIterator>::Item) -> Ordering,
            I: IntoIterator,
        { ... }
        pub fn partial_cmp<I>(self, other: I) -> Option<Ordering>
        where
            I: IntoIterator,
            Self::Item: PartialOrd<<I as IntoIterator>::Item>,
        { ... }
        pub fn partial_cmp_by<I, F>(
            self,
            other: I,
            partial_cmp: F
        ) -> Option<Ordering>
        where
            F: FnMut(Self::Item, <I as IntoIterator>::Item) -> Option<Ordering>,
            I: IntoIterator,
        { ... }
        pub fn eq<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialEq<<I as IntoIterator>::Item>,
        { ... }
        pub fn eq_by<I, F>(self, other: I, eq: F) -> bool
        where
            F: FnMut(Self::Item, <I as IntoIterator>::Item) -> bool,
            I: IntoIterator,
        { ... }
        pub fn ne<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialEq<<I as IntoIterator>::Item>,
        { ... }
        pub fn lt<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialOrd<<I as IntoIterator>::Item>,
        { ... }
        pub fn le<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialOrd<<I as IntoIterator>::Item>,
        { ... }
        pub fn gt<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialOrd<<I as IntoIterator>::Item>,
        { ... }
        pub fn ge<I>(self, other: I) -> bool
        where
            I: IntoIterator,
            Self::Item: PartialOrd<<I as IntoIterator>::Item>,
        { ... }
        pub fn is_sorted(self) -> bool
        where
            Self::Item: PartialOrd<Self::Item>,
        { ... }
        pub fn is_sorted_by<F>(self, compare: F) -> bool
        where
            F: FnMut(&Self::Item, &Self::Item) -> Option<Ordering>,
        { ... }
        pub fn is_sorted_by_key<F, K>(self, f: F) -> bool
        where
            F: FnMut(Self::Item) -> K,
            K: PartialOrd<K>,
        { ... }
            */
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
