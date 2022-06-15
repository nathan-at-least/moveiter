//! A family of iteration traits which use move semantics.
//!
//! Move semantics consume and iterator, and if there is a next item, a new iterator is returned
//! with the item, whereas when iteration completes, no new iterator is available. This prevents
//! incorrect states during type checking avoiding the need for runtime safety mechanisms like
//! [std::iter::FusedIterator].
//!
//! The family covers sync/async as well as two different termination conditions: explicit terminal values or endless iteration. These two orthogonal axes result in four traits: `TerminalMoveIterator`, `EndlessMoveIterator`, [AsyncTerminalMoveIterator], and `AsyncEndlessMoveIterator`. This allows producer and consumer code to have fine-grained type safety around the iteration semantics.
mod asyn;

pub use self::asyn::AsyncTerminalMoveIterator;
