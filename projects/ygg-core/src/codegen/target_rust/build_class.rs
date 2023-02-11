use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::{fmt::Debug, ops::Range};
use yggdrasil_ir::{
    grammar::GrammarInfo,
    nodes::ExpressionNode,
    rule::{GrammarRule, GrammarRuleKind, RuleDerive, YggdrasilIdentifier},
    traits::FieldMap,
};

use super::*;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct YggdrasilNamespace {
    #[cfg(debug_assertions)]
    pub snippet: String,
    pub identifiers: Vec<YggdrasilIdentifier>,
    pub range: Range<usize>,
}

pub struct ClassObject {
    name: String,
    derives: RuleDerive,
    file: FieldMap,
}

impl ToTokens for ClassObject {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        let derives = &self.derives;
        let file = &self.file;
    }
}

#[test]
fn test() {
    let mut codegen = RustCodegen::default();
    let info = GrammarInfo::default();
    let a = ExpressionNode::character('a').with_tag("a");
    let b = ExpressionNode::character('b').with_tag("b");
    let c = ExpressionNode::character('c').with_tag("c");
    let rule = GrammarRule {
        name: YggdrasilIdentifier { name: "Test".to_string(), span: Default::default() },
        r#type: "TestNode".to_string(),
        document: "aa\nbb\ncccc".to_string(),
        public: true,
        derives: Default::default(),
        auto_inline: false,
        auto_boxed: false,
        entry: false,
        kind: GrammarRuleKind::Class,
        body: a | b | c,
        range: Default::default(),
    };

    rule.write_class(&mut codegen, &info).unwrap();
    rule.write_rust(&mut codegen, &info).unwrap();
    println!("{}", codegen.buffer);
}
// /// `ab{1,3}c`
// fn parse_output1(state: YState) -> YResult<Output1> {
//     let start = state.start_offset;
//     let (state, a) = state.parse_char('a')?;
//     let (state, b) = state.parse_repeats(1, 3, |state| state.parse_char('b'))?;
//     let (state, c) = state.parse_char('c')?;
//     let range = start..state.start_offset;
//     Parsed::ok(state, Output1 { a, b, c, range })
// }
impl WriteRust for GrammarRule {
    fn write_rust(&self, w: &mut RustCodegen, info: &GrammarInfo) -> std::fmt::Result {
        // writeln!(w, "/// {}", info.rule_prefix)?;
        writeln!(w, "fn {}(state: YState) -> YResult<{}> {{", w.consume_name(&self.name.name), self.r#type)?;
        if w.enable_position {
            writeln!(w, "    let start = state.start_offset;")?;
        }
        if self.body.is_choice() {
            println!("body: {:#?}", self.collect_class_parameters());
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
        self.derives.write_rust(w, info)?;
        if self.public {
            w.write_str("pub ")?;
        }
        let name = w.node_name(&self.name.name);
        match &self.kind {
            GrammarRuleKind::Class => {
                writeln!(w, "struct {} {{", name)?;
                for field in self.collect_class_parameters() {
                    write!(w, "    {},", field)?;
                }
                writeln!(w, "}}")?;
            }
            GrammarRuleKind::Union => {
                writeln!(w, "enum {} {{", name)?;
                for (variant, fields) in self.collect_union_parameters() {
                    writeln!(w, "    {} {{", variant)?;
                    for field in fields {
                        write!(w, "        {},", field)?;
                    }
                    writeln!(w, "    }},")?;
                }
                writeln!(w, "}}")?;
            }
            GrammarRuleKind::Climb => {}
        }
        Ok(())
    }
}

impl WriteRust for RuleDerive {
    fn write_rust(&self, w: &mut RustCodegen, _: &GrammarInfo) -> std::fmt::Result {
        for (key, value) in self.derives.iter() {
            if key.is_empty() {
                writeln!(w, "#[derive({})]", value.iter().join(", "))?
            }
            else {
                writeln!(w, "#[cfg_attr(feature = \"{}\", derive({}))]", key, value.iter().join(", "))?
            }
        }
        Ok(())
    }
}
