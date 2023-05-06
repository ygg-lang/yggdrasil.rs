use yggdrasil_rt::{YggdrasilError, YggdrasilRule};

impl<R> From<YggdrasilError<R>> for crate::YggdrasilError
where
    R: YggdrasilRule,
{
    fn from(value: YggdrasilError<R>) -> Self {
        crate::YggdrasilError::runtime_error(value)
    }
}
