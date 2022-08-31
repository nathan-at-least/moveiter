//! A family of iteration traits which use move semantics.
//!
//! Move semantics consume and iterator, and if there is a next item, a new iterator is returned
//! with the item, whereas when iteration completes, no new iterator is available. This prevents
//! incorrect states during type checking avoiding the need for runtime safety mechanisms like
//! [std::iter::FusedIterator].
mod asyn;
mod syn;

pub mod adapters;

pub use self::asyn::{
    AsyncEndlessMoveIterator, AsyncFiniteMoveIterator, AsyncTerminalMoveIterator,
};

pub use self::syn::EndlessMoveIterator;
