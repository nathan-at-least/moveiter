use crate::TerminalIterator;

pub enum SkipWhile<I, F>
where
    I: TerminalIterator,
    F: FnMut(&<I as TerminalIterator>::Item) -> bool,
{
    Skipping(I, F),
    Latched(I),
}

impl<I, F> SkipWhile<I, F>
where
    I: TerminalIterator,
    F: FnMut(&<I as TerminalIterator>::Item) -> bool,
{
    pub fn new(it: I, f: F) -> Self {
        SkipWhile::Skipping(it, f)
    }
}

impl<I, F> TerminalIterator for SkipWhile<I, F>
where
    I: TerminalIterator,
    F: FnMut(&<I as TerminalIterator>::Item) -> bool,
{
    type Item = <I as TerminalIterator>::Item;
    type Terminal = <I as TerminalIterator>::Terminal;

    fn into_next_result(self) -> Result<(Self, Self::Item), Self::Terminal> {
        use SkipWhile::*;

        match self {
            Skipping(it, mut f) => {
                let (next, x) = it.scan_state(|x| if f(&x) { None } else { Some(Ok(x)) })?;
                Ok((Latched(next), x))
            }
            Latched(it) => it.map_state(Latched),
        }
    }
}
