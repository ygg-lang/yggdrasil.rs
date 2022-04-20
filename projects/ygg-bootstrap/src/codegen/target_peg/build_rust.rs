use peginator::{codegen::CodegenGrammar, grammar::Grammar, PegParser, PrettyParseError};

fn ts() {
    let parsed_grammar =
        Grammar::parse(&grammar).map_err(|err| PrettyParseError::from_parse_error(&err, &grammar, source.to_str()))?;
    let generated_code = format!("{}\n{}", source_header, parsed_grammar.generate_code(&self.settings)?);
    fs::write(destination, &generated_code)?;
    if self.format {
        Command::new("rustfmt").arg(destination).status()?;
    };
}
