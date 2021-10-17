use crate::residual::{Iteration, Iterator, Next, Residual};
use std::fmt::Debug;
use test_case::test_case;

#[derive(Debug)]
struct MyTermIt(usize);

// Mutation-style impl:
impl Iterator for MyTermIt {
    type Item = usize;
    type Residual = ();

    fn into_next(self) -> Iteration<MyTermIt, usize, ()> {
        if self.0 == 3 {
            Residual(())
        } else {
            Next(MyTermIt(self.0 + 1), self.0)
        }
    }
}

#[test_case(MyTermIt(0))] // Tests hand-coded impl.
#[test_case(0..3)] // Tests Iterator->TerminalIter blanket impl.
fn unrolled_test<TI>(ti: TI)
where
    TI: Iterator<Item = usize, Residual = ()> + Debug,
{
    let (s0, x0) = ti.into_next().unwrap_next();
    assert_eq!(0, x0);

    let (s1, x1) = s0.into_next().unwrap_next();
    assert_eq!(1, x1);

    let (s2, x2) = s1.into_next().unwrap_next();
    assert_eq!(2, x2);

    assert_eq!((), s2.into_next().unwrap_residual());
}

#[test_case(MyTermIt(0))] // Tests hand-coded impl.
#[test_case(0..3)] // Tests Iterator->TerminalIter blanket impl.
fn newyears_loop_test<TI>(mut ti: TI)
where
    TI: Iterator<Item = usize, Residual = ()> + Debug,
{
    for expected in 0..3 {
        let (nextti, x) = ti.into_next().unwrap_next();
        assert_eq!(expected, x);
        ti = nextti;
    }
    assert_eq!((), ti.into_next().unwrap_residual());
}

#[test_case(MyTermIt(0))] // Tests hand-coded impl.
#[test_case(0..3)] // Tests Iterator->TerminalIter blanket impl.
fn test_qmark_operator<TI>(ti: TI)
where
    TI: Iterator<Item = usize, Residual = ()>,
{
    fn add_first_five_elements<TI2>(mut ti: TI2) -> Iteration<TI2, usize, ()>
    where
        TI2: Iterator<Item = usize, Residual = ()>,
    {
        let mut sum = 0;

        for _ in 0..42 {
            let (newti, x) = ti.into_next()?;
            ti = newti;
            sum += x;
        }

        unreachable!("The iteration should not have completed; sum: {:?}", sum);
    }

    // This should break early because its loop is longer than the iterator:
    let r = add_first_five_elements(ti).unwrap_residual();
    assert_eq!((), r);
}
