
use super::*;

impl<'i> GrammarContext<'i> {
    pub(super) fn read_import(&mut self, call: ImportStatement) {
        let url = match Self::resolve_url(self.url, &call.path.data) {
            Ok(o) => {o}
            Err(_) => {
                // TODO: report error
                return;
            }
        };
        if let Some((r, _)) = self.import.get(&url) {
            self.must_not_duplicate("Import", format!("{:?} already imported", call.path), call.range, *r)
        }
        self.import.insert(url, (call.range, call.symbol_alias));
    }
    fn resolve_url(url: &Url, path: &str) -> Result<Url> {
        match path {
            path if path.starts_with(">") => {
               Ok(WORKSPACE_ROOT.read()?.join(&path[1..path.len()])?)
            }
            path if path.starts_with("@") => {
                Ok(GLOBAL_ROOT.read()?.join(&path[1..path.len()])?)
            }
            _ => {
                Ok(url.clone())
            }
        }
    }
}
