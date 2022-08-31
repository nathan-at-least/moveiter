//! The synchronous family of move iterator traits.

mod endless;
mod finite;
// mod terminal;

pub use self::endless::EndlessMoveIterator;
pub use self::finite::FiniteMoveIterator;
// pub use self::terminal::TerminalMoveIterator;
