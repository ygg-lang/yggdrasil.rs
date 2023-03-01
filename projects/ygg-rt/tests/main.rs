use crate::json5::{Json5Language, Json5Rule};
use yggdrasil_rt::YggdrasilLanguage;

mod json5;

#[test]
fn test_array() {
    let out = Json5Language::parse_cst("[1, null, ]", Json5Rule::Value).unwrap();
    println!("{:#}", out);
    println!("Short Form:\n{}", out);
}
