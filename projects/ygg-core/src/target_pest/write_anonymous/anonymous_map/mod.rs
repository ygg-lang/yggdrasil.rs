use std::collections::BTreeMap;

pub struct AnonymousMap {
    inner: BTreeMap<String, String>,
}

impl AnonymousMap {
    pub fn insert(&mut self, key: String) -> Option<String> {
        let value = match Self::special_name(&key) {
            Some(s) => s,
            _ if Self::small_ascii(&key) => key.to_owned(),
            _ => self.inner.len().to_string(),
        };
        self.inner.insert(key, value)
    }
    #[inline]
    pub fn get(&self, key: &str) -> Option<String> {
        self.inner.get(key).map(|s| format!("ano_{}", s))
    }
    #[inline]
    pub fn insert_and_get(&mut self, key: &str) -> Option<String> {
        if let None = self.inner.get(key) {
            self.insert(key.to_string())
        }
        self.get(key)
    }

    fn small_ascii(key: &str) -> bool {
        if key.len() >= 22 {
            return false;
        }
        for c in key.chars() {
            if !c.is_alphanumeric() {
                return false;
            }
        }
        return true;
    }

    fn special_name(key: &str) -> Option<String> {
        let name = match key {
            // paired
            "<" => "op_lt",
            ">" => "op_gt",
            "{" => "op_lcub",
            "}" => "op_rcub",
            "(" => "op_lpar",
            ")" => "op_rpar",
            "[" => "op_lsqb",
            "]" => "op_rsqb",
            // escaped
            "\\" => "op_bsol",
            "\"" => "op_quot",
            // other input
            "`" => "op_grave",
            "~" => "op_plus",
            "!" => "op_plus",
            "@" => "op_plus",
            "#" => "op_plus",
            "$" => "op_plus",
            "%" => "op_plus",
            "^" => "op_plus",
            "&" => "op_amp",
            "*" => "op_mul",
            "-" => "op_sub",
            "_" => "op_underscore",
            "+" => "op_add",
            "=" => "op_plus",
            ":" => "op_plus",
            ";" => "op_plus",
            "," => "op_plus",
            "." => "op_plus",
            "/" => "op_plus",
            "?" => "op_plus",
            "|" => "op_plus",
            // other
            "<<" => "op_shift",
            // rest
            _ => "",
        };
        match name.is_empty() {
            true => Some(name.to_string()),
            false => None,
        }
    }
}
