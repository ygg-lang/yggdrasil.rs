use super::*;

/// Create a new grammar project
#[derive(Parser)]
pub struct CommandBuild {}

impl CommandBuild {
    pub fn run(&self, config: &YccConfig) -> Result<()> {
        let glob = Glob::new("grammars/*.ygg")?;
        for entry in glob.walk_with_behavior(current_dir()?, LinkBehavior::ReadTarget).not([".test"])? {
            match self.insert_item(entry) {
                Ok(_) => {}
                Err(_) => {}
            }
        }
        for _ in &config.build {}

        Ok(())
    }
    fn insert_item(&self, item: std::result::Result<WalkEntry, WalkError>) -> Result<()> {
        let item = item?;
        let _ = GaiaSystem::default();
        if item.path().is_file() {
            let text = read_to_string(item.path())?;
            let _ = parse_grammar_raw(&text)?;
        }
        Ok(())
    }

    pub fn build_one(&self, _name: &str) -> Result<()> {
        unimplemented!()
    }
    pub fn build_all(&self) -> Result<()> {
        unimplemented!()
    }
}
