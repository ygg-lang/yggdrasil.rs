use super::*;

mod builtin;
mod final_part;

impl PestCST {
    pub fn write_parse(&self, f: &mut impl Write) -> std::fmt::Result {
        writeln!(f, "use super::*;")?;
        writeln!(f)?;
        self.write_final(f)?;
        Ok(())
    }
}

#[test]
fn test() {
    let c = PestCST { ignores: vec![String::from("WS")] };
    let buffer = &mut String::new();
    c.write_parse(buffer).ok();
    println!("{}", buffer);
}
