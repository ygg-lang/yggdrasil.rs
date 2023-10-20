use std::{
    env::current_dir,
    fmt::Formatter,
    fs::{read_dir, read_to_string, DirEntry, ReadDir},
    path::{Path, PathBuf},
};

use serde::{
    de::{Error, Visitor},
    Deserialize, Deserializer, Serialize,
};
use wax::Any;

use yggdrasil_error::YggdrasilError;
use yggdrasil_shared::codegen::{BuildRailway, BuildRust};

pub use self::patterns::YccPatterns;

mod modes;
mod patterns;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct YccConfig {
    pub include: YccPatterns,
    pub build: Vec<YccBuild>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum YccBuild {
    Rail(BuildRailway),
    Rust(BuildRust),
}

impl YccConfig {
    pub fn load_path<P: AsRef<Path>>(config: Option<P>) -> Result<Self, YggdrasilError> {
        let mut file = ConfigResolver::default();
        file.resolve(config);
        let config: Self = match file.extension() {
            "json" | "json5" => json5::from_str(file.content())?,
            "toml" => toml::from_str(file.content())?,

            _ => {
                unimplemented!()
            }
        };
        println!("{:#?}", config);
        Ok(config)
    }
}

#[derive(Default)]
struct ConfigResolver {
    txt: String,
    ext: String,
}

impl ConfigResolver {
    pub fn resolve<P: AsRef<Path>>(&mut self, config: Option<P>) {
        if let Err(e) = self.try_resolve(config) {
            tracing::warn!("{}", e)
        }
    }
    fn try_resolve<P: AsRef<Path>>(&mut self, config: Option<P>) -> Result<(), YggdrasilError> {
        match config {
            Some(s) => self.user_config(s.as_ref().canonicalize()?),
            None => {
                tracing::warn!("ConfigError: 未指定配置文件, 正在查询 `Yggdrasil.json5`");
                self.default_config(read_dir(current_dir()?)?)
            }
        }
    }
    fn user_config(&mut self, path: PathBuf) -> Result<(), YggdrasilError> {
        self.txt = read_to_string(&path)?;
        match path.extension().and_then(|s| s.to_str()) {
            Some(s) => self.ext = s.to_string(),
            None => {}
        };
        Ok(())
    }

    fn default_config(&mut self, path: ReadDir) -> Result<(), YggdrasilError> {
        match find_default(path) {
            Some(s) => self.user_config(s),
            None => Ok(()),
        }
    }
}

impl ConfigResolver {
    pub fn content(&self) -> &str {
        if self.txt.is_empty() { include_str!("Default.toml") } else { &self.txt }
    }
    pub fn extension(&self) -> &str {
        if self.ext.is_empty() {
            tracing::warn!("ConfigError: 无法识别格式, 假定为 toml");
            "toml"
        }
        else {
            &self.ext
        }
    }
}

fn find_default(dir: ReadDir) -> Option<PathBuf> {
    for file in dir {
        match check_file(file) {
            Some(s) => return Some(s),
            None => {}
        }
    }
    None
}

fn check_file(input: std::io::Result<DirEntry>) -> Option<PathBuf> {
    let file = input.ok()?.path();
    if !file.is_file() {
        return None;
    }
    let name = file.file_name()?;
    if name.eq_ignore_ascii_case("ycc") || name.eq_ignore_ascii_case("yggdrasil") {
        return Some(file);
    }
    None
}
