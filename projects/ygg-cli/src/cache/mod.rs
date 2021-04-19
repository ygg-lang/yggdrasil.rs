use std::{lazy::SyncLazy, sync::RwLock};

pub static CACHE_MANAGER: SyncLazy<RwLock<CacheManager>> = SyncLazy::new(|| RwLock::new(CacheManager::new()));

pub struct CacheManager {}

pub struct GrammarCache {}

pub struct TypeCache {}

impl CacheManager {
    pub fn new() -> Self {
        Self {}
    }
}

fn find_target_path() {}

fn check_workspace() {}
