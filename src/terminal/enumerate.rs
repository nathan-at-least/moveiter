use crate::TerminalIterator;

pub struct Enumerate<T>
where
    T: TerminalIterator,
{
    t: T,
    ix: usize,
}

impl<T> Enumerate<T>
where
    T: TerminalIterator,
{
    pub fn new(t: T) -> Self {
        Enumerate { t, ix: 0 }
    }
}

impl<T> TerminalIterator for Enumerate<T>
where
    T: TerminalIterator,
{
    type Item = (usize, <T as TerminalIterator>::Item);
    type Terminal = <T as TerminalIterator>::Terminal;

    fn into_next_result(self) -> Result<(Self, Self::Item), Self::Terminal> {
        let ix = self.ix;
        self.t
            .map_state(|t| Enumerate { t, ix: ix + 1 })
            .map(|(s, x)| (s, (ix, x)))
    }
}
