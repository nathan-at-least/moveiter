use crate::TerminalIterator;

pub struct Zip<T, U>
where
    T: TerminalIterator,
    U: TerminalIterator,
{
    t: T,
    u: U,
}

impl<T, U> Zip<T, U>
where
    T: TerminalIterator,
    U: TerminalIterator,
{
    pub fn new(t: T, u: U) -> Self {
        Zip { t, u }
    }
}

pub enum ZipTerminal<T, U>
where
    T: TerminalIterator,
    U: TerminalIterator,
{
    LeftTerm(
        <T as TerminalIterator>::Terminal,
        (U, <U as TerminalIterator>::Item),
    ),
    RightTerm(
        (T, <T as TerminalIterator>::Item),
        <U as TerminalIterator>::Terminal,
    ),
    BothTerm(
        <T as TerminalIterator>::Terminal,
        <U as TerminalIterator>::Terminal,
    ),
}

impl<T, U> TerminalIterator for Zip<T, U>
where
    T: TerminalIterator,
    U: TerminalIterator,
{
    type Item = (<T as TerminalIterator>::Item, <U as TerminalIterator>::Item);
    type Terminal = ZipTerminal<T, U>;

    fn into_next_result(self) -> Result<(Self, Self::Item), Self::Terminal> {
        let tres = self.t.into_next_result();
        let ures = self.u.into_next_result();

        match (tres, ures) {
            (Ok((tnext, titem)), Ok((unext, uitem))) => {
                Ok((Zip::new(tnext, unext), (titem, uitem)))
            }
            (Err(tterm), Ok((unext, uitem))) => Err(ZipTerminal::LeftTerm(tterm, (unext, uitem))),
            (Ok((tnext, titem)), Err(uterm)) => Err(ZipTerminal::RightTerm((tnext, titem), uterm)),
            (Err(tterm), Err(uterm)) => Err(ZipTerminal::BothTerm(tterm, uterm)),
        }
    }
}

impl<T, U> ZipTerminal<T, U>
where
    T: TerminalIterator,
    U: TerminalIterator,
{
    /// Discard any pending items on either side to return the two terminals.
    pub fn complete(
        self,
    ) -> (
        <T as TerminalIterator>::Terminal,
        <U as TerminalIterator>::Terminal,
    ) {
        use ZipTerminal::*;

        match self {
            LeftTerm(tterm, (ustate, _)) => (tterm, ustate.terminate()),
            RightTerm((tstate, _), uterm) => (tstate.terminate(), uterm),
            BothTerm(tterm, uterm) => (tterm, uterm),
        }
    }
}
