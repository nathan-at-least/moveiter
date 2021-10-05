use crate::MoveIterator;

/// A wrapper for any [`MoveIterator`] type which provides `std::iter::Iterator`.
#[derive(Debug)]
pub struct MoveStdIter<MI>(Option<MI>)
where
    MI: MoveIterator;

impl<MI> From<MI> for MoveStdIter<MI>
where
    MI: MoveIterator,
{
    fn from(mi: MI) -> Self {
        MoveStdIter(Some(mi))
    }
}

impl<MI> Iterator for MoveStdIter<MI>
where
    MI: MoveIterator,
{
    type Item = <MI as MoveIterator>::Item;

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
