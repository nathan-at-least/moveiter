use crate::TerminalIterator;

pub enum Intersperse<T>
where
    T: TerminalIterator,
    <T as TerminalIterator>::Item: Clone,
{
    Start {
        t: T,
        sep: <T as TerminalIterator>::Item,
    },
    SepThenItem {
        t: T,
        sep: <T as TerminalIterator>::Item,
        item: <T as TerminalIterator>::Item,
    },
    ItemReady {
        t: T,
        sep: <T as TerminalIterator>::Item,
        item: <T as TerminalIterator>::Item,
    },
    Done {
        term: <T as TerminalIterator>::Terminal,
    },
}

impl<T> Intersperse<T>
where
    T: TerminalIterator,
    <T as TerminalIterator>::Item: Clone,
{
    pub fn new(t: T, sep: <T as TerminalIterator>::Item) -> Self {
        Intersperse::Start { t, sep }
    }
}

impl<T> TerminalIterator for Intersperse<T>
where
    T: TerminalIterator,
    <T as TerminalIterator>::Item: Clone,
{
    type Item = <T as TerminalIterator>::Item;
    type Terminal = <T as TerminalIterator>::Terminal;

    fn into_next_result(self) -> Result<(Self, Self::Item), Self::Terminal> {
        use Intersperse::*;

        let follow_item = |t: T, sep, item| {
            let nextself = match t.into_next_result() {
                Ok((t2, item2)) => SepThenItem {
                    t: t2,
                    sep: sep,
                    item: item2,
                },
                Err(term) => Done { term },
            };

            (nextself, item)
        };

        match self {
            Start { t, sep } => {
                let (t2, item) = t.into_next_result()?;
                Ok(follow_item(t2, sep, item))
            }
            SepThenItem { t, sep, item } => {
                let nextself = ItemReady {
                    t,
                    sep: sep.clone(),
                    item,
                };
                Ok((nextself, sep))
            }
            ItemReady { t, sep, item } => Ok(follow_item(t, sep, item)),
            Done { term } => Err(term),
        }
    }
}
