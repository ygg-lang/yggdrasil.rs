use std::{fs::File, io::Write, str::FromStr};
use yggdrasil_rt::YggdrasilParser;

#[test]
fn ready() {
    println!("it works!")
}

const BOOTSTRAP: &'static str = include_str!("bootstrap.ygg");

mod standard {
    use yggdrasil_parser::bootstrap::*;

    use super::*;

    #[test]
    fn test_bootstrap() {
        let cst = BootstrapParser::parse_cst(BOOTSTRAP, BootstrapRule::Root).unwrap();
        println!("Short Form:\n{}", cst);
        let ast = RootNode::from_str(include_str!("bootstrap.ygg")).unwrap();
        let mut file = File::create("tests/bootstrap.ron").unwrap();
        file.write_all(format!("{:#?}", ast).as_bytes()).unwrap();
        // file.write_all(out.to_string().as_bytes()).unwrap();
    }

    #[test]
    fn test_classes() {
        let text = r##"text class RegexInner {
    /([^\\\\\\/]|\\\\.)+/
}"##;
        let cst = BootstrapParser::parse_cst(text, BootstrapRule::ClassStatement).unwrap();
        println!("Short Form:\n{}", cst);
        let ast = ClassStatementNode::from_str(text).unwrap();
        println!("{ast:#?}")
    }

    #[test]
    fn test_regex() {
        let text = r##"/([^\\\\\\/]|\\\\.)+/"##;
        let cst = BootstrapParser::parse_cst(text, BootstrapRule::RegexEmbed).unwrap();
        println!("Short Form:\n{}", cst);
        let ast = RegexEmbedNode::from_str(text).unwrap();
        println!("{ast:#?}")
    }

    #[test]
    fn test_atomic() {
        let text = r##"'->'"##;
        let cst = BootstrapParser::parse_cst(text, BootstrapRule::Expression).unwrap();
        println!("Short Form:\n{}", cst);
        let ast = ExpressionNode::from_str(text).unwrap();
        println!("{ast:#?}")
    }
}

mod preview {
    use yggdrasil_parser::antlr::*;

    use super::*;

    #[test]
    fn test_bootstrap() {
        let cst = BootstrapParser::parse_cst(BOOTSTRAP, BootstrapRule::Root).unwrap();
        println!("Short Form:\n{}", cst);
        let ast = RootNode::from_str(include_str!("bootstrap.ygg")).unwrap();
        let mut file = File::create("tests/bootstrap.ron").unwrap();
        file.write_all(format!("{:#?}", ast).as_bytes()).unwrap();
        // file.write_all(out.to_string().as_bytes()).unwrap();
    }

    #[test]
    fn test_classes() {
        let text = r##"class ClassStatement {
    DecoratorCall* ModifierCall* ^KW_CLASS (name:Identifier) ("->" cast:Identifier)? OP_REMARK? ClassBlock
}"##;
        let cst = BootstrapParser::parse_cst(text, BootstrapRule::ClassStatement).unwrap();
        println!("Short Form:\n{}", cst);
        let ast = ClassStatementNode::from_str(text).unwrap();
        println!("{ast:#?}")
    }

    #[test]
    fn test_external() {
        let text = r###"external string {
            rust: crate::utils::helper
        }"###;
        let cst = BootstrapParser::parse_cst(text, BootstrapRule::ExternalStatement).unwrap();
        println!("Short Form:\n{}", cst);
        let ast = ExternalStatementNode::from_str(text).unwrap();
        println!("{ast:#?}")
    }
    #[test]
    fn test_comment() {
        let text = r##"//"##;
        let cst = BootstrapParser::parse_cst(text, BootstrapRule::Comment).unwrap();
        println!("Short Form:\n{}", cst);
        let ast = CommentNode::from_str(text).unwrap();
        println!("{ast:#?}")
    }
    #[test]
    fn test_regex() {
        let text = r##"/    | 增加 | \+=
    | 减 | -
    | 减少 | -=
    | 大于 | >
    | 大于等于 | >=
    | 小于 | <
    | 小于等于 | <=
    | 为 | 等于 | ==
    | 不等于 | !=
    | 有 | has | contains
    | && | 且
    | [|]{2} | 或/"##;
        let cst = BootstrapParser::parse_cst(text, BootstrapRule::RegexEmbed).unwrap();
        println!("Short Form:\n{}", cst);
        let ast = RegexEmbedNode::from_str(text).unwrap();
        println!("{ast:#?}")
    }
    #[test]
    fn test_string() {
        let text = r##"'->'"##;
        let mut cst = BootstrapParser::parse_cst(text, BootstrapRule::Expression).unwrap();
        println!("Short Form:\n{}", cst);
        let root = cst.next().unwrap();
        println!("{:?}", root.get_tag());

        let ast = ExpressionNode::from_str(text).unwrap();
        println!("{ast:#?}")
    }
}
