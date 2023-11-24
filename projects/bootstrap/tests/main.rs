use yggdrasil_shared::{codegen::BuildRust, Failure, FileCache, Success};

#[test]
#[ignore]
fn debug() -> std::io::Result<()> {
    // let input = read_to_string(r#"C:\Users\Dell\CLionProjects\dejavu-engine\projects\dejavu-parser\grammars\dejavu.ygg"#)?;
    // let output = r#"C:\Users\Dell\CLionProjects\dejavu-engine\projects\dejavu-parser\src\dejavu"#;
    // let builder = BuildRust::default();
    // builder.generate(&input, output).unwrap();
    Ok(())
}

#[test]
#[ignore]
fn debug_v() -> std::io::Result<()> {
    let mut cache = FileCache::default();
    let file = cache.load_local(r#"C:\Users\Dell\CLionProjects\valkyrie.rs\projects\valkyrie-parser\grammars\valkyrie.ygg"#)?;
    let output = r#"C:\Users\Dell\CLionProjects\valkyrie.rs\projects\valkyrie-parser\src\codegen"#;
    let builder = BuildRust { range_type: "u32".to_string(), ..Default::default() };
    match builder.generate(file, &mut cache, output) {
        Success { value: _, diagnostics } => {
            for x in diagnostics {
                x.as_report().print(&cache)?
            }
        }
        Failure { fatal, diagnostics } => {
            fatal.as_report().eprint(&cache)?;
            for x in diagnostics {
                x.as_report().print(&cache)?
            }
        }
    };
    Ok(())
}

#[test]
#[ignore]
fn debug_re0() -> std::io::Result<()> {
    // let input = read_to_string(r#"C:\Users\Dell\CLionProjects\re0-script\projects\re0-vm\grammars\re0.ygg"#)?;
    // let output = r#"C:\Users\Dell\CLionProjects\re0-script\projects\re0-vm\src\codegen"#;
    // let builder = BuildRust::default();
    // builder.generate(&input, output).unwrap();
    Ok(())
}

#[test]
#[ignore]
fn debug_wolfram() -> std::io::Result<()> {
    // let input = read_to_string(r#"C:\Users\Dell\IdeaProjects\wolfram-parser\projects\wolfram-parser\grammars\wolfram.ygg"#)?;
    // let output = r#"C:\Users\Dell\IdeaProjects\wolfram-parser\projects\wolfram-parser\src\parser\codegen"#;
    // let builder = BuildRust::default();
    // builder.generate(&input, output).unwrap();
    Ok(())
}
