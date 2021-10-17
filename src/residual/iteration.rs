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
