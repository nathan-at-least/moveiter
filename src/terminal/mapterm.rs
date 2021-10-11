use crate::TerminalIterator;

pub struct MapTerm<T, F, U>
where
    T: TerminalIterator,
    F: FnOnce(<T as TerminalIterator>::Terminal) -> U,
{
    ti: T,
    f: F,
}

impl<T, F, U> MapTerm<T, F, U>
where
    T: TerminalIterator,
    F: FnOnce(<T as TerminalIterator>::Terminal) -> U,
{
    pub fn new(ti: T, f: F) -> Self {
        MapTerm { ti, f }
    }
}

impl<T, F, U> TerminalIterator for MapTerm<T, F, U>
where
    T: TerminalIterator,
    F: FnOnce(<T as TerminalIterator>::Terminal) -> U,
{
    type Item = <T as TerminalIterator>::Item;
    type Terminal = U;

    fn into_next_result(self) -> Result<(Self, Self::Item), U> {
        match self.ti.into_next_result() {
            Ok((nextti, x)) => {
                let nextself = MapTerm {
                    ti: nextti,
                    f: self.f,
                };
                Ok((nextself, x))
            }
            Err(term) => Err((self.f)(term)),
        }
    }
}
