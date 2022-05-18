use std::{fs::File, io::prelude::*, path::Path};

#[test]
fn ready() {
    println!("it, works!")
}

pub fn convert(input: &str) -> anyhow::Result<()> {
    pest_meta::parse_and_optimize(include_str!("grammar.pest"))?;
    Ok(())
}
