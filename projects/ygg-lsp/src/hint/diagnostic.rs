
use super::*;

impl Backend {
    pub async fn check_the_file(&self, url: &Url) -> Result<()> {
        let mut store = GRAMMAR_MANAGER.write().await;
        self.client.publish_diagnostics(url.to_owned(), store.collect_diagnostic(url).map_err(|_| Error::internal_error())?, None).await;
        Ok(())
    }
}