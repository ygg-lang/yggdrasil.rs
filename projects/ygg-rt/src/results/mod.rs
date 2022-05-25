#[derive(Debug, Clone)]
pub struct YError {}

impl YError {
    pub fn excepted_character(c: char) -> Self {
        Self {}
    }
    pub fn excepted_character_range(s: char, e: char) -> Self {
        Self {}
    }
    pub fn excepted_string(s: &'static str) -> Self {
        Self {}
    }
    pub fn uninitialized(s: &'static str) -> Self {
        Self {}
    }
}
