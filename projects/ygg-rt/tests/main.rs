use crate::json5::{Json5Language, Json5Rule};
use yggdrasil_rt::YggdrasilParser;

mod json5;

#[test]
fn test() {
    let out = Json5Language::parse("[1]", Json5Rule::Array).unwrap();
    println!("{:#?}", out)
}
