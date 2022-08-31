//! A family of iteration traits which use move semantics.
//!
//! Move semantics consume and iterator, and if there is a next item, a new iterator is returned
//! with the item, whereas when iteration completes, no new iterator is available. This prevents
//! incorrect states during type checking avoiding the need for runtime safety mechanisms like
//! [std::iter::FusedIterator].
//!
//! There are two axes defining the trait family: sync/async and the kind of
//! termination. Termination can be "endless", "finite", or "terminal", where "terminal" produces
//! a typed value as the final step. This leads to six traits.
//!
//! This allows implementors and consumers to select the appropriate trait with fine-grained
//! semantics.
//!
//! # At a Glance
//!
//! Each trait is based on a single provided `into_next` method:
//!
//! | Trait                       | Method |
//! |-----------------------------|---------------------------------------------------------------------------|
//! | [EndlessMoveIterator]       | `fn into_next(self) -> (Self, Self::Item);`                               |
//! | [FiniteMoveIterator]        | `fn into_next(self) -> Option<(Self, Self::Item)>;`                       |
//! | [TerminalMoveIterator]      | `fn into_next(self) -> Either<(Self, Self::Item), Self::Terminal>;`       |
//! | [AsyncEndlessMoveIterator]  | `async fn into_next(self) -> (Self, Self::Item);`                         |
//! | [AsyncFiniteMoveIterator]   | `async fn into_next(self) -> Option<(Self, Self::Item)>;`                 |
//! | [AsyncTerminalMoveIterator] | `async fn into_next(self) -> Either<(Self, Self::Item), Self::Terminal>;` |
//!
//! # Blanket Implementations
//!
//! There are blanket impls of any [std::iter::Iterator] for any of the non-endless traits:
//!
//! - [FiniteMoveIterator]
//! - [TerminalMoveIterator]
//! - [AsyncFiniteMoveIterator]
//! - [AsyncTerminalMoveIterator]
//!
//! This allows any interfaces that accept these generic bounds to take a [std::iter::Iterator]
//! value seamlessly.
//!
//! # Adaptation
//!
//! Several of the traits provide default methods to adapt them to provide a related trait via a
//! newtype wrapper. This helps consuming code and implementations to each tailor their semantics
//! precisely yet still interoperate, where the result has coherent semantics:
//!
//! | Implemented Trait | Adaptation Method | Resulting Interface |
//! |-------------------|-------------------|---------------------|
//! | [EndlessMoveIterator]      | [into_async](EndlessMoveIterator::into_async)                                                    | [AsyncEndlessMoveIterator]  |
//! |                            | [into_finite_move_iterator](EndlessMoveIterator::into_finite_move_iterator)                      | [FiniteMoveIterator]        |
//! |                            | [into_terminal_move_iterator](EndlessMoveIterator::into_terminal_move_iterator)                  | [TerminalMoveIterator]      |
//! | [FiniteMoveIterator]       | [into_async](FiniteMoveIterator::into_async)                                                     | [AsyncFiniteMoveIterator]   |
//! |                            | [into_terminal_move_iterator](FiniteMoveIterator::into_terminal_move_iterator)                   | [TerminalMoveIterator]      |
//! | [TerminalMoveIterator]     | [into_async](TerminalMoveIterator::into_async)                                                   | [AsyncTerminalMoveIterator] |
//! | [AsyncEndlessMoveIterator] | [into_async_finite_move_iterator](AsyncEndlessMoveIterator::into_async_finite_move_iterator)     | [AsyncFiniteMoveIterator]   |
//! |                            | [into_async_terminal_move_iterator](AsyncEndlessMoveIterator::into_async_terminal_move_iterator) | [AsyncTerminalMoveIterator] |
//! | [AsyncFiniteMoveIterator]  | [into_async_terminal_move_iterator](AsyncFiniteMoveIterator::into_async_terminal_move_iterator)  | [AsyncTerminalMoveIterator] |
//!
//! Adaptation with [TerminalMoveIterator::into_async] which propagates `Terminal` type, whereas
//! any other adaptation into a [TerminalMoveIterator] uses `()` as the `Terminal` type.
mod asyn;
pub(crate) mod optutil;
mod syn;

pub mod adapters;

pub use self::asyn::{
    AsyncEndlessMoveIterator, AsyncFiniteMoveIterator, AsyncTerminalMoveIterator,
};

pub use self::syn::{EndlessMoveIterator, FiniteMoveIterator, TerminalMoveIterator};
