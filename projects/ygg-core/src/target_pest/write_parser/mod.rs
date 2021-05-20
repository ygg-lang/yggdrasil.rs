use super::*;
use self::builtin::*;

mod builtin;


pub fn write_symbol() -> std::fmt::Result {
    let mut s = String::new();

    sealed_unicode(s, "XID_START")?;
    sealed_final()

}