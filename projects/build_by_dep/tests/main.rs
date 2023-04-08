use build_by_script::json5::{Json5Parser, Json5Rule, ValueNode};
use yggdrasil_rt::{YggdrasilNode, YggdrasilParser};

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test_unicode() {
    let cst = Json5Parser::parse_cst("{int: 1, bool: [true, false]}", Json5Rule::Value).unwrap();
    println!("Short Form:\n{}", cst);
    let first = ValueNode::from_cst(cst).unwrap();
    println!("{:#?}", first)
}

#[test]
fn test_ascii() {
    let cst = Json5Parser::parse_cst("[true, false, 1, 2, null]", Json5Rule::Value).unwrap();
    println!("Short Form:\n{}", cst);
    let first = ValueNode::from_cst(cst).unwrap();
    println!("{:#?}", first)
}
