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
fn size_hint<TI>(ti: TI)
where
    TI: TerminalIterator<Item = usize, Terminal = ()>,
{
    assert_eq!((3, Some(3)), ti.size_hint());
}

#[test_case(MyTermIt(0))] // Tests hand-coded impl.
#[test_case(0..3)] // Tests Iterator->MoveIter blanket impl.
fn count<TI>(ti: TI)
where
    TI: TerminalIterator<Item = usize, Terminal = ()>,
{
    assert_eq!(3, ti.count());
}

#[test_case(MyTermIt(0))] // Tests hand-coded impl.
#[test_case(0..3)] // Tests Iterator->MoveIter blanket impl.
fn last<TI>(ti: TI)
where
    TI: TerminalIterator<Item = usize, Terminal = ()>,
{
    assert_eq!(Some(2), ti.last());
}

#[test_case(MyTermIt(0))] // Tests hand-coded impl.
#[test_case(0..3)] // Tests Iterator->MoveIter blanket impl.
fn nth_0<TI>(ti: TI)
where
    TI: TerminalIterator<Item = usize, Terminal = ()> + Debug,
{
    assert_eq!(Ok(0), ti.nth(0).map(|(_, x)| x));
}

#[test_case(MyTermIt(0))] // Tests hand-coded impl.
#[test_case(0..3)] // Tests Iterator->MoveIter blanket impl.
fn nth_2<TI>(ti: TI)
where
    TI: TerminalIterator<Item = usize, Terminal = ()> + Debug,
{
    assert_eq!(Ok(2), ti.nth(2).map(|(_, x)| x));
}

#[test_case(MyTermIt(0))] // Tests hand-coded impl.
#[test_case(0..3)] // Tests Iterator->MoveIter blanket impl.
fn nth_42<TI>(ti: TI)
where
    TI: TerminalIterator<Item = usize, Terminal = ()> + Debug,
{
    assert_eq!(Err(()), ti.nth(42).map(|(_, x)| x));
}

#[test_case(MyTermIt(0))] // Tests hand-coded impl.
#[test_case(0..3)] // Tests Iterator->MoveIter blanket impl.
fn step_by_2<TI>(ti: TI)
where
    TI: TerminalIterator<Item = usize, Terminal = ()> + Debug,
{
    let sb2 = ti.step_by(2);

    let (s0, x0) = sb2.into_next_result().unwrap();
    assert_eq!(x0, 0);

    let (s1, x1) = s0.into_next_result().unwrap();
    assert_eq!(x1, 2);

    let res = s1.into_next_result();
    assert!(res.is_err());
}

#[test_case(MyTermIt(0))] // Tests hand-coded impl.
#[test_case(0..3)] // Tests Iterator->MoveIter blanket impl.
fn map_term_and_chain<TI>(ti: TI)
where
    TI: TerminalIterator<Item = usize, Terminal = ()> + Debug,
{
    let termed = ti.map_term(|()| 42);
    let other = (3..5).map_term(|()| "foo");
    let chained = termed.chain(other);

    let mut sum = 0;
    let (ta, tb) = chained.for_each(|x| {
        sum += x;
        None
    });

    assert_eq!(ta, 42);
    assert_eq!(tb, "foo");
    assert_eq!(sum, 10);
}

#[test_case(MyTermIt(0))] // Tests hand-coded impl.
#[test_case(0..3)] // Tests Iterator->MoveIter blanket impl.
fn map_term_and_zip_complete<TI>(ti: TI)
where
    TI: TerminalIterator<Item = usize, Terminal = ()> + Debug,
{
    let termed = ti.map_term(|()| 42);
    let other = (10..20).map_term(|()| "foo");
    let zipped = termed.zip(other);

    let mut sum = 0;
    let zipterm = zipped.for_each(|(a, b)| {
        sum += a * b;
        None
    });
    let (ta, tb) = zipterm.complete();

    assert_eq!(ta, 42);
    assert_eq!(tb, "foo");
    assert_eq!(sum, 0 * 10 + 1 * 11 + 2 * 12);
}

#[test_case(MyTermIt(0))] // Tests hand-coded impl.
#[test_case(0..3)] // Tests Iterator->MoveIter blanket impl.
fn map_term_and_zip_then_continue<TI>(ti: TI)
where
    TI: TerminalIterator<Item = usize, Terminal = ()> + Debug,
{
    use crate::terminal::ZipTerminal::LeftTerm;

    let termed = ti.map_term(|()| 42);
    let other = (10..16).map_term(|()| "foo");
    let zipped = termed.zip(other);

    let mut sum = 0;
    let zipterm = zipped.for_each(|(a, b)| {
        sum += a * b;
        None
    });
    assert_eq!(sum, 0 * 10 + 1 * 11 + 2 * 12);

    match zipterm {
        LeftTerm(aterm, (bstate, bitem)) => {
            assert_eq!(aterm, 42);
            assert_eq!(bitem, 13);

            let mut sum2 = 0;
            let bterm = bstate.for_each(|x| {
                sum2 += x;
                None
            });
            assert_eq!(sum2, 14 + 15);
            assert_eq!(bterm, "foo");
        }
        _ => {
            panic!("zipped terminated incorrectly.");
        }
    }
}

#[test]
fn intersperse() {
    let mut sum = 0;
    MyTermIt(0).intersperse(5).for_each(|x| {
        sum += x;
        None
    });
    assert_eq!(sum, 13);
}
