use std::{
    collections::BTreeMap,
    sync::{LazyLock, Mutex},
};
use yggdrasil_error::{FileCache, Validate, Validation, YggdrasilError};
use yggdrasil_shared::{
    optimize::{CodeOptimizer, InsertIgnore, RefineRules, RemarkTags},
    parse_grammar_raw, GrammarInfo,
};

static CACHE_MANAGER: LazyLock<Mutex<CacheManager>> = LazyLock::new(|| Mutex::new(CacheManager::default()));

#[derive(Default)]
struct CacheManager {
    grammars: BTreeMap<String, GrammarInfo>,
}

/// Proxy interface for global cache
#[derive(Default)]
pub struct GaiaSystem {
    cache: FileCache,
}

pub struct GrammarCache {
    hash: u64,
    input: GrammarInfo,
    optimized: GrammarInfo,
}

pub struct TypeCache {}

impl GaiaSystem {
    /// Insert new
    pub fn insert(&self, text: &str) -> Result<(), YggdrasilError> {
        let text = parse_grammar_raw(text)?;
        CACHE_MANAGER.lock()?.grammars.insert(text.name.text.clone(), text);
        Ok(())
    }

    pub fn get_grammar(&self, language: &str) -> Result<GrammarInfo, YggdrasilError> {
        match CACHE_MANAGER.lock()?.grammars.get(language) {
            Some(s) => Ok(s.clone()),
            None => Err(YggdrasilError::runtime_error("no grammar")),
        }
    }

    pub fn get_optimized(&self, language: &str) -> Validation<GrammarInfo> {
        let mut errors = vec![];
        let mut info = self.get_grammar(language).validate(&mut errors)?;
        info = RefineRules::default().optimize(&info).validate(&mut errors)?;
        info = InsertIgnore::default().optimize(&info).validate(&mut errors)?;
        info = RemarkTags::default().optimize(&info).validate(&mut errors)?;
        Validation::Success { value: info, diagnostics: errors }
    }
}

fn find_target_path() {}

fn check_workspace() {}
