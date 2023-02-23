use super::*;

impl Hash for DataKind {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            DataKind::Integer(_) => {
                state.write("DataKind::Integer".as_bytes());
            }
            DataKind::String(s) => {
                state.write("DataKind::String".as_bytes());
                state.write(s.as_bytes())
            }
            DataKind::Character(v) => {
                state.write("DataKind::Character".as_bytes());
                state.write_u32(*v as u32);
            }
            DataKind::CharacterBuiltin(_) => {
                state.write("DataKind::CharacterBuiltin".as_bytes());
            }
            DataKind::CharacterRange(v) => {
                state.write("DataKind::CharacterRange".as_bytes());
                state.write_u32(*v.start() as u32);
                state.write_u32(*v.end() as u32);
            }
            DataKind::CharacterFused(_) => {
                state.write("DataKind::CharacterFused".as_bytes());
            }
        }
    }
}

impl PartialEq<Self> for DataKind {
    fn eq(&self, other: &Self) -> bool {
        let s = RandomState::new();
        let mut h1 = s.build_hasher();
        let mut h2 = s.build_hasher();
        self.hash(&mut h1);
        other.hash(&mut h2);
        h1.finish() == h2.finish()
    }
}

impl Eq for DataKind {}

impl Serialize for DataKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            DataKind::Integer(_) => {
                unimplemented!()
            }
            DataKind::String(v) => serializer.serialize_str(v),
            DataKind::Character(v) => serializer.serialize_char(*v),
            DataKind::CharacterBuiltin(v) => {
                let mut state = serializer.serialize_tuple_struct("CharacterBuiltin", 1)?;
                state.serialize_field(v)?;
                state.end()
            }
            DataKind::CharacterRange(_) => {
                unimplemented!()
            }
            DataKind::CharacterFused(_) => {
                unimplemented!()
            }
        }
    }
}

impl<'de> Deserialize<'de> for DataKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        todo!()
    }
}

impl<'de> Visitor<'de> for DataKind {
    type Value = Self;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        write!(formatter, "Unknown type")
    }
}
