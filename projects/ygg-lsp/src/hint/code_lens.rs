use super::*;

#[rustfmt::skip]
pub async fn code_lens_provider(params: CodeLensParams) -> Result<Option<Vec<CodeLens>>> {
    let url = params.text_document.uri;
    HINT_MANAGER.write().await.update(&url).await.ok();
    let out = match HINT_MANAGER.read().await.get(&url) {
        Some(s) => {Some(s.code_lens.to_owned())},
        None => {None}
    };
    Ok(out)
}
