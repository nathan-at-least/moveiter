use crate::TerminalIterator;

pub struct TakeWhile<I, F>(I, F)
where
    I: TerminalIterator,
    F: FnMut(&<I as TerminalIterator>::Item) -> bool;

impl<I, F> TakeWhile<I, F>
where
    I: TerminalIterator,
    F: FnMut(&<I as TerminalIterator>::Item) -> bool,
{
    pub fn new(it: I, f: F) -> Self {
        TakeWhile(it, f)
    }
}

impl<I, F> TerminalIterator for TakeWhile<I, F>
where
    I: TerminalIterator,
    F: FnMut(&<I as TerminalIterator>::Item) -> bool,
{
    type Item = <I as TerminalIterator>::Item;
    type Terminal = <I as TerminalIterator>::Terminal;

    fn into_next_result(self) -> Result<(Self, Self::Item), Self::Terminal> {
        let TakeWhile(s, mut f) = self;
        let (s2, x) = s.into_next_result()?;
        if f(&x) {
            Ok((TakeWhile(s2, f), x))
        } else {
            Err(s2.terminate())
        }
    }
}
