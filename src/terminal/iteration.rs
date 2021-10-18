use std::ops::{ControlFlow, FromResidual, Try};

pub enum Iteration<S, X> {
    Next(S, X),
    Terminal,
}

pub use Iteration::{Next, Terminal};

impl<S, X> Iteration<S, X> {
    pub fn unwrap_next(self) -> (S, X) {
        match self {
            Next(s, x) => (s, x),
            _ => panic!("terminal::Iteration::Terminal...).unwrap_next()"),
        }
    }

    pub fn unwrap_terminal(self) {
        match self {
            Terminal => {}
            _ => panic!("terminal::Iteration::Next(...).unwrap_residual()"),
        }
    }

    pub fn map<F, Y>(self, f: F) -> Iteration<S, Y>
    where
        F: FnOnce(X) -> Y,
    {
        match self {
            Next(s, x) => Next(s, f(x)),
            Terminal => Terminal,
        }
    }
}

impl<S, X> Try for Iteration<S, X> {
    type Output = (S, X);
    type Residual = ();

    fn from_output((s, x): (S, X)) -> Self {
        Next(s, x)
    }

    fn branch(self) -> ControlFlow<<Self as Try>::Residual, Self::Output> {
        match self {
            Terminal => ControlFlow::Break(()),
            Next(s, x) => ControlFlow::Continue((s, x)),
        }
    }
}

impl<S, X> FromResidual for Iteration<S, X> {
    fn from_residual(_residual: ()) -> Self {
        Terminal
    }
}
