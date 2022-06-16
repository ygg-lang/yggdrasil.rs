use askama::Template;

use yggdrasil_ir::GrammarInfo;

use crate::codegen::RustCodegen;

#[derive(Template)]
#[template(path = "rust/ignored.djv", escape = "none")]
struct IgnoreItems {
    items: Vec<IgnoredItem>,
}

struct IgnoredItem {
    class: String,
    consume: String,
}

impl RustCodegen {
    fn ignored_rules(&self, info: &GrammarInfo) -> IgnoreItems {
        IgnoreItems {
            items: info
                .ignores
                .iter()
                .map(|name| IgnoredItem { class: self.get_class_name(name), consume: self.get_parse_name(name) })
                .collect(),
        }
    }
}

#[test]
fn test() {
    let mut info = GrammarInfo::default();
    info.ignores.insert("Whitespace".to_string());
    info.ignores.insert("CommentLine".to_string());
    let codegen = RustCodegen::default();
    let items = codegen.ignored_rules(&info);
    let result = items.render().unwrap();
    println!("{}", result);
}
