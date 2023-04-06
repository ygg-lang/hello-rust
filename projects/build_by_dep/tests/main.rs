use build_by_script::json5::{Json5Parser, Json5Rule, ValueNode};
use yggdrasil_rt::{YggdrasilError, YggdrasilNode, YggdrasilParser};

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test_array() {
    let cst = Json5Parser::parse_cst("{int: 1, bool: [true, false]}", Json5Rule::Value).unwrap();
    println!("Short Form:\n{}", cst);
    let first = ValueNode::from_cst(cst).unwrap();
    println!("{:#?}", first)
}

#[test]
fn test_array2() {
    let cst = Json5Parser::parse_cst("[true, false, 1, 2, null]", Json5Rule::Value).unwrap();
    println!("Short Form:\n{}", cst);
    let first = ValueNode::from_cst(cst).unwrap();
    println!("{:#?}", first)
}
