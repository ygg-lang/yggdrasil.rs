use std::fmt::{Debug, Formatter};

pub struct Node<Rule> {
    pub rule: Rule,
    pub start: usize,
    pub end: usize,
    pub children: Vec<Node<Rule>>,
    pub label: Option<&'static str>,
    pub alternative: Option<&'static str>,
}

impl Debug for Node<R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let w = &mut f.debug_struct("Node");
        w.field("rule", &self.rule);
        w.field("range", &format!("{}-{}", self.start, self.end));
        if let Some(s) = self.label {
            w.field("label", &s);
        };
        if let Some(s) = self.alternative {
            w.field("branch", &s);
        };
        w.field("children", &self.children);
        w.finish()
    }
}