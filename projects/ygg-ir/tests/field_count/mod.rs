use yggdrasil_ir::{
    data::RuleReference,
    nodes::{ExpressionBody, UnaryExpression, YggdrasilExpression, YggdrasilOperator},
    rule::YggdrasilIdentifier,
};

#[test]
pub fn counter() {
    let id = YggdrasilIdentifier { text: "a".to_string(), range: Default::default() };
    let base = YggdrasilExpression {
        tag: Some(id.clone()),
        remark: false,
        body: ExpressionBody::Rule(RuleReference { name: id, boxed: false, inline: false }),
    };
    let m = base.field_map();
    println!("{:?}", m);
    let unary = UnaryExpression {
        base: Box::new(base),
        operators: vec![YggdrasilOperator::OPTIONAL, YggdrasilOperator::RepeatsBetween { min: 2, max: 3 }],
    };
    let c = unary.counter();
    println!("{:?}", c);
    let m = unary.field_map();
    println!("{:?}", m);
}
