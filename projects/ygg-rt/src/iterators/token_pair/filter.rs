use super::*;

impl<'i, N> Iterator for TokenTreeFilterTags<'i, N>
where
    N: YggdrasilNode,
{
    type Item = Result<N, YggdrasilError<N::Rule>>;

    fn next(&mut self) -> Option<Self::Item> {
        let pair = self.tree.next()?;
        let tag = pair.get_tag()?;
        if tag.eq(&self.target) {
            return Some(N::from_pair(pair));
        }
        self.next()
    }
}

impl<'i, N> Iterator for TokenTreeFilterRules<'i, N>
where
    N: YggdrasilNode,
{
    type Item = Result<N, YggdrasilError<N::Rule>>;

    fn next(&mut self) -> Option<Self::Item> {
        let pair = self.tree.next()?;
        if pair.get_rule().eq(&self.target) {
            return Some(N::from_pair(pair));
        }
        self.next()
    }
}
