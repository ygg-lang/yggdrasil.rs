pub use pex::helpers::*;
use pex::{ParseResult, ParseState, StopBecause};

/// Parse the given state as a single quote string.
///
/// # Arguments
///
/// - `''`
/// - `""`
///
/// # Examples
///
/// ```
/// use pex::ParseState;
/// use yggdrasil_rt::helpers::single_quote_string;
/// single_quote_string(ParseState::new("'hello'"));
/// ```
pub fn single_quote_string<'i>(state: ParseState<'i>) -> ParseResult<&'i str> {
    let mut offset = 0;
    let mut rest = state.rest_text.chars().peekable();
    match rest.next() {
        Some('\'') => offset += 1,
        _ => StopBecause::missing_character('\'', state.start_offset)?,
    }
    let mut closed = false;
    while let Some(c) = rest.next() {
        match c {
            '\'' => {
                closed = true;
                offset += 1;
                break;
            }
            '\\' => {
                offset += 1;
                match rest.next() {
                    Some(c) => {
                        offset += 1;
                        match c {
                            '\'' | '\\' => {}
                            _ => StopBecause::unexpected_character(c, state.start_offset + offset)?,
                        }
                    }
                    None => StopBecause::unexpected_end(state.start_offset + offset)?,
                }
            }
            any => offset += any.len_utf8(),
        }
    }
    state.advance(offset)
}
