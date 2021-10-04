use crate::MoveIterator;

/// Any `MoveIterator` type can be wrapped by a `StdIteratorAdapter` to provide
/// `std::iter::Iterator` support.
#[derive(Debug)]
pub struct StdIteratorAdapter<MI>(Option<MI>)
where
    MI: MoveIterator;

impl<MI> From<MI> for StdIteratorAdapter<MI>
where
    MI: MoveIterator,
{
    fn from(mi: MI) -> Self {
        StdIteratorAdapter(Some(mi))
    }
}

impl<MI> Iterator for StdIteratorAdapter<MI>
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
