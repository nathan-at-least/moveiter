use crate::MoveIterator;

pub trait TerminalIterator: Sized {
    type Item;
    type Terminal;

    fn into_next_result(self) -> Result<(Self, Self::Item), Self::Terminal>;
}

impl<T> TerminalIterator for T
where
    T: MoveIterator,
{
    type Item = <Self as MoveIterator>::Item;
    type Terminal = ();

    fn into_next_result(self) -> Result<(Self, Self::Item), ()> {
        self.into_next_option().ok_or(())
    }
}
