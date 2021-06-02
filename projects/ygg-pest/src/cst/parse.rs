use super::*;

#[inline]
pub fn program(s: RuleState) -> RuleResult {
    s.rule(Rule::program, |s| {
        s.sequence(|s| {
            self::SOI(s)
                .and_then(|s| self::SKIP(s))
                .and_then(|s| s.sequence(|s| s.optional(|s| self::statement(s).and_then(|s| s.repeat(|s| s.sequence(|s| self::SKIP(s).and_then(|s| self::statement(s))))))))
                .and_then(|s| self::SKIP(s))
                .and_then(|s| self::EOI(s))
        })
    })
}

#[inline]
pub fn statement(s: RuleState) -> RuleResult {
    s.rule(Rule::statement, |s| {
        self::comment_doc(s)
            .or_else(|s| self::macro_call(s))
            .or_else(|s| self::macro_define(s))
            .or_else(|s| self::empty_statement(s))
            .or_else(|s| s.sequence(|s| self::grammar_statement(s).and_then(|s| self::SKIP(s)).and_then(|s| s.optional(|s| self::eos(s)))))
            .or_else(|s| s.sequence(|s| self::fragment_statement(s).and_then(|s| self::SKIP(s)).and_then(|s| s.optional(|s| self::eos(s)))))
            .or_else(|s| s.sequence(|s| self::ignore_statement(s).and_then(|s| self::SKIP(s)).and_then(|s| s.optional(|s| self::eos(s)))))
            .or_else(|s| s.sequence(|s| self::assign_statement(s).and_then(|s| self::SKIP(s)).and_then(|s| s.optional(|s| self::eos(s)))))
    })
}

#[inline]
pub fn empty_statement(s: RuleState) -> RuleResult {
    s.rule(Rule::empty_statement, |s| self::eos(s))
}

#[inline]
pub fn eos(s: RuleState) -> RuleResult {
    s.rule(Rule::eos, |s| s.atomic(Atomicity::Atomic, |s| s.match_string(";")))
}

#[inline]
pub fn grammar_statement(s: RuleState) -> RuleResult {
    s.rule(Rule::grammar_statement, |s| {
        s.sequence(|s| {
            self::grammar(s).and_then(|s| self::SKIP(s)).and_then(|s| self::SYMBOL(s)).and_then(|s| self::SKIP(s)).and_then(|s| {
                self::string(s).or_else(|s| {
                    s.sequence(|s| {
                        s.match_string("{")
                            .and_then(|s| self::SKIP(s))
                            .and_then(|s| {
                                s.optional(|s| {
                                    s.sequence(|s| {
                                        self::string(s)
                                            .and_then(|s| self::SKIP(s))
                                            .and_then(|s| {
                                                s.sequence(|s| {
                                                    s.optional(|s| {
                                                        s.sequence(|s| s.match_string(",").and_then(|s| self::SKIP(s)).and_then(|s| self::string(s)))
                                                            .and_then(|s| s.repeat(|s| s.sequence(|s| self::SKIP(s).and_then(|s| s.sequence(|s| s.match_string(",").and_then(|s| self::SKIP(s)).and_then(|s| self::string(s)))))))
                                                    })
                                                })
                                            })
                                            .and_then(|s| self::SKIP(s))
                                            .and_then(|s| s.optional(|s| s.match_string(",")))
                                    })
                                })
                            })
                            .and_then(|s| self::SKIP(s))
                            .and_then(|s| s.match_string("}"))
                    })
                })
            })
        })
    })
}

#[inline]
pub fn grammar(s: RuleState) -> RuleResult {
    s.match_string("grammar!")
}

#[inline]
pub fn fragment_statement(s: RuleState) -> RuleResult {
    s.rule(Rule::fragment_statement, |s| s.sequence(|s| self::fragment(s).and_then(|s| self::SKIP(s)).and_then(|s| self::SYMBOL(s))))
}

#[inline]
pub fn fragment(s: RuleState) -> RuleResult {
    s.match_string("fragment!")
}

#[inline]
pub fn import_statement(s: RuleState) -> RuleResult {
    s.rule(Rule::import_statement, |s| {
        s.sequence(|s| {
            self::import(s).and_then(|s| self::SKIP(s)).and_then(|s| {
                self::string(s).or_else(|s| {
                    s.sequence(|s| {
                        self::string(s)
                            .and_then(|s| self::SKIP(s))
                            .and_then(|s| s.match_string("{"))
                            .and_then(|s| self::SKIP(s))
                            .and_then(|s| {
                                s.optional(|s| {
                                    s.sequence(|s| {
                                        self::symbol_alias(s)
                                            .and_then(|s| self::SKIP(s))
                                            .and_then(|s| {
                                                s.sequence(|s| {
                                                    s.optional(|s| {
                                                        s.sequence(|s| s.match_string(",").and_then(|s| self::SKIP(s)).and_then(|s| self::symbol_alias(s)))
                                                            .and_then(|s| s.repeat(|s| s.sequence(|s| self::SKIP(s).and_then(|s| s.sequence(|s| s.match_string(",").and_then(|s| self::SKIP(s)).and_then(|s| self::symbol_alias(s)))))))
                                                    })
                                                })
                                            })
                                            .and_then(|s| self::SKIP(s))
                                            .and_then(|s| s.optional(|s| s.match_string(",")))
                                    })
                                })
                            })
                            .and_then(|s| self::SKIP(s))
                            .and_then(|s| s.match_string("}"))
                    })
                })
            })
        })
    })
}

#[inline]
pub fn import(s: RuleState) -> RuleResult {
    s.match_string("import!")
}

#[inline]
pub fn ignore_statement(s: RuleState) -> RuleResult {
    s.rule(Rule::ignore_statement, |s| {
        s.sequence(|s| {
            self::UNNAMED(s, "ignore!").and_then(|s| self::SKIP(s)).and_then(|s| {
                self::SYMBOL(s).or_else(|s| {
                    s.sequence(|s| {
                        s.match_string("{")
                            .and_then(|s| self::SKIP(s))
                            .and_then(|s| {
                                s.optional(|s| {
                                    s.sequence(|s| {
                                        self::SYMBOL(s)
                                            .and_then(|s| self::SKIP(s))
                                            .and_then(|s| {
                                                s.sequence(|s| {
                                                    s.optional(|s| {
                                                        s.sequence(|s| s.match_string(",").and_then(|s| self::SKIP(s)).and_then(|s| self::SYMBOL(s)))
                                                            .and_then(|s| s.repeat(|s| s.sequence(|s| self::SKIP(s).and_then(|s| s.sequence(|s| s.match_string(",").and_then(|s| self::SKIP(s)).and_then(|s| self::SYMBOL(s)))))))
                                                    })
                                                })
                                            })
                                            .and_then(|s| self::SKIP(s))
                                            .and_then(|s| s.optional(|s| s.match_string(",")))
                                    })
                                })
                            })
                            .and_then(|s| self::SKIP(s))
                            .and_then(|s| s.match_string("}"))
                    })
                })
            })
        })
    })
}

#[inline]
pub fn ignore(s: RuleState) -> RuleResult {
    self::UNNAMED(s, "ignore!")
}

#[inline]
pub fn UNNAMED<'i>(s: RuleState<'i>, input: &'i str) -> RuleResult<'i> {
    s.rule(Rule::UNNAMED, |s| s.match_string(input))
}

#[inline]
pub fn UNNAMED2<'i>(s: RuleState<'i>, input: &'i str) -> RuleResult<'i> {
    s.match_string(input)
}

#[inline]
pub fn assign_statement(s: RuleState) -> RuleResult {
    s.rule(Rule::assign_statement, |s| {
        s.sequence(|s| self::SYMBOL(s).and_then(|s| self::SKIP(s)).and_then(|s| self::assign_kind(s)).and_then(|s| self::SKIP(s)).and_then(|s| s.optional(|s| s.match_string("|"))).and_then(|s| self::SKIP(s)).and_then(|s| self::expr(s)))
    })
}

#[inline]
pub fn assign_kind(s: RuleState) -> RuleResult {
    s.rule(Rule::assign_kind, |s| s.atomic(Atomicity::Atomic, |s| s.sequence(|s| s.repeat(|s| s.match_string("^").or_else(|s| s.match_string("_")).or_else(|s| s.match_string("@"))).and_then(|s| s.match_string("=")))))
}

#[inline]
#[rustfmt::skip]
pub fn expr(s: RuleState) -> RuleResult {
    s.rule(Rule::expr, |s| {
        s.sequence(|s|
            s.match_string("(").and_then(|s| self::SKIP(s)).and_then(|s| s.optional(|s| s.match_string("|"))).and_then(|s| self::SKIP(s)).and_then(|s| self::expr(s)).and_then(|s| self::SKIP(s)).and_then(|s| s.match_string(")")))
            .and_then(|s| s.tag_branch("Priority"))
            .or_else(|s| { s.sequence(|s| { self::data(s).and_then(|s| self::SKIP(s)).and_then(|s| s.optional(|s| s.sequence(|s| s.match_string(":").and_then(|s| self::SKIP(s)).and_then(|s| self::SYMBOL(s))))).and_then(|s| self::SKIP(s)).and_then(|s| s.match_string("<-")).and_then(|s| self::SKIP(s)).and_then(|s| self::expr(s)) }) })
            .and_then(|s| s.tag_branch("Mark"))
            .or_else(|s| { s.sequence(|s| { self::data(s).and_then(|s| self::SKIP(s)).and_then(|s| s.match_string("|").or_else(|s| s.match_string("/"))).and_then(|s| self::SKIP(s)).and_then(|s| self::expr(s)).and_then(|s| self::SKIP(s)).and_then(|s| s.optional(|s| s.sequence(|s| s.match_string("#").and_then(|s| self::SKIP(s)).and_then(|s| self::SYMBOL(s))))).and_then(|s| self::SKIP(s)).and_then(|s| s.optional(|s| s.sequence(|s| s.match_string(":").and_then(|s| self::SKIP(s)).and_then(|s| self::SYMBOL(s))))) }) })
            .and_then(|s| s.tag_branch("Choice"))
            .or_else(|s| s.sequence(|s| self::data(s).and_then(|s| self::SKIP(s)).and_then(|s| s.match_string("~")).and_then(|s| self::SKIP(s)).and_then(|s| self::expr(s))))
            .and_then(|s| s.tag_branch("Concat"))
            .or_else(|s| s.sequence(|s| self::data(s).and_then(|s| self::SKIP(s)).and_then(|s| self::slice(s))))
            .and_then(|s| s.tag_branch("Slice"))
            .or_else(|s| s.sequence(|s| self::data(s).and_then(|s| self::SKIP(s)).and_then(|s| self::suffix(s))))
            .and_then(|s| s.tag_branch("Suffix"))
            .or_else(|s| s.sequence(|s| self::prefix(s).and_then(|s| self::SKIP(s)).and_then(|s| self::expr(s))))
            .and_then(|s| s.tag_branch("Prefix"))
            .or_else(|s| self::data(s))
            .and_then(|s| s.tag_branch("Data"))
    })
}

#[inline]
pub fn prefix(s: RuleState) -> RuleResult {
    s.rule(Rule::prefix, |s| s.atomic(Atomicity::Atomic, |s| s.match_string("!").or_else(|s| s.match_string("&")).or_else(|s| s.match_string("^")).or_else(|s| s.match_string("*")).or_else(|s| s.match_string("%"))))
}

#[inline]
pub fn suffix(s: RuleState) -> RuleResult {
    s.rule(Rule::suffix, |s| s.atomic(Atomicity::Atomic, |s| s.match_string("?").or_else(|s| s.match_string("+")).or_else(|s| s.match_string("-")).or_else(|s| s.match_string("*"))))
}

#[inline]
pub fn data(s: RuleState) -> RuleResult {
    s.rule(Rule::data, |s| self::macro_call(s).or_else(|s| self::regex_range(s)).or_else(|s| self::list(s)).or_else(|s| self::symbol_path(s)).or_else(|s| self::integer(s)))
}

#[inline]
pub fn list(s: RuleState) -> RuleResult {
    s.rule(Rule::list, |s| {
        s.sequence(|s| {
            s.match_string("{")
                .and_then(|s| self::SKIP(s))
                .and_then(|s| {
                    s.optional(|s| {
                        s.sequence(|s| {
                            self::data(s)
                                .and_then(|s| self::SKIP(s))
                                .and_then(|s| {
                                    s.sequence(|s| {
                                        s.optional(|s| {
                                            s.sequence(|s| s.match_string(",").and_then(|s| self::SKIP(s)).and_then(|s| self::data(s)))
                                                .and_then(|s| s.repeat(|s| s.sequence(|s| self::SKIP(s).and_then(|s| s.sequence(|s| s.match_string(",").and_then(|s| self::SKIP(s)).and_then(|s| self::data(s)))))))
                                        })
                                    })
                                })
                                .and_then(|s| self::SKIP(s))
                                .and_then(|s| s.optional(|s| s.match_string(",")))
                        })
                    })
                })
                .and_then(|s| self::SKIP(s))
                .and_then(|s| s.match_string("}"))
        })
    })
}

#[inline]
pub fn slice(s: RuleState) -> RuleResult {
    s.rule(Rule::slice, |s| {
        s.sequence(|s| {
            s.match_string("{")
                .and_then(|s| self::SKIP(s))
                .and_then(|s| self::integer(s))
                .and_then(|s| self::SKIP(s))
                .and_then(|s| s.match_string(","))
                .and_then(|s| self::SKIP(s))
                .and_then(|s| self::integer(s))
                .and_then(|s| self::SKIP(s))
                .and_then(|s| s.match_string("}"))
        })
    })
}

#[inline]
pub fn regex_range(s: RuleState) -> RuleResult {
    s.rule(Rule::regex_range, |s| {
        s.atomic(Atomicity::Atomic, |s| {
            s.sequence(|s| {
                s.match_string("[")
                    .and_then(|s| s.repeat(|s| s.sequence(|s| s.lookahead(false, |s| s.match_string("]")).and_then(|s| self::ANY(s))).or_else(|s| s.sequence(|s| s.match_string("\\").and_then(|s| self::ANY(s))))))
                    .and_then(|s| s.match_string("]"))
            })
        })
    })
}

#[inline]
pub fn macro_call(s: RuleState) -> RuleResult {
    s.rule(Rule::macro_call, |s| {
        s.sequence(|s| {
            s.match_string("@")
                .and_then(|s| self::SKIP(s))
                .and_then(|s| self::symbol_path(s))
                .and_then(|s| self::SKIP(s))
                .and_then(|s| s.match_string("("))
                .and_then(|s| self::SKIP(s))
                .and_then(|s| {
                    s.optional(|s| {
                        s.sequence(|s| {
                            self::macro_kv(s)
                                .and_then(|s| self::SKIP(s))
                                .and_then(|s| {
                                    s.sequence(|s| {
                                        s.optional(|s| {
                                            s.sequence(|s| s.match_string(",").and_then(|s| self::SKIP(s)).and_then(|s| self::macro_kv(s)))
                                                .and_then(|s| s.repeat(|s| s.sequence(|s| self::SKIP(s).and_then(|s| s.sequence(|s| s.match_string(",").and_then(|s| self::SKIP(s)).and_then(|s| self::macro_kv(s)))))))
                                        })
                                    })
                                })
                                .and_then(|s| self::SKIP(s))
                                .and_then(|s| s.optional(|s| s.match_string(",")))
                        })
                    })
                })
                .and_then(|s| self::SKIP(s))
                .and_then(|s| s.match_string(")"))
        })
    })
}

#[inline]
pub fn macro_define(s: RuleState) -> RuleResult {
    s.rule(Rule::macro_define, |s| {
        s.sequence(|s| {
            s.match_string("macro!")
                .and_then(|s| self::SKIP(s))
                .and_then(|s| self::symbol_path(s))
                .and_then(|s| self::SKIP(s))
                .and_then(|s| s.match_string("("))
                .and_then(|s| self::SKIP(s))
                .and_then(|s| {
                    s.optional(|s| {
                        s.sequence(|s| {
                            self::macro_arg(s)
                                .and_then(|s| self::SKIP(s))
                                .and_then(|s| {
                                    s.sequence(|s| {
                                        s.optional(|s| {
                                            s.sequence(|s| s.match_string(",").and_then(|s| self::SKIP(s)).and_then(|s| self::macro_arg(s)))
                                                .and_then(|s| s.repeat(|s| s.sequence(|s| self::SKIP(s).and_then(|s| s.sequence(|s| s.match_string(",").and_then(|s| self::SKIP(s)).and_then(|s| self::macro_arg(s)))))))
                                        })
                                    })
                                })
                                .and_then(|s| self::SKIP(s))
                                .and_then(|s| s.optional(|s| s.match_string(",")))
                        })
                    })
                })
                .and_then(|s| self::SKIP(s))
                .and_then(|s| s.match_string(")"))
                .and_then(|s| self::SKIP(s))
                .and_then(|s| self::block(s))
        })
    })
}

#[inline]
pub fn macro_kv(s: RuleState) -> RuleResult {
    s.rule(Rule::macro_kv, |s| s.sequence(|s| self::SYMBOL(s).and_then(|s| self::SKIP(s)).and_then(|s| s.match_string("=")).and_then(|s| self::SKIP(s)).and_then(|s| self::expr(s))).or_else(|s| self::expr(s)))
}

#[inline]
pub fn macro_arg(s: RuleState) -> RuleResult {
    s.rule(Rule::macro_arg, |s| {
        s.sequence(|s| {
            self::SYMBOL(s)
                .and_then(|s| self::SKIP(s))
                .and_then(|s| s.optional(|s| s.sequence(|s| s.match_string(":").and_then(|s| self::SKIP(s)).and_then(|s| self::SYMBOL(s)))))
                .and_then(|s| self::SKIP(s))
                .and_then(|s| s.optional(|s| s.sequence(|s| s.match_string("=").and_then(|s| self::SKIP(s)).and_then(|s| self::expr(s)))))
        })
    })
}

#[inline]
pub fn block(s: RuleState) -> RuleResult {
    s.rule(Rule::block, |s| {
        s.sequence(|s| s.match_string("{").and_then(|s| self::SKIP(s)).and_then(|s| s.optional(|s| s.match_string("return"))).and_then(|s| self::SKIP(s)).and_then(|s| self::expr(s)).and_then(|s| self::SKIP(s)).and_then(|s| s.match_string("}")))
    })
}

#[inline]
pub fn string(s: RuleState) -> RuleResult {
    s.rule(Rule::string, |s| {
        s.atomic(Atomicity::Atomic, |s| {
            s.sequence(|s| {
                s.match_string("'")
                    .and_then(|s| s.repeat(|s| s.sequence(|s| s.lookahead(false, |s| s.match_string("'")).and_then(|s| self::ANY(s))).or_else(|s| s.sequence(|s| s.match_string("\\").and_then(|s| self::ANY(s))))))
                    .and_then(|s| s.match_string("'"))
            })
            .or_else(|s| {
                s.sequence(|s| {
                    s.match_string("\"")
                        .and_then(|s| s.repeat(|s| s.sequence(|s| s.lookahead(false, |s| s.match_string("\"")).and_then(|s| self::ANY(s))).or_else(|s| s.sequence(|s| s.match_string("\\").and_then(|s| self::ANY(s))))))
                        .and_then(|s| s.match_string("\""))
                })
            })
        })
    })
}

#[inline]
pub fn integer(s: RuleState) -> RuleResult {
    s.rule(Rule::integer, |s| {
        s.atomic(Atomicity::Atomic, |s| s.match_string("0").or_else(|s| s.sequence(|s| self::ASCII_NONZERO_DIGIT(s).and_then(|s| s.repeat(|s| s.sequence(|s| s.optional(|s| s.match_string("_")).and_then(|s| self::ASCII_DIGIT(s))))))))
    })
}

#[inline]
pub fn special(s: RuleState) -> RuleResult {
    s.rule(Rule::special, |s| s.atomic(Atomicity::Atomic, |s| s.match_string("true").or_else(|s| s.match_string("false")).or_else(|s| s.match_string("null"))))
}

#[inline]
pub fn comment_doc(s: RuleState) -> RuleResult {
    s.rule(Rule::comment_doc, |s| s.atomic(Atomicity::Atomic, |s| s.sequence(|s| s.match_string("///").and_then(|s| s.repeat(|s| s.sequence(|s| s.lookahead(false, |s| self::NEWLINE(s)).and_then(|s| self::ANY(s))))))))
}

#[inline]
pub fn comment_s_l(s: RuleState) -> RuleResult {
    s.rule(Rule::comment_s_l, |s| s.atomic(Atomicity::Atomic, |s| s.sequence(|s| s.match_string("//").and_then(|s| s.repeat(|s| s.sequence(|s| s.lookahead(false, |s| self::NEWLINE(s)).and_then(|s| self::ANY(s))))))))
}

#[inline]
pub fn comment_m_l(s: RuleState) -> RuleResult {
    s.rule(Rule::comment_m_l, |s| {
        s.atomic(Atomicity::Atomic, |s| {
            s.sequence(|s| s.match_string("/*").and_then(|s| s.repeat(|s| self::comment_m_l(s).or_else(|s| s.sequence(|s| s.lookahead(false, |s| s.match_string("*/")).and_then(|s| self::ANY(s)))))).and_then(|s| s.match_string("*/")))
        })
    })
}

#[inline]
pub fn COMMENT(s: RuleState) -> RuleResult {
    s.rule(Rule::COMMENT, |s| s.atomic(Atomicity::Atomic, |s| self::comment_s_l(s).or_else(|s| self::comment_m_l(s))))
}

#[inline]
pub fn symbol_path(s: RuleState) -> RuleResult {
    s.rule(Rule::symbol_path, |s| {
        s.sequence(|s| {
            self::SYMBOL(s).and_then(|s| self::SKIP(s)).and_then(|s| {
                s.sequence(|s| {
                    s.optional(|s| {
                        s.sequence(|s| s.match_string("::").or_else(|s| s.match_string(".")).and_then(|s| self::SKIP(s)).and_then(|s| self::SYMBOL(s)))
                            .and_then(|s| s.repeat(|s| s.sequence(|s| self::SKIP(s).and_then(|s| s.sequence(|s| s.match_string("::").or_else(|s| s.match_string(".")).and_then(|s| self::SKIP(s)).and_then(|s| self::SYMBOL(s)))))))
                    })
                })
            })
        })
    })
}

#[inline]
pub fn symbol_alias(s: RuleState) -> RuleResult {
    s.rule(Rule::symbol_alias, |s| s.sequence(|s| self::SYMBOL(s).and_then(|s| self::SKIP(s)).and_then(|s| s.optional(|s| s.sequence(|s| s.match_string("as").and_then(|s| self::SKIP(s)).and_then(|s| self::SYMBOL(s)))))))
}

#[inline]
pub fn SYMBOL(s: RuleState) -> RuleResult {
    s.rule(Rule::SYMBOL, |s| s.atomic(Atomicity::Atomic, |s| s.sequence(|s| s.match_string("_").or_else(|s| self::XID_START(s)).and_then(|s| s.repeat(|s| self::XID_CONTINUE(s))))))
}

#[inline]
pub fn WHITESPACE(s: RuleState) -> RuleResult {
    s.atomic(Atomicity::CompoundAtomic, |s| s.rule(Rule::WHITESPACE, |s| self::COMMENT(s).or_else(|s| self::WHITE_SPACE(s)).or_else(|s| self::NEWLINE(s))))
}

#[inline]
pub fn WHITE_SPACE(s: RuleState) -> RuleResult {
    s.rule(Rule::WHITE_SPACE, |s| s.atomic(Atomicity::Atomic, |s| s.sequence(|s| s.match_string(" ").or_else(|s| s.match_string("\t")).and_then(|s| s.repeat(|s| s.match_string(" ").or_else(|s| s.match_string("\t")))))))
}

#[inline]
pub fn NEWLINE(s: RuleState) -> RuleResult {
    s.rule(Rule::NEWLINE, |s| {
        s.atomic(Atomicity::Atomic, |s| {
            s.sequence(|s| s.match_string("\r\n").or_else(|s| s.match_string("\r")).or_else(|s| s.match_string("\n")).and_then(|s| s.repeat(|s| s.match_string("\r\n").or_else(|s| s.match_string("\r")).or_else(|s| s.match_string("\n")))))
        })
    })
}

#[inline]
pub fn ASCII_DIGIT(s: RuleState) -> RuleResult {
    s.match_range('0'..'9')
}

#[inline]
pub fn ASCII_NONZERO_DIGIT(s: RuleState) -> RuleResult {
    s.match_range('1'..'9')
}

#[inline]
fn XID_CONTINUE(s: RuleState) -> RuleResult {
    s.match_char_by(::pest::unicode::XID_CONTINUE)
}

#[inline]
fn XID_START(s: RuleState) -> RuleResult {
    s.match_char_by(::pest::unicode::XID_START)
}

#[inline]
pub fn SKIP(state: RuleState) -> RuleResult {
    match state.atomicity() == Atomicity::NonAtomic {
        true => state.repeat(|state| self::WHITESPACE(state)),
        false => Ok(state),
    }
}

#[inline]
pub fn ANY(s: RuleState) -> RuleResult {
    s.skip(1)
}

#[inline]
pub fn EOI(s: RuleState) -> RuleResult {
    s.rule(Rule::EOI, |s| s.end_of_input())
}

#[inline]
pub fn SOI(s: RuleState) -> RuleResult {
    s.start_of_input()
}
