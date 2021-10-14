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
        let StepBy { ti, step, nextstep } = self;
        let mut state = ti;
        if nextstep > 0 {
            state = state.skip(nextstep - 1)?;
        }
        state.map_state(|ti| StepBy {
            ti,
            step,
            nextstep: step,
        })
    }
}
