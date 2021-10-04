use crate::TerminalIterator;

#[derive(Debug)]
struct NewyearsCountdown(usize);

// Mutation-style impl:
impl TerminalIterator for NewyearsCountdown {
    type Item = usize;
    type Terminal = &'static str;

    fn into_next_result(self) -> Result<(NewyearsCountdown, usize), &'static str> {
        if self.0 == 0 {
            Err("Happy New Year!")
        } else {
            let x = self.0 - 1;
            Ok((NewyearsCountdown(x), x))
        }
    }
}

#[test]
fn newyears_unrolled_test() {
    let c = NewyearsCountdown(3);

    let (s0, x0) = c.into_next_result().unwrap();
    assert_eq!(2, x0);

    let (s1, x1) = s0.into_next_result().unwrap();
    assert_eq!(1, x1);

    let (s2, x2) = s1.into_next_result().unwrap();
    assert_eq!(0, x2);

    assert_eq!("Happy New Year!", s2.into_next_result().unwrap_err());
}

#[test]
fn newyears_loop_test() {
    let mut c = NewyearsCountdown(3);
    for expected in (0..3).rev() {
        let (nextc, x) = c.into_next_result().unwrap();
        assert_eq!(expected, x);
        c = nextc;
    }
    assert_eq!("Happy New Year!", c.into_next_result().unwrap_err());
}
