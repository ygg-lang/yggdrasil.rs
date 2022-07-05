use std::sync::LazyLock;

pub static mut TEXT_MANAGER: LazyLock<TextManager> = LazyLock::new(|| TextManager::default());

pub struct TextManager {}

impl Default for TextManager {
    fn default() -> Self {
        Self {}
    }
}
