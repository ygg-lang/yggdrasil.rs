use self::builtin::*;
use yggdrasil_core::codegen::target_pest::*;

mod builtin;

#[test]
pub fn write_symbol() -> std::fmt::Result {
    let buffer = &mut String::new();
    writeln!(buffer, "use super::*;")?;
    writeln!(buffer)?;

    sealed_unicode(buffer, "XID_START")?;
    sealed_unicode(buffer, "XID_CONTINUE")?;
    writeln!(buffer)?;
    sealed_skip(buffer)?;
    writeln!(buffer)?;
    sealed_final(buffer)?;
    println!("{}", buffer);
    Ok(())
}
