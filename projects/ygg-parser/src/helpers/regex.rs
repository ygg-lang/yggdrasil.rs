use super::*;
use crate::bootstrap::{CategoryNode, RegexRangeNode};

impl<'i> AstBuilder<'i> for RegexRangeNode<'i> {
    type Output = YggdrasilRegex;

    fn build(&self, _: &ParseContext, _: &mut ParseState) -> Result<Self::Output> {
        Ok(YggdrasilRegex::new(self.get_str().trim(), self.get_range()))
    }
}

impl<'i> AstBuilder<'i> for CategoryNode<'i> {
    type Output = YggdrasilRegex;

    fn build(&self, _: &ParseContext, _: &mut ParseState) -> Result<Self::Output> {
        let r = self.get_range();
        let regex = match self.group() {
            Some(g) => YggdrasilRegex::new(format!("\\p{{{}={}}}", g.get_str(), self.script().get_str()), r),
            None => YggdrasilRegex::new(format!("\\p{{{}}}", self.script().get_str()), r),
        };
        Ok(regex)
    }
}

impl<'i> AstBuilder<'i> for RegexEmbedNode<'i> {
    type Output = YggdrasilRegex;

    fn build(&self, _: &ParseContext, _: &mut ParseState) -> Result<Self::Output> {
        let mut buffer = String::with_capacity(self.get_str().len());
        for item in self.regex_item() {
            match item {
                RegexItemNode::EscapedCharacter(v) => match v.get_chars().last() {
                    Some(c) => match c {
                        's' => buffer.push_str("\\s"),
                        'n' => buffer.push_str("\\n"),
                        'r' => buffer.push_str("\\r"),
                        'd' => buffer.push_str("\\d"),
                        'p' => buffer.push_str("\\p"),
                        c @ ('(' | ')' | '[' | ']' | '{' | '}') => {
                            buffer.push_str("\\");
                            buffer.push(c);
                        }
                        '.' => buffer.push_str("\\."),
                        '\\' => buffer.push_str("\\\\"),
                        _ => buffer.push(c),
                    },
                    None => {}
                },
                RegexItemNode::RegexCharacter(v) => buffer.push_str(v.get_str()),
            }
        }

        Ok(YggdrasilRegex { raw: buffer, span: self.get_range() })
    }
}
