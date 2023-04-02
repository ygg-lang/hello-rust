use build_by_script::json5::{Json5Parser, Json5Rule};
use yggdrasil_rt::YggdrasilParser;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test_array() {
    let cst = Json5Parser::parse_cst("[1, null, ]", Json5Rule::Value).unwrap();
    println!("{:#}", cst);
    println!("Short Form:\n{}", cst);
    for pair in cst.flatten() {
        println!("{:#}", pair);
    }
}
