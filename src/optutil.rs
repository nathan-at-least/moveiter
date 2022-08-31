pub(crate) trait OptionUpdate<S> {
    fn update_in_place<F, R>(&mut self, f: F) -> Option<R>
    where
        F: FnOnce(S) -> Option<(S, R)>;
}

impl<S> OptionUpdate<S> for Option<S> {
    fn update_in_place<F, R>(&mut self, f: F) -> Option<R>
    where
        F: FnOnce(S) -> Option<(S, R)>,
    {
        if let Some((s, r)) = self.take().and_then(f) {
            *self = Some(s);
            Some(r)
        } else {
            None
        }
    }
}
