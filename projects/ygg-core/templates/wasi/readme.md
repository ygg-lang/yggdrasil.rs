```
{%- for field in rule.class_fields() %}
{%- if field.count.is_one() %}
{%- else if field.count.is_optional() %}
{%- else %}
    fn {{ field.field_name() }}(&self) -> Vec<JsonNode> {
        let mut children = Vec::with_capacity(self.node.count_children() as usize);
        let mut iter = self.node.get_children(false);
        loop {
            match iter.next() {
                Some(s) => match s.get_rule().get_tag() {
                    "string" => children.push(JsonNode::Str(JsonStringNode::new(JsonStringNative { node: s }))),
                    #[cfg(debug_assertions)]
                    s => unreachable!(
                        "branch tag `{}` is not possible here, check whether the grammar version is correct",
                        s.get_rule().get_tag()
                    ),
                    _ => break,
                },
                None => break,
            }
        }
        return children;
    }
{%- endif %}
{%- endfor %}
```

```shell
impl {{ self.node_trait(rule) }} for {{ self.node_native(rule) }} {
    fn ctor(super_: SyntaxNode) -> Result<{{ self.node_wasi(rule) }}, ParseError> {
        Ok({{ self.node_wasi(rule) }}::new(Self { node: super_.into_inner() }))
    }
    fn parse_string(text: String, offset: u32) -> Result<{{ self.node_wasi(rule) }}, ParseError> {
        let input: Rc<str> = Rc::from(text);
        let language = NativeLanguage { name: "{{ self.language_name() }}", glob: &["*.json5"] };
        let root = match crate::wit::parse_cst::parse_cst(&input, {{ self.token_name() }}::{{ self.rule_variant(rule) }})?.next() {
            Some(root) => root,
            None => Err(ParseError::MissingRoot)?,
        };
        Ok({{ self.node_wasi(rule) }}::new(Self { node: NativeSyntaxData::new(input.clone(), root, &language) }))
    }
    fn get_text(&self) -> String {
        self.node.get_text()
    }
}
```