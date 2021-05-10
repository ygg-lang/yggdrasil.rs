use convert_case::{Case, Casing};
use lsp_types::Url;

use super::{
    hints::{duplicate_declaration_error, name_missing, top_area_error},
    *,
};
use crate::{
    ast::{AssignStatement,  Program, Statement},
    manager::HintItems,
    Result,
};

impl Program {
    pub fn build_grammar(self, url: Url) -> Result<(GrammarState, HintItems)> {
        let mut is_top_area = true;
        let mut is_grammar = None;
        let mut name_position = Default::default();
        let mut name = None;
        let mut rule_map = Map::<String, YGGRule>::default();
        let mut extensions = vec![];
        let mut ignores = vec![];
        let mut diag = vec![];
        let mut lens = vec![];
        for stmt in self.statement {
            match stmt {
                Statement::GrammarStatement(s) => {
                    if !is_top_area {
                        diag.push(top_area_error("Grammar", "Grammar statement must be declared at the top", s.range))
                    }
                    match is_grammar {
                        Some(true) => diag.push(duplicate_declaration_error(
                            "Grammar",
                            "Already declaration as `grammar!`",
                            s.range,
                            &url,
                            name_position,
                        )),
                        Some(false) => diag.push(duplicate_declaration_error(
                            "Grammar",
                            "Already declaration as `fragment!`",
                            s.range,
                            &url,
                            name_position,
                        )),
                        None => {
                            is_grammar = Some(true);
                            name_position = s.range;
                            extensions = s.ext;
                            name = Some(s.id.data)
                        }
                    }
                }
                Statement::FragmentStatement(s) => {
                    if !is_top_area {
                        diag.push(top_area_error("Fragment", "Fragment statement must be declared at the top", s.range))
                    }
                    match is_grammar {
                        Some(true) => diag.push(duplicate_declaration_error(
                            "Fragment",
                            "Already declaration as `grammar!`",
                            s.range,
                            &url,
                            name_position,
                        )),
                        Some(false) => diag.push(duplicate_declaration_error(
                            "Fragment",
                            "Already declaration as `fragment!`",
                            s.range,
                            &url,
                            name_position,
                        )),
                        None => {
                            is_grammar = Some(false);
                            name_position = s.range;
                            name = Some(s.id.data)
                        }
                    }
                }
                Statement::IgnoreStatement(s) => {
                    if !is_top_area {
                        diag.push(top_area_error("Ignore", "Ignore statement must be declared at the top", s.range))
                    }
                    if !ignores.is_empty() {
                        diag.push(duplicate_declaration_error(
                            "Ignore",
                            "Already declaration ignore statement",
                            s.range,
                            &url,
                            name_position,
                        ))
                    }
                    else {
                        ignores = s.rules;
                    }
                }
                Statement::AssignStatement(s) => {
                    is_top_area = false;
                    let rule = YGGRule::from(*s);
                    match rule_map.get(&rule.name.data) {
                        Some(old) => diag.push(duplicate_declaration_error(
                            "Rule",
                            format!("Already declaration as Rule `{}`", old.name.data),
                            rule.range,
                            &url,
                            old.name.range,
                        )),
                        None => {
                            rule_map.insert(rule.name.data.to_owned(), rule);
                        }
                    }
                }
                Statement::EmptyStatement(_) => continue,
            }
        }

        let name = Identifier {
            data: match name {
                Some(s) => s,
                None => {
                    lens.push(name_missing());
                    String::from("<anonymous>")
                }
            },
            range: name_position,
        };

        let state = GrammarState { name, extensions, rule_map, ignores, url, is_grammar: is_grammar.unwrap_or(false) };

        let hint = HintItems { diagnostic: diag, code_lens: lens, document_symbol: vec![] };

        Ok((state, hint))
    }
}

impl From<AssignStatement> for YGGRule {
    fn from(s: AssignStatement) -> Self {
        let mut name = s.id.data;
        let mut ty = Identifier { data: name.to_case(Case::UpperCamel), range: s.id.range };
        let force_inline = name.starts_with("_");
        // if !force_inline {
        //     ty = Some(Identifier { data: name.to_case(Case::UpperCamel), range: s.id.range })
        // }
        let mut eliminate_unmarked = false;
        let mut eliminate_unnamed = false;
        match s.eq.as_str() {
            "_=" => eliminate_unnamed = true,
            "^=" => eliminate_unmarked = true,
            _ => (),
        }
        let expression = ExpressionNode::from(s.rhs);
        Self {
            name: Identifier { data: name, range: s.id.range },
            ty,
            force_inline,
            already_inline: false,
            eliminate_unmarked,
            eliminate_unnamed,
            expression,
            range: s.range,
        }
    }
}
