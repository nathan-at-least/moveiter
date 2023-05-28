#![doc = include_str!("../README.md")]
mod asyn;
pub(crate) mod optutil;
mod syn;

pub mod adapters;

pub use self::asyn::{
    AsyncEndlessMoveIterator, AsyncFiniteMoveIterator, AsyncTerminalMoveIterator,
};

pub use self::syn::{
    terminal_move_iterator_from_result_iterator, terminal_move_iterator_into_result_iterator,
    EndlessMoveIterator, FiniteMoveIterator, TerminalMoveIterator,
};
