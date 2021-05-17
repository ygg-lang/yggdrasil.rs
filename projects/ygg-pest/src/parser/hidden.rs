#![allow(non_snake_case, unused_variables)]

use super::*;

#[inline]
pub fn skip(state: RuleState) -> RuleResult {
    Ok(state)
}
