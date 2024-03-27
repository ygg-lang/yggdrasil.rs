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