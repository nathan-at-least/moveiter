//! Adapters wrap different kinds of iterators to provide other iterator interfaces.
//!
//! These are constructed from traits methods such as
//! [AsyncFiniteMoveIterator::into_async_terminal_move_iterator](crate::AsyncFiniteMoveIterator::into_async_terminal_move_iterator).

mod aemi_as_finite;
mod aemi_as_terminal;
mod afmi_as_terminal;
mod emi_as_async;
mod emi_as_finite;
mod emi_as_iterator;
mod emi_as_terminal;
mod fmi_as_async;
mod fmi_as_iterator;
mod fmi_as_terminal;
mod tmi_as_async;

pub use self::aemi_as_finite::AemiAsFinite;
pub use self::aemi_as_terminal::AemiAsTerminal;
pub use self::afmi_as_terminal::AfmiAsTerminal;
pub use self::emi_as_async::EmiAsAsync;
pub use self::emi_as_finite::EmiAsFinite;
pub use self::emi_as_iterator::EmiAsIterator;
pub use self::emi_as_terminal::EmiAsTerminal;
pub use self::fmi_as_async::FmiAsAsync;
pub use self::fmi_as_iterator::FmiAsIterator;
pub use self::fmi_as_terminal::FmiAsTerminal;
pub use self::tmi_as_async::TmiAsAsync;
