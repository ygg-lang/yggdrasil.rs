use railroad::svg::Element;
pub use railroad::RailroadNode;

pub use helper::*;
use yggdrasil_ir::{
    ChoiceExpression, CodeGenerator, ConcatExpression, DataKind, ExpressionKind, ExpressionNode, GrammarInfo, GrammarRule,
    Operator, RuleReference, UnaryExpression, Validation,
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
    type Output = Diagram<VerticalGrid>;

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
    fn as_railroad(&self) -> Box<dyn RailroadNode>;
}

impl AsRailroad for GrammarInfo {
    fn as_railroad(&self) -> Box<dyn RailroadNode> {
        box VerticalGrid::new(self.rules.iter().map(|(_, rule)| rule.as_railroad()).collect())
    }
}

impl AsRailroad for GrammarRule {
    fn as_railroad(&self) -> Box<dyn RailroadNode> {
        let mut s = Sequence::default();
        s.push(box SimpleStart);
        s.push(box RuleName::new(self.name.to_string()));
        s.push(self.body.as_railroad());
        s.push(box SimpleEnd);
        return box s;
    }
}

impl AsRailroad for ExpressionNode {
    fn as_railroad(&self) -> Box<dyn RailroadNode> {
        self.kind.as_railroad()
    }
}

impl AsRailroad for ExpressionKind {
    fn as_railroad(&self) -> Box<dyn RailroadNode> {
        match self {
            ExpressionKind::Choice(e) => e.as_railroad(),
            ExpressionKind::Concat(e) => e.as_railroad(),
            ExpressionKind::Unary(e) => e.as_railroad(),
            ExpressionKind::Rule(e) => e.as_railroad(),
            ExpressionKind::Data(e) => e.as_railroad(),
            ExpressionKind::Function(e) => box Terminal::new(e.name.to_string(), &vec!["function"]),
        }
    }
}

impl AsRailroad for ChoiceExpression {
    fn as_railroad(&self) -> Box<dyn RailroadNode> {
        box Choice::new(self.branches.iter().map(|e| e.as_railroad()).collect())
    }
}

impl AsRailroad for ConcatExpression {
    fn as_railroad(&self) -> Box<dyn RailroadNode> {
        // TODO: maybe stack
        box Sequence::new(self.into_iter().map(|e| e.as_railroad()).collect())
    }
}

impl AsRailroad for UnaryExpression {
    fn as_railroad(&self) -> Box<dyn RailroadNode> {
        let mut base = self.base.as_railroad();
        for o in &self.ops {
            match o {
                Operator::Optional => base = box Optional::new(base),
                Operator::Repeats => {
                    base = box Repeat::new(base, Comment::new("*".to_string()));
                }
                Operator::Repeat1 => {
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
                Operator::Boxing => continue,
                Operator::Recursive => continue,
                Operator::Negative => continue,
                Operator::Remark => continue,
            }
        }
        return base;
    }
}

impl AsRailroad for DataKind {
    fn as_railroad(&self) -> Box<dyn RailroadNode> {
        match self {
            DataKind::Integer(v) => box Terminal::new(v.to_string(), &vec!["string"]),
            DataKind::String(v) => box Terminal::new(v.to_string(), &vec!["string"]),
            DataKind::CharacterAny => box Terminal::new("ANY".to_string(), &vec!["character"]),
            DataKind::Character(v) => box Terminal::new(v.to_string(), &vec!["string"]),
            DataKind::CharacterBuiltin(v) => box Terminal::new(v.to_string(), &vec!["string"]),
            DataKind::CharacterRange(v) => box Terminal::new(format!("{}-{}", v.start(), v.end()), &vec!["string"]),
            DataKind::CharacterFused(v) => box Terminal::new(v.to_string(), &vec!["string"]),
            DataKind::Ignored => box Terminal::new("Null".to_string(), &vec!["character"]),
            DataKind::Boolean(_) => box Terminal::new("Boolean".to_string(), &vec!["character"]),
            DataKind::StringFused(_) => box Terminal::new("StringFused".to_string(), &vec!["string"]),
        }
    }
}

impl AsRailroad for RuleReference {
    fn as_railroad(&self) -> Box<dyn RailroadNode> {
        let mut class = vec!["symbol"];
        if self.inline {
            class.push("inline")
        }
        if self.name.len() == 1 {
            box Link::new(NonTerminal::new(self.name.to_string(), &class), format!("#{}", self.name.to_string()))
        }
        else {
            // TODO: External link
            box NonTerminal::new(self.name.to_string(), &class)
        }
    }
}
