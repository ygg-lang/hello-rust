use yggdrasil_rt::YggdrasilLanguage;

use build_by_script::{Json5Language, Json5Rule};

#[test]
fn ready() {
    println!("it works!")
}



#[test]
fn test_array() {
    let out = Json5Language::parse_cst("[1, null, ]", Json5Rule::Value).unwrap();
    println!("{:#}", out);
    println!("Short Form:\n{}", out);
}
