mod css;

pub use self::css::DEFAULT_CSS;
use std::borrow::Borrow;
use std::fmt::Debug;
use std::lazy::SyncLazy;
use std::str::FromStr;
use lsp_types::Url;
use railroad::{self, svg, Diagram, End, Sequence, Start, Optional, SimpleStart, SimpleEnd, HorizontalGrid, VerticalGrid, Stack, RailroadNode, Comment, Link};
use yggdrasil_bootstrap::ast::{SymbolPath, YggParser};
use crate::frontend::{GrammarContext, GrammarInfo, Rule, Translator};
use crate::frontend::rule::{ExpressionNode, RefinedData, RefinedExpression};
use crate::Result;

pub static EXAMPLE_URL: SyncLazy<Url> = SyncLazy::new(|| Url::from_str("file://example/path").unwrap());


pub fn assert_codegen(text: &str) -> Result<VerticalGrid> {
    let mut ctx = GrammarContext::new(text, &EXAMPLE_URL);
    let mut parser = YggParser::default();
    let mut grammar = parser.parse_program(text)?.translate(&mut ctx)?;
    grammar.optimize_local()?;
    Ok(grammar.into_railroad())
}

impl GrammarInfo {
    pub fn into_railroad(self) -> VerticalGrid {
        VerticalGrid::new(self.rule_map.into_iter().map(|(_, rule)| rule.into_railroad()).collect())
    }
}

impl Rule {
    pub fn into_railroad(self) -> Box<dyn RailroadNode> {
        let mut s = Sequence::default();
        s.push(box SimpleStart);
        s.push(box Comment::new(self.name.data));
        s.push(self.expression.into_railroad());
        return box s;
    }
}

impl ExpressionNode {
    pub fn into_railroad(self) -> Box<dyn RailroadNode> {
        self.node.into_railroad()
    }
}

impl RefinedExpression {
    pub fn into_railroad(self) -> Box<dyn RailroadNode> {
        match self {
            Self::Data(v) => { v.into_railroad() }
            Self::Unary(_) => { unimplemented!() }
            Self::Choice(_) => { unimplemented!() }
            Self::Concat(_) => { unimplemented!() }
        }
    }
}

impl RefinedData {
    pub fn into_railroad(self) -> Box<dyn RailroadNode> {
        match self {
            RefinedData::Symbol(v) => {
                box Link::new(NonTerminal::new(v.to_string(), &vec!["symbol"]), "a".to_string())
                // box NonTerminal::new(v.to_string(), &vec!["symbol"])
            }
            RefinedData::String(v) => {
                box Terminal::new(format!("{:?}", v), &vec!["integer"])
            }
            RefinedData::Regex(_) => { unimplemented!() }
            RefinedData::Integer(v) => {
                box Terminal::new(v.to_string(), &vec!["integer"])
            }
        }
    }
}

pub struct Terminal;



impl Terminal {
    pub fn new(context: String, classes: &[&str]) -> railroad::Terminal {
        let mut nt = railroad::Terminal::new(context);
        if classes.is_empty() { return nt; }
        let class = nt.attr("class".to_owned()).or_insert(String::new());
        *class = classes.join(" ");
        return nt;
    }
}

pub struct NonTerminal;

impl NonTerminal {
    pub fn new(context: String, classes: &[&str]) -> railroad::NonTerminal {
        let mut nt = railroad::NonTerminal::new(context);
        if classes.is_empty() { return nt; }
        let class = nt.attr("class".to_owned()).or_insert(String::new());
        *class = classes.join(" ");
        return nt;
    }
}

pub struct RuleName;

impl RuleName {
    pub fn new(context: String, id: &str) -> railroad::Comment {
        let mut nt = railroad::Comment::new(context);
        nt.attr("id".to_owned()).or_insert(id.to_string());
        return nt;
    }
}

#[test]
fn test() {
    let text = r#"
    a = b;
    b = 2;
    c = "3";
    "#;
    let grid = assert_codegen(text).unwrap();
    let mut dia = Diagram::new(grid);
    dia.add_element(svg::Element::new("style").set("type", "text/css").raw_text(DEFAULT_CSS));
    println!("{}", dia);
}
