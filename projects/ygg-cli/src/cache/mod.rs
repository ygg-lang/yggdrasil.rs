use std::sync::{LazyLock, RwLock};

pub static CACHE_MANAGER: LazyLock<RwLock<CacheManager>> = LazyLock::new(|| RwLock::new(CacheManager::new()));

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
