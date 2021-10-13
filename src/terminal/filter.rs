use crate::TerminalIterator;

pub struct Filter<I, F>
where
    I: TerminalIterator,
    F: FnMut(&<I as TerminalIterator>::Item) -> bool,
{
    it: I,
    f: F,
}

impl<I, F> Filter<I, F>
where
    I: TerminalIterator,
    F: FnMut(&<I as TerminalIterator>::Item) -> bool,
{
    pub fn new(it: I, f: F) -> Self {
        Filter { it, f }
    }
}

impl<I, F> TerminalIterator for Filter<I, F>
where
    I: TerminalIterator,
    F: FnMut(&<I as TerminalIterator>::Item) -> bool,
{
    type Item = <I as TerminalIterator>::Item;
    type Terminal = <I as TerminalIterator>::Terminal;

    fn into_next_result(mut self) -> Result<(Self, Self::Item), Self::Terminal> {
        let mut state = self.it;

        loop {
            let (newstate, t) = state.into_next_result()?;
            state = newstate;

            if (self.f)(&t) {
                let nextself = Filter {
                    it: state,
                    f: self.f,
                };
                return Ok((nextself, t));
            }
        }
    }
}
