# `moveiter` crate

A family of traits similar to `std::iter::Iterator` which use move semantics rather than mutation.

## TODO / Ideas

- Replace `TerminalIterator`'s `Result` with a custom type that implements `Try` for termination ergonomics.
- Replace that `Result` with `Either` which reads clearer in types but may have worse ergonomics.
- Implement all of the `std::iter::Iterator` analog methods.
- Implement async variations of all sync `*Iterator` traits.
