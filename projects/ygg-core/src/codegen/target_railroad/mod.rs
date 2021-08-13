mod css;
mod helper;

pub use self::css::DEFAULT_CSS;
pub use helper::*;

use crate::{
    frontend::{
        rule::{ExpressionNode, Operator, RefinedChoice, RefinedConcat, RefinedData, RefinedExpression, RefinedUnary},
        GrammarContext, GrammarInfo, Rule, Translator,
    },
    Result,
};
use lsp_types::Url;
use railroad::{RailroadNode};
use std::{lazy::SyncLazy, str::FromStr};
use yggdrasil_bootstrap::ast::{YggParser};

impl GrammarInfo {
    pub fn into_railroad(self) -> VerticalGrid {
        VerticalGrid::new(self.rule_map.into_iter().map(|(_, rule)| rule.into_railroad()).collect())
    }
}

impl Rule {
    pub fn into_railroad(self) -> Box<dyn RailroadNode> {
        let mut s = Sequence::default();
        s.push(box SimpleStart);
        s.push(box RuleName::new(self.name.data));
        s.push(self.expression.into_railroad());
        s.push(box SimpleEnd);
        return box s;
    }
}

impl ExpressionNode {
    pub fn into_railroad(self) -> Box<dyn RailroadNode> {
        self.node.into_railroad(self.inline_token)
    }
}

impl RefinedExpression {
    pub fn into_railroad(self, inline: bool) -> Box<dyn RailroadNode> {
        match self {
            Self::Data(v) => v.into_railroad(inline),
            Self::Unary(v) => v.into_railroad(),
            Self::Choice(v) => v.into_railroad(),
            Self::Concat(v) => v.into_railroad(),
        }
    }
}

impl RefinedData {
    pub fn into_railroad(self, inline: bool) -> Box<dyn RailroadNode> {
        match self {
            Self::Symbol(v) => {
                let mut class = vec!["symbol"];
                if inline {
                    class.push("inline")
                }
                if v.symbol.len() == 1 {
                    box Link::new(NonTerminal::new(v.to_string(), &class), format!("#{}", v.to_string()))
                }
                else {
                    // TODO: External link
                    box NonTerminal::new(v.to_string(), &class)
                }
            }
            Self::String(v) => box Terminal::new(v, &vec!["string"]),
            Self::Regex(v) => box Terminal::new(v.to_string(), &vec!["regex"]),
            Self::Integer(v) => box Terminal::new(v.to_string(), &vec!["string"]),
        }
    }
}

impl RefinedUnary {
    pub fn into_railroad(self) -> Box<dyn RailroadNode> {
        let mut base = self.base.into_railroad();
        for o in self.ops {
            match o {
                Operator::Optional => base = box Optional::new(base),
                Operator::Repeats => {
                    base = box Repeat::new(base, Comment::new("*".to_string()));
                }
                Operator::Repeats1 => {
                    base = box Repeat::new(base, Comment::new("+".to_string()));
                }
                Operator::RepeatsBetween(s, e) => {
                    let start = match s {
                        Some(s) => s.to_string(),
                        None => String::from("0"),
                    };
                    let end = match e {
                        Some(s) => s.to_string(),
                        None => String::from("∞"),
                    };
                    let c = Comment::new(format!("[{}, {}]", start, end));
                    base = box Repeat::new(base, c);
                }
                Operator::Mark => continue,
                Operator::Recursive => continue,
            }
        }
        return base;
    }
}

impl RefinedChoice {
    pub fn into_railroad(self) -> Box<dyn RailroadNode> {
        box Choice::new(self.inner.into_iter().map(|e| e.into_railroad()).collect())
    }
}

impl RefinedConcat {
    pub fn into_railroad(self) -> Box<dyn RailroadNode> {
        // TODO: maybe stack
        let mut out = Sequence::default();
        out.push(self.base.into_railroad());
        for (_, e) in self.rest {
            out.push(e.into_railroad());
        }
        return box out;
    }
}

pub static EXAMPLE_URL: SyncLazy<Url> = SyncLazy::new(|| Url::from_str("file://example/path").unwrap());

pub fn assert_codegen(text: &str) -> Result<VerticalGrid> {
    let mut ctx = GrammarContext::new(text, &EXAMPLE_URL);
    let mut parser = YggParser::default();
    let mut grammar = parser.parse_program(text)?.translate(&mut ctx)?;
    // grammar.optimize_local()?;
    Ok(grammar.into_railroad())
}

#[test]
fn test() {
    let text = r#"
    a = b | c?;
    b = 2? ~ "3"+;
    "#;
    let grid = assert_codegen(text).unwrap();
    let mut dia = Diagram::new(grid);
    dia.add_element(svg::Element::new("style").set("type", "text/css").raw_text(DEFAULT_CSS));
    println!("{}", dia);
}
