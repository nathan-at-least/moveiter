use crate::TerminalIterator;

pub enum Chain<T, U>
where
    T: TerminalIterator,
    U: TerminalIterator<Item = <T as TerminalIterator>::Item>,
{
    InT(T, U),
    InU(<T as TerminalIterator>::Terminal, U),
}

impl<T, U> Chain<T, U>
where
    T: TerminalIterator,
    U: TerminalIterator<Item = <T as TerminalIterator>::Item>,
{
    pub fn new(t: T, u: U) -> Self {
        Chain::InT(t, u)
    }
}

impl<T, U> TerminalIterator for Chain<T, U>
where
    T: TerminalIterator,
    U: TerminalIterator<Item = <T as TerminalIterator>::Item>,
{
    type Item = <T as TerminalIterator>::Item;
    type Terminal = (
        <T as TerminalIterator>::Terminal,
        <U as TerminalIterator>::Terminal,
    );

    fn into_next_result(self) -> Result<(Self, Self::Item), Self::Terminal> {
        use Chain::*;

        match self {
            InT(t, u) => match t.into_next_result() {
                Ok((nextt, x)) => Ok((InT(nextt, u), x)),
                Err(termt) => InU(termt, u).into_next_result(),
            },
            InU(termt, u) => match u.into_next_result() {
                Ok((nextu, y)) => Ok((InU(termt, nextu), y)),
                Err(termu) => Err((termt, termu)),
            },
        }
    }
}
