use crate::terminal;
use test_case::test_case;

#[derive(Debug)]
struct NToThree(usize);

// Mutation-style impl:
impl terminal::Iterator for NToThree {
    type Item = usize;

    fn into_next_option(self) -> Option<(NToThree, usize)> {
        if self.0 == 3 {
            None
        } else {
            Some((NToThree(self.0 + 1), self.0))
        }
    }
}

#[test_case(NToThree(0))] // Tests hand-coded impl.
#[test_case(0..3)] // Tests Iterator blanket impl.
fn zero_to_three_unrolled_test<MI>(mi: MI)
where
    MI: terminal::Iterator<Item = usize>,
{
    let (s0, x0) = mi.into_next_option().unwrap();
    assert_eq!(0, x0);

    let (s1, x1) = s0.into_next_option().unwrap();
    assert_eq!(1, x1);

    let (s2, x2) = s1.into_next_option().unwrap();
    assert_eq!(2, x2);

    assert!(s2.into_next_option().is_none());
}

#[test_case(NToThree(0))] // Tests hand-coded impl.
#[test_case(0..3)] // Tests Iterator blanket impl.
fn zero_to_three_loop_test<MI>(mut mi: MI)
where
    MI: terminal::Iterator<Item = usize>,
{
    for expected in 0..3 {
        let (nextmi, x) = mi.into_next_option().unwrap();
        assert_eq!(expected, x);
        mi = nextmi;
    }

    assert!(mi.into_next_option().is_none());
}

#[test_case(NToThree(0))] // Tests hand-coded impl.
#[test_case(0..3)] // Tests Iterator blanket impl.
fn into_iter_test_loop<MI>(mi: MI)
where
    MI: terminal::Iterator<Item = usize>,
{
    for (expected, actual) in (0..3).zip(mi.into_iter()) {
        assert_eq!(expected, actual);
    }
}
