use crate::json5::{Json5Language, Json5Rule};
use yggdrasil_rt::YggdrasilParser;

mod json5;

#[test]
fn test_array() {
    let out = Json5Language::parse("{a: [1, null, ]}", Json5Rule::Value).unwrap();
    println!("{:#?}", out);
    println!("{:#}", out);
    println!("Short Form:\n{}", out);
}
