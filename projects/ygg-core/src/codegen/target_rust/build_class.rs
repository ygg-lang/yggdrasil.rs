use std::fmt::Debug;

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
// /// `ab{1,3}c`
// fn parse_output1(state: YState) -> YResult<Output1> {
//     let start = state.start_offset;
//     let Parsed(state, a) = state.parse_char('a')?;
//     let Parsed(state, b) = state.parse_repeats(1, 3, |state| state.parse_char('b'))?;
//     let Parsed(state, c) = state.parse_char('c')?;
//     let range = start..state.start_offset;
//     Parsed::ok(state, Output1 { a, b, c, range })
// }
impl WriteRust for GrammarRule {
    fn write_rust(&self, w: &mut RustCodegen, info: &GrammarInfo) -> std::fmt::Result {
        // writeln!(w, "/// {}", info.rule_prefix)?;
        writeln!(w, "fn {}(state: YState) -> YResult<{}> {{", w.get_class_name(&self.name), self.r#type)?;
        if w.enable_position {
            writeln!(w, "    let start = state.start_offset;")?;
        }
        if w.enable_position {
            writeln!(w, "    let range = start..state.start_offset;")?;
        }
        writeln!(w, "    Parsed::ok(state, {} {{ range }})", self.r#type)?;
        writeln!(w, "}}")?;
        Ok(())
    }
    fn write_class(&self, w: &mut RustCodegen, info: &GrammarInfo) -> std::fmt::Result {
        for line in self.document.lines() {
            writeln!(w, "/// {}", line)?;
        }
        writeln!(w, "{}", self.derives)?;
        if self.public {
            w.write_str("pub ")?;
        }
        match self.union {
            true => {
                writeln!(w, "enum {} {{", self.r#type)?;
                for (variant, fields) in self.collect_union_parameters() {
                    writeln!(w, "    {} {{", variant)?;
                    for field in fields {
                        write!(w, "        {},", field)?;
                    }
                    writeln!(w, "    }},")?;
                }
                writeln!(w, "}}")?;
            }
            false => {
                writeln!(w, "struct {} {{", self.r#type)?;
                for field in self.collect_class_parameters() {
                    write!(w, "    {},", field)?;
                }
                writeln!(w, "}}")?;
            }
        }
        Ok(())
    }
}
