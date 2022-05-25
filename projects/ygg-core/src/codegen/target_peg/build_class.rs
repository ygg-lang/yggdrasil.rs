use super::*;

#[test]
fn test() {
    let mut codegen = RustCodegen::default();
    let info = GrammarInfo::default();
    let a = ExpressionNode::character('a').tag("a");
    let b = ExpressionNode::character('b').tag("b");
    let c = ExpressionNode::character('c').tag("c");
    let rule = GrammarRule {
        name: "Test".to_string(),
        r#type: "TestNode".to_string(),
        document: "aa\nbb\ncccc".to_string(),
        public: true,
        derives: Default::default(),
        auto_inline: false,
        auto_boxed: false,
        entry: false,
        union: false,
        force_export: false,
        body: a | b | c,
        range: Default::default(),
    };
    rule.write_class(&mut codegen, &info)?;
    println!("{}", codegen.buffer);
}

impl WriteRust for GrammarRule {
    fn write_rust(&self, w: &mut RustCodegen, info: &GrammarInfo) -> std::fmt::Result {
        match self.r#type.as_str() {
            "str" | "string" => w.write_str("@position @string\n")?,
            "char" | "character" => w.write_str("@char\n")?,
            _ => w.write_str("@position\n")?,
        }
        write!(w, "{}{}{} = ", info.rule_prefix, self.name, info.rule_suffix)?;
        self.body.write_rust(w, info)?;
        w.semicolon();

        // 'a' 'b' 'c'

        Ok(())
    }
    fn write_class(&self, w: &mut RustCodegen, info: &GrammarInfo) -> std::fmt::Result {
        for line in self.document.lines() {
            writeln!(w, "/// {}", line)?;
        }
        for derive in &self.derives {
            writeln!(w, "#[derive({})]", derive)?;
        }
        if self.public {
            w.write_str("pub ")?;
        }
        match self.union {
            true => {
                writeln!(w, "enum {} {{", self.r#type)?;
            }
            false => {
                writeln!(w, "struct {} {{", self.r#type)?;
            }
        }
        Ok(())
    }
}
