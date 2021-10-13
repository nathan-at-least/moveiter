use crate::TerminalIterator;

pub struct Peekable<T>(
    Result<(T, <T as TerminalIterator>::Item), <T as TerminalIterator>::Terminal>,
)
where
    T: TerminalIterator;

impl<T> Peekable<T>
where
    T: TerminalIterator,
{
    pub fn new(t: T) -> Self {
        Peekable(t.into_next_result())
    }

    pub fn peek(&self) -> Option<&<T as TerminalIterator>::Item> {
        self.0.as_ref().ok().map(|(_, item)| item)
    }
}

impl<T> TerminalIterator for Peekable<T>
where
    T: TerminalIterator,
{
    type Item = <T as TerminalIterator>::Item;
    type Terminal = <T as TerminalIterator>::Terminal;

    fn into_next_result(self) -> Result<(Self, Self::Item), Self::Terminal> {
        match self.0 {
            Ok((t, x)) => {
                let nextself = Peekable(t.into_next_result());
                Ok((nextself, x))
            }
            Err(term) => Err(term),
        }
    }
}
