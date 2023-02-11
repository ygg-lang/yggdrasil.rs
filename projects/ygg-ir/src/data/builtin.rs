use super::*;

impl ExpressionKind {
    pub fn rule(name: &str) -> Self {
        let data = match name {
            "ANY" => DataKind::CharacterAny,
            "XID_START" | "XID_CONTINUE" => DataKind::CharacterBuiltin(name.to_string()),
            _ => {
                todo!();
            }
        };
        Self::Data(Box::new(data))
    }
}
