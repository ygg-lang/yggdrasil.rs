use super::*;

mod builtin;
mod final_part;

impl PestCSTW {
    pub fn write_parse(&self, f: &mut impl Write) -> std::fmt::Result {
        writeln!(f, "use super::*;")?;
        writeln!(f)?;
        self.write_rule(f)?;
        Ok(())
    }
}

#[test]
fn test() {
    let c = PestCSTW { rules: vec![String::from("WS"), String::from("WS")], ignores: vec![String::from("WS")] };
    let buffer = &mut String::new();
    c.write_parse(buffer).ok();
    println!("{}", buffer);
}
