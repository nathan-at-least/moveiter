use crate::TerminalIterator;

pub struct Map<I, F, U>
where
    I: TerminalIterator,
    F: Fn(<I as TerminalIterator>::Item) -> U,
{
    it: I,
    f: F,
}

impl<I, F, U> Map<I, F, U>
where
    I: TerminalIterator,
    F: Fn(<I as TerminalIterator>::Item) -> U,
{
    pub fn new(it: I, f: F) -> Self {
        Map { it, f }
    }
}

impl<I, F, U> TerminalIterator for Map<I, F, U>
where
    I: TerminalIterator,
    F: Fn(<I as TerminalIterator>::Item) -> U,
{
    type Item = U;
    type Terminal = <I as TerminalIterator>::Terminal;

    fn into_next_result(self) -> Result<(Self, Self::Item), Self::Terminal> {
        let (it, t) = self.it.into_next_result()?;
        let u = (self.f)(t);
        let nextself = Map { it, f: self.f };
        Ok((nextself, u))
    }
}
