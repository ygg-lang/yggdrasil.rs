// pub mod ast;
// pub mod cst;
//
pub use yggdrasil_shared::{Result, YggdrasilError};
//
// pub mod shared {
//     pub use yggdrasil_shared::*;
// }
mod ygg;

use peginator::buildscript::Compile;
use std::path::{Path, PathBuf};

#[test]
fn test() {}
