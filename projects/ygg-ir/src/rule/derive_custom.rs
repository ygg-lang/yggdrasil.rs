use quote::ToTokens;
use syn::__private::TokenStream;

pub struct CustomDerive {
    pub feature: String,
    pub custom: Vec<String>,
}

impl ToTokens for CustomDerive {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if self. { }
    }
}

impl CustomDerive {
    pub fn has_condition(&self) -> bool {
        !self.feature.is_empty() || !self.custom.is_empty()
    }
}

// #[derive(Debug)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]