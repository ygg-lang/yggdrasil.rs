use pest::prec_climber::{Assoc, Operator};
use super::*;
use yggdrasil_shared::records::join_position;

impl Data {
    pub fn position(&self) -> OffsetRange {
        match self {
            Data::SymbolPath(v) => v.range,
            Data::Integer(v) => v.range,
            Data::String(v) => v.range,
            Data::Macro => {
                unimplemented!()
            }
            Data::Regex => {
                unimplemented!()
            }
        }
    }
}

impl Expression {
    pub fn position(&self) -> OffsetRange {
        match self {
            Expression::Data(v) => v.position(),
            Expression::UnarySuffix(v) => v.range,
            Expression::UnaryPrefix(v) => v.range,
            Expression::Concat(v) => v.range,
            Expression::Choice(v) => v.range,
            Expression::Mark(v) => v.range,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ExpressionResolver {
    pub base: Expression,
    pub rest: Vec<ConcatExpressionRest>,
}

#[derive(Clone, Debug)]
pub struct ConcatExpressionRest {
    pub expr: Expression,
    pub position: OffsetRange,
}

impl ExpressionResolver {
    pub fn flat<'a, 'b>(pair: Pair<'a, Rule>, store: &'b mut Vec<Pair<'a, Rule>>) {
        if pair.as_node_tag() == Some("__rec_expr") {
            for pair in pair.into_inner() {
                store.push(pair)
            }
        }
    }


    pub fn build(left: Pair<Rule>, nodes: Vec<Pair<Rule>>, errors: &mut Vec<Error>) -> Result<Self> {
        use pest::prec_climber::PrecClimber;
        // println!("{:#?}",rest_nested);
        let mut rest = vec![left];
        rest.extend(nodes);
        let pc = PrecClimber::new(vec![
            Operator::new(Rule::__aux_expr_choice, Assoc::Left) ,
            Operator::new(Rule::__aux_expr_concat, Assoc::Left) ,
            Operator::new(Rule::__aux_expr_mark, Assoc::Left)
        ]);

        let primary = |lhs: Pair<Rule>| {
            Expression::parse(lhs, errors)
        };
        let infix = |lhs: Result<Expression>, op: Pair<Rule>, rhs: Result<Expression>| {
            println!("{:#?}", lhs);
            println!("{}", op);
            println!("{:#?}", rhs);
            unimplemented!()
        };
        let result = pc.climb(rest.into_iter(), primary, infix).unwrap();
        println!("{:#?}", result);
        //let mut ptr = 0;
        //

        // loop {
        //     println!("ptr: {}, len: {}", ptr, nodes.len());
        //     match nodes.get(ptr) {
        //         Some(pair) => {
        //             if pair.as_node_tag() == Some("__rec_expr") {
        //                 println!(" __rec_expr!");
        //                 let pair = nodes.remove(ptr);
        //                 for inner in pair.into_inner() {
        //                     nodes.push(inner)
        //                 }
        //             }
        //             else {
        //                 ptr +=1;
        //             }
        //
        //         },
        //         None => {break}
        //     }
        // }


        unreachable!()
    }

    // pub fn left_associative(self) -> ConcatExpression {
    //     let mut rest = self.rest.into_iter();
    //     let rhs = rest.next().unwrap();
    //     let position = join_position::<_, Rule>(self.base.position(), rhs.position);
    //     let mut out = ConcatExpression { base: self.base, rest: rhs.expr, position };
    //     for term in rest {
    //         let position = join_position::<_, Rule>(out.position, rhs.position);
    //         out = ConcatExpression {
    //             base: Expression::Concat(Box::new(out)),
    //             rest: term.expr,
    //             position,
    //         }
    //     }
    //     return out;
    // }
    pub fn dyn_associative(self) -> ConcatExpression {
        let start = self.base.position();
        let end = self.rest.last().map(|p| p.position).unwrap_or_default();
        ConcatExpression {
            base: self.base,
            rest: self.rest.into_iter().map(|i| i.expr).collect(),
            range: join_position::<_, Rule>(start, end),
        }
    }
}
