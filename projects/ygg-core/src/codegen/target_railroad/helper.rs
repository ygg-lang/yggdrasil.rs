pub use railroad::{
    Choice, Comment, Diagram, End, HorizontalGrid, Link, Optional, Repeat, Sequence, SimpleEnd, SimpleStart, Stack, Start,
    VerticalGrid,
};

pub struct Terminal;

impl Terminal {
    pub fn new(context: String, classes: &[&str]) -> railroad::Terminal {
        let mut nt = railroad::Terminal::new(context);
        if classes.is_empty() {
            return nt;
        }
        let class = nt.attr("class".to_owned()).or_insert(String::new());
        *class = classes.join(" ");
        return nt;
    }
}

pub struct NonTerminal;

impl NonTerminal {
    pub fn new(context: String, classes: &[&str]) -> railroad::NonTerminal {
        let mut nt = railroad::NonTerminal::new(context);
        if classes.is_empty() {
            return nt;
        }
        let class = nt.attr("class".to_owned()).or_insert(String::new());
        *class = classes.join(" ");
        return nt;
    }
}

pub struct RuleName;

impl RuleName {
    pub fn new(name: String) -> railroad::Comment {
        let mut nt = railroad::Comment::new(name.to_owned());
        nt.attr("id".to_owned()).or_insert(name);
        return nt;
    }
}
