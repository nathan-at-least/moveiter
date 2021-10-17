use std::ops::{ControlFlow, FromResidual, Try};

pub enum Iteration<S, X, R> {
    Next(S, X),
    Residual(R),
}

pub use Iteration::{Next, Residual};

impl<S, X, R> Iteration<S, X, R> {
    pub fn unwrap_next(self) -> (S, X) {
        match self {
            Next(s, x) => (s, x),
            _ => panic!("Iteration::Residual(...).unwrap_next()"),
        }
    }

    pub fn unwrap_residual(self) -> R {
        match self {
            Residual(r) => r,
            _ => panic!("Iteration::Next(...).unwrap_residual()"),
        }
    }
}

impl<S, X, R> Try for Iteration<S, X, R> {
    type Output = (S, X);
    type Residual = R;

    fn from_output((s, x): (S, X)) -> Self {
        Next(s, x)
    }

    fn branch(self) -> ControlFlow<<Self as Try>::Residual, Self::Output> {
        match self {
            Residual(r) => ControlFlow::Break(r),
            Next(s, x) => ControlFlow::Continue((s, x)),
        }
    }
}

impl<S, X, R> FromResidual for Iteration<S, X, R> {
    fn from_residual(residual: R) -> Self {
        Iteration::Residual(residual)
    }
}
