use super::*;

impl<'i> AstBuilder<'i> for StringNormalNode<'i> {
    type Output = YggdrasilText;

    fn build(&self, ctx: &ParseContext, state: &mut ParseState) -> Result<Self::Output> {
        let mut buffer = String::with_capacity(self.string_item().len() * 2);
        for item in self.string_item() {
            match item.build(ctx, state) {
                Ok(o) => buffer.push_str(&o),
                Err(e) => state.add_error(e),
            }
        }
        Ok(YggdrasilText::new(buffer, self.get_range()))
    }
}

impl<'i> AstBuilder<'i> for StringItemNode<'i> {
    type Output = Cow<'i, str>;

    fn build(&self, ctx: &ParseContext, state: &mut ParseState) -> Result<Self::Output> {
        let cow = match self {
            Self::EscapedCharacter(unicode) => Cow::Owned(unicode.build(ctx, state)?.to_string()),
            Self::EscapedUnicode(unicode) => Cow::Owned(unicode.build(ctx, state)?.to_string()),
            Self::TextAny(s) => Cow::Borrowed(s.get_str()),
        };
        Ok(cow)
    }
}

impl<'i> AstBuilder<'i> for EscapedCharacterNode<'i> {
    type Output = char;

    fn build(&self, _: &ParseContext, _: &mut ParseState) -> Result<Self::Output> {
        let c = match self.get_chars().last() {
            Some(c) => match c {
                'r' => '\r',
                'n' => '\n',
                _ => c,
            },
            None => unreachable!(),
        };
        Ok(c)
    }
}

impl<'i> AstBuilder<'i> for EscapedUnicodeNode<'i> {
    type Output = char;

    fn build(&self, ctx: &ParseContext, _: &mut ParseState) -> Result<Self::Output> {
        match u32::from_str_radix(self.get_str(), 16) {
            Ok(o) => match char::from_u32(o) {
                Some(s) => Ok(s),
                None => Err(YggdrasilError::syntax_error("out of range", self.get_range()).with_file(ctx.file)),
            },
            Err(_) => Err(YggdrasilError::syntax_error("not unicode", self.get_range()).with_file(ctx.file)),
        }
    }
}
