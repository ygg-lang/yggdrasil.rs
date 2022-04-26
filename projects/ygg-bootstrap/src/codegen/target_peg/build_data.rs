use super::*;

impl WritePeg for DataKind {
    fn write_peg(&self, w: &mut PegCodegen, info: &GrammarInfo) -> std::fmt::Result {
        match self {
            DataKind::CharacterAny => {
                w.write_str("char")?;
            }
            DataKind::String(s) => {
                w.write_char('"')?;
                w.write_str(s)?;
                w.write_char('"')?;
            }
            DataKind::Integer(_) => {
                unimplemented!()
            }
            DataKind::Character(c) => w.char_token(*c),
            DataKind::CharacterRange(r) => {
                todo!();

                // w.char_token(r.start);
                // w.write_str("..")?;
                // w.char_token(r.end);
            }
            DataKind::Ignored => {
                todo!()
            }
            DataKind::Boolean(_) => {
                todo!()
            }
            DataKind::StringFused(_) => {
                todo!()
            }
            DataKind::CharacterBuiltin(_) => {
                todo!()
            }
            DataKind::CharacterFused(_) => {
                todo!()
            }
        }
        Ok(())
    }
}

impl WritePeg for RuleReference {
    fn write_peg(&self, w: &mut PegCodegen, info: &GrammarInfo) -> std::fmt::Result {
        todo!()
    }
}
