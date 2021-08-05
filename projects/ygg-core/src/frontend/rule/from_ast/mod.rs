use super::{
    hints::{duplicate_declaration_error, name_missing, top_area_error},
    *,
};

pub struct GrammarContext<'i> {
    url: &'i Url,
    text: &'i str,
    is_top_area: bool,
}

impl<'i> GrammarContext<'i> {
    #[inline]
    pub fn new(text: &'i str, url: &'i Url) -> Self {
        Self { text, url, is_top_area: true }
    }
    #[inline]
    pub fn get_text(&self) -> &'i str {
        self.text
    }
    #[inline]
    pub fn get_url(&self) -> &'i Url {
        self.url
    }
    #[inline]
    pub fn get_lines(&self) -> LineBreaks<'i> {
        LineBreaks::new(&self.text)
    }
    #[inline]
    pub fn get_lsp_range(&self, offsets: (usize, usize)) -> Range {
        self.get_lines().get_lsp_range(offsets.0, offsets.1)
    }
}

pub trait Translator {
    fn translate(self, url: &GrammarContext) -> Result<(GrammarInfo, HintItems)>;
}

impl Translator for Program {
    fn translate(self, file: &GrammarContext) -> Result<(GrammarInfo, HintItems)> {
        let mut is_top_area = true;
        let mut is_grammar = None;
        let mut name_position = Default::default();
        let mut name = None;
        let mut rule_map = Map::<String, Rule>::default();
        let mut extensions = vec![];
        let mut ignores = vec![];
        let mut diag = vec![];
        let mut lens = vec![];
        let mut doc_buffer = String::new();
        for stmt in self.statement {
            match stmt {
                Statement::Grammar(s) => {
                    if !is_top_area {
                        diag.push(top_area_error("Grammar", "Grammar statement must be declared at the top", s.range, file))
                    }
                    match is_grammar {
                        Some(true) => diag.push(duplicate_declaration_error(
                            "Grammar",
                            "Already declaration as `grammar!`",
                            s.range,
                            name_position,
                            file,
                        )),
                        Some(false) => diag.push(duplicate_declaration_error(
                            "Grammar",
                            "Already declaration as `fragment!`",
                            s.range,
                            name_position,
                            file,
                        )),
                        None => {
                            is_grammar = Some(true);
                            name_position = s.range;
                            extensions = s.ext;
                            name = Some(s.id.data)
                        }
                    }
                }
                Statement::Fragment(s) => {
                    if !is_top_area {
                        diag.push(top_area_error("Fragment", "Fragment statement must be declared at the top", s.range, file))
                    }
                    match is_grammar {
                        Some(true) => diag.push(duplicate_declaration_error(
                            "Fragment",
                            "Already declaration as `grammar!`",
                            s.range,
                            name_position,
                            &file,
                        )),
                        Some(false) => diag.push(duplicate_declaration_error(
                            "Fragment",
                            "Already declaration as `fragment!`",
                            s.range,
                            name_position,
                            file,
                        )),
                        None => {
                            is_grammar = Some(false);
                            name_position = s.range;
                            name = Some(s.id.data)
                        }
                    }
                }
                Statement::Ignore(s) => {
                    if !is_top_area {
                        diag.push(top_area_error("Ignore", "Ignore statement must be declared at the top", s.range, file))
                    }
                    if !ignores.is_empty() {
                        diag.push(duplicate_declaration_error(
                            "Ignore",
                            "Already declaration ignore statement",
                            s.range,
                            name_position,
                            file,
                        ))
                    } else {
                        ignores = s.rules;
                    }
                }
                Statement::Assign(s) => {
                    is_top_area = false;
                    let mut rule = Rule::from(*s);
                    swap(&mut rule.doc, &mut doc_buffer);
                    match rule_map.get(&rule.name.data) {
                        Some(old) => diag.push(duplicate_declaration_error(
                            "Rule",
                            format!("Already declaration as Rule `{}`", old.name.data),
                            rule.range,
                            old.name.range,
                            file,
                        )),
                        None => {
                            rule_map.insert(rule.name.data.to_owned(), rule);
                        }
                    }
                }
                Statement::CommentDocument(text) => doc_buffer.extend(text.doc.chars().chain("\n".chars())),
                Statement::Import(_) => {
                    unimplemented!()
                }
                Statement::MacroCall(_) => { unimplemented!() }
            }
        }

        let name = Symbol {
            data: match name {
                Some(s) => s,
                None => {
                    lens.push(name_missing());
                    String::from("<anonymous>")
                }
            },
            range: name_position,
        };

        let state = GrammarInfo {
            name,
            extensions,
            rule_map,
            ignores,
            url: file.url.to_owned(),
            text: file.get_text().to_owned(),
            is_grammar: is_grammar.unwrap_or(false),
        };

        let hint = HintItems { diagnostic: diag, code_lens: lens, document_symbol: vec![] };

        Ok((state, hint))
    }
}

impl From<AssignStatement> for Rule {
    fn from(s: AssignStatement) -> Self {
        let name = &s.id.data;
        let ty = Symbol { data: name.to_case(Case::UpperCamel), range: s.id.range };
        let force_inline = name.starts_with("_");
        // if !force_inline {
        //     ty = Some(Symbol { data: name.to_case(Case::UpperCamel), range: s.id.range })
        // }
        let mut try_inline_token = false;
        let mut eliminate_unmarked = false;
        let mut eliminate_unnamed = false;
        for char in s.eq.chars() {
            match char {
                '_' => eliminate_unnamed = true,
                '^' => eliminate_unmarked = true,
                '@' => try_inline_token = true,
                _ => (),
            }
        }
        let mut expression = ExpressionNode::from(s.rhs);
        expression.inline_token = try_inline_token;
        Self {
            name: s.id,
            ty,
            doc: "".to_string(),
            force_inline,
            force_box: false,
            already_inline: false,
            eliminate_unmarked,
            eliminate_unnamed,
            expression,
            range: s.range,
        }
    }
}
