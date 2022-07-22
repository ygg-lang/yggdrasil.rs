use super::*;

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct ConcatExpression {
    sequence: Vec<ExpressionNode>,
}

impl<'i> IntoIterator for &'i ConcatExpression {
    type Item = &'i ExpressionNode;
    type IntoIter = Iter<'i, ExpressionNode>;

    fn into_iter(self) -> Self::IntoIter {
        self.sequence.iter()
    }
}

impl<'i> IntoIterator for &'i mut ConcatExpression {
    type Item = &'i mut ExpressionNode;
    type IntoIter = IterMut<'i, ExpressionNode>;

    fn into_iter(self) -> Self::IntoIter {
        self.sequence.iter_mut()
    }
}

impl ConcatExpression {
    pub fn new(lhs: impl Into<ExpressionNode>, rhs: impl Into<ExpressionNode>, soft: bool) -> Self {
        let mut sequence = vec![];
        sequence.push(lhs.into());
        if soft {
            sequence.push(ExpressionNode::ignored())
        }
        sequence.push(rhs.into());
        Self { sequence }
    }
    pub fn to_node<S>(self, tag: S) -> ExpressionNode
    where
        S: Into<String>,
    {
        ExpressionNode { tag: tag.into(), kind: ExpressionKind::Concat(Box::new(self)) }
    }
}

impl Add<Self> for ExpressionNode {
    type Output = Self;
    /// soft concat
    #[inline(never)]
    fn add(self, other: Self) -> Self::Output {
        join(self, other, true)
    }
}

impl BitAnd<Self> for ExpressionNode {
    type Output = Self;
    /// atomic concat
    #[inline(never)]
    fn bitand(self, other: Self) -> Self::Output {
        join(self, other, false)
    }
}

// add extra ignore if is a soft concat
#[inline(always)]
fn join(mut lhs: ExpressionNode, mut rhs: ExpressionNode, soft: bool) -> ExpressionNode {
    match (&mut lhs.kind, &mut rhs.kind) {
        (ExpressionKind::Concat(a), ExpressionKind::Concat(b)) => {
            if soft {
                a.sequence.push(ExpressionNode::ignored())
            }
            a.sequence.extend(b.sequence.iter().cloned());
            lhs
        }
        (ExpressionKind::Concat(a), _) => {
            if soft {
                a.sequence.push(ExpressionNode::ignored())
            }
            a.sequence.push(rhs);
            lhs
        }
        // a:A b:B
        (_, ExpressionKind::Concat(b)) => {
            let mut sequence = vec![];
            sequence.push(lhs);
            if soft {
                sequence.push(ExpressionNode::ignored())
            }
            sequence.extend(b.sequence.iter().cloned());
            ConcatExpression { sequence }.to_node("")
        }
        (_, _) => {
            let mut sequence = vec![];
            sequence.push(lhs);
            if soft {
                sequence.push(ExpressionNode::ignored())
            }
            sequence.push(rhs);
            ConcatExpression { sequence }.to_node("")
        }
    }
}
