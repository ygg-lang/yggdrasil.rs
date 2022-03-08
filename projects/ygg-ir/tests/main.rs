use character_set::DumpAction;

#[test]
fn ready() {
    println!("it works!")
}

fn dumper() -> DumpAction {
    let dump = DumpAction { name: "TEST".to_string(), ..Default::default() };
    return dump;
}
