pub struct Iteration<S, X>(pub S, pub X);

impl<S, X> Iteration<S, X> {
    pub fn map_item<F, Y>(self, f: F) -> Iteration<S, Y>
    where
        F: FnOnce(X) -> Y,
    {
        let Iteration(s, x) = self;
        Iteration(s, f(x))
    }

    pub fn map_state<F, T>(self, f: F) -> Iteration<T, X>
    where
        F: FnOnce(S) -> T,
    {
        let Iteration(s, x) = self;
        Iteration(f(s), x)
    }
}
