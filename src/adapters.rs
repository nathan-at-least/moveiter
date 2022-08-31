//! Adapters wrap different kinds of iterators to provide other iterator interfaces.
//!
//! These are constructed from traits methods such as
//! [AsyncFiniteMoveIterator::into_async_terminal_move_iterator](crate::AsyncFiniteMoveIterator::into_async_terminal_move_iterator).

mod aemi_as_finite;
mod aemi_as_terminal;
mod afmi_as_terminal;

pub use self::aemi_as_finite::AemiAsFinite;
pub use self::aemi_as_terminal::AemiAsTerminal;
pub use self::afmi_as_terminal::AfmiAsTerminal;
