use crate::TerminalIterator;

pub struct StepBy<T>
where
    T: TerminalIterator,
{
    ti: T,
    step: usize,
    nextstep: usize,
}

impl<T> StepBy<T>
where
    T: TerminalIterator,
{
    pub fn new(ti: T, step: usize) -> StepBy<T> {
        StepBy {
            ti,
            step,
            nextstep: 0,
        }
    }
}

impl<T> TerminalIterator for StepBy<T>
where
    T: TerminalIterator,
{
    type Item = <T as TerminalIterator>::Item;
    type Terminal = <T as TerminalIterator>::Terminal;

    fn into_next_result(self) -> Result<(Self, Self::Item), Self::Terminal> {
        let mut state = self.ti;
        if self.nextstep > 0 {
            state = state.skip(self.nextstep - 1)?;
        }
        let (state, item) = state.into_next_result()?;
        let nextself = StepBy {
            ti: state,
            step: self.step,
            nextstep: self.step,
        };
        Ok((nextself, item))
    }
}
