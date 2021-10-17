use crate::ResidualIterator;
use std::fmt::Debug;
use test_case::test_case;

#[derive(Debug)]
struct MyTermIt(usize);

// Mutation-style impl:
impl ResidualIterator for MyTermIt {
    type Item = usize;
    type Residual = ();

    fn into_next_result(self) -> Result<(MyTermIt, usize), ()> {
        if self.0 == 3 {
            Err(())
        } else {
            Ok((MyTermIt(self.0 + 1), self.0))
        }
    }
}

#[test_case(MyTermIt(0))] // Tests hand-coded impl.
#[test_case(0..3)] // Tests Iterator->MoveIter blanket impl.
fn unrolled_test<TI>(ti: TI)
where
    TI: ResidualIterator<Item = usize, Residual = ()> + Debug,
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
    TI: ResidualIterator<Item = usize, Residual = ()> + Debug,
{
    for expected in 0..3 {
        let (nextti, x) = ti.into_next_result().unwrap();
        assert_eq!(expected, x);
        ti = nextti;
    }
    assert_eq!((), ti.into_next_result().unwrap_err());
}
