use super::*;

impl<'i, N> Iterator for TokenTreeFilterTag<'i, N>
where
    N: YggdrasilNode<'i>,
{
    type Item = Result<N, YggdrasilError<N::Rule>>;

    fn next(&mut self) -> Option<Self::Item> {
        let pair = self.tree.next()?;
        match pair.get_tag() {
            Some(s) if self.target.eq(s) => Some(N::from_pair(pair)),
            _ => self.next(),
        }
    }
}

impl<'i, N> Iterator for TokenTreeFilterRule<'i, N>
where
    N: YggdrasilNode<'i>,
    <N as YggdrasilNode<'i>>::Rule: Clone,
{
    type Item = Result<N, YggdrasilError<N::Rule>>;

    fn next(&mut self) -> Option<Self::Item> {
        let pair = self.tree.next()?;
        if pair.get_rule().get_enum().eq(&self.target.get_enum()) {
            return Some(N::from_pair(pair));
        }
        self.next()
    }
}
