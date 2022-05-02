use super::*;

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct ConcatExpression {
    pub sequence: Vec<ExpressionNode>,
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
    pub fn push(&mut self, other: ExpressionNode, soft: bool) {
        if soft {
            self.sequence.push(ExpressionNode::ignored())
        }
        match other.kind {
            ExpressionKind::Concat(rhs) => self.sequence.extend(rhs.sequence.into_iter()),
            _ => self.sequence.push(other),
        }
    }
    pub fn append(&mut self, other: ExpressionNode, soft: bool) {
        let ignored = if soft { vec![ExpressionNode::ignored()] } else { vec![] };
        match other.kind {
            ExpressionKind::Concat(lhs) => {
                self.sequence = [lhs.sequence, ignored, take(&mut self.sequence)].concat();
            }
            _ => {
                self.sequence = [vec![other], ignored, take(&mut self.sequence)].concat();
            }
        }
    }
}

impl Add<Self> for ExpressionNode {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        join(self, other, true)
    }
}

impl BitAnd<Self> for ExpressionNode {
    type Output = Self;

    fn bitand(self, other: Self) -> Self::Output {
        join(self, other, false)
    }
}

fn join(mut lhs: ExpressionNode, mut rhs: ExpressionNode, soft: bool) -> ExpressionNode {
    match (&mut lhs.kind, &mut rhs.kind) {
        (ExpressionKind::Concat(a), _) => {
            a.push(rhs, soft);
            lhs
        }
        (_, ExpressionKind::Concat(b)) => {
            b.append(lhs, soft);
            rhs
        }
        (_, _) => {
            let concat = ConcatExpression::new(lhs, rhs, soft);
            ExpressionNode { kind: ExpressionKind::Concat(Box::new(concat)), tag: "".to_string() }
        }
    }
}
