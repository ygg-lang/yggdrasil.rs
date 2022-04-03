use super::*;

//
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct RuleReference {
    pub tag: String,
    pub name: String,
    pub boxed: bool,
    pub inline: bool,
}

impl Display for RuleReference {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.boxed {
            write!(f, "@box(")?
        }
        if !self.tag.is_empty() {
            write!(f, "{}:", self.tag)?
        }
        if self.inline {
            write!(f, "_")?
        }
        write!(f, "{}", self.name)?;
        if self.boxed {
            write!(f, ")")?
        }
        Ok(())
    }
}

impl RuleReference {
    pub fn new(name: &str) -> Self {
        Self { tag: "".to_string(), name: name.trim_start_matches("_").to_string(), boxed: false, inline: name.starts_with('_') }
    }
}

impl ExpressionKind {
    pub fn rule(name: &str) -> Self {
        let data = match name {
            "ANY" => DataKind::CharacterAny,
            "XID_START" => {
                todo!();
                // DataKind::CharacterSet(name.to_string())
            }
            _ => {
                todo!()
                // DataKind::Rule(RuleReference::new(name))
            }
        };
        Self::Data(Box::new(data))
    }
    pub fn string(string: String) -> Self {
        let data = DataKind::String(string);
        Self::Data(Box::new(data))
    }
    pub fn builtin(name: &str) -> Option<Self> {
        // let builtin = &["XID_START"];
        // if builtin.contains(&name) {
        //     let data = DataKind::CharacterBuiltin(name.to_string());
        //     Some(ExpressionNode::Data(Box::new(data)))
        // }
        // else {
        //     return None;
        // }
        todo!()
    }
}
