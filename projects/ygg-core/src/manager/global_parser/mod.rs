use yggdrasil_bootstrap::ast::ASTBuilder;
use std::lazy::SyncLazy;
use tokio::sync::RwLock;

#[rustfmt::skip]
pub static PARSER_MANAGER: SyncLazy<RwLock<ASTBuilder>> = SyncLazy::new(|| {
    RwLock::new(ASTBuilder::default())
});
