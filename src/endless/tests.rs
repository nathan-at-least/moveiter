use crate::EndlessIterator;
use test_case::test_case;

#[derive(Debug)]
struct MutCounter(usize);

// Mutation-style impl:
impl EndlessIterator for MutCounter {
    type Item = usize;

    fn into_next(mut self) -> (MutCounter, usize) {
        let item = self.0;
        self.0 += 1;
        (self, item)
    }
}

#[derive(Debug)]
struct FuncCounter(usize);

// Functional-style impl:
impl EndlessIterator for FuncCounter {
    type Item = usize;

    fn into_next(self) -> (FuncCounter, usize) {
        (FuncCounter(self.0 + 1), self.0)
    }
}

#[test_case(MutCounter(0))]
#[test_case(FuncCounter(0))]
fn counter_unrolled_test<C>(c: C)
where
    C: EndlessIterator<Item = usize>,
{
    let (s0, x0) = c.into_next();
    assert_eq!(0, x0);

    let (s1, x1) = s0.into_next();
    assert_eq!(1, x1);

    let (_, x2) = s1.into_next();
    assert_eq!(2, x2);
}

#[test_case(MutCounter(0))]
#[test_case(FuncCounter(0))]
fn counter_loop_test<C>(mut c: C)
where
    C: EndlessIterator<Item = usize>,
{
    for expected in 0..1042 {
        let (nextc, x) = c.into_next();
        assert_eq!(expected, x);
        c = nextc;
    }
}