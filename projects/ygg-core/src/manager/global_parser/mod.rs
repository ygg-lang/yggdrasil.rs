use std::lazy::SyncLazy;
use tokio::sync::RwLock;
use yggdrasil_bootstrap::ast::YggParser;

#[rustfmt::skip]
pub static PARSER_MANAGER: SyncLazy<RwLock<YggParser>> = SyncLazy::new(|| {
    RwLock::new(YggParser::default())
});
