use super::*;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct YggdrasilBranch {
    pub tag: Option<YggdrasilIdentifier>,
    pub branch: YggdrasilExpression,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct YggdrasilVariant {
    pub variant: YggdrasilIdentifier,
    pub class: YggdrasilIdentifier,
    pub additional_rule: Option<GrammarRule>,
}

impl YggdrasilBranch {
    pub fn as_variant(&self, name: &str, index: usize) -> YggdrasilVariant {
        let mut branch = YggdrasilVariant { variant: Default::default(), class: Default::default(), additional_rule: None };
        match self.tag.as_ref() {
            Some(s) => branch.variant = s.clone(),
            None => {
                // TODO: use expression range as span
                branch.variant = YggdrasilIdentifier { text: format!("{name}{index}").to_case(Case::UpperCamel), span: Default::default() }
            }
        };
        match self.branch.as_identifier() {
            Some(s) => {
                branch.variant = s.clone();
                branch.class = s.clone();
            }
            None => {
                branch.class = YggdrasilIdentifier { text: format!("{name}{index}").to_case(Case::UpperCamel), span: branch.variant.span };
                branch.additional_rule = Some(GrammarRule {
                    name: branch.class.clone(),
                    redirect: None,
                    attributes: GrammarRuleAttributes::default().with_hidden(true),
                    body: GrammarBody::Class { term: self.branch.clone() },
                    range: Default::default(),
                })
            }
        };
        branch
    }
}
