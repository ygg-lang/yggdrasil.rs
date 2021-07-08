use std::lazy::SyncLazy;
use tokio::sync::RwLock;
use yggdrasil_bootstrap::ast::ASTBuilder;

#[rustfmt::skip]
pub static PARSER_MANAGER: SyncLazy<RwLock<ASTBuilder>> = SyncLazy::new(|| {
    RwLock::new(ASTBuilder::default())
});
