use super::*;

impl Backend {
    pub async fn check_the_file(&self, url: &Url) {
        let mut store = GRAMMAR_MANAGER.write().await;
        match store.collect_diagnostic(url) {
            Ok(diag) => self.client.publish_diagnostics(url.to_owned(), diag, None).await,
            Err(_) => (),
        }
    }
}
