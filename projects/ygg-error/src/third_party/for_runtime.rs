use crate::YggdrasilErrorKind;
use diagnostic::FileID;
use yggdrasil_rt::{YggdrasilError, YggdrasilRule};

impl<R> From<YggdrasilError<R>> for crate::YggdrasilError
where
    R: YggdrasilRule,
{
    fn from(value: YggdrasilError<R>) -> Self {
        YggdrasilErrorKind::Syntax {
            message: format!("Syntax Error: {}", value.variant.to_string()),
            hint: value.variant.to_string(),
            span: FileID::default().with_range(value.location.clone()),
        }
        .into()
    }
}
