use railroad::{svg::Element, Node};

pub use helper::*;
use yggdrasil_error::Validation;
use yggdrasil_ir::{
    data::RuleReference,
    grammar::GrammarInfo,
    nodes::{ChoiceExpression, ConcatExpression, ExpressionBody, UnaryExpression, YggdrasilExpression, YggdrasilOperator},
    rule::{GrammarBody, GrammarRule},
    traits::CodeGenerator,
};

mod helper;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BuildRailway {
    pub with_css: bool,
    pub css: String,
    pub contain_ignored: bool,
}

impl Default for BuildRailway {
    fn default() -> Self {
        Self { with_css: true, css: include_str!("default.css").to_string(), contain_ignored: true }
    }
}

impl BuildRailway {}

impl CodeGenerator for BuildRailway {
    type Output = Diagram<VerticalGrid<Box<dyn Node>>>;

    fn generate(&mut self, info: &GrammarInfo) -> Validation<Self::Output> {
        let grid = VerticalGrid::new(info.rules.iter().map(|(_, rule)| rule.as_railroad(self)).collect());
        let mut diagram = Diagram::new(grid);
        let mut element = Element::new("style").set("type", "text/css");
        if self.with_css {
            element = element.raw_text(&self.css);
        }
        diagram.add_element(element);
        Validation::Success { value: diagram, diagnostics: vec![] }
    }
}

trait AsRailroad {
    fn as_railroad(&self, config: &BuildRailway) -> Box<dyn Node>;
}

impl AsRailroad for GrammarInfo {
    fn as_railroad(&self, config: &BuildRailway) -> Box<dyn Node> {
        Box::new(VerticalGrid::new(self.rules.iter().map(|(_, rule)| rule.as_railroad(config)).collect()))
    }
}

impl AsRailroad for GrammarRule {
    fn as_railroad(&self, config: &BuildRailway) -> Box<dyn Node> {
        let mut s = Sequence::<Box<dyn Node>>::default();
        s.push(Box::new(SimpleStart));
        s.push(Box::new(RuleName::new(self.name.text.to_string())));
        match &self.body {
            GrammarBody::Class { term } => {
                s.push(term.as_railroad(config));
            }
            GrammarBody::Union { branches } => {
                let concat = ChoiceExpression { branches: branches.iter().map(|v| v.branch.clone()).collect() };
                s.push(concat.as_railroad(config));
            }
            GrammarBody::Climb { .. } => {}
        }
        s.push(Box::new(SimpleEnd));
        return Box::new(s);
    }
}

impl AsRailroad for YggdrasilExpression {
    fn as_railroad(&self, config: &BuildRailway) -> Box<dyn Node> {
        self.body.as_railroad(config)
    }
}

impl AsRailroad for ExpressionBody {
    fn as_railroad(&self, config: &BuildRailway) -> Box<dyn Node> {
        match self {
            ExpressionBody::Choice(e) => e.as_railroad(config),
            ExpressionBody::Concat(e) => e.as_railroad(config),
            ExpressionBody::Unary(e) => e.as_railroad(config),
            ExpressionBody::Rule(e) => e.as_railroad(config),
            ExpressionBody::Call(e) => Box::new(Terminal::new(e.name.to_string(), &vec!["function"])),
            ExpressionBody::Ignored => Box::new(Terminal::new("IGNORED".to_string(), &vec!["character"])),
            ExpressionBody::Text(v) => Box::new(Terminal::new(v.text.to_string(), &vec!["string"])),
            ExpressionBody::CharacterAny => Box::new(Terminal::new("ANY".to_string(), &vec!["character"])),
            ExpressionBody::CharacterRestOfLine => Box::new(Terminal::new("RestOfLine".to_string(), &vec!["character"])),
            ExpressionBody::CharacterRange(v) => Box::new(Terminal::new(format!("{}-{}", v.start(), v.end()), &vec!["string"])),
            ExpressionBody::Integer(v) => Box::new(Terminal::new(v.to_string(), &vec!["string"])),
            ExpressionBody::Boolean(_) => Box::new(Terminal::new("Boolean".to_string(), &vec!["character"])),
            ExpressionBody::Regex(v) => Box::new(Terminal::new(v.raw.to_string(), &vec!["string"])),
        }
    }
}

impl AsRailroad for ChoiceExpression {
    fn as_railroad(&self, config: &BuildRailway) -> Box<dyn Node> {
        Box::new(Choice::new(self.branches.iter().map(|e| e.as_railroad(config)).collect()))
    }
}

impl AsRailroad for ConcatExpression {
    fn as_railroad(&self, config: &BuildRailway) -> Box<dyn Node> {
        // TODO: maybe stack
        Box::new(Sequence::new(self.sequence.iter().map(|e| e.as_railroad(config)).collect()))
    }
}

impl AsRailroad for UnaryExpression {
    fn as_railroad(&self, config: &BuildRailway) -> Box<dyn Node> {
        let mut base = self.base.as_railroad(config);
        for o in &self.operators {
            match o {
                YggdrasilOperator::RepeatsBetween { min, max } => {
                    if *min == 0 && *max == 1 {
                        base = Box::new(Optional::new(base))
                    }
                    else {
                        let start = min.to_string();
                        let end = if *max >= 65536 { String::from("∞") } else { max.to_string() };
                        let c = Comment::new(format!("[{}, {}]", start, end));
                        base = Box::new(Repeat::new(base, c));
                    }
                }
                YggdrasilOperator::Boxing => continue,
                YggdrasilOperator::Recursive => continue,
                YggdrasilOperator::Negative => continue,
                YggdrasilOperator::Positive => continue,
            }
        }
        return base;
    }
}

impl AsRailroad for RuleReference {
    fn as_railroad(&self, _: &BuildRailway) -> Box<dyn Node> {
        let mut class = vec!["symbol"];
        if self.inline {
            class.push("inline")
        }
        Box::new(Link::new(NonTerminal::new(self.name.text.to_string(), &class), format!("#{}", self.name.text.to_string())))
        // TODO: External link
        // Box::new(NonTerminal::new(self.name.to_string(), &class))
    }
}
