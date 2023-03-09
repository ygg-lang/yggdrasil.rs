use super::*;

impl<'i> RustWriteReadme<'i> {
    pub fn railway_svg(&self) -> String {
        self.railroad.to_string()
    }
    pub fn railway_min(&self) -> String {
        let mut html_minifier = html_minifier::HTMLMinifier::new();
        html_minifier.digest(self.railway_svg()).ok();
        unsafe { String::from_utf8_unchecked(html_minifier.get_html().to_vec()) }
    }
}
