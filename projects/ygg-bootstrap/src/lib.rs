//
pub use yggdrasil_shared::{Result, YggdrasilError};

pub use self::as_peg::as_peg;

// pub mod ast;
mod as_peg;
//
// pub mod shared {
//     pub use yggdrasil_shared::*;
// }
//noinspection DuplicatedCode
pub mod ast;
