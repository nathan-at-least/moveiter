use crate::TerminalIterator;

pub struct FilterMap<I, F, U>
where
    I: TerminalIterator,
    F: Fn(<I as TerminalIterator>::Item) -> Option<U>,
{
    it: I,
    f: F,
}

impl<I, F, U> FilterMap<I, F, U>
where
    I: TerminalIterator,
    F: Fn(<I as TerminalIterator>::Item) -> Option<U>,
{
    pub fn new(it: I, f: F) -> Self {
        FilterMap { it, f }
    }
}

impl<I, F, U> TerminalIterator for FilterMap<I, F, U>
where
    I: TerminalIterator,
    F: Fn(<I as TerminalIterator>::Item) -> Option<U>,
{
    type Item = U;
    type Terminal = <I as TerminalIterator>::Terminal;

    fn into_next_result(self) -> Result<(Self, Self::Item), Self::Terminal> {
        let mut state = self.it;

        loop {
            let (newstate, t) = state.into_next_result()?;
            state = newstate;

            if let Some(u) = (self.f)(t) {
                let nextself = FilterMap {
                    it: state,
                    f: self.f,
                };
                return Ok((nextself, u));
            }
        }
    }
}
