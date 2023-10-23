use super::*;

#[derive(Clone)]
pub struct Utf8View<'i> {
    utf8: Peekable<Chars<'i>>,
    offset: u32,
}

impl<'i> InputStream for Utf8View<'i> {
    fn read<R>(&mut self) -> Result<Character, YggdrasilError<R>> {
        match self.utf8.next() {
            Some(s) => {
                let out = Character { unicode: s, offset: self.offset };
                self.offset += len_utf8(s);
                Ok(out)
            }
            None => {
                todo!()
            }
        }
    }

    fn peek<R>(&mut self) -> Result<Character, YggdrasilError<R>> {
        match self.utf8.peek() {
            Some(s) => {
                let out = Character { unicode: *s, offset: self.offset };
                self.offset += len_utf8(*s);
                Ok(out)
            }
            None => {
                todo!()
            }
        }
    }
}

const MAX_ONE_B: u32 = 0x80;
const MAX_TWO_B: u32 = 0x800;
const MAX_THREE_B: u32 = 0x10000;

#[inline]
const fn len_utf8(char: char) -> u32 {
    let code = char as u32;
    if code < MAX_ONE_B {
        1
    }
    else if code < MAX_TWO_B {
        2
    }
    else if code < MAX_THREE_B {
        3
    }
    else {
        4
    }
}
