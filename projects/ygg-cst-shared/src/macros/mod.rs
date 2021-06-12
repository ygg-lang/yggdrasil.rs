





#[macro_export]
macro_rules! ignore_terms {
    ($r:ident, $($rule:ident),*) => {
       |s| self::$r(s) $(.or_else(|s| self::$rule(s)))*
    };
}

#[macro_export]
macro_rules! tag_node {
    ($i:ident, $id:ident, $name:literal) => {
        self::$id($i).and_then(|s| s.tag_node($name))
    };
    ($id:ident, $name:literal) => {
        |s| self::$id(s).and_then(|s| s.tag_node($name))
    };
}

#[macro_export]
macro_rules! tag_branch {
    ($id:ident, $rule:ident, $name:literal, $e:expr) => {
        let $id = match $id.rule(Rule::$rule, $e) {
            Ok(o) => return o.tag_branch($name),
            Err(e) => e,
        };
    };
}

#[macro_export]
macro_rules! match_charset {
    ($i:ident, $($p:tt)+) => {
        $i.match_char_by(|s| matches!(s, $($p)+))
    };
    ($($p:tt)+) => {
        |s| s.match_char_by(|s| matches!(s, $($p)+))
    };
}

