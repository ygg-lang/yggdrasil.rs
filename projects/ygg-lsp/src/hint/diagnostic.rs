use super::*;

impl Backend {
    pub async fn diagnostics_provider(&self, url: &Url) {
        HINT_MANAGER.write().await.update(&url).await.ok();
        let diag = match HINT_MANAGER.read().await.get(url) {
            Some(s) => s.diagnostic.to_owned(),
            None => {
                vec![]
            }
        };
        self.client.publish_diagnostics(url.to_owned(), diag, None).await
    }
}
