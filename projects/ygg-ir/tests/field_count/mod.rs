use yggdrasil_ir::{
    data::RuleReference,
    nodes::{ExpressionBody, UnaryExpression, YggdrasilExpression, YggdrasilOperator},
    rule::{YggdrasilCounter, YggdrasilIdentifier},
};

#[test]
pub fn counter() {
    let id = YggdrasilIdentifier { text: "a".to_string(), span: Default::default() };
    let base = YggdrasilExpression {
        tag: Some(id.clone()),
        remark: false,
        body: ExpressionBody::Rule(RuleReference { name: id, boxed: false, inline: false }),
    };
    let m = base.field_map();
    println!("{:?}", m);
    let unary = UnaryExpression {
        base: Box::new(base),
        operators: vec![
            YggdrasilOperator::RepeatsBetween(YggdrasilCounter::OPTIONAL),
            YggdrasilOperator::RepeatsBetween(YggdrasilCounter::new(2, 3)),
        ],
    };
    let c = unary.counter();
    println!("{:?}", c);
    let m = unary.field_map();
    println!("{:?}", m);
}

#[test]
pub fn counter32() {
    println!("{:?}", u32::from_str_radix("A", 16));
    println!("{:?}", u32::from_str_radix("AB", 16));
    println!("{:?}", u32::from_str_radix("ABC", 16));
    println!("{:?}", u32::from_str_radix("ABCD", 16));
    println!("{:?}", u32::from_str_radix("ABCDE", 16));
    println!("{:?}", u32::from_str_radix("10FFFF", 16));
}
