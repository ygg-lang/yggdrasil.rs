use super::*;

impl Backend {
    pub async fn diagnostics_provider(&self, url: &Url) {
        HINT_MANAGER.update(&url).await.ok();
        let diag = match HINT_MANAGER.get(url) {
            Some(s) => s.diagnostic.to_owned(),
            None => vec![],
        };
        self.client.publish_diagnostics(url.to_owned(), diag, None).await
    }
}
