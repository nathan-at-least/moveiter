//! The `iterfam` crate provides a set of iterator-like traits for finer grained type constraints
//! on different types of iteration, from the most general to the most restricted:
//! `TerminalIterator`, `MoveIterator`, and `EndlessIterator`.
//!
//! All of these traits use move semantics where the iterator state is moved during iteration
//!into a newstate when producing a next item. For example, `MoveIterator` is most similar to
//! `std::iter::Iterator` with the following iteration method signature:
//!
//! ```
//! fn into_next_option(self) -> Option<(Self, Self::Item)>;
//! ```
//!
//! This approach has two notable characteristics:
//! - Implementations can use a more functional style, rather than a mutating style.
//! - Iteration termination is enforced by the type system, so the `std::iter::Iterator` issue
//!   of calling `next` on a "finished" iterator and workarounds like `Iterator::fuse`
//!   are unnecessary.
mod endless;
mod moveiter;
mod terminal;

pub use self::endless::EndlessIterator;
pub use self::moveiter::MoveIterator;
pub use self::terminal::TerminalIterator;
