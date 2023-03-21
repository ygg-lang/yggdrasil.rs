use super::*;

impl<'i, R> Debug for TokenStream<'i, R>
where
    R: YggdrasilRule,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("TokenStream").field("pairs", &self.clone().collect::<Vec<_>>()).finish()
    }
}
