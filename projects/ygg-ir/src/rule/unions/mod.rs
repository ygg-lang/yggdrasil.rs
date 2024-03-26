use super::*;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct YggdrasilBranch {
    pub tag: Option<YggdrasilIdentifier>,
    pub branch: YggdrasilExpression,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum YggdrasilVariant {
    Reference { variant: YggdrasilIdentifier },
    Additional { variant: YggdrasilIdentifier, rule: GrammarRule },
}

impl YggdrasilBranch {
    pub fn as_variant(&self, name: &str, index: usize) -> YggdrasilVariant {
        let id = match self.tag.as_ref() {
            Some(s) => s.clone(),
            None => {
                match self.branch.as_identifier() {
                    Some(s) => s.clone(),
                    None => {
                        // TODO: use expression range as span
                        YggdrasilIdentifier { text: format!("{name}{index}").to_case(Case::UpperCamel), span: Default::default() }
                    }
                }
            }
        };
        match self.branch.as_identifier() {
            Some(_) => YggdrasilVariant::Reference { variant: id },
            None => {
                let rule = GrammarRule {
                    name: YggdrasilIdentifier { text: format!("{name}{index}").to_case(Case::UpperCamel), span: id.span },
                    redirect: None,
                    attributes: GrammarRuleAttributes::default().with_hidden(true),
                    body: GrammarBody::Class { term: self.branch.clone() },
                    range: Default::default(),
                };
                YggdrasilVariant::Additional { variant: id, rule }
            }
        }
    }
}
