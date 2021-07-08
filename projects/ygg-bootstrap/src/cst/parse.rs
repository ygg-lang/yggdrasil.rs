use regex::Regex;
use std::collections::HashMap;
use unicode_xid::UnicodeXID;

#[derive(Clone, Debug)]
pub struct Node {
    pub rule: Rule,
    pub start: usize,
    pub end: usize,
    pub children: Vec<Node>,
    pub label: Option<&'static str>,
    pub alternative: Option<&'static str>,
}

impl Node {
    fn new(rule: Rule, start: usize, end: usize, label: Option<&'static str>, alternative: Option<&'static str>) -> Self {
        Self {
            rule,
            start,
            end,
            children: Vec::new(),
            label,
            alternative,
        }
    }

    pub fn print_to_string(&self, input: &str) -> String {
        let children = if self.children.is_empty() {
            String::new()
        } else {
            format!(", {}", self.children.iter().map(|node| node.print_to_string(input)).collect::<Vec<String>>().join(", "))
        };

        format!("({:?}, {}\"{}\"{})", self.rule, if let Some(alt) = self.alternative { format!("alt={}, ", alt) } else { String::new() }, escape_string(&input[self.start..self.end]), children,)
    }

    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    pub fn as_str<'s>(&self, input: &'s str) -> &'s str {
        &input[self.start..self.end]
    }
}

fn escape_string(str: &str) -> String {
    let mut res = String::new();

    for ch in str.chars() {
        match ch {
            '\\' => res.push_str("\\\\"),
            '\t' => res.push_str("\\t"),
            '\n' => res.push_str("\\n"),
            '\r' => res.push_str("\\r"),
            '"' => res.push_str("\\\""),
            ch => res.push(ch),
        }
    }

    res
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
#[allow(non_camel_case_types)]
enum Terminal {
    Literal_28,
    Literal_11,
    Literal_27,
    Literal_14,
    Literal_19,
    Literal_18,
    Literal_9,
    Literal_10,
    Literal_23,
    Literal_21,
    Literal_1,
    Literal_22,
    Literal_30,
    Literal_4,
    Literal_31,
    Literal_17,
    Literal_29,
    Literal,
    Literal_12,
    Literal_16,
    Literal_8,
    Literal_13,
    Literal_20,
    Literal_7,
    Literal_24,
    Literal_26,
    Literal_25,
    Literal_5,
    Literal_6,
    _as,
    fragment_0,
    grammar_0,
    ignore_0,
    import_0,
    _macro,
    _return,
    Literal_0,
    Literal_3,
    Literal_2,
    Literal_15,
    Regex_0,
    Regex,
    Regex_1,
    Regex_2,
    s,
    Regex_3,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum Rule {
    Terminal,
    List,
    MustNotMatch,
    Any,
    More,
    program,
    statement,
    empty_statement,
    eos,
    grammar_statement,
    grammar,
    fragment_statement,
    fragment,
    import_statement,
    import,
    symbol_alias,
    ignore_statement,
    ignore,
    assign_statement,
    assign_kind,
    expr,
    expr_next,
    term,
    mark_branch,
    infix,
    prefix,
    suffix,
    data,
    list,
    slice,
    regex_range,
    macro_call,
    macro_then,
    macro_kv,
    macro_define,
    macro_arg,
    block,
    string,
    integer,
    symbol_path,
    symbol,
    IGNORE,
    SPACE,
    NEWLINE,
    COMMENT,
    Dot,
    EOI,
    XID_IDENTIFIER,
}

#[allow(non_snake_case)]
pub struct PEG {
    terminal_memo: HashMap<(usize, Terminal), Option<usize>>,
    rule_memo: HashMap<(usize, Rule), Result<Node, usize>>,
    regex_Regex_0: Regex,
    regex_Regex: Regex,
    regex_Regex_1: Regex,
    regex_Regex_2: Regex,
    regex_s: Regex,
    regex_Regex_3: Regex,
}

impl PEG {
    pub fn new() -> Self {
        Self {
            terminal_memo: HashMap::new(),
            rule_memo: HashMap::new(),
            regex_Regex_0: Regex::new(r########"^"([^\\"]|\\.)*""########).unwrap(),
            regex_Regex: Regex::new(r########"^'([^\\']|\\.)*'"########).unwrap(),
            regex_Regex_1: Regex::new(r########"^0|[1-9](_?[0-9])*"########).unwrap(),
            regex_Regex_2: Regex::new(
                r########"^[
]+"########,
            )
            .unwrap(),
            regex_s: Regex::new(r########"^[\s]+"########).unwrap(),
            regex_Regex_3: Regex::new(
                r########"^[^
]*"########,
            )
            .unwrap(),
        }
    }

    pub fn parse(&mut self, input: &str) -> Result<Node, (usize, usize)> {
        self.rule_memo.clear();
        self.terminal_memo.clear();

        self.rule_program(0, input).map_err(|pos| {
            let mut line_no = 1;
            let mut col_no = pos;

            for l in input.char_indices().filter_map(|(index, c)| if c == '\n' { Some(index + 1) } else { None }) {
                if pos < l {
                    break;
                }

                if pos == l {
                    // chomp off new line
                    col_no -= 1;
                    break;
                }

                col_no = pos - l;

                line_no += 1;
            }

            (line_no, col_no)
        })
    }

    #[allow(non_snake_case)]
    fn rule_program(&mut self, pos: usize, input: &str) -> Result<Node, usize> {
        let key = (pos, Rule::program);

        if let Some(res) = self.rule_memo.get(&key) {
            return res.clone();
        }

        let res = {
            let mut list = Vec::new();
            let start = pos;

            self.rule_IGNORE(pos, input)
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    {
                        let mut list = Vec::new();
                        let start = pos;
                        let mut pos = pos;

                        while let Ok(node) = {
                            let mut list = Vec::new();
                            let start = pos;

                            self.rule_statement(pos, input)
                                .and_then(|node| {
                                    let pos = node.end;
                                    list.push(node);
                                    self.rule_IGNORE(pos, input)
                                })
                                .map(|node| {
                                    let end = node.end;
                                    list.push(node);

                                    Node {
                                        rule: Rule::List,
                                        start,
                                        end,
                                        children: list,
                                        label: None,
                                        alternative: None,
                                    }
                                })
                        } {
                            if pos == node.end {
                                // must be making progress
                                break;
                            }
                            pos = node.end;
                            list.push(node);
                        }

                        Ok(Node {
                            rule: Rule::Any,
                            start,
                            end: pos,
                            children: list,
                            label: None,
                            alternative: None,
                        })
                    }
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.builtin_eoi(pos, input, None, None)
                })
                .map(|node| {
                    let end = node.end;
                    list.push(node);

                    Node {
                        rule: Rule::program,
                        start,
                        end,
                        children: list,
                        label: None,
                        alternative: None,
                    }
                })
        };

        // Note that failure to match is also cached using Err()
        self.rule_memo.insert(key, res.clone());

        res
    }

    #[allow(non_snake_case)]
    fn rule_statement(&mut self, pos: usize, input: &str) -> Result<Node, usize> {
        let key = (pos, Rule::statement);

        if let Some(res) = self.rule_memo.get(&key) {
            return res.clone();
        }

        let res = self
            .rule_empty_statement(pos, input)
            .map(|node| Node {
                rule: Rule::statement,
                start: node.start,
                end: node.end,
                children: vec![node],
                label: None,
                alternative: None,
            })
            .or_else(|_| {
                let mut list = Vec::new();
                let start = pos;

                self.rule_grammar_statement(pos, input)
                    .and_then(|node| {
                        let pos = node.end;
                        list.push(node);
                        self.rule_IGNORE(pos, input)
                    })
                    .and_then(|node| {
                        let pos = node.end;
                        list.push(node);
                        self.rule_eos(pos, input).or_else(|_| Ok(Node::new(Rule::Terminal, pos, pos, None, None)))
                    })
                    .map(|node| {
                        let end = node.end;
                        list.push(node);

                        Node {
                            rule: Rule::statement,
                            start,
                            end,
                            children: list,
                            label: None,
                            alternative: Some("grammar"),
                        }
                    })
            })
            .or_else(|_| {
                let mut list = Vec::new();
                let start = pos;

                self.rule_fragment_statement(pos, input)
                    .and_then(|node| {
                        let pos = node.end;
                        list.push(node);
                        self.rule_IGNORE(pos, input)
                    })
                    .and_then(|node| {
                        let pos = node.end;
                        list.push(node);
                        self.rule_eos(pos, input).or_else(|_| Ok(Node::new(Rule::Terminal, pos, pos, None, None)))
                    })
                    .map(|node| {
                        let end = node.end;
                        list.push(node);

                        Node {
                            rule: Rule::statement,
                            start,
                            end,
                            children: list,
                            label: None,
                            alternative: Some("fragment"),
                        }
                    })
            })
            .or_else(|_| {
                let mut list = Vec::new();
                let start = pos;

                self.rule_import_statement(pos, input)
                    .and_then(|node| {
                        let pos = node.end;
                        list.push(node);
                        self.rule_IGNORE(pos, input)
                    })
                    .and_then(|node| {
                        let pos = node.end;
                        list.push(node);
                        self.rule_eos(pos, input).or_else(|_| Ok(Node::new(Rule::Terminal, pos, pos, None, None)))
                    })
                    .map(|node| {
                        let end = node.end;
                        list.push(node);

                        Node {
                            rule: Rule::statement,
                            start,
                            end,
                            children: list,
                            label: None,
                            alternative: Some("import"),
                        }
                    })
            })
            .or_else(|_| {
                let mut list = Vec::new();
                let start = pos;

                self.rule_ignore_statement(pos, input)
                    .and_then(|node| {
                        let pos = node.end;
                        list.push(node);
                        self.rule_IGNORE(pos, input)
                    })
                    .and_then(|node| {
                        let pos = node.end;
                        list.push(node);
                        self.rule_eos(pos, input).or_else(|_| Ok(Node::new(Rule::Terminal, pos, pos, None, None)))
                    })
                    .map(|node| {
                        let end = node.end;
                        list.push(node);

                        Node {
                            rule: Rule::statement,
                            start,
                            end,
                            children: list,
                            label: None,
                            alternative: Some("ignore"),
                        }
                    })
            })
            .or_else(|_| {
                let mut list = Vec::new();
                let start = pos;

                self.rule_assign_statement(pos, input)
                    .and_then(|node| {
                        let pos = node.end;
                        list.push(node);
                        self.rule_IGNORE(pos, input)
                    })
                    .and_then(|node| {
                        let pos = node.end;
                        list.push(node);
                        self.rule_eos(pos, input).or_else(|_| Ok(Node::new(Rule::Terminal, pos, pos, None, None)))
                    })
                    .map(|node| {
                        let end = node.end;
                        list.push(node);

                        Node {
                            rule: Rule::statement,
                            start,
                            end,
                            children: list,
                            label: None,
                            alternative: Some("assign"),
                        }
                    })
            })
            .or_else(|_| {
                let mut list = Vec::new();
                let start = pos;

                self.rule_macro_then(pos, input)
                    .and_then(|node| {
                        let pos = node.end;
                        list.push(node);
                        self.rule_IGNORE(pos, input)
                    })
                    .and_then(|node| {
                        let pos = node.end;
                        list.push(node);
                        self.rule_eos(pos, input).or_else(|_| Ok(Node::new(Rule::Terminal, pos, pos, None, None)))
                    })
                    .map(|node| {
                        let end = node.end;
                        list.push(node);

                        Node {
                            rule: Rule::statement,
                            start,
                            end,
                            children: list,
                            label: None,
                            alternative: Some("macro_call"),
                        }
                    })
            })
            .or_else(|_| {
                let mut list = Vec::new();
                let start = pos;

                self.rule_macro_define(pos, input)
                    .and_then(|node| {
                        let pos = node.end;
                        list.push(node);
                        self.rule_IGNORE(pos, input)
                    })
                    .and_then(|node| {
                        let pos = node.end;
                        list.push(node);
                        self.rule_eos(pos, input).or_else(|_| Ok(Node::new(Rule::Terminal, pos, pos, None, None)))
                    })
                    .map(|node| {
                        let end = node.end;
                        list.push(node);

                        Node {
                            rule: Rule::statement,
                            start,
                            end,
                            children: list,
                            label: None,
                            alternative: Some("macro_def"),
                        }
                    })
            });

        // Note that failure to match is also cached using Err()
        self.rule_memo.insert(key, res.clone());

        res
    }

    #[allow(non_snake_case)]
    fn rule_empty_statement(&mut self, pos: usize, input: &str) -> Result<Node, usize> {
        let key = (pos, Rule::empty_statement);

        if let Some(res) = self.rule_memo.get(&key) {
            return res.clone();
        }

        let res = self.rule_eos(pos, input).map(|node| Node {
            rule: Rule::empty_statement,
            start: node.start,
            end: node.end,
            children: vec![node],
            label: None,
            alternative: None,
        });

        // Note that failure to match is also cached using Err()
        self.rule_memo.insert(key, res.clone());

        res
    }

    #[allow(non_snake_case)]
    fn rule_eos(&mut self, pos: usize, input: &str) -> Result<Node, usize> {
        let key = (pos, Rule::eos);

        if let Some(res) = self.rule_memo.get(&key) {
            return res.clone();
        }

        let res = self.match_terminal(pos, input, Terminal::Literal).map(|end| Node::new(Rule::eos, pos, end, None, None)).ok_or(pos);

        // Note that failure to match is also cached using Err()
        self.rule_memo.insert(key, res.clone());

        res
    }

    #[allow(non_snake_case)]
    fn rule_grammar_statement(&mut self, pos: usize, input: &str) -> Result<Node, usize> {
        let key = (pos, Rule::grammar_statement);

        if let Some(res) = self.rule_memo.get(&key) {
            return res.clone();
        }

        let res = {
            let mut list = Vec::new();
            let start = pos;

            self.rule_grammar(pos, input)
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_IGNORE(pos, input)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_symbol(pos, input)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_IGNORE(pos, input)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.match_terminal(pos, input, Terminal::Literal_0).map(|end| Node::new(Rule::Terminal, pos, end, None, None)).ok_or(pos)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_IGNORE(pos, input)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    {
                        let mut list = Vec::new();
                        let start = pos;

                        self.rule_string(pos, input)
                            .and_then(|node| {
                                let pos = node.end;
                                list.push(node);
                                self.rule_IGNORE(pos, input)
                            })
                            .and_then(|node| {
                                let pos = node.end;
                                list.push(node);
                                {
                                    let mut list = Vec::new();
                                    let start = pos;
                                    let mut pos = pos;

                                    while let Ok(node) = {
                                        let mut list = Vec::new();
                                        let start = pos;

                                        self.match_terminal(pos, input, Terminal::Literal_1)
                                            .map(|end| Node::new(Rule::Terminal, pos, end, None, None))
                                            .ok_or(pos)
                                            .and_then(|node| {
                                                let pos = node.end;
                                                list.push(node);
                                                self.rule_IGNORE(pos, input)
                                            })
                                            .and_then(|node| {
                                                let pos = node.end;
                                                list.push(node);
                                                self.rule_string(pos, input)
                                            })
                                            .map(|node| {
                                                let end = node.end;
                                                list.push(node);

                                                Node {
                                                    rule: Rule::List,
                                                    start,
                                                    end,
                                                    children: list,
                                                    label: None,
                                                    alternative: None,
                                                }
                                            })
                                    } {
                                        if pos == node.end {
                                            // must be making progress
                                            break;
                                        }
                                        pos = node.end;
                                        list.push(node);
                                    }

                                    Ok(Node {
                                        rule: Rule::Any,
                                        start,
                                        end: pos,
                                        children: list,
                                        label: None,
                                        alternative: None,
                                    })
                                }
                            })
                            .and_then(|node| {
                                let pos = node.end;
                                list.push(node);
                                self.rule_IGNORE(pos, input)
                            })
                            .and_then(|node| {
                                let pos = node.end;
                                list.push(node);
                                self.match_terminal(pos, input, Terminal::Literal_1)
                                    .map(|end| Node::new(Rule::Terminal, pos, end, None, None))
                                    .ok_or(pos)
                                    .or_else(|_| Ok(Node::new(Rule::Terminal, pos, pos, None, None)))
                            })
                            .map(|node| {
                                let end = node.end;
                                list.push(node);

                                Node {
                                    rule: Rule::List,
                                    start,
                                    end,
                                    children: list,
                                    label: None,
                                    alternative: None,
                                }
                            })
                    }
                    .or_else(|_| Ok(Node::new(Rule::Terminal, pos, pos, None, None)))
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_IGNORE(pos, input)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.match_terminal(pos, input, Terminal::Literal_2).map(|end| Node::new(Rule::Terminal, pos, end, None, None)).ok_or(pos)
                })
                .map(|node| {
                    let end = node.end;
                    list.push(node);

                    Node {
                        rule: Rule::grammar_statement,
                        start,
                        end,
                        children: list,
                        label: None,
                        alternative: None,
                    }
                })
        }
        .or_else(|_| {
            let mut list = Vec::new();
            let start = pos;

            self.rule_grammar(pos, input)
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_IGNORE(pos, input)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_symbol(pos, input)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_IGNORE(pos, input)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_string(pos, input).or_else(|_| Ok(Node::new(Rule::Terminal, pos, pos, None, None)))
                })
                .map(|node| {
                    let end = node.end;
                    list.push(node);

                    Node {
                        rule: Rule::grammar_statement,
                        start,
                        end,
                        children: list,
                        label: None,
                        alternative: None,
                    }
                })
        });

        // Note that failure to match is also cached using Err()
        self.rule_memo.insert(key, res.clone());

        res
    }

    #[allow(non_snake_case)]
    fn rule_grammar(&mut self, pos: usize, input: &str) -> Result<Node, usize> {
        let key = (pos, Rule::grammar);

        if let Some(res) = self.rule_memo.get(&key) {
            return res.clone();
        }

        let res = self.match_terminal(pos, input, Terminal::grammar_0).map(|end| Node::new(Rule::grammar, pos, end, None, None)).ok_or(pos);

        // Note that failure to match is also cached using Err()
        self.rule_memo.insert(key, res.clone());

        res
    }

    #[allow(non_snake_case)]
    fn rule_fragment_statement(&mut self, pos: usize, input: &str) -> Result<Node, usize> {
        let key = (pos, Rule::fragment_statement);

        if let Some(res) = self.rule_memo.get(&key) {
            return res.clone();
        }

        let res = {
            let mut list = Vec::new();
            let start = pos;

            self.rule_fragment(pos, input)
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_IGNORE(pos, input)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_symbol(pos, input)
                })
                .map(|node| {
                    let end = node.end;
                    list.push(node);

                    Node {
                        rule: Rule::fragment_statement,
                        start,
                        end,
                        children: list,
                        label: None,
                        alternative: None,
                    }
                })
        };

        // Note that failure to match is also cached using Err()
        self.rule_memo.insert(key, res.clone());

        res
    }

    #[allow(non_snake_case)]
    fn rule_fragment(&mut self, pos: usize, input: &str) -> Result<Node, usize> {
        let key = (pos, Rule::fragment);

        if let Some(res) = self.rule_memo.get(&key) {
            return res.clone();
        }

        let res = self.match_terminal(pos, input, Terminal::fragment_0).map(|end| Node::new(Rule::fragment, pos, end, None, None)).ok_or(pos);

        // Note that failure to match is also cached using Err()
        self.rule_memo.insert(key, res.clone());

        res
    }

    #[allow(non_snake_case)]
    fn rule_import_statement(&mut self, pos: usize, input: &str) -> Result<Node, usize> {
        let key = (pos, Rule::import_statement);

        if let Some(res) = self.rule_memo.get(&key) {
            return res.clone();
        }

        let res = {
            let mut list = Vec::new();
            let start = pos;

            self.rule_import(pos, input)
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_IGNORE(pos, input)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_string(pos, input)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_IGNORE(pos, input)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.match_terminal(pos, input, Terminal::Literal_0).map(|end| Node::new(Rule::Terminal, pos, end, None, None)).ok_or(pos)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_IGNORE(pos, input)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    {
                        let mut list = Vec::new();
                        let start = pos;

                        self.rule_symbol_alias(pos, input)
                            .and_then(|node| {
                                let pos = node.end;
                                list.push(node);
                                self.rule_IGNORE(pos, input)
                            })
                            .and_then(|node| {
                                let pos = node.end;
                                list.push(node);
                                {
                                    let mut list = Vec::new();
                                    let start = pos;
                                    let mut pos = pos;

                                    while let Ok(node) = {
                                        let mut list = Vec::new();
                                        let start = pos;

                                        self.match_terminal(pos, input, Terminal::Literal_1)
                                            .map(|end| Node::new(Rule::Terminal, pos, end, None, None))
                                            .ok_or(pos)
                                            .and_then(|node| {
                                                let pos = node.end;
                                                list.push(node);
                                                self.rule_IGNORE(pos, input)
                                            })
                                            .and_then(|node| {
                                                let pos = node.end;
                                                list.push(node);
                                                self.rule_symbol_alias(pos, input)
                                            })
                                            .map(|node| {
                                                let end = node.end;
                                                list.push(node);

                                                Node {
                                                    rule: Rule::List,
                                                    start,
                                                    end,
                                                    children: list,
                                                    label: None,
                                                    alternative: None,
                                                }
                                            })
                                    } {
                                        if pos == node.end {
                                            // must be making progress
                                            break;
                                        }
                                        pos = node.end;
                                        list.push(node);
                                    }

                                    Ok(Node {
                                        rule: Rule::Any,
                                        start,
                                        end: pos,
                                        children: list,
                                        label: None,
                                        alternative: None,
                                    })
                                }
                            })
                            .and_then(|node| {
                                let pos = node.end;
                                list.push(node);
                                self.rule_IGNORE(pos, input)
                            })
                            .and_then(|node| {
                                let pos = node.end;
                                list.push(node);
                                self.match_terminal(pos, input, Terminal::Literal_1)
                                    .map(|end| Node::new(Rule::Terminal, pos, end, None, None))
                                    .ok_or(pos)
                                    .or_else(|_| Ok(Node::new(Rule::Terminal, pos, pos, None, None)))
                            })
                            .map(|node| {
                                let end = node.end;
                                list.push(node);

                                Node {
                                    rule: Rule::List,
                                    start,
                                    end,
                                    children: list,
                                    label: None,
                                    alternative: None,
                                }
                            })
                    }
                    .or_else(|_| Ok(Node::new(Rule::Terminal, pos, pos, None, None)))
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_IGNORE(pos, input)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.match_terminal(pos, input, Terminal::Literal_2).map(|end| Node::new(Rule::Terminal, pos, end, None, None)).ok_or(pos)
                })
                .map(|node| {
                    let end = node.end;
                    list.push(node);

                    Node {
                        rule: Rule::import_statement,
                        start,
                        end,
                        children: list,
                        label: None,
                        alternative: None,
                    }
                })
        }
        .or_else(|_| {
            let mut list = Vec::new();
            let start = pos;

            self.rule_import(pos, input)
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_IGNORE(pos, input)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_string(pos, input)
                })
                .map(|node| {
                    let end = node.end;
                    list.push(node);

                    Node {
                        rule: Rule::import_statement,
                        start,
                        end,
                        children: list,
                        label: None,
                        alternative: None,
                    }
                })
        });

        // Note that failure to match is also cached using Err()
        self.rule_memo.insert(key, res.clone());

        res
    }

    #[allow(non_snake_case)]
    fn rule_import(&mut self, pos: usize, input: &str) -> Result<Node, usize> {
        let key = (pos, Rule::import);

        if let Some(res) = self.rule_memo.get(&key) {
            return res.clone();
        }

        let res = self.match_terminal(pos, input, Terminal::import_0).map(|end| Node::new(Rule::import, pos, end, None, None)).ok_or(pos);

        // Note that failure to match is also cached using Err()
        self.rule_memo.insert(key, res.clone());

        res
    }

    #[allow(non_snake_case)]
    fn rule_symbol_alias(&mut self, pos: usize, input: &str) -> Result<Node, usize> {
        let key = (pos, Rule::symbol_alias);

        if let Some(res) = self.rule_memo.get(&key) {
            return res.clone();
        }

        let res = {
            let mut list = Vec::new();
            let start = pos;

            {
                let mut list = Vec::new();
                let start = pos;

                self.rule_symbol(pos, input)
                    .and_then(|node| {
                        let pos = node.end;
                        list.push(node);
                        self.rule_IGNORE(pos, input)
                    })
                    .and_then(|node| {
                        let pos = node.end;
                        list.push(node);
                        self.match_terminal(pos, input, Terminal::_as).map(|end| Node::new(Rule::Terminal, pos, end, None, None)).ok_or(pos)
                    })
                    .and_then(|node| {
                        let pos = node.end;
                        list.push(node);
                        self.rule_IGNORE(pos, input)
                    })
                    .map(|node| {
                        let end = node.end;
                        list.push(node);

                        Node {
                            rule: Rule::List,
                            start,
                            end,
                            children: list,
                            label: None,
                            alternative: None,
                        }
                    })
            }
            .or_else(|_| Ok(Node::new(Rule::Terminal, pos, pos, None, None)))
            .and_then(|node| {
                let pos = node.end;
                list.push(node);
                self.rule_symbol(pos, input)
            })
            .map(|node| {
                let end = node.end;
                list.push(node);

                Node {
                    rule: Rule::symbol_alias,
                    start,
                    end,
                    children: list,
                    label: None,
                    alternative: None,
                }
            })
        };

        // Note that failure to match is also cached using Err()
        self.rule_memo.insert(key, res.clone());

        res
    }

    #[allow(non_snake_case)]
    fn rule_ignore_statement(&mut self, pos: usize, input: &str) -> Result<Node, usize> {
        let key = (pos, Rule::ignore_statement);

        if let Some(res) = self.rule_memo.get(&key) {
            return res.clone();
        }

        let res = {
            let mut list = Vec::new();
            let start = pos;

            self.rule_ignore(pos, input)
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_IGNORE(pos, input)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_symbol(pos, input)
                })
                .map(|node| {
                    let end = node.end;
                    list.push(node);

                    Node {
                        rule: Rule::ignore_statement,
                        start,
                        end,
                        children: list,
                        label: None,
                        alternative: None,
                    }
                })
        }
        .or_else(|_| {
            let mut list = Vec::new();
            let start = pos;

            self.rule_ignore(pos, input)
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_IGNORE(pos, input)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.match_terminal(pos, input, Terminal::Literal_0).map(|end| Node::new(Rule::Terminal, pos, end, None, None)).ok_or(pos)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_IGNORE(pos, input)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    {
                        let mut list = Vec::new();
                        let start = pos;

                        self.rule_symbol(pos, input)
                            .and_then(|node| {
                                let pos = node.end;
                                list.push(node);
                                self.rule_IGNORE(pos, input)
                            })
                            .and_then(|node| {
                                let pos = node.end;
                                list.push(node);
                                {
                                    let mut list = Vec::new();
                                    let start = pos;
                                    let mut pos = pos;

                                    while let Ok(node) = {
                                        let mut list = Vec::new();
                                        let start = pos;

                                        self.match_terminal(pos, input, Terminal::Literal_1)
                                            .map(|end| Node::new(Rule::Terminal, pos, end, None, None))
                                            .ok_or(pos)
                                            .and_then(|node| {
                                                let pos = node.end;
                                                list.push(node);
                                                self.rule_IGNORE(pos, input)
                                            })
                                            .and_then(|node| {
                                                let pos = node.end;
                                                list.push(node);
                                                self.rule_symbol(pos, input)
                                            })
                                            .map(|node| {
                                                let end = node.end;
                                                list.push(node);

                                                Node {
                                                    rule: Rule::List,
                                                    start,
                                                    end,
                                                    children: list,
                                                    label: None,
                                                    alternative: None,
                                                }
                                            })
                                    } {
                                        if pos == node.end {
                                            // must be making progress
                                            break;
                                        }
                                        pos = node.end;
                                        list.push(node);
                                    }

                                    Ok(Node {
                                        rule: Rule::Any,
                                        start,
                                        end: pos,
                                        children: list,
                                        label: None,
                                        alternative: None,
                                    })
                                }
                            })
                            .and_then(|node| {
                                let pos = node.end;
                                list.push(node);
                                self.rule_IGNORE(pos, input)
                            })
                            .and_then(|node| {
                                let pos = node.end;
                                list.push(node);
                                self.match_terminal(pos, input, Terminal::Literal_1)
                                    .map(|end| Node::new(Rule::Terminal, pos, end, None, None))
                                    .ok_or(pos)
                                    .or_else(|_| Ok(Node::new(Rule::Terminal, pos, pos, None, None)))
                            })
                            .map(|node| {
                                let end = node.end;
                                list.push(node);

                                Node {
                                    rule: Rule::List,
                                    start,
                                    end,
                                    children: list,
                                    label: None,
                                    alternative: None,
                                }
                            })
                    }
                    .or_else(|_| Ok(Node::new(Rule::Terminal, pos, pos, None, None)))
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_IGNORE(pos, input)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.match_terminal(pos, input, Terminal::Literal_2).map(|end| Node::new(Rule::Terminal, pos, end, None, None)).ok_or(pos)
                })
                .map(|node| {
                    let end = node.end;
                    list.push(node);

                    Node {
                        rule: Rule::ignore_statement,
                        start,
                        end,
                        children: list,
                        label: None,
                        alternative: None,
                    }
                })
        });

        // Note that failure to match is also cached using Err()
        self.rule_memo.insert(key, res.clone());

        res
    }

    #[allow(non_snake_case)]
    fn rule_ignore(&mut self, pos: usize, input: &str) -> Result<Node, usize> {
        let key = (pos, Rule::ignore);

        if let Some(res) = self.rule_memo.get(&key) {
            return res.clone();
        }

        let res = self.match_terminal(pos, input, Terminal::ignore_0).map(|end| Node::new(Rule::ignore, pos, end, None, None)).ok_or(pos);

        // Note that failure to match is also cached using Err()
        self.rule_memo.insert(key, res.clone());

        res
    }

    #[allow(non_snake_case)]
    fn rule_assign_statement(&mut self, pos: usize, input: &str) -> Result<Node, usize> {
        let key = (pos, Rule::assign_statement);

        if let Some(res) = self.rule_memo.get(&key) {
            return res.clone();
        }

        let res = {
            let mut list = Vec::new();
            let start = pos;

            self.rule_symbol(pos, input)
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_IGNORE(pos, input)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_assign_kind(pos, input)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_IGNORE(pos, input)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.match_terminal(pos, input, Terminal::Literal_3)
                        .map(|end| Node::new(Rule::Terminal, pos, end, None, None))
                        .ok_or(pos)
                        .or_else(|_| self.match_terminal(pos, input, Terminal::Literal_4).map(|end| Node::new(Rule::Terminal, pos, end, None, None)).ok_or(pos))
                        .or_else(|_| Ok(Node::new(Rule::Terminal, pos, pos, None, None)))
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_IGNORE(pos, input)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_expr(pos, input)
                })
                .map(|node| {
                    let end = node.end;
                    list.push(node);

                    Node {
                        rule: Rule::assign_statement,
                        start,
                        end,
                        children: list,
                        label: None,
                        alternative: None,
                    }
                })
        };

        // Note that failure to match is also cached using Err()
        self.rule_memo.insert(key, res.clone());

        res
    }

    #[allow(non_snake_case)]
    fn rule_assign_kind(&mut self, pos: usize, input: &str) -> Result<Node, usize> {
        let key = (pos, Rule::assign_kind);

        if let Some(res) = self.rule_memo.get(&key) {
            return res.clone();
        }

        let res = {
            let mut list = Vec::new();
            let start = pos;

            {
                let mut list = Vec::new();
                let start = pos;
                let mut pos = pos;

                while let Ok(node) = self
                    .match_terminal(pos, input, Terminal::Literal_5)
                    .map(|end| Node::new(Rule::Terminal, pos, end, None, None))
                    .ok_or(pos)
                    .or_else(|_| self.match_terminal(pos, input, Terminal::Literal_6).map(|end| Node::new(Rule::Terminal, pos, end, None, None)).ok_or(pos))
                    .or_else(|_| self.match_terminal(pos, input, Terminal::Literal_7).map(|end| Node::new(Rule::Terminal, pos, end, None, None)).ok_or(pos))
                {
                    if pos == node.end {
                        // must be making progress
                        break;
                    }
                    pos = node.end;
                    list.push(node);
                }

                Ok(Node {
                    rule: Rule::Any,
                    start,
                    end: pos,
                    children: list,
                    label: None,
                    alternative: None,
                })
            }
            .and_then(|node| {
                let pos = node.end;
                list.push(node);
                self.match_terminal(pos, input, Terminal::Literal_8).map(|end| Node::new(Rule::Terminal, pos, end, None, None)).ok_or(pos)
            })
            .map(|node| {
                let end = node.end;
                list.push(node);

                Node {
                    rule: Rule::assign_kind,
                    start,
                    end,
                    children: list,
                    label: None,
                    alternative: None,
                }
            })
        };

        // Note that failure to match is also cached using Err()
        self.rule_memo.insert(key, res.clone());

        res
    }

    #[allow(non_snake_case)]
    fn rule_expr(&mut self, pos: usize, input: &str) -> Result<Node, usize> {
        let key = (pos, Rule::expr);

        if let Some(res) = self.rule_memo.get(&key) {
            return res.clone();
        }

        let res = {
            let mut list = Vec::new();
            let start = pos;

            self.rule_term(pos, input)
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_IGNORE(pos, input)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    {
                        let mut list = Vec::new();
                        let start = pos;
                        let mut pos = pos;

                        while let Ok(node) = {
                            let mut list = Vec::new();
                            let start = pos;

                            self.rule_expr_next(pos, input)
                                .and_then(|node| {
                                    let pos = node.end;
                                    list.push(node);
                                    self.rule_IGNORE(pos, input)
                                })
                                .map(|node| {
                                    let end = node.end;
                                    list.push(node);

                                    Node {
                                        rule: Rule::List,
                                        start,
                                        end,
                                        children: list,
                                        label: None,
                                        alternative: None,
                                    }
                                })
                        } {
                            if pos == node.end {
                                // must be making progress
                                break;
                            }
                            pos = node.end;
                            list.push(node);
                        }

                        Ok(Node {
                            rule: Rule::Any,
                            start,
                            end: pos,
                            children: list,
                            label: None,
                            alternative: None,
                        })
                    }
                })
                .map(|node| {
                    let end = node.end;
                    list.push(node);

                    Node {
                        rule: Rule::expr,
                        start,
                        end,
                        children: list,
                        label: None,
                        alternative: None,
                    }
                })
        };

        // Note that failure to match is also cached using Err()
        self.rule_memo.insert(key, res.clone());

        res
    }

    #[allow(non_snake_case)]
    fn rule_expr_next(&mut self, pos: usize, input: &str) -> Result<Node, usize> {
        let key = (pos, Rule::expr_next);

        if let Some(res) = self.rule_memo.get(&key) {
            return res.clone();
        }

        let res = {
            let mut list = Vec::new();
            let start = pos;

            self.rule_infix(pos, input)
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_IGNORE(pos, input)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_term(pos, input)
                })
                .map(|node| {
                    let end = node.end;
                    list.push(node);

                    Node {
                        rule: Rule::expr_next,
                        start,
                        end,
                        children: list,
                        label: None,
                        alternative: Some("infix"),
                    }
                })
        }
        .or_else(|_| {
            let mut list = Vec::new();
            let start = pos;

            self.rule_term(pos, input)
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_IGNORE(pos, input)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    match self.match_terminal(pos, input, Terminal::Literal_8).map(|end| Node::new(Rule::Terminal, pos, end, None, None)).ok_or(pos) {
                        Ok(_) => Err(pos),
                        Err(_) => Ok(Node::new(Rule::MustNotMatch, pos, pos, None, None)),
                    }
                })
                .map(|node| {
                    let end = node.end;
                    list.push(node);

                    Node {
                        rule: Rule::expr_next,
                        start,
                        end,
                        children: list,
                        label: None,
                        alternative: Some("other"),
                    }
                })
        });

        // Note that failure to match is also cached using Err()
        self.rule_memo.insert(key, res.clone());

        res
    }

    #[allow(non_snake_case)]
    fn rule_term(&mut self, pos: usize, input: &str) -> Result<Node, usize> {
        let key = (pos, Rule::term);

        if let Some(res) = self.rule_memo.get(&key) {
            return res.clone();
        }

        let res = {
            let mut list = Vec::new();
            let start = pos;

            self.match_terminal(pos, input, Terminal::Literal_9)
                .map(|end| Node::new(Rule::Terminal, pos, end, None, None))
                .ok_or(pos)
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_IGNORE(pos, input)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.match_terminal(pos, input, Terminal::Literal_4)
                        .map(|end| Node::new(Rule::Terminal, pos, end, None, None))
                        .ok_or(pos)
                        .or_else(|_| Ok(Node::new(Rule::Terminal, pos, pos, None, None)))
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_IGNORE(pos, input)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_expr(pos, input)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_IGNORE(pos, input)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.match_terminal(pos, input, Terminal::Literal_10).map(|end| Node::new(Rule::Terminal, pos, end, None, None)).ok_or(pos)
                })
                .map(|node| {
                    let end = node.end;
                    list.push(node);

                    Node {
                        rule: Rule::term,
                        start,
                        end,
                        children: list,
                        label: None,
                        alternative: Some("priority"),
                    }
                })
        }
        .or_else(|_| {
            let mut list = Vec::new();
            let start = pos;

            {
                let mut list = Vec::new();
                let start = pos;
                let mut pos = pos;

                while let Ok(node) = self.rule_prefix(pos, input) {
                    if pos == node.end {
                        // must be making progress
                        break;
                    }
                    pos = node.end;
                    list.push(node);
                }

                Ok(Node {
                    rule: Rule::Any,
                    start,
                    end: pos,
                    children: list,
                    label: None,
                    alternative: None,
                })
            }
            .and_then(|node| {
                let pos = node.end;
                list.push(node);
                self.rule_IGNORE(pos, input)
            })
            .and_then(|node| {
                let pos = node.end;
                list.push(node);
                self.rule_data(pos, input)
            })
            .and_then(|node| {
                let pos = node.end;
                list.push(node);
                self.rule_IGNORE(pos, input)
            })
            .and_then(|node| {
                let pos = node.end;
                list.push(node);
                {
                    let mut list = Vec::new();
                    let start = pos;
                    let mut pos = pos;

                    while let Ok(node) = {
                        let mut list = Vec::new();
                        let start = pos;

                        match self.rule_assign_kind(pos, input) {
                            Ok(_) => Err(pos),
                            Err(_) => Ok(Node::new(Rule::MustNotMatch, pos, pos, None, None)),
                        }
                        .and_then(|node| {
                            let pos = node.end;
                            list.push(node);
                            self.rule_suffix(pos, input)
                        })
                        .map(|node| {
                            let end = node.end;
                            list.push(node);

                            Node {
                                rule: Rule::List,
                                start,
                                end,
                                children: list,
                                label: None,
                                alternative: None,
                            }
                        })
                    }
                    .or_else(|_| self.rule_slice(pos, input))
                    .or_else(|_| self.rule_mark_branch(pos, input))
                    {
                        if pos == node.end {
                            // must be making progress
                            break;
                        }
                        pos = node.end;
                        list.push(node);
                    }

                    Ok(Node {
                        rule: Rule::Any,
                        start,
                        end: pos,
                        children: list,
                        label: None,
                        alternative: None,
                    })
                }
            })
            .map(|node| {
                let end = node.end;
                list.push(node);

                Node {
                    rule: Rule::term,
                    start,
                    end,
                    children: list,
                    label: None,
                    alternative: Some("atom"),
                }
            })
        });

        // Note that failure to match is also cached using Err()
        self.rule_memo.insert(key, res.clone());

        res
    }

    #[allow(non_snake_case)]
    fn rule_mark_branch(&mut self, pos: usize, input: &str) -> Result<Node, usize> {
        let key = (pos, Rule::mark_branch);

        if let Some(res) = self.rule_memo.get(&key) {
            return res.clone();
        }

        let res = {
            let mut list = Vec::new();
            let start = pos;

            self.match_terminal(pos, input, Terminal::Literal_6)
                .map(|end| Node::new(Rule::Terminal, pos, end, None, None))
                .ok_or(pos)
                .or_else(|_| self.match_terminal(pos, input, Terminal::Literal_5).map(|end| Node::new(Rule::Terminal, pos, end, None, None)).ok_or(pos))
                .or_else(|_| self.match_terminal(pos, input, Terminal::Literal_11).map(|end| Node::new(Rule::Terminal, pos, end, None, None)).ok_or(pos))
                .or_else(|_| self.match_terminal(pos, input, Terminal::Literal_12).map(|end| Node::new(Rule::Terminal, pos, end, None, None)).ok_or(pos))
                .or_else(|_| self.match_terminal(pos, input, Terminal::Literal_13).map(|end| Node::new(Rule::Terminal, pos, end, None, None)).ok_or(pos))
                .or_else(|_| Ok(Node::new(Rule::Terminal, pos, pos, None, None)))
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.match_terminal(pos, input, Terminal::Literal_14).map(|end| Node::new(Rule::Terminal, pos, end, None, None)).ok_or(pos)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_symbol(pos, input)
                })
                .map(|node| {
                    let end = node.end;
                    list.push(node);

                    Node {
                        rule: Rule::mark_branch,
                        start,
                        end,
                        children: list,
                        label: None,
                        alternative: None,
                    }
                })
        };

        // Note that failure to match is also cached using Err()
        self.rule_memo.insert(key, res.clone());

        res
    }

    #[allow(non_snake_case)]
    fn rule_infix(&mut self, pos: usize, input: &str) -> Result<Node, usize> {
        let key = (pos, Rule::infix);

        if let Some(res) = self.rule_memo.get(&key) {
            return res.clone();
        }

        let res = self
            .match_terminal(pos, input, Terminal::Literal_3)
            .map(|end| Node::new(Rule::infix, pos, end, None, None))
            .ok_or(pos)
            .or_else(|_| self.match_terminal(pos, input, Terminal::Literal_4).map(|end| Node::new(Rule::infix, pos, end, None, None)).ok_or(pos))
            .or_else(|_| self.match_terminal(pos, input, Terminal::Literal_15).map(|end| Node::new(Rule::infix, pos, end, None, None)).ok_or(pos))
            .or_else(|_| self.match_terminal(pos, input, Terminal::Literal_16).map(|end| Node::new(Rule::infix, pos, end, None, None)).ok_or(pos))
            .or_else(|_| self.match_terminal(pos, input, Terminal::Literal_17).map(|end| Node::new(Rule::infix, pos, end, None, None)).ok_or(pos));

        // Note that failure to match is also cached using Err()
        self.rule_memo.insert(key, res.clone());

        res
    }

    #[allow(non_snake_case)]
    fn rule_prefix(&mut self, pos: usize, input: &str) -> Result<Node, usize> {
        let key = (pos, Rule::prefix);

        if let Some(res) = self.rule_memo.get(&key) {
            return res.clone();
        }

        let res = self
            .match_terminal(pos, input, Terminal::Literal_11)
            .map(|end| Node::new(Rule::prefix, pos, end, None, None))
            .ok_or(pos)
            .or_else(|_| self.match_terminal(pos, input, Terminal::Literal_18).map(|end| Node::new(Rule::prefix, pos, end, None, None)).ok_or(pos))
            .or_else(|_| self.match_terminal(pos, input, Terminal::Literal_5).map(|end| Node::new(Rule::prefix, pos, end, None, None)).ok_or(pos))
            .or_else(|_| self.match_terminal(pos, input, Terminal::Literal_19).map(|end| Node::new(Rule::prefix, pos, end, None, None)).ok_or(pos));

        // Note that failure to match is also cached using Err()
        self.rule_memo.insert(key, res.clone());

        res
    }

    #[allow(non_snake_case)]
    fn rule_suffix(&mut self, pos: usize, input: &str) -> Result<Node, usize> {
        let key = (pos, Rule::suffix);

        if let Some(res) = self.rule_memo.get(&key) {
            return res.clone();
        }

        let res = self
            .match_terminal(pos, input, Terminal::Literal_20)
            .map(|end| Node::new(Rule::suffix, pos, end, None, None))
            .ok_or(pos)
            .or_else(|_| self.match_terminal(pos, input, Terminal::Literal_21).map(|end| Node::new(Rule::suffix, pos, end, None, None)).ok_or(pos))
            .or_else(|_| self.match_terminal(pos, input, Terminal::Literal_22).map(|end| Node::new(Rule::suffix, pos, end, None, None)).ok_or(pos))
            .or_else(|_| self.match_terminal(pos, input, Terminal::Literal_23).map(|end| Node::new(Rule::suffix, pos, end, None, None)).ok_or(pos));

        // Note that failure to match is also cached using Err()
        self.rule_memo.insert(key, res.clone());

        res
    }

    #[allow(non_snake_case)]
    fn rule_data(&mut self, pos: usize, input: &str) -> Result<Node, usize> {
        let key = (pos, Rule::data);

        if let Some(res) = self.rule_memo.get(&key) {
            return res.clone();
        }

        let res = self
            .rule_macro_call(pos, input)
            .map(|node| Node {
                rule: Rule::data,
                start: node.start,
                end: node.end,
                children: vec![node],
                label: None,
                alternative: Some("macro"),
            })
            .or_else(|_| {
                self.rule_regex_range(pos, input).map(|node| Node {
                    rule: Rule::data,
                    start: node.start,
                    end: node.end,
                    children: vec![node],
                    label: None,
                    alternative: Some("regex"),
                })
            })
            .or_else(|_| {
                self.rule_list(pos, input).map(|node| Node {
                    rule: Rule::data,
                    start: node.start,
                    end: node.end,
                    children: vec![node],
                    label: None,
                    alternative: Some("list"),
                })
            })
            .or_else(|_| {
                self.rule_symbol_path(pos, input).map(|node| Node {
                    rule: Rule::data,
                    start: node.start,
                    end: node.end,
                    children: vec![node],
                    label: None,
                    alternative: Some("symbol"),
                })
            })
            .or_else(|_| {
                self.rule_integer(pos, input).map(|node| Node {
                    rule: Rule::data,
                    start: node.start,
                    end: node.end,
                    children: vec![node],
                    label: None,
                    alternative: Some("integer"),
                })
            })
            .or_else(|_| {
                self.rule_string(pos, input).map(|node| Node {
                    rule: Rule::data,
                    start: node.start,
                    end: node.end,
                    children: vec![node],
                    label: None,
                    alternative: Some("string"),
                })
            });

        // Note that failure to match is also cached using Err()
        self.rule_memo.insert(key, res.clone());

        res
    }

    #[allow(non_snake_case)]
    fn rule_list(&mut self, pos: usize, input: &str) -> Result<Node, usize> {
        let key = (pos, Rule::list);

        if let Some(res) = self.rule_memo.get(&key) {
            return res.clone();
        }

        let res = {
            let mut list = Vec::new();
            let start = pos;

            self.match_terminal(pos, input, Terminal::Literal_0)
                .map(|end| Node::new(Rule::Terminal, pos, end, None, None))
                .ok_or(pos)
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_IGNORE(pos, input)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    {
                        let mut list = Vec::new();
                        let start = pos;

                        self.rule_data(pos, input)
                            .and_then(|node| {
                                let pos = node.end;
                                list.push(node);
                                self.rule_IGNORE(pos, input)
                            })
                            .and_then(|node| {
                                let pos = node.end;
                                list.push(node);
                                {
                                    let mut list = Vec::new();
                                    let start = pos;
                                    let mut pos = pos;

                                    while let Ok(node) = {
                                        let mut list = Vec::new();
                                        let start = pos;

                                        self.match_terminal(pos, input, Terminal::Literal_1)
                                            .map(|end| Node::new(Rule::Terminal, pos, end, None, None))
                                            .ok_or(pos)
                                            .and_then(|node| {
                                                let pos = node.end;
                                                list.push(node);
                                                self.rule_IGNORE(pos, input)
                                            })
                                            .and_then(|node| {
                                                let pos = node.end;
                                                list.push(node);
                                                self.rule_data(pos, input)
                                            })
                                            .map(|node| {
                                                let end = node.end;
                                                list.push(node);

                                                Node {
                                                    rule: Rule::List,
                                                    start,
                                                    end,
                                                    children: list,
                                                    label: None,
                                                    alternative: None,
                                                }
                                            })
                                    } {
                                        if pos == node.end {
                                            // must be making progress
                                            break;
                                        }
                                        pos = node.end;
                                        list.push(node);
                                    }

                                    Ok(Node {
                                        rule: Rule::Any,
                                        start,
                                        end: pos,
                                        children: list,
                                        label: None,
                                        alternative: None,
                                    })
                                }
                            })
                            .and_then(|node| {
                                let pos = node.end;
                                list.push(node);
                                self.rule_IGNORE(pos, input)
                            })
                            .and_then(|node| {
                                let pos = node.end;
                                list.push(node);
                                self.match_terminal(pos, input, Terminal::Literal_1)
                                    .map(|end| Node::new(Rule::Terminal, pos, end, None, None))
                                    .ok_or(pos)
                                    .or_else(|_| Ok(Node::new(Rule::Terminal, pos, pos, None, None)))
                            })
                            .map(|node| {
                                let end = node.end;
                                list.push(node);

                                Node {
                                    rule: Rule::List,
                                    start,
                                    end,
                                    children: list,
                                    label: None,
                                    alternative: None,
                                }
                            })
                    }
                    .or_else(|_| Ok(Node::new(Rule::Terminal, pos, pos, None, None)))
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_IGNORE(pos, input)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.match_terminal(pos, input, Terminal::Literal_2).map(|end| Node::new(Rule::Terminal, pos, end, None, None)).ok_or(pos)
                })
                .map(|node| {
                    let end = node.end;
                    list.push(node);

                    Node {
                        rule: Rule::list,
                        start,
                        end,
                        children: list,
                        label: None,
                        alternative: None,
                    }
                })
        };

        // Note that failure to match is also cached using Err()
        self.rule_memo.insert(key, res.clone());

        res
    }

    #[allow(non_snake_case)]
    fn rule_slice(&mut self, pos: usize, input: &str) -> Result<Node, usize> {
        let key = (pos, Rule::slice);

        if let Some(res) = self.rule_memo.get(&key) {
            return res.clone();
        }

        let res = {
            let mut list = Vec::new();
            let start = pos;

            self.match_terminal(pos, input, Terminal::Literal_0)
                .map(|end| Node::new(Rule::Terminal, pos, end, None, None))
                .ok_or(pos)
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_IGNORE(pos, input)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_integer(pos, input)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_IGNORE(pos, input)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.match_terminal(pos, input, Terminal::Literal_1).map(|end| Node::new(Rule::Terminal, pos, end, None, None)).ok_or(pos)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_IGNORE(pos, input)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_integer(pos, input)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_IGNORE(pos, input)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.match_terminal(pos, input, Terminal::Literal_2).map(|end| Node::new(Rule::Terminal, pos, end, None, None)).ok_or(pos)
                })
                .map(|node| {
                    let end = node.end;
                    list.push(node);

                    Node {
                        rule: Rule::slice,
                        start,
                        end,
                        children: list,
                        label: None,
                        alternative: None,
                    }
                })
        };

        // Note that failure to match is also cached using Err()
        self.rule_memo.insert(key, res.clone());

        res
    }

    #[allow(non_snake_case)]
    fn rule_regex_range(&mut self, pos: usize, input: &str) -> Result<Node, usize> {
        let key = (pos, Rule::regex_range);

        if let Some(res) = self.rule_memo.get(&key) {
            return res.clone();
        }

        let res = {
            let mut list = Vec::new();
            let start = pos;

            self.match_terminal(pos, input, Terminal::Literal_24)
                .map(|end| Node::new(Rule::Terminal, pos, end, None, None))
                .ok_or(pos)
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_IGNORE(pos, input)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    {
                        let mut list = Vec::new();
                        let start = pos;
                        let mut pos = pos;

                        while let Ok(node) = {
                            let mut list = Vec::new();
                            let start = pos;

                            match self.match_terminal(pos, input, Terminal::Literal_25).map(|end| Node::new(Rule::Terminal, pos, end, None, None)).ok_or(pos) {
                                Ok(_) => Err(pos),
                                Err(_) => Ok(Node::new(Rule::MustNotMatch, pos, pos, None, None)),
                            }
                            .and_then(|node| {
                                let pos = node.end;
                                list.push(node);
                                self.rule_IGNORE(pos, input)
                            })
                            .and_then(|node| {
                                let pos = node.end;
                                list.push(node);
                                self.builtin_dot(pos, input, None, None)
                            })
                            .map(|node| {
                                let end = node.end;
                                list.push(node);

                                Node {
                                    rule: Rule::List,
                                    start,
                                    end,
                                    children: list,
                                    label: None,
                                    alternative: None,
                                }
                            })
                        }
                        .or_else(|_| {
                            let mut list = Vec::new();
                            let start = pos;

                            self.match_terminal(pos, input, Terminal::Literal_26)
                                .map(|end| Node::new(Rule::Terminal, pos, end, None, None))
                                .ok_or(pos)
                                .and_then(|node| {
                                    let pos = node.end;
                                    list.push(node);
                                    self.rule_IGNORE(pos, input)
                                })
                                .and_then(|node| {
                                    let pos = node.end;
                                    list.push(node);
                                    self.builtin_dot(pos, input, None, None)
                                })
                                .map(|node| {
                                    let end = node.end;
                                    list.push(node);

                                    Node {
                                        rule: Rule::List,
                                        start,
                                        end,
                                        children: list,
                                        label: None,
                                        alternative: None,
                                    }
                                })
                        }) {
                            if pos == node.end {
                                // must be making progress
                                break;
                            }
                            pos = node.end;
                            list.push(node);
                        }

                        Ok(Node {
                            rule: Rule::Any,
                            start,
                            end: pos,
                            children: list,
                            label: None,
                            alternative: None,
                        })
                    }
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_IGNORE(pos, input)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.match_terminal(pos, input, Terminal::Literal_25).map(|end| Node::new(Rule::Terminal, pos, end, None, None)).ok_or(pos)
                })
                .map(|node| {
                    let end = node.end;
                    list.push(node);

                    Node {
                        rule: Rule::regex_range,
                        start,
                        end,
                        children: list,
                        label: None,
                        alternative: None,
                    }
                })
        };

        // Note that failure to match is also cached using Err()
        self.rule_memo.insert(key, res.clone());

        res
    }

    #[allow(non_snake_case)]
    fn rule_macro_call(&mut self, pos: usize, input: &str) -> Result<Node, usize> {
        let key = (pos, Rule::macro_call);

        if let Some(res) = self.rule_memo.get(&key) {
            return res.clone();
        }

        let res = {
            let mut list = Vec::new();
            let start = pos;

            self.match_terminal(pos, input, Terminal::Literal_7)
                .map(|end| Node::new(Rule::Terminal, pos, end, None, None))
                .ok_or(pos)
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_symbol_path(pos, input)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_IGNORE(pos, input)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.match_terminal(pos, input, Terminal::Literal_9).map(|end| Node::new(Rule::Terminal, pos, end, None, None)).ok_or(pos)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_IGNORE(pos, input)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    {
                        let mut list = Vec::new();
                        let start = pos;

                        self.rule_macro_kv(pos, input)
                            .and_then(|node| {
                                let pos = node.end;
                                list.push(node);
                                self.rule_IGNORE(pos, input)
                            })
                            .and_then(|node| {
                                let pos = node.end;
                                list.push(node);
                                {
                                    let mut list = Vec::new();
                                    let start = pos;
                                    let mut pos = pos;

                                    while let Ok(node) = {
                                        let mut list = Vec::new();
                                        let start = pos;

                                        self.match_terminal(pos, input, Terminal::Literal_1)
                                            .map(|end| Node::new(Rule::Terminal, pos, end, None, None))
                                            .ok_or(pos)
                                            .and_then(|node| {
                                                let pos = node.end;
                                                list.push(node);
                                                self.rule_IGNORE(pos, input)
                                            })
                                            .and_then(|node| {
                                                let pos = node.end;
                                                list.push(node);
                                                self.rule_macro_kv(pos, input)
                                            })
                                            .map(|node| {
                                                let end = node.end;
                                                list.push(node);

                                                Node {
                                                    rule: Rule::List,
                                                    start,
                                                    end,
                                                    children: list,
                                                    label: None,
                                                    alternative: None,
                                                }
                                            })
                                    } {
                                        if pos == node.end {
                                            // must be making progress
                                            break;
                                        }
                                        pos = node.end;
                                        list.push(node);
                                    }

                                    Ok(Node {
                                        rule: Rule::Any,
                                        start,
                                        end: pos,
                                        children: list,
                                        label: None,
                                        alternative: None,
                                    })
                                }
                            })
                            .and_then(|node| {
                                let pos = node.end;
                                list.push(node);
                                self.rule_IGNORE(pos, input)
                            })
                            .and_then(|node| {
                                let pos = node.end;
                                list.push(node);
                                self.match_terminal(pos, input, Terminal::Literal_1)
                                    .map(|end| Node::new(Rule::Terminal, pos, end, None, None))
                                    .ok_or(pos)
                                    .or_else(|_| Ok(Node::new(Rule::Terminal, pos, pos, None, None)))
                            })
                            .map(|node| {
                                let end = node.end;
                                list.push(node);

                                Node {
                                    rule: Rule::List,
                                    start,
                                    end,
                                    children: list,
                                    label: None,
                                    alternative: None,
                                }
                            })
                    }
                    .or_else(|_| Ok(Node::new(Rule::Terminal, pos, pos, None, None)))
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_IGNORE(pos, input)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.match_terminal(pos, input, Terminal::Literal_10).map(|end| Node::new(Rule::Terminal, pos, end, None, None)).ok_or(pos)
                })
                .map(|node| {
                    let end = node.end;
                    list.push(node);

                    Node {
                        rule: Rule::macro_call,
                        start,
                        end,
                        children: list,
                        label: None,
                        alternative: None,
                    }
                })
        };

        // Note that failure to match is also cached using Err()
        self.rule_memo.insert(key, res.clone());

        res
    }

    #[allow(non_snake_case)]
    fn rule_macro_then(&mut self, pos: usize, input: &str) -> Result<Node, usize> {
        let key = (pos, Rule::macro_then);

        if let Some(res) = self.rule_memo.get(&key) {
            return res.clone();
        }

        let res = {
            let mut list = Vec::new();
            let start = pos;

            self.match_terminal(pos, input, Terminal::Literal_27)
                .map(|end| Node::new(Rule::Terminal, pos, end, None, None))
                .ok_or(pos)
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_symbol_path(pos, input)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    {
                        let mut list = Vec::new();
                        let start = pos;

                        self.rule_IGNORE(pos, input)
                            .and_then(|node| {
                                let pos = node.end;
                                list.push(node);
                                self.match_terminal(pos, input, Terminal::Literal_9).map(|end| Node::new(Rule::Terminal, pos, end, None, None)).ok_or(pos)
                            })
                            .and_then(|node| {
                                let pos = node.end;
                                list.push(node);
                                self.rule_IGNORE(pos, input)
                            })
                            .and_then(|node| {
                                let pos = node.end;
                                list.push(node);
                                {
                                    let mut list = Vec::new();
                                    let start = pos;

                                    self.rule_macro_kv(pos, input)
                                        .and_then(|node| {
                                            let pos = node.end;
                                            list.push(node);
                                            self.rule_IGNORE(pos, input)
                                        })
                                        .and_then(|node| {
                                            let pos = node.end;
                                            list.push(node);
                                            {
                                                let mut list = Vec::new();
                                                let start = pos;
                                                let mut pos = pos;

                                                while let Ok(node) = {
                                                    let mut list = Vec::new();
                                                    let start = pos;

                                                    self.match_terminal(pos, input, Terminal::Literal_1)
                                                        .map(|end| Node::new(Rule::Terminal, pos, end, None, None))
                                                        .ok_or(pos)
                                                        .and_then(|node| {
                                                            let pos = node.end;
                                                            list.push(node);
                                                            self.rule_IGNORE(pos, input)
                                                        })
                                                        .and_then(|node| {
                                                            let pos = node.end;
                                                            list.push(node);
                                                            self.rule_macro_kv(pos, input)
                                                        })
                                                        .map(|node| {
                                                            let end = node.end;
                                                            list.push(node);

                                                            Node {
                                                                rule: Rule::List,
                                                                start,
                                                                end,
                                                                children: list,
                                                                label: None,
                                                                alternative: None,
                                                            }
                                                        })
                                                } {
                                                    if pos == node.end {
                                                        // must be making progress
                                                        break;
                                                    }
                                                    pos = node.end;
                                                    list.push(node);
                                                }

                                                Ok(Node {
                                                    rule: Rule::Any,
                                                    start,
                                                    end: pos,
                                                    children: list,
                                                    label: None,
                                                    alternative: None,
                                                })
                                            }
                                        })
                                        .and_then(|node| {
                                            let pos = node.end;
                                            list.push(node);
                                            self.rule_IGNORE(pos, input)
                                        })
                                        .and_then(|node| {
                                            let pos = node.end;
                                            list.push(node);
                                            self.match_terminal(pos, input, Terminal::Literal_1)
                                                .map(|end| Node::new(Rule::Terminal, pos, end, None, None))
                                                .ok_or(pos)
                                                .or_else(|_| Ok(Node::new(Rule::Terminal, pos, pos, None, None)))
                                        })
                                        .map(|node| {
                                            let end = node.end;
                                            list.push(node);

                                            Node {
                                                rule: Rule::List,
                                                start,
                                                end,
                                                children: list,
                                                label: None,
                                                alternative: None,
                                            }
                                        })
                                }
                                .or_else(|_| Ok(Node::new(Rule::Terminal, pos, pos, None, None)))
                            })
                            .and_then(|node| {
                                let pos = node.end;
                                list.push(node);
                                self.rule_IGNORE(pos, input)
                            })
                            .and_then(|node| {
                                let pos = node.end;
                                list.push(node);
                                self.match_terminal(pos, input, Terminal::Literal_10).map(|end| Node::new(Rule::Terminal, pos, end, None, None)).ok_or(pos)
                            })
                            .map(|node| {
                                let end = node.end;
                                list.push(node);

                                Node {
                                    rule: Rule::List,
                                    start,
                                    end,
                                    children: list,
                                    label: None,
                                    alternative: None,
                                }
                            })
                    }
                    .or_else(|_| Ok(Node::new(Rule::Terminal, pos, pos, None, None)))
                })
                .map(|node| {
                    let end = node.end;
                    list.push(node);

                    Node {
                        rule: Rule::macro_then,
                        start,
                        end,
                        children: list,
                        label: None,
                        alternative: None,
                    }
                })
        };

        // Note that failure to match is also cached using Err()
        self.rule_memo.insert(key, res.clone());

        res
    }

    #[allow(non_snake_case)]
    fn rule_macro_kv(&mut self, pos: usize, input: &str) -> Result<Node, usize> {
        let key = (pos, Rule::macro_kv);

        if let Some(res) = self.rule_memo.get(&key) {
            return res.clone();
        }

        let res = {
            let mut list = Vec::new();
            let start = pos;

            self.rule_symbol(pos, input)
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_IGNORE(pos, input)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.match_terminal(pos, input, Terminal::Literal_28).map(|end| Node::new(Rule::Terminal, pos, end, None, None)).ok_or(pos)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_IGNORE(pos, input)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_expr(pos, input)
                })
                .map(|node| {
                    let end = node.end;
                    list.push(node);

                    Node {
                        rule: Rule::macro_kv,
                        start,
                        end,
                        children: list,
                        label: None,
                        alternative: None,
                    }
                })
        }
        .or_else(|_| {
            self.rule_expr(pos, input).map(|node| Node {
                rule: Rule::macro_kv,
                start: node.start,
                end: node.end,
                children: vec![node],
                label: None,
                alternative: None,
            })
        });

        // Note that failure to match is also cached using Err()
        self.rule_memo.insert(key, res.clone());

        res
    }

    #[allow(non_snake_case)]
    fn rule_macro_define(&mut self, pos: usize, input: &str) -> Result<Node, usize> {
        let key = (pos, Rule::macro_define);

        if let Some(res) = self.rule_memo.get(&key) {
            return res.clone();
        }

        let res = {
            let mut list = Vec::new();
            let start = pos;

            self.match_terminal(pos, input, Terminal::_macro)
                .map(|end| Node::new(Rule::Terminal, pos, end, None, None))
                .ok_or(pos)
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_IGNORE(pos, input)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_symbol(pos, input)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_IGNORE(pos, input)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.match_terminal(pos, input, Terminal::Literal_9).map(|end| Node::new(Rule::Terminal, pos, end, None, None)).ok_or(pos)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_IGNORE(pos, input)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    {
                        let mut list = Vec::new();
                        let start = pos;

                        self.rule_macro_arg(pos, input)
                            .and_then(|node| {
                                let pos = node.end;
                                list.push(node);
                                self.rule_IGNORE(pos, input)
                            })
                            .and_then(|node| {
                                let pos = node.end;
                                list.push(node);
                                {
                                    let mut list = Vec::new();
                                    let start = pos;
                                    let mut pos = pos;

                                    while let Ok(node) = {
                                        let mut list = Vec::new();
                                        let start = pos;

                                        self.match_terminal(pos, input, Terminal::Literal_1)
                                            .map(|end| Node::new(Rule::Terminal, pos, end, None, None))
                                            .ok_or(pos)
                                            .and_then(|node| {
                                                let pos = node.end;
                                                list.push(node);
                                                self.rule_IGNORE(pos, input)
                                            })
                                            .and_then(|node| {
                                                let pos = node.end;
                                                list.push(node);
                                                self.rule_macro_arg(pos, input)
                                            })
                                            .map(|node| {
                                                let end = node.end;
                                                list.push(node);

                                                Node {
                                                    rule: Rule::List,
                                                    start,
                                                    end,
                                                    children: list,
                                                    label: None,
                                                    alternative: None,
                                                }
                                            })
                                    } {
                                        if pos == node.end {
                                            // must be making progress
                                            break;
                                        }
                                        pos = node.end;
                                        list.push(node);
                                    }

                                    Ok(Node {
                                        rule: Rule::Any,
                                        start,
                                        end: pos,
                                        children: list,
                                        label: None,
                                        alternative: None,
                                    })
                                }
                            })
                            .and_then(|node| {
                                let pos = node.end;
                                list.push(node);
                                self.rule_IGNORE(pos, input)
                            })
                            .and_then(|node| {
                                let pos = node.end;
                                list.push(node);
                                self.match_terminal(pos, input, Terminal::Literal_1)
                                    .map(|end| Node::new(Rule::Terminal, pos, end, None, None))
                                    .ok_or(pos)
                                    .or_else(|_| Ok(Node::new(Rule::Terminal, pos, pos, None, None)))
                            })
                            .map(|node| {
                                let end = node.end;
                                list.push(node);

                                Node {
                                    rule: Rule::List,
                                    start,
                                    end,
                                    children: list,
                                    label: None,
                                    alternative: None,
                                }
                            })
                    }
                    .or_else(|_| Ok(Node::new(Rule::Terminal, pos, pos, None, None)))
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_IGNORE(pos, input)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.match_terminal(pos, input, Terminal::Literal_10).map(|end| Node::new(Rule::Terminal, pos, end, None, None)).ok_or(pos)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_IGNORE(pos, input)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_block(pos, input)
                })
                .map(|node| {
                    let end = node.end;
                    list.push(node);

                    Node {
                        rule: Rule::macro_define,
                        start,
                        end,
                        children: list,
                        label: None,
                        alternative: None,
                    }
                })
        };

        // Note that failure to match is also cached using Err()
        self.rule_memo.insert(key, res.clone());

        res
    }

    #[allow(non_snake_case)]
    fn rule_macro_arg(&mut self, pos: usize, input: &str) -> Result<Node, usize> {
        let key = (pos, Rule::macro_arg);

        if let Some(res) = self.rule_memo.get(&key) {
            return res.clone();
        }

        let res = {
            let mut list = Vec::new();
            let start = pos;

            self.rule_symbol(pos, input)
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_IGNORE(pos, input)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    {
                        let mut list = Vec::new();
                        let start = pos;

                        self.match_terminal(pos, input, Terminal::Literal_17)
                            .map(|end| Node::new(Rule::Terminal, pos, end, None, None))
                            .ok_or(pos)
                            .and_then(|node| {
                                let pos = node.end;
                                list.push(node);
                                self.rule_IGNORE(pos, input)
                            })
                            .and_then(|node| {
                                let pos = node.end;
                                list.push(node);
                                self.rule_symbol(pos, input)
                            })
                            .map(|node| {
                                let end = node.end;
                                list.push(node);

                                Node {
                                    rule: Rule::List,
                                    start,
                                    end,
                                    children: list,
                                    label: None,
                                    alternative: None,
                                }
                            })
                    }
                    .or_else(|_| Ok(Node::new(Rule::Terminal, pos, pos, None, None)))
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_IGNORE(pos, input)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    {
                        let mut list = Vec::new();
                        let start = pos;

                        self.match_terminal(pos, input, Terminal::Literal_8)
                            .map(|end| Node::new(Rule::Terminal, pos, end, None, None))
                            .ok_or(pos)
                            .and_then(|node| {
                                let pos = node.end;
                                list.push(node);
                                self.rule_IGNORE(pos, input)
                            })
                            .and_then(|node| {
                                let pos = node.end;
                                list.push(node);
                                self.rule_expr(pos, input)
                            })
                            .map(|node| {
                                let end = node.end;
                                list.push(node);

                                Node {
                                    rule: Rule::List,
                                    start,
                                    end,
                                    children: list,
                                    label: None,
                                    alternative: None,
                                }
                            })
                    }
                    .or_else(|_| Ok(Node::new(Rule::Terminal, pos, pos, None, None)))
                })
                .map(|node| {
                    let end = node.end;
                    list.push(node);

                    Node {
                        rule: Rule::macro_arg,
                        start,
                        end,
                        children: list,
                        label: None,
                        alternative: None,
                    }
                })
        };

        // Note that failure to match is also cached using Err()
        self.rule_memo.insert(key, res.clone());

        res
    }

    #[allow(non_snake_case)]
    fn rule_block(&mut self, pos: usize, input: &str) -> Result<Node, usize> {
        let key = (pos, Rule::block);

        if let Some(res) = self.rule_memo.get(&key) {
            return res.clone();
        }

        let res = {
            let mut list = Vec::new();
            let start = pos;

            self.match_terminal(pos, input, Terminal::Literal_0)
                .map(|end| Node::new(Rule::Terminal, pos, end, None, None))
                .ok_or(pos)
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_IGNORE(pos, input)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.match_terminal(pos, input, Terminal::_return)
                        .map(|end| Node::new(Rule::Terminal, pos, end, None, None))
                        .ok_or(pos)
                        .or_else(|_| Ok(Node::new(Rule::Terminal, pos, pos, None, None)))
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_IGNORE(pos, input)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_expr(pos, input)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_IGNORE(pos, input)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.match_terminal(pos, input, Terminal::Literal_2).map(|end| Node::new(Rule::Terminal, pos, end, None, None)).ok_or(pos)
                })
                .map(|node| {
                    let end = node.end;
                    list.push(node);

                    Node {
                        rule: Rule::block,
                        start,
                        end,
                        children: list,
                        label: None,
                        alternative: None,
                    }
                })
        };

        // Note that failure to match is also cached using Err()
        self.rule_memo.insert(key, res.clone());

        res
    }

    #[allow(non_snake_case)]
    fn rule_string(&mut self, pos: usize, input: &str) -> Result<Node, usize> {
        let key = (pos, Rule::string);

        if let Some(res) = self.rule_memo.get(&key) {
            return res.clone();
        }

        let res = self
            .match_terminal(pos, input, Terminal::Regex)
            .map(|end| Node::new(Rule::string, pos, end, None, None))
            .ok_or(pos)
            .or_else(|_| self.match_terminal(pos, input, Terminal::Regex_0).map(|end| Node::new(Rule::string, pos, end, None, None)).ok_or(pos));

        // Note that failure to match is also cached using Err()
        self.rule_memo.insert(key, res.clone());

        res
    }

    #[allow(non_snake_case)]
    fn rule_integer(&mut self, pos: usize, input: &str) -> Result<Node, usize> {
        let key = (pos, Rule::integer);

        if let Some(res) = self.rule_memo.get(&key) {
            return res.clone();
        }

        let res = self.match_terminal(pos, input, Terminal::Regex_1).map(|end| Node::new(Rule::integer, pos, end, None, None)).ok_or(pos);

        // Note that failure to match is also cached using Err()
        self.rule_memo.insert(key, res.clone());

        res
    }

    #[allow(non_snake_case)]
    fn rule_symbol_path(&mut self, pos: usize, input: &str) -> Result<Node, usize> {
        let key = (pos, Rule::symbol_path);

        if let Some(res) = self.rule_memo.get(&key) {
            return res.clone();
        }

        let res = {
            let mut list = Vec::new();
            let start = pos;

            self.rule_symbol(pos, input)
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.rule_IGNORE(pos, input)
                })
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    {
                        let mut list = Vec::new();
                        let start = pos;
                        let mut pos = pos;

                        while let Ok(node) = {
                            let mut list = Vec::new();
                            let start = pos;

                            self.match_terminal(pos, input, Terminal::Literal_29)
                                .map(|end| Node::new(Rule::Terminal, pos, end, None, None))
                                .ok_or(pos)
                                .or_else(|_| self.match_terminal(pos, input, Terminal::Literal_30).map(|end| Node::new(Rule::Terminal, pos, end, None, None)).ok_or(pos))
                                .and_then(|node| {
                                    let pos = node.end;
                                    list.push(node);
                                    self.rule_IGNORE(pos, input)
                                })
                                .and_then(|node| {
                                    let pos = node.end;
                                    list.push(node);
                                    self.rule_symbol(pos, input)
                                })
                                .map(|node| {
                                    let end = node.end;
                                    list.push(node);

                                    Node {
                                        rule: Rule::List,
                                        start,
                                        end,
                                        children: list,
                                        label: None,
                                        alternative: None,
                                    }
                                })
                        } {
                            if pos == node.end {
                                // must be making progress
                                break;
                            }
                            pos = node.end;
                            list.push(node);
                        }

                        Ok(Node {
                            rule: Rule::Any,
                            start,
                            end: pos,
                            children: list,
                            label: None,
                            alternative: None,
                        })
                    }
                })
                .map(|node| {
                    let end = node.end;
                    list.push(node);

                    Node {
                        rule: Rule::symbol_path,
                        start,
                        end,
                        children: list,
                        label: None,
                        alternative: None,
                    }
                })
        };

        // Note that failure to match is also cached using Err()
        self.rule_memo.insert(key, res.clone());

        res
    }

    #[allow(non_snake_case)]
    fn rule_symbol(&mut self, pos: usize, input: &str) -> Result<Node, usize> {
        let key = (pos, Rule::symbol);

        if let Some(res) = self.rule_memo.get(&key) {
            return res.clone();
        }

        let res = self.builtin_xid_identifier(pos, input, None, None).map(|node| Node {
            rule: Rule::symbol,
            start: node.start,
            end: node.end,
            children: vec![node],
            label: None,
            alternative: None,
        });

        // Note that failure to match is also cached using Err()
        self.rule_memo.insert(key, res.clone());

        res
    }

    #[allow(non_snake_case)]
    fn rule_IGNORE(&mut self, pos: usize, input: &str) -> Result<Node, usize> {
        let key = (pos, Rule::IGNORE);

        if let Some(res) = self.rule_memo.get(&key) {
            return res.clone();
        }

        let res = self
            .rule_SPACE(pos, input)
            .map(|node| Node {
                rule: Rule::IGNORE,
                start: node.start,
                end: node.end,
                children: vec![node],
                label: None,
                alternative: None,
            })
            .or_else(|_| {
                self.rule_NEWLINE(pos, input).map(|node| Node {
                    rule: Rule::IGNORE,
                    start: node.start,
                    end: node.end,
                    children: vec![node],
                    label: None,
                    alternative: None,
                })
            })
            .or_else(|_| {
                self.rule_COMMENT(pos, input).map(|node| Node {
                    rule: Rule::IGNORE,
                    start: node.start,
                    end: node.end,
                    children: vec![node],
                    label: None,
                    alternative: None,
                })
            })
            .or_else(|_| Ok(Node::new(Rule::Terminal, pos, pos, None, None)));

        // Note that failure to match is also cached using Err()
        self.rule_memo.insert(key, res.clone());

        res
    }

    #[allow(non_snake_case)]
    fn rule_SPACE(&mut self, pos: usize, input: &str) -> Result<Node, usize> {
        let key = (pos, Rule::SPACE);

        if let Some(res) = self.rule_memo.get(&key) {
            return res.clone();
        }

        let res = self.match_terminal(pos, input, Terminal::s).map(|end| Node::new(Rule::SPACE, pos, end, None, None)).ok_or(pos);

        // Note that failure to match is also cached using Err()
        self.rule_memo.insert(key, res.clone());

        res
    }

    #[allow(non_snake_case)]
    fn rule_NEWLINE(&mut self, pos: usize, input: &str) -> Result<Node, usize> {
        let key = (pos, Rule::NEWLINE);

        if let Some(res) = self.rule_memo.get(&key) {
            return res.clone();
        }

        let res = self.match_terminal(pos, input, Terminal::Regex_2).map(|end| Node::new(Rule::NEWLINE, pos, end, None, None)).ok_or(pos);

        // Note that failure to match is also cached using Err()
        self.rule_memo.insert(key, res.clone());

        res
    }

    #[allow(non_snake_case)]
    fn rule_COMMENT(&mut self, pos: usize, input: &str) -> Result<Node, usize> {
        let key = (pos, Rule::COMMENT);

        if let Some(res) = self.rule_memo.get(&key) {
            return res.clone();
        }

        let res = {
            let mut list = Vec::new();
            let start = pos;

            self.match_terminal(pos, input, Terminal::Literal_31)
                .map(|end| Node::new(Rule::Terminal, pos, end, None, None))
                .ok_or(pos)
                .and_then(|node| {
                    let pos = node.end;
                    list.push(node);
                    self.match_terminal(pos, input, Terminal::Regex_3).map(|end| Node::new(Rule::Terminal, pos, end, None, None)).ok_or(pos)
                })
                .map(|node| {
                    let end = node.end;
                    list.push(node);

                    Node {
                        rule: Rule::COMMENT,
                        start,
                        end,
                        children: list,
                        label: None,
                        alternative: None,
                    }
                })
        };

        // Note that failure to match is also cached using Err()
        self.rule_memo.insert(key, res.clone());

        res
    }

    fn builtin_eoi(&self, pos: usize, input: &str, label: Option<&'static str>, alternative: Option<&'static str>) -> Result<Node, usize> {
        // not worth caching
        if pos == input.len() { Ok(Node::new(Rule::EOI, pos, pos, label, alternative)) } else { Err(pos) }
    }

    fn builtin_dot(&self, pos: usize, input: &str, label: Option<&'static str>, alternative: Option<&'static str>) -> Result<Node, usize> {
        // not worth caching
        let mut chars = input[pos..].char_indices();
        if chars.next().is_some() {
            if let Some((len, _)) = chars.next() {
                Ok(Node::new(Rule::Dot, pos, pos + len, label, alternative))
            } else {
                Ok(Node::new(Rule::Dot, pos, input.len(), label, alternative))
            }
        } else {
            Err(pos)
        }
    }

    fn builtin_xid_identifier(&mut self, pos: usize, input: &str, label: Option<&'static str>, alternative: Option<&'static str>) -> Result<Node, usize> {
        let key = (pos, Rule::XID_IDENTIFIER);

        if let Some(res) = self.rule_memo.get(&key) {
            {
                let mut res = res.clone();
                if let Ok(res) = &mut res {
                    res.label = label;
                    res.alternative = alternative;
                }
                return res;
            }
        }

        let mut chars = input[pos..].char_indices();
        let mut end = pos;
        let mut res = if let Some((_, ch)) = chars.next() {
            if UnicodeXID::is_xid_start(ch) || ch == '_' {
                while {
                    if let Some((off, ch)) = chars.next() {
                        end = pos + off;
                        UnicodeXID::is_xid_continue(ch)
                    } else {
                        false
                    }
                } {}

                Ok(Node::new(Rule::XID_IDENTIFIER, pos, end, label, alternative))
            } else {
                Err(pos)
            }
        } else {
            Err(pos)
        };

        self.rule_memo.insert(key, res.clone());

        if let Ok(res) = &mut res {
            res.label = label;
            res.alternative = alternative;
        }

        res
    }

    fn match_terminal(&mut self, pos: usize, input: &str, terminal: Terminal) -> Option<usize> {
        let key = (pos, terminal);

        if let Some(res) = self.terminal_memo.get(&key) {
            return *res;
        }

        let res = if pos > input.len() {
            None
        } else {
            match terminal {
                Terminal::Literal_28 => {
                    if input[pos..].starts_with(" <-") {
                        Some(pos + " <-".len())
                    } else {
                        None
                    }
                }
                Terminal::Literal_11 => {
                    if input[pos..].starts_with('!') {
                        Some(pos + "!".len())
                    } else {
                        None
                    }
                }
                Terminal::Literal_27 => {
                    if input[pos..].starts_with('#') {
                        Some(pos + "#".len())
                    } else {
                        None
                    }
                }
                Terminal::Literal_14 => {
                    if input[pos..].starts_with('$') {
                        Some(pos + "$".len())
                    } else {
                        None
                    }
                }
                Terminal::Literal_19 => {
                    if input[pos..].starts_with('%') {
                        Some(pos + "%".len())
                    } else {
                        None
                    }
                }
                Terminal::Literal_18 => {
                    if input[pos..].starts_with('&') {
                        Some(pos + "&".len())
                    } else {
                        None
                    }
                }
                Terminal::Literal_9 => {
                    if input[pos..].starts_with('(') {
                        Some(pos + "(".len())
                    } else {
                        None
                    }
                }
                Terminal::Literal_10 => {
                    if input[pos..].starts_with(')') {
                        Some(pos + ")".len())
                    } else {
                        None
                    }
                }
                Terminal::Literal_23 => {
                    if input[pos..].starts_with('*') {
                        Some(pos + "*".len())
                    } else {
                        None
                    }
                }
                Terminal::Literal_21 => {
                    if input[pos..].starts_with('+') {
                        Some(pos + "+".len())
                    } else {
                        None
                    }
                }
                Terminal::Literal_1 => {
                    if input[pos..].starts_with(',') {
                        Some(pos + ",".len())
                    } else {
                        None
                    }
                }
                Terminal::Literal_22 => {
                    if input[pos..].starts_with('-') {
                        Some(pos + "-".len())
                    } else {
                        None
                    }
                }
                Terminal::Literal_30 => {
                    if input[pos..].starts_with('.') {
                        Some(pos + ".".len())
                    } else {
                        None
                    }
                }
                Terminal::Literal_4 => {
                    if input[pos..].starts_with('/') {
                        Some(pos + "/".len())
                    } else {
                        None
                    }
                }
                Terminal::Literal_31 => {
                    if input[pos..].starts_with("//") {
                        Some(pos + "//".len())
                    } else {
                        None
                    }
                }
                Terminal::Literal_17 => {
                    if input[pos..].starts_with(':') {
                        Some(pos + ":".len())
                    } else {
                        None
                    }
                }
                Terminal::Literal_29 => {
                    if input[pos..].starts_with("::") {
                        Some(pos + "::".len())
                    } else {
                        None
                    }
                }
                Terminal::Literal => {
                    if input[pos..].starts_with(';') {
                        Some(pos + ";".len())
                    } else {
                        None
                    }
                }
                Terminal::Literal_12 => {
                    if input[pos..].starts_with('<') {
                        Some(pos + "<".len())
                    } else {
                        None
                    }
                }
                Terminal::Literal_16 => {
                    if input[pos..].starts_with("<-") {
                        Some(pos + "<-".len())
                    } else {
                        None
                    }
                }
                Terminal::Literal_8 => {
                    if input[pos..].starts_with('=') {
                        Some(pos + "=".len())
                    } else {
                        None
                    }
                }
                Terminal::Literal_13 => {
                    if input[pos..].starts_with('>') {
                        Some(pos + ">".len())
                    } else {
                        None
                    }
                }
                Terminal::Literal_20 => {
                    if input[pos..].starts_with('?') {
                        Some(pos + "?".len())
                    } else {
                        None
                    }
                }
                Terminal::Literal_7 => {
                    if input[pos..].starts_with('@') {
                        Some(pos + "@".len())
                    } else {
                        None
                    }
                }
                Terminal::Literal_24 => {
                    if input[pos..].starts_with('[') {
                        Some(pos + "[".len())
                    } else {
                        None
                    }
                }
                Terminal::Literal_26 => {
                    if input[pos..].starts_with("\\\\") {
                        Some(pos + "\\\\".len())
                    } else {
                        None
                    }
                }
                Terminal::Literal_25 => {
                    if input[pos..].starts_with(']') {
                        Some(pos + "]".len())
                    } else {
                        None
                    }
                }
                Terminal::Literal_5 => {
                    if input[pos..].starts_with('^') {
                        Some(pos + "^".len())
                    } else {
                        None
                    }
                }
                Terminal::Literal_6 => {
                    if input[pos..].starts_with('_') {
                        Some(pos + "_".len())
                    } else {
                        None
                    }
                }
                Terminal::_as => {
                    if input[pos..].starts_with("as") {
                        Some(pos + "as".len())
                    } else {
                        None
                    }
                }
                Terminal::fragment_0 => {
                    if input[pos..].starts_with("fragment!") {
                        Some(pos + "fragment!".len())
                    } else {
                        None
                    }
                }
                Terminal::grammar_0 => {
                    if input[pos..].starts_with("grammar!") {
                        Some(pos + "grammar!".len())
                    } else {
                        None
                    }
                }
                Terminal::ignore_0 => {
                    if input[pos..].starts_with("ignore!") {
                        Some(pos + "ignore!".len())
                    } else {
                        None
                    }
                }
                Terminal::import_0 => {
                    if input[pos..].starts_with("import!") {
                        Some(pos + "import!".len())
                    } else {
                        None
                    }
                }
                Terminal::_macro => {
                    if input[pos..].starts_with("macro!") {
                        Some(pos + "macro!".len())
                    } else {
                        None
                    }
                }
                Terminal::_return => {
                    if input[pos..].starts_with("return") {
                        Some(pos + "return".len())
                    } else {
                        None
                    }
                }
                Terminal::Literal_0 => {
                    if input[pos..].starts_with('{') {
                        Some(pos + "{".len())
                    } else {
                        None
                    }
                }
                Terminal::Literal_3 => {
                    if input[pos..].starts_with('|') {
                        Some(pos + "|".len())
                    } else {
                        None
                    }
                }
                Terminal::Literal_2 => {
                    if input[pos..].starts_with('}') {
                        Some(pos + "}".len())
                    } else {
                        None
                    }
                }
                Terminal::Literal_15 => {
                    if input[pos..].starts_with('~') {
                        Some(pos + "~".len())
                    } else {
                        None
                    }
                }
                Terminal::Regex_0 => self.regex_Regex_0.find(&input[pos..]).map(|m| m.end() + pos),
                Terminal::Regex => self.regex_Regex.find(&input[pos..]).map(|m| m.end() + pos),
                Terminal::Regex_1 => self.regex_Regex_1.find(&input[pos..]).map(|m| m.end() + pos),
                Terminal::Regex_2 => self.regex_Regex_2.find(&input[pos..]).map(|m| m.end() + pos),
                Terminal::s => self.regex_s.find(&input[pos..]).map(|m| m.end() + pos),
                Terminal::Regex_3 => self.regex_Regex_3.find(&input[pos..]).map(|m| m.end() + pos),
            }
        };

        // Note that failure to match is also cached using None
        self.terminal_memo.insert(key, res);

        res
    }
}
