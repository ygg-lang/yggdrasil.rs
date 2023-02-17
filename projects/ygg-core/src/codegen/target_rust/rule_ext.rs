use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use std::{fmt::Debug, ops::Range};
use yggdrasil_ir::{
    grammar::GrammarInfo,
    nodes::{ExpressionKind, ExpressionNode},
    rule::{GrammarRule, GrammarRuleKind, RuleDerive, YggdrasilIdentifier},
    traits::FieldMap,
};

use super::*;

pub(super) trait RuleExt {
    fn safe_rule_name(&self) -> String;
    fn parser_name(&self) -> String;

    fn parser_expression(&self) -> String;
}

impl RuleExt for GrammarRule {
    fn safe_rule_name(&self) -> String {
        self.name.text.to_string()
    }

    fn parser_name(&self) -> String {
        format!("parse_{}", self.name.text).to_case(Case::Snake)
    }

    fn parser_expression(&self) -> String {
        match &self.body.kind {
            ExpressionKind::Function(_) => "parse_Function(s)".to_string(),
            ExpressionKind::Choice(_) => "parse_Choice(s)".to_string(),
            ExpressionKind::Concat(_) => "parse_Concat(s)".to_string(),
            ExpressionKind::Unary(_) => "parse_Unary(s)".to_string(),
            ExpressionKind::Rule(_) => "parse_Rule(s)".to_string(),
            ExpressionKind::Text(v) => {
                format!("s.match_string({:?})", v.text)
            }
            ExpressionKind::Regex(_) => "parse_Regex(s)".to_string(),
            ExpressionKind::Data(_) => "parse_Data(s)".to_string(),
        }
    }
}

pub struct ClassObject {
    name: String,
    derives: RuleDerive,
    file: FieldMap,
}

#[test]
fn test() {
    let mut codegen = RustCodegen::default();
    let info = GrammarInfo::default();
    let a = ExpressionNode::character('a').with_tag("a");
    let b = ExpressionNode::character('b').with_tag("b");
    let c = ExpressionNode::character('c').with_tag("c");
    let rule = GrammarRule {
        name: YggdrasilIdentifier { text: "Test".to_string(), span: Default::default() },
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
        writeln!(w, "fn {}(state: YState) -> YResult<{}> {{", w.consume_name(&self.name.text), self.r#type)?;
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
        let name = w.node_name(&self.name.text);
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
        write!(w, "{}", self)
    }
}
