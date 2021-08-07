use std::lazy::SyncLazy;
use lsp_types::Url;
//use tokio::sync::RwLock;
use std::sync::RwLock;
use yggdrasil_bootstrap::ast::YggParser;

#[rustfmt::skip]
pub static PARSER_MANAGER: SyncLazy<RwLock<YggParser>> = SyncLazy::new(||
    RwLock::new(YggParser::default())
);

#[rustfmt::skip]
pub static WORKSPACE_ROOT: SyncLazy<RwLock<Url>> = SyncLazy::new(||
    RwLock::new(Url::parse("file://example.net").unwrap())
);

#[rustfmt::skip]
pub static GLOBAL_ROOT: SyncLazy<RwLock<Url>> = SyncLazy::new(||
    RwLock::new(Url::parse("file://example.net").unwrap())
);