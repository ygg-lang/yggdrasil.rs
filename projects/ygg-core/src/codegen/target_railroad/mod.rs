use railroad::{svg::Element, Node};

pub use helper::*;
use yggdrasil_error::Validation;
use yggdrasil_ir::{
    data::RuleReference,
    grammar::GrammarInfo,
    nodes::{ChoiceExpression, ConcatExpression, ExpressionKind, UnaryExpression, YggdrasilExpression, YggdrasilOperator},
    rule::GrammarRule,
    traits::CodeGenerator,
};

mod helper;

pub struct Railroad {
    pub with_css: bool,
    pub css: String,
}

impl Default for Railroad {
    fn default() -> Self {
        Self { with_css: true, css: include_str!("default.css").to_string() }
    }
}

impl Railroad {}

impl CodeGenerator for Railroad {
    type Output = Diagram<VerticalGrid<Box<dyn Node>>>;

    fn generate(&mut self, info: &GrammarInfo) -> Validation<Self::Output> {
        let grid = VerticalGrid::new(info.rules.iter().map(|(_, rule)| rule.as_railroad()).collect());
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
    fn as_railroad(&self) -> Box<dyn Node>;
}

impl AsRailroad for GrammarInfo {
    fn as_railroad(&self) -> Box<dyn Node> {
        Box::new(VerticalGrid::new(self.rules.iter().map(|(_, rule)| rule.as_railroad()).collect()))
    }
}

impl AsRailroad for GrammarRule {
    fn as_railroad(&self) -> Box<dyn Node> {
        let mut s = Sequence::<Box<dyn Node>>::default();
        s.push(Box::new(SimpleStart));
        s.push(Box::new(RuleName::new(self.name.text.to_string())));
        match &self.body {
            Some(e) => {
                s.push(e.as_railroad());
            }
            None => {}
        }
        s.push(Box::new(SimpleEnd));
        return Box::new(s);
    }
}

impl AsRailroad for YggdrasilExpression {
    fn as_railroad(&self) -> Box<dyn Node> {
        self.kind.as_railroad()
    }
}

impl AsRailroad for ExpressionKind {
    fn as_railroad(&self) -> Box<dyn Node> {
        match self {
            ExpressionKind::Choice(e) => e.as_railroad(),
            ExpressionKind::Concat(e) => e.as_railroad(),
            ExpressionKind::Unary(e) => e.as_railroad(),
            ExpressionKind::Rule(e) => e.as_railroad(),
            ExpressionKind::Function(e) => Box::new(Terminal::new(e.name.to_string(), &vec!["function"])),
            ExpressionKind::Ignored => Box::new(Terminal::new("IGNORED".to_string(), &vec!["character"])),
            ExpressionKind::Text(v) => Box::new(Terminal::new(v.text.to_string(), &vec!["string"])),
            ExpressionKind::CharacterAny => Box::new(Terminal::new("ANY".to_string(), &vec!["character"])),
            ExpressionKind::CharacterRange(v) => Box::new(Terminal::new(format!("{}-{}", v.start(), v.end()), &vec!["string"])),
            ExpressionKind::Integer(v) => Box::new(Terminal::new(v.to_string(), &vec!["string"])),
            ExpressionKind::Boolean(_) => Box::new(Terminal::new("Boolean".to_string(), &vec!["character"])),
            ExpressionKind::Regex(v) => Box::new(Terminal::new(v.raw.to_string(), &vec!["string"])),
        }
    }
}

impl AsRailroad for ChoiceExpression {
    fn as_railroad(&self) -> Box<dyn Node> {
        Box::new(Choice::new(self.branches.iter().map(|e| e.as_railroad()).collect()))
    }
}

impl AsRailroad for ConcatExpression {
    fn as_railroad(&self) -> Box<dyn Node> {
        // TODO: maybe stack
        Box::new(Sequence::new(self.sequence.iter().map(|e| e.as_railroad()).collect()))
    }
}

impl AsRailroad for UnaryExpression {
    fn as_railroad(&self) -> Box<dyn Node> {
        let mut base = self.base.as_railroad();
        for o in &self.operators {
            match o {
                YggdrasilOperator::Optional => base = Box::new(Optional::new(base)),
                YggdrasilOperator::Repeats => {
                    base = Box::new(Repeat::new(base, Comment::new("*".to_string())));
                }
                YggdrasilOperator::Repeat1 => {
                    base = Box::new(Repeat::new(base, Comment::new("+".to_string())));
                }
                YggdrasilOperator::RepeatsBetween(s, e) => {
                    let start = match s {
                        Some(s) => s.to_string(),
                        None => String::from("0"),
                    };
                    let end = match e {
                        Some(s) => s.to_string(),
                        None => String::from("∞"),
                    };
                    let c = Comment::new(format!("[{}, {}]", start, end));
                    base = Box::new(Repeat::new(base, c));
                }
                YggdrasilOperator::Boxing => continue,
                YggdrasilOperator::Recursive => continue,
                YggdrasilOperator::Negative => continue,
            }
        }
        return base;
    }
}

impl AsRailroad for RuleReference {
    fn as_railroad(&self) -> Box<dyn Node> {
        let mut class = vec!["symbol"];
        if self.inline {
            class.push("inline")
        }
        Box::new(Link::new(NonTerminal::new(self.name.text.to_string(), &class), format!("#{}", self.name.text.to_string())))
        // TODO: External link
        // Box::new(NonTerminal::new(self.name.to_string(), &class))
    }
}
