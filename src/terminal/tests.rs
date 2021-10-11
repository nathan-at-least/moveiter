use crate::TerminalIterator;
use std::fmt::Debug;
use test_case::test_case;

#[derive(Debug)]
struct MyTermIt(usize);

// Mutation-style impl:
impl TerminalIterator for MyTermIt {
    type Item = usize;
    type Terminal = ();

    fn into_next_result(self) -> Result<(MyTermIt, usize), ()> {
        if self.0 == 3 {
            Err(())
        } else {
            Ok((MyTermIt(self.0 + 1), self.0))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (3, Some(3))
    }
}

#[test_case(MyTermIt(0))] // Tests hand-coded impl.
#[test_case(0..3)] // Tests Iterator->MoveIter blanket impl.
fn unrolled_test<TI>(ti: TI)
where
    TI: TerminalIterator<Item = usize, Terminal = ()> + Debug,
{
    let (s0, x0) = ti.into_next_result().unwrap();
    assert_eq!(0, x0);

    let (s1, x1) = s0.into_next_result().unwrap();
    assert_eq!(1, x1);

    let (s2, x2) = s1.into_next_result().unwrap();
    assert_eq!(2, x2);

    assert_eq!((), s2.into_next_result().unwrap_err());
}

#[test_case(MyTermIt(0))] // Tests hand-coded impl.
#[test_case(0..3)] // Tests Iterator->MoveIter blanket impl.
fn newyears_loop_test<TI>(mut ti: TI)
where
    TI: TerminalIterator<Item = usize, Terminal = ()> + Debug,
{
    for expected in 0..3 {
        let (nextti, x) = ti.into_next_result().unwrap();
        assert_eq!(expected, x);
        ti = nextti;
    }
    assert_eq!((), ti.into_next_result().unwrap_err());
}

#[test_case(MyTermIt(0))] // Tests hand-coded impl.
#[test_case(0..3)] // Tests Iterator->MoveIter blanket impl.
fn size_hint<TI>(mi: TI)
where
    TI: TerminalIterator<Item = usize, Terminal = ()>,
{
    assert_eq!((3, Some(3)), mi.size_hint());
}

#[test_case(MyTermIt(0))] // Tests hand-coded impl.
#[test_case(0..3)] // Tests Iterator->MoveIter blanket impl.
fn count<TI>(mi: TI)
where
    TI: TerminalIterator<Item = usize, Terminal = ()>,
{
    assert_eq!(3, mi.count());
}
