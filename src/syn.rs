//! The synchronous family of move iterator traits.

mod endless;
mod finite;
mod terminal;

pub use self::endless::EndlessMoveIterator;
pub use self::finite::FiniteMoveIterator;
pub use self::terminal::{terminal_move_iterator_from_result_iterator, TerminalMoveIterator};
