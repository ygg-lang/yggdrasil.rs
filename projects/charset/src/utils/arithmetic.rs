use crate::CharacterSet;
use std::ops::Neg;

impl Neg for CharacterSet {
    type Output = CharacterSet;

    fn neg(self) -> Self::Output {
        let mut set = self.all;
        for value in set.iter_mut() {
            *value = !*value
        }
        CharacterSet { all: set }
    }
}
