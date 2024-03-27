```
        let _span = pair.get_span();
        Ok(Self {
{%- for field in rule.class_fields() %}
{%- if field.field_type(self.grammar).is_empty() %}
            // Missing rule {{ field.rhs }}
{%- else %}
{%- if field.count.is_one() %}
            {{ field.field_name()|safe_rust_id }}: pair.take_tagged_one::<{{ field.field_type(self.grammar) }}>(Cow::Borrowed("{{ field.field_name() }}"))?,
{%- else if field.count.is_optional() %}
            {{ field.field_name()|safe_rust_id }}: pair.take_tagged_option::<{{ field.field_type(self.grammar) }}>(Cow::Borrowed("{{ field.field_name() }}")),
{%- else %}
            {{ field.field_name()|safe_rust_id }}: pair.take_tagged_items::<{{ field.field_type(self.grammar) }}>(Cow::Borrowed("{{ field.field_name() }}")).collect::<Result<Vec<_>, _>>()?,
{%- endif %}
{%- endif %}
{%- endfor %}
{%- if rule.captures.text %}
            text: pair.get_string(),
{%- endif %}
            span: Range { start: _span.start() as {{ self.config.range_type }}, end: _span.end() as {{ self.config.range_type }} },
        })

```