use crate::MoveIterator;

#[derive(Debug)]
struct Countdown(usize);

// Mutation-style impl:
impl MoveIterator for Countdown {
    type Item = usize;

    fn into_next_option(self) -> Option<(Countdown, usize)> {
        if self.0 == 0 {
            None
        } else {
            let x = self.0 - 1;
            Some((Countdown(x), x))
        }
    }
}

#[test]
fn countdown_unrolled_test() {
    let c = Countdown(3);

    let (s0, x0) = c.into_next_option().unwrap();
    assert_eq!(2, x0);

    let (s1, x1) = s0.into_next_option().unwrap();
    assert_eq!(1, x1);

    let (s2, x2) = s1.into_next_option().unwrap();
    assert_eq!(0, x2);

    assert!(s2.into_next_option().is_none());
}

#[test]
fn countdown_loop_test() {
    let mut c = Countdown(3);
    for expected in (0..3).rev() {
        let (nextc, x) = c.into_next_option().unwrap();
        assert_eq!(expected, x);
        c = nextc;
    }

    assert!(c.into_next_option().is_none());
}
