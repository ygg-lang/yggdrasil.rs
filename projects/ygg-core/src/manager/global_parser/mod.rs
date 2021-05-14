use crate::ast::YGGBuilder;
use std::lazy::SyncLazy;
use tokio::sync::RwLock;

#[rustfmt::skip]
pub static PARSER_MANAGER: SyncLazy<RwLock<YGGBuilder>> = SyncLazy::new(|| {
    RwLock::new(YGGBuilder::new().expect("Parser initialization failed"))
});
