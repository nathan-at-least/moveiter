use crate::terminal;

/// A wrapper for any [`terminal::Iterator`] type which provides `std::iter::Iterator`.
#[derive(Debug)]
pub struct TerminalStdIter<MI>(Option<MI>)
where
    MI: terminal::Iterator;

impl<MI> From<MI> for TerminalStdIter<MI>
where
    MI: terminal::Iterator,
{
    fn from(mi: MI) -> Self {
        TerminalStdIter(Some(mi))
    }
}

impl<MI> Iterator for TerminalStdIter<MI>
where
    MI: terminal::Iterator,
{
    type Item = <MI as terminal::Iterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.take() {
            None => None,
            Some(mi) => match mi.into_next_option() {
                None => None,
                Some((newstate, item)) => {
                    self.0 = Some(newstate);
                    Some(item)
                }
            },
        }
    }
}
